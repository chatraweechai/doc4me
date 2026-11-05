//! Insert › Screenshot › Screen Clipping (`Services::screen_clip`): the system's own screen
//! clipping, waited for on a background thread, and the clip handed to the UI.
//! - **Windows**: the Snipping Tool's screen clipping (`ms-screenclip:`, what Windows+Shift+S
//!   starts), which puts the clip on the clipboard. The clipboard's sequence number is watched
//!   (cheap: nothing is read until it changes) for up to two minutes until a picture turns up.
//! - **macOS**: `screencapture -i -x` (drag out a part of the screen; no camera sound) into a
//!   private temporary folder. The system asks once for Screen Recording permission.
//! - **Linux and the BSDs**: the desktop's Screenshot portal, interactive (GNOME, KDE, wlroots
//!   desktops, and from inside the Flatpak). Without one, the first of `gnome-screenshot`,
//!   `spectacle`, `xfce4-screenshooter` and `flameshot` found on `PATH`, in region mode, saving
//!   into a private temporary folder.
//!
//! The clip comes from another program: the UI checks it like a pasted picture.

use std::sync::mpsc;

use doc4me_ui_egui::screen_clip::ScreenClip;

/// The `Services::screen_clip` hook.
pub type Hook = Box<dyn Fn() -> Result<mpsc::Receiver<ScreenClip>, String>>;

/// Waits for the clip once the clipping started (on the background thread).
#[cfg_attr(not(any(unix, windows)), allow(dead_code))]
type Finish = Box<dyn FnOnce() -> ScreenClip + Send>;

/// The hook for this platform; `None` where there is no screen clipping.
pub fn hook(ctx: &egui::Context) -> Option<Hook> {
    #[cfg(any(unix, windows))]
    {
        let ctx = ctx.clone();
        Some(Box::new(move || {
            let finish = platform::start()?;
            let (tx, rx) = mpsc::channel();
            let ctx = ctx.clone();
            std::thread::Builder::new()
                .name("screen-clip".into())
                .spawn(move || {
                    // The UI may have stopped waiting; then nobody needs the answer.
                    let _ = tx.send(finish());
                    ctx.request_repaint();
                })
                .map_err(|e| format!("couldn't wait for the clip: {e}"))?;
            Ok(rx)
        }))
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = ctx;
        None
    }
}

/// A clip a tool saved as a file: `Ok(None)` when there is none (the user cancelled).
#[cfg(unix)]
fn read_clip(path: &std::path::Path) -> ScreenClip {
    use doc4me_ui_egui::paste_picture::{ClipboardPicture, MAX_FILE_BYTES};
    match std::fs::metadata(path) {
        Err(_) => Ok(None),
        Ok(m) if !m.is_file() || m.len() == 0 => Ok(None),
        Ok(m) if m.len() > MAX_FILE_BYTES => Err("the clip is larger than 200 MB".into()),
        Ok(_) => std::fs::read(path).map(|b| Some(ClipboardPicture::Encoded(b))).map_err(|e| format!("couldn't read the clip: {e}")),
    }
}

/// A folder only this user can read, removed with everything in it when dropped: the tools save
/// the clip there, so no other user can swap or read it.
#[cfg(unix)]
struct TempDir(std::path::PathBuf);

#[cfg(unix)]
impl TempDir {
    fn new() -> Result<Self, String> {
        use std::os::unix::fs::DirBuilderExt;
        let mut random = [0u8; 8];
        getrandom::fill(&mut random).map_err(|e| e.to_string())?;
        let name: String = random.iter().map(|b| format!("{b:02x}")).collect();
        let dir = std::env::temp_dir().join(format!("doc4me-clip-{name}"));
        // Fails when anything (a planted link included) already has the name.
        std::fs::DirBuilder::new().mode(0o700).create(&dir).map_err(|e| format!("couldn't make a temporary folder: {e}"))?;
        Ok(Self(dir))
    }

