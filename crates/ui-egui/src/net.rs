//! Requests to free web services: Insert › Online Pictures (Openverse), Review › Translate
//! (MyMemory or a LibreTranslate server), Insert › Get Add-ins (the add-in catalog in Doc4Me's
//! repository) and Dictate's speech model. The host makes them ([`crate::Services::fetch`]: the
//! desktop app through the system's `curl`, the web app through the browser) on its own thread
//! and answers on a channel, waking the UI. Only `https` addresses are asked for, every answer is
//! capped in size, and what comes back is checked by whoever asked for it.

use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{Receiver, TryRecvError};

/// A request to a web service.
pub struct FetchRequest {
    /// An `https` address.
    pub url: String,
    /// POST this body with this content type; without it, GET.
    pub body: Option<(String, Vec<u8>)>,
    /// The most bytes taken: a longer answer is an error.
    pub max_bytes: u64,
    /// Desktop: write the answer to this file instead of handing it over (large downloads);
    /// the answer is then empty. Unset on the web.
    pub to_file: Option<std::path::PathBuf>,
    /// Bytes received so far, for a progress bar.
    pub progress: Option<Arc<AtomicU64>>,
}

impl FetchRequest {
    /// GET `url`, at most `max_bytes`.
    pub fn get(url: impl Into<String>, max_bytes: u64) -> FetchRequest {
        FetchRequest { url: url.into(), body: None, max_bytes, to_file: None, progress: None }
    }
}

/// The host's answer: the body of a successful (2xx) answer, or why there is none.
pub type FetchAnswer = Result<Vec<u8>, String>;

/// Whether `url` may be requested: `https` only, no spaces or control characters, a host.
pub fn allowed(url: &str) -> bool {
    let Some(rest) = url.get(..8).filter(|s| s.eq_ignore_ascii_case("https://")).map(|_| &url[8..]) else { return false };
    url.len() <= 8192 && !rest.is_empty() && !rest.starts_with(['/', '?', '#']) && !url.chars().any(|c| c.is_whitespace() || c.is_control())
}

/// Send `req` through the host. An error when there is no web access here or the address
/// isn't allowed.
pub(crate) fn start(app: &crate::WordApp, req: FetchRequest) -> Result<Receiver<FetchAnswer>, String> {
    if !allowed(&req.url) {
        return Err(tl!("only https addresses can be used").to_string());
    }
    let fetch = app.services.fetch.as_ref().ok_or_else(|| tl!("web access isn't available here").to_string())?;
    Ok(fetch(req))
}

/// A request in flight, polled once a frame.
pub(crate) struct Pending(Receiver<FetchAnswer>);

impl Pending {
    pub(crate) fn new(rx: Receiver<FetchAnswer>) -> Pending {
        Pending(rx)
    }

    /// The answer once it has come (`None` while waiting); a host that dropped the request
    /// without answering counts as an error.
    pub(crate) fn poll(&self) -> Option<FetchAnswer> {
        match self.0.try_recv() {
            Ok(answer) => Some(answer),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Err("the request was dropped".into())),
        }
    }
}

/// `s` escaped for a URL query value (RFC 3986 unreserved characters kept).
pub fn query_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_https_addresses_are_requested() {
        assert!(allowed("https://api.openverse.org/v1/images/?q=cat"));
        assert!(allowed("HTTPS://example.com"));
        for bad in
            ["http://example.com", "file:///etc/passwd", "https://", "https:///x", "https://a b", "https://a\nb", "ftp://x", "javascript:alert(1)"]
        {
            assert!(!allowed(bad), "{bad}");
        }
        assert_eq!(query_escape("แมว & dog/1"), "%E0%B9%81%E0%B8%A1%E0%B8%A7%20%26%20dog%2F1");
    }

    #[test]
    fn no_host_service_means_no_request() {
        let app = crate::WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), crate::Services::default());
        assert!(start(&app, FetchRequest::get("https://example.com", 10)).is_err());
        let services = crate::Services { fetch: Some(Box::new(|_| std::sync::mpsc::channel().1)), ..Default::default() };
        let app = crate::WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), services);
        assert!(start(&app, FetchRequest::get("http://example.com", 10)).is_err(), "not https");
        let rx = start(&app, FetchRequest::get("https://example.com", 10)).unwrap();
        assert!(Pending::new(rx).poll().is_some_and(|a| a.is_err()), "dropped by the host");
    }
}
