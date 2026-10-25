//! Web requests for the online features (`Services::fetch`) through the browser's `fetch`. The
//! services Doc4Me uses (Openverse, MyMemory, raw.githubusercontent.com) allow it from any page
//! (CORS). Answers are capped in size like on the desktop.

use std::sync::atomic::Ordering;
use std::sync::mpsc;

use doc4me_ui_egui::net::{FetchAnswer, FetchRequest};
use wasm_bindgen::JsCast as _;
use wasm_bindgen_futures::JsFuture;

pub fn hook(ctx: egui::Context) -> Box<dyn Fn(FetchRequest) -> mpsc::Receiver<FetchAnswer>> {
    Box::new(move |req| {
        let (tx, rx) = mpsc::channel();
        let ctx = ctx.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = tx.send(fetch(&req).await);
            ctx.request_repaint();
        });
        rx
    })
}

async fn fetch(req: &FetchRequest) -> FetchAnswer {
    let js = |e: wasm_bindgen::JsValue| e.as_string().unwrap_or_else(|| format!("{e:?}"));
    if !doc4me_ui_egui::net::allowed(&req.url) {
        return Err("only https addresses can be used".into());
    }
    let init = web_sys::RequestInit::new();
    init.set_mode(web_sys::RequestMode::Cors);
    let headers = web_sys::Headers::new().map_err(js)?;
    if let Some((content_type, body)) = &req.body {
        init.set_method("POST");
        headers.set("Content-Type", content_type).map_err(js)?;
        init.set_body(&js_sys::Uint8Array::from(body.as_slice()).into());
    } else {
        init.set_method("GET");
    }
    init.set_headers(&headers);
    let request = web_sys::Request::new_with_str_and_init(&req.url, &init).map_err(js)?;
    let window = web_sys::window().ok_or("no browser window")?;
    let response: web_sys::Response = JsFuture::from(window.fetch_with_request(&request)).await.map_err(js)?.dyn_into().map_err(js)?;
    if !response.ok() {
        return Err(format!("the service answered {} {}", response.status(), response.status_text()));
    }
    let buffer = JsFuture::from(response.array_buffer().map_err(js)?).await.map_err(js)?;
    let bytes = js_sys::Uint8Array::new(&buffer);
    if u64::from(bytes.length()) > req.max_bytes {
        return Err("the answer is larger than expected".into());
    }
    if let Some(p) = &req.progress {
        p.store(u64::from(bytes.length()), Ordering::Relaxed);
    }
    Ok(bytes.to_vec())
}
