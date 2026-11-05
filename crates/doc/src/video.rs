//! Insert › Media › Online Video: a picture that stands for a video on the web.
//!
//! Word keeps the video as HTML to show in place of the picture (an `<iframe>` of the video
//! site's player) in the picture's `wp15:webVideoPr` extension ([MS-ODRAWXML] 2.19), with the
//! player's size in pixels, and shows the picture as the video's poster frame. Doc4Me keeps that
//! HTML as it was read, writes it back, and plays the video by opening its address in the
//! browser ([`WebVideo::watch_url`]). The HTML it makes itself is one `<iframe>` built from a
//! checked web address, never HTML pasted by the user.

use serde::{Deserialize, Serialize};

/// Most bytes of embedded HTML kept from a file.
pub const MAX_HTML: usize = 64 << 10;

/// The player size Doc4Me gives a video when the embed code names none (16:9), in pixels.
pub const DEFAULT_SIZE: (u32, u32) = (640, 360);

/// An online video ([MS-ODRAWXML] `CT_WebVideoPr`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebVideo {
    /// The HTML that shows the video (`embeddedHtml`).
    pub html: String,
    /// The player's size, pixels.
    pub w: u32,
    pub h: u32,
}

impl WebVideo {
    /// A video from what the user typed in Insert Video: a web address (`https://…`, YouTube and
    /// Vimeo pages become their players) or an `<iframe>` embed code, from which only the player's
    /// address and size are taken. An error says what's wrong with it.
    pub fn from_input(input: &str) -> Result<WebVideo, String> {
        let input = input.trim();
        if input.is_empty() {
            return Err("enter the address or embed code of an online video".into());
        }
        let (src, size) = if input.get(..7).is_some_and(|s| s.eq_ignore_ascii_case("<iframe")) {
            let src = attr(input, "src").ok_or("the embed code has no src address")?;
            let num = |name| attr(input, name).and_then(|v| v.trim().trim_end_matches("px").parse::<u32>().ok()).filter(|v| (16..=4096).contains(v));
            let size = match (num("width"), num("height")) {
                (Some(w), Some(h)) => (w, h),
                _ => DEFAULT_SIZE,
            };
            (src, size)
        } else {
            (input, DEFAULT_SIZE)
        };
        let src = player_url(src).ok_or("only web (https) addresses of online videos can be inserted")?;
        let html = format!(
            r#"<iframe width="{}" height="{}" src="{}" frameborder="0" allow="autoplay; encrypted-media; picture-in-picture" allowfullscreen></iframe>"#,
            size.0,
            size.1,
            escape_attr(&src)
        );
        Ok(WebVideo { html, w: size.0, h: size.1 })
    }

    /// The address of the player in the HTML (its first `src`), if it is a web address.
    pub fn src(&self) -> Option<String> {
        let src = attr(&self.html, "src")?;
        let src = unescape_attr(src.trim());
        let src = if src.starts_with("//") { format!("https:{src}") } else { src };
        is_web(&src).then_some(src)
    }

    /// Where to watch the video in a browser: the video site's own page for a YouTube or Vimeo
    /// player, else the player's address.
    pub fn watch_url(&self) -> Option<String> {
        let src = self.src()?;
        let (host, path) = host_path(&src)?;
        if is_youtube(host)
            && let Some(id) = path.strip_prefix("/embed/").map(|p| p.split(['?', '/', '#']).next().unwrap_or("")).filter(|id| video_id(id))
        {
            return Some(format!("https://www.youtube.com/watch?v={id}"));
        }
        if host.eq_ignore_ascii_case("player.vimeo.com")
            && let Some(id) = path.strip_prefix("/video/").map(|p| p.split(['?', '/', '#']).next().unwrap_or("")).filter(|id| digits(id))
        {
            return Some(format!("https://vimeo.com/{id}"));
        }
        Some(src)
    }
}

fn is_web(url: &str) -> bool {
    let lower = url.get(..8).map(str::to_ascii_lowercase).unwrap_or_default();
    (lower.starts_with("https://") || lower.starts_with("http://"))
        && url.len() <= 2048
        && !url.chars().any(|c| c.is_whitespace() || c.is_control() || matches!(c, '"' | '<' | '>' | '\\'))
}

/// The host and the rest (path, query) of a web address.
fn host_path(url: &str) -> Option<(&str, &str)> {
    let rest = url.split_once("://")?.1;
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let host = rest[..end].rsplit('@').next()?.split(':').next()?;
    Some((host, &rest[end..]))
}

fn is_youtube(host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    matches!(host.as_str(), "youtube.com" | "www.youtube.com" | "m.youtube.com" | "youtube-nocookie.com" | "www.youtube-nocookie.com")
}

