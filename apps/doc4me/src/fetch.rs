//! Web requests for the online features (`Services::fetch`) through the system's `curl`: part of
//! Windows 10 (1803) and later (`System32\curl.exe`), macOS (`/usr/bin/curl`) and nearly every
//! Linux and BSD desktop. Using it adds no TLS stack to Doc4Me, and curl follows the system's
//! certificates and the `HTTPS_PROXY` setting. Requests are `https` only (redirects too), time
//! out, and their answers are capped in size while they are read.

use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::Ordering;
use std::sync::mpsc;

use doc4me_ui_egui::net::{FetchAnswer, FetchRequest};

/// The `Services::fetch` hook.
pub type Hook = Box<dyn Fn(FetchRequest) -> mpsc::Receiver<FetchAnswer>>;

/// Longest a request may take, seconds (a speech model download is the largest).
const MAX_SECONDS: &str = "1800";

pub fn hook(ctx: &egui::Context) -> Hook {
    let ctx = ctx.clone();
    Box::new(move |req| {
        let (tx, rx) = mpsc::channel();
        let ctx = ctx.clone();
        let spawned = std::thread::Builder::new().name("fetch".into()).spawn(move || {
            let _ = tx.send(fetch(&req));
            ctx.request_repaint();
        });
        if let Err(e) = spawned {
            log::warn!("fetch: {e}");
        }
        rx
    })
}

/// curl on this system, by absolute path.
fn curl() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let root = std::env::var_os("SystemRoot").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
        Some(root.join("System32").join("curl.exe")).filter(|p| p.is_file())
    }
    #[cfg(target_os = "macos")]
    {
        Some(PathBuf::from("/usr/bin/curl")).filter(|p| p.is_file())
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        std::env::split_paths(&std::env::var_os("PATH")?).filter(|d| d.is_absolute()).map(|d| d.join("curl")).find(|p| p.is_file())
    }
}

/// The arguments for `req` (the address comes last, after `--`, so it can't be read as an option).
fn args(req: &FetchRequest) -> Vec<String> {
    let mut a: Vec<String> = [
        "--silent",
        "--show-error",
        "--fail",
        "--location",
        "--max-redirs",
        "5",
        "--proto",
        "=https",
        "--proto-redir",
        "=https",
        "--connect-timeout",
        "20",
        "--max-time",
        MAX_SECONDS,
    ]
    .map(str::to_string)
    .to_vec();
    a.extend(["--max-filesize".into(), req.max_bytes.to_string()]);
    a.extend(["--user-agent".into(), format!("Doc4Me/{} (+https://github.com/chatraweechai/doc4me)", env!("CARGO_PKG_VERSION"))]);
    if let Some((content_type, _)) = &req.body {
        a.extend(["--header".into(), format!("Content-Type: {}", content_type.replace(['\r', '\n'], ""))]);
        a.extend(["--data-binary".into(), "@-".into()]);
    }
    a.extend(["--".into(), req.url.clone()]);
    a
}

pub(crate) fn fetch(req: &FetchRequest) -> FetchAnswer {
    if !doc4me_ui_egui::net::allowed(&req.url) {
        return Err("only https addresses can be used".into());
    }
    let curl = curl().ok_or("the system's curl isn't installed (it is needed for online features)")?;
    let mut cmd = Command::new(&curl);
    cmd.args(args(req)).stdin(if req.body.is_some() { Stdio::piped() } else { Stdio::null() }).stdout(Stdio::piped()).stderr(Stdio::piped());
    #[cfg(target_os = "windows")]
    {
        // No console window flashing up for curl.
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    let mut child = cmd.spawn().map_err(|e| format!("couldn't start curl: {e}"))?;
    if let (Some((_, body)), Some(mut stdin)) = (&req.body, child.stdin.take()) {
        // A small body: written before reading the answer.
        let _ = stdin.write_all(body);
    }
    let mut stdout = child.stdout.take().ok_or("no output from curl")?;
    let mut file = match &req.to_file {
        Some(path) => Some(std::fs::File::create(path).map_err(|e| format!("couldn't write {}: {e}", path.display()))?),
        None => None,
    };
    let mut body = Vec::new();
    let mut total = 0u64;
    let mut buf = vec![0u8; 64 << 10];
    let result = loop {
        let n = match stdout.read(&mut buf) {
            Ok(0) => break Ok(()),
            Ok(n) => n,
            Err(e) => break Err(format!("couldn't read the answer: {e}")),
        };
        total += n as u64;
        if total > req.max_bytes {
            break Err("the answer is larger than expected".into());
        }
        if let Some(p) = &req.progress {
            p.store(total, Ordering::Relaxed);
        }
        let chunk = &buf[..n];
        match &mut file {
            Some(f) => {
                if let Err(e) = f.write_all(chunk) {
                    break Err(format!("couldn't write the download: {e}"));
                }
            }
            None => body.extend_from_slice(chunk),
        }
    };
    if result.is_err() {
        let _ = child.kill();
    }
    let mut err = String::new();
    if let Some(mut e) = child.stderr.take() {
        let _ = e.by_ref().take(4096).read_to_string(&mut err);
    }
    let status = child.wait().map_err(|e| format!("curl: {e}"))?;
    result?;
    if !status.success() {
        let err = err.trim();
        return Err(if err.is_empty() { format!("the request failed ({status})") } else { err.chars().take(300).collect() });
    }
    if let Some(f) = file {
        f.sync_all().map_err(|e| format!("couldn't write the download: {e}"))?;
    }
    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curl_is_asked_for_https_only_with_the_address_last() {
        let mut req = FetchRequest::get("https://api.openverse.org/v1/images/?q=-o", 1000);
        req.body = Some(("application/json\r\nX-Evil: 1".into(), b"{}".to_vec()));
        let a = args(&req);
        assert_eq!(a.last().map(String::as_str), Some("https://api.openverse.org/v1/images/?q=-o"));
        assert_eq!(a[a.len() - 2], "--");
        let at = |k: &str| a.iter().position(|x| x == k).map(|i| a[i + 1].as_str());
        assert_eq!(at("--proto"), Some("=https"));
        assert_eq!(at("--proto-redir"), Some("=https"));
        assert_eq!(at("--max-filesize"), Some("1000"));
        assert_eq!(at("--header"), Some("Content-Type: application/jsonX-Evil: 1"), "no header injection");
        assert!(fetch(&FetchRequest::get("http://example.com", 10)).is_err());
    }

    /// Against the real services, only with `DOC4ME_NET_TESTS=1` (CI has no network for them).
    #[test]
    fn real_requests_when_asked() {
        if std::env::var("DOC4ME_NET_TESTS").as_deref() != Ok("1") {
            return;
        }
        let json = fetch(&FetchRequest::get("https://api.openverse.org/v1/images/?q=cat&page_size=2", 1 << 20)).unwrap();
        assert!(doc4me_ui_egui::online_pictures::parse_results(&json).is_ok_and(|h| !h.is_empty()));
        assert!(fetch(&FetchRequest::get("https://api.openverse.org/v1/images/?q=cat&page_size=20", 100)).is_err(), "capped");
        let mut post = FetchRequest::get("https://httpbin.org/post", 1 << 20);
        post.body = Some(("application/json".into(), br#"{"a":1}"#.to_vec()));
        assert!(fetch(&post).is_ok_and(|b| String::from_utf8_lossy(&b).contains(r#"\"a\":1"#)));
    }
}