    /// The picture the tool saved in the folder, whatever it named it.
    fn clip(&self) -> ScreenClip {
        let Ok(entries) = std::fs::read_dir(&self.0) else { return Ok(None) };
        let mut files: Vec<_> = entries.flatten().map(|e| e.path()).filter(|p| p.is_file()).collect();
        files.sort();
        match files.first() {
            Some(file) => read_clip(file),
            None => Ok(None),
        }
    }
}

#[cfg(unix)]
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The path of a `file://` address (the Screenshot portal's answer), `%XX` escapes decoded.
#[cfg(all(unix, not(target_os = "macos")))]
fn file_uri_path(uri: &str) -> Option<std::path::PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    // `file:///home/…`, or `file://localhost/home/…`.
    let path = rest.strip_prefix("localhost").unwrap_or(rest);
    if !path.starts_with('/') {
        return None;
    }
    let bytes = path.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = std::str::from_utf8(bytes.get(i + 1..i + 3)?).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    use std::os::unix::ffi::OsStringExt;
    Some(std::ffi::OsString::from_vec(out).into())
}

#[cfg(windows)]
mod platform {
    use std::time::{Duration, Instant};

    use super::Finish;

    /// How long the clipboard is watched for the clip.
    const MAX_WAIT: Duration = Duration::from_secs(120);