fn video_id(id: &str) -> bool {
    (6..=20).contains(&id.len()) && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn digits(id: &str) -> bool {
    (1..=20).contains(&id.len()) && id.bytes().all(|b| b.is_ascii_digit())
}

/// The player address for a video's web address: YouTube and Vimeo pages become their embedded
/// players; other `https` addresses are taken as players already. `None` for anything but a web
/// address.
fn player_url(url: &str) -> Option<String> {
    let url = unescape_attr(url.trim());
    let url = if url.starts_with("//") { format!("https:{url}") } else { url };
    if !is_web(&url) {
        return None;
    }
    let (host, path) = host_path(&url)?;
    let lower = host.to_ascii_lowercase();
    let first = |p: &str| p.split(['?', '/', '#', '&']).next().unwrap_or("").to_string();
    let youtube = if is_youtube(&lower) {
        if let Some(q) = path.strip_prefix("/watch?").or_else(|| path.strip_prefix("/watch/?")) {
            q.split('&').find_map(|kv| kv.strip_prefix("v=")).map(first)
        } else {
            ["/embed/", "/shorts/", "/live/", "/v/"].iter().find_map(|p| path.strip_prefix(p)).map(first)
        }
    } else if lower == "youtu.be" {
        path.strip_prefix('/').map(first)
    } else {
        None
    };
    if let Some(id) = youtube.filter(|id| video_id(id)) {
        return Some(format!("https://www.youtube.com/embed/{id}"));
    }
    if matches!(lower.as_str(), "vimeo.com" | "www.vimeo.com")
        && let Some(id) = path.strip_prefix('/').map(first).filter(|id| digits(id))
    {
        return Some(format!("https://player.vimeo.com/video/{id}"));
    }
    // An unknown site: its address is the player, but only over https.
    url.get(..8).is_some_and(|s| s.eq_ignore_ascii_case("https://")).then_some(url)
}

/// The value of the first `name="…"` (or `'…'`, or bare) attribute in an HTML tag soup.
fn attr<'a>(html: &'a str, name: &str) -> Option<&'a str> {
    let lower = html.to_ascii_lowercase();
    let mut from = 0;
    while let Some(i) = lower.get(from..)?.find(name) {
        let at = from + i;
        from = at + name.len();
        // A whole attribute name: preceded by space, followed by `=`.
        if !lower[..at].ends_with(|c: char| c.is_whitespace()) {
            continue;
        }
        let rest = html[from..].trim_start();
        let Some(rest) = rest.strip_prefix('=') else { continue };
        let rest = rest.trim_start();
        return Some(match rest.chars().next()? {
            q @ ('"' | '\'') => {
                let body = &rest[1..];
                &body[..body.find(q)?]
            }
            _ => &rest[..rest.find(|c: char| c.is_whitespace() || c == '>').unwrap_or(rest.len())],
        });
    }
    None
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Undo the HTML escapes an attribute value commonly carries (`&amp;` in a query string).
fn unescape_attr(s: &str) -> String {
    s.replace("&quot;", "\"").replace("&#39;", "'").replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_pages_become_players() {
        for (input, src) in [
            ("https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=42", "https://www.youtube.com/embed/dQw4w9WgXcQ"),
            ("https://youtu.be/dQw4w9WgXcQ?si=abc", "https://www.youtube.com/embed/dQw4w9WgXcQ"),
            ("https://m.youtube.com/shorts/dQw4w9WgXcQ", "https://www.youtube.com/embed/dQw4w9WgXcQ"),
            ("  https://vimeo.com/76979871  ", "https://player.vimeo.com/video/76979871"),
            ("https://example.com/player?id=1&x=2", "https://example.com/player?id=1&x=2"),
        ] {
            let v = WebVideo::from_input(input).unwrap();
            assert_eq!(v.src().as_deref(), Some(src), "{input}");
            assert_eq!((v.w, v.h), DEFAULT_SIZE);
            assert!(v.html.starts_with("<iframe ") && v.html.ends_with("</iframe>"), "{}", v.html);
        }
        let v = WebVideo::from_input("https://example.com/player?id=1&x=2").unwrap();
        assert!(v.html.contains("id=1&amp;x=2"), "escaped in the HTML: {}", v.html);
        assert_eq!(
            WebVideo::from_input("https://www.youtube.com/watch?v=dQw4w9WgXcQ").unwrap().watch_url().as_deref(),
            Some("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
        );
        assert_eq!(WebVideo::from_input("https://vimeo.com/76979871").unwrap().watch_url().as_deref(), Some("https://vimeo.com/76979871"));
    }

    #[test]
    fn embed_code_gives_only_its_address_and_size() {
        let code = r#"<iframe width="560" height="315" src="https://www.youtube.com/embed/dQw4w9WgXcQ?si=x" title="YouTube video player" onload="alert(1)"></iframe><script>alert(2)</script>"#;
        let v = WebVideo::from_input(code).unwrap();
        assert_eq!((v.w, v.h), (560, 315));
        assert_eq!(v.src().as_deref(), Some("https://www.youtube.com/embed/dQw4w9WgXcQ"));
        assert!(!v.html.contains("alert") && !v.html.contains("script") && !v.html.contains("onload"), "{}", v.html);
        // Sizes out of range fall back to 16:9.
        let v = WebVideo::from_input(r#"<IFRAME width=99999 height='2' src='//player.vimeo.com/video/1'>"#).unwrap();
        assert_eq!((v.w, v.h), DEFAULT_SIZE);
        assert_eq!(v.src().as_deref(), Some("https://player.vimeo.com/video/1"));
    }

    #[test]
    fn only_web_addresses_are_videos() {
        for bad in [
            "",
            "javascript:alert(1)",
            "file:///etc/passwd",
            "http://example.com/video",
            r#"<iframe src="javascript:alert(1)"></iframe>"#,
            r#"<iframe width="1"></iframe>"#,
            "https://exa mple.com/",
            "https://example.com/\"onload=\"x",
        ] {
            assert!(WebVideo::from_input(bad).is_err(), "{bad:?}");
        }
        // HTML read from a file plays only from a web address.
        let v = WebVideo { html: r#"<iframe src="javascript:alert(1)">"#.into(), w: 1, h: 1 };
        assert_eq!(v.watch_url(), None);
        let v = WebVideo { html: r#"<iframe data-x="1" src="http://example.com/v?a=1&amp;b=2">"#.into(), w: 1, h: 1 };
        assert_eq!(v.watch_url().as_deref(), Some("http://example.com/v?a=1&b=2"));
    }
}