    pub(super) fn start() -> Result<Finish, String> {
        let before = clipboard_win::raw::seq_num();
        open::that_detached("ms-screenclip:").map_err(|e| format!("couldn't start the Snipping Tool: {e}"))?;
        Ok(Box::new(move || {
            let mut seen = before;
            let deadline = Instant::now() + MAX_WAIT;
            while Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(250));
                let now = clipboard_win::raw::seq_num();
                if now == seen {
                    continue;
                }
                seen = now;
                // Something else may have been copied meanwhile: only a picture ends the wait.
                if let Some(picture) = crate::clipboard_picture() {
                    return Ok(Some(picture));
                }
            }
            Ok(None)
        }))
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use std::process::Command;

    use super::{Finish, TempDir};

    pub(super) fn start() -> Result<Finish, String> {
        let dir = TempDir::new()?;
        let file = dir.0.join("clip.png");
        let mut child = Command::new("/usr/sbin/screencapture")
            .args(["-i", "-x"])
            .arg(&file)
            .spawn()
            .map_err(|e| format!("couldn't start screencapture: {e}"))?;
        Ok(Box::new(move || {
            let _ = child.wait();
            dir.clip()
        }))
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
mod platform {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::process::Command;

    use zbus::blocking::{Connection, Proxy};
    use zbus::zvariant::{OwnedValue, Value};

    use super::{Finish, TempDir, file_uri_path, read_clip};

    /// Region-mode screenshot tools, tried in order, and their arguments before the place to save
    /// to: a file in the private folder, or the folder itself for `xfce4-screenshooter`.
    const TOOLS: &[(&str, &[&str], bool)] = &[
        ("gnome-screenshot", &["--area", "--file"], true),
        ("spectacle", &["--region", "--background", "--nonotify", "--output"], true),
        ("xfce4-screenshooter", &["--region", "--save"], false),
        ("flameshot", &["gui", "--path"], true),
    ];

    pub(super) fn start() -> Result<Finish, String> {
        match portal() {
            Ok(finish) => Ok(finish),
            Err(e) => {
                log::info!("screen clipping: no Screenshot portal ({e}); trying the desktop's screenshot tools");
                tool()
            }
        }
    }

    /// The Screenshot portal, interactive: the desktop lets the user pick the part of the screen.
    fn portal() -> zbus::Result<Finish> {
        let connection = Connection::session()?;
        let mut random = [0u8; 8];
        getrandom::fill(&mut random).map_err(|e| zbus::Error::Failure(e.to_string()))?;
        let token: String = std::iter::once("doc4me".to_string()).chain(random.iter().map(|b| format!("{b:02x}"))).collect();
        // The portal answers on a Request object named after our bus name and the token; it is
        // listened to before asking, so a quick answer isn't missed.
        let sender = connection.unique_name().ok_or_else(|| zbus::Error::Failure("no bus name".into()))?.trim_start_matches(':').replace('.', "_");
        let request_path = format!("/org/freedesktop/portal/desktop/request/{sender}/{token}");
        let request = Proxy::new_owned(connection.clone(), "org.freedesktop.portal.Desktop", request_path, "org.freedesktop.portal.Request")?;
        let responses = request.receive_signal("Response")?;
        let portal =
            Proxy::new(&connection, "org.freedesktop.portal.Desktop", "/org/freedesktop/portal/desktop", "org.freedesktop.portal.Screenshot")?;
        let options: HashMap<&str, Value<'_>> = HashMap::from([("handle_token", Value::from(token.as_str())), ("interactive", Value::from(true))]);
        let _: zbus::zvariant::OwnedObjectPath = portal.call("Screenshot", &("", options))?;
        Ok(Box::new(move || {
            // Kept until the answer: the portal cancels a request whose caller left the bus.
            let _connection = connection;
            let _request = &request;
            let mut responses = responses;
            let Some(message) = responses.next() else { return Ok(None) };
            let (code, results) = message.body().deserialize::<(u32, HashMap<String, OwnedValue>)>().map_err(|e| e.to_string())?;
            match code {
                0 => {
                    let uri = results.get("uri").and_then(|v| <&str>::try_from(v).ok()).ok_or("the desktop's screenshot gave no picture")?;
                    let path = file_uri_path(uri).ok_or_else(|| format!("the desktop's screenshot is somewhere unreadable ({uri})"))?;
                    read_clip(&path)
                }
                // Cancelled by the user.
                1 => Ok(None),
                // The desktop's portal has no working screenshot behind it.
                _ => tool().and_then(|finish| finish()),
            }
        }))
    }

    /// The first region-mode screenshot tool on `PATH`.
    fn tool() -> Result<Finish, String> {
        let found = TOOLS.iter().find_map(|&(name, args, file)| on_path(name).map(|path| (path, args, file)));
        let Some((path, args, file)) = found else {
            return Err("no screenshot tool found (install gnome-screenshot, Spectacle, xfce4-screenshooter or Flameshot)".into());
        };
        let dir = TempDir::new()?;
        let target = if file { dir.0.join("clip.png") } else { dir.0.clone() };
        let mut child = Command::new(&path).args(args).arg(&target).spawn().map_err(|e| format!("couldn't start {}: {e}", path.display()))?;
        Ok(Box::new(move || {
            let _ = child.wait();
            dir.clip()
        }))
    }

    fn on_path(name: &str) -> Option<PathBuf> {
        std::env::split_paths(&std::env::var_os("PATH")?).filter(|d| d.is_absolute()).map(|d| d.join(name)).find(|p| p.is_file())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(unix, not(target_os = "macos")))]
    #[test]
    fn portal_addresses_become_paths() {
        assert_eq!(file_uri_path("file:///home/me/Pictures/Screenshot%20from%202026.png"), Some("/home/me/Pictures/Screenshot from 2026.png".into()));
        assert_eq!(file_uri_path("file://localhost/tmp/a.png"), Some("/tmp/a.png".into()));
        assert_eq!(file_uri_path("file:///tmp/%E0%B8%A0.png"), Some("/tmp/ภ.png".into()));
        assert_eq!(file_uri_path("https://example.com/a.png"), None);
        assert_eq!(file_uri_path("file://server/share/a.png"), None);
        assert_eq!(file_uri_path("file:///tmp/bad%2"), None);
    }

    #[cfg(unix)]
    #[test]
    fn the_private_folder_hands_over_the_clip_and_goes_away() {
        use std::os::unix::fs::PermissionsExt;
        let dir = TempDir::new().unwrap();
        let path = dir.0.clone();
        assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o700);
        assert!(matches!(dir.clip(), Ok(None)), "nothing saved: cancelled");
        std::fs::write(path.join("Screenshot_1.png"), b"\x89PNG....").unwrap();
        assert!(matches!(dir.clip(), Ok(Some(doc4me_ui_egui::paste_picture::ClipboardPicture::Encoded(b))) if b.starts_with(b"\x89PNG")));
        drop(dir);
        assert!(!path.exists(), "removed with the clip");
    }
}
