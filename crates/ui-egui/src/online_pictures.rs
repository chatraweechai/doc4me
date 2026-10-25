//! Insert › Illustrations › Pictures › Online Pictures.
//!
//! Word searches Bing; Doc4Me searches [Openverse](https://openverse.org), WordPress's free and
//! open search of openly licensed pictures (Creative Commons and public domain: Flickr, Wikimedia
//! Commons, museums…), which needs no account or key. The picked picture is downloaded through
//! [`crate::net`] and inserted with `insert.onlinePicture`, which adds the credit line its
//! licence asks for. Everything that comes back is from the network: the JSON is read field by
//! field, pictures are decoded with size limits, and links are web addresses only.

use egui::{Color32, Sense, vec2};
use serde_json::{Value, json};

use crate::WordApp;
use crate::net::{FetchRequest, Pending};
use crate::theme::Tokens;

/// The Openverse image search.
const SEARCH: &str = "https://api.openverse.org/v1/images/";
/// Pictures per search.
const PAGE: usize = 20;
/// Largest search answer, picture and thumbnail taken.
const MAX_JSON: u64 = 2 << 20;
const MAX_PICTURE: u64 = 25 << 20;
const MAX_THUMB: u64 = 4 << 20;
/// Longest side of a thumbnail as shown, and of any picture decoded for one.
const THUMB_PX: u32 = 160;
const MAX_DECODE_SIDE: u32 = 8000;

/// One search result.
pub struct Hit {
    pub title: String,
    pub creator: String,
    pub license: String,
    pub license_version: String,
    pub license_url: String,
    /// The picture's page (where it comes from).
    pub source: String,
    /// The picture itself, and Openverse's smaller copy of it.
    pub url: String,
    pub thumbnail: String,
    thumb: Thumb,
}

enum Thumb {
    Waiting(Pending),
    Ready(egui::TextureHandle),
    Failed,
}

/// The Online Pictures window.
#[derive(Default)]
pub struct Browser {
    pub query: String,
    /// Only pictures that may be used commercially (Openverse's `license_type=commercial`).
    pub commercial: bool,
    search: Option<Pending>,
    pub results: Vec<Hit>,
    pub selected: Option<usize>,
    /// The picture being downloaded, and whether the thumbnail is the fallback being tried.
    downloading: Option<(usize, Pending, bool)>,
    pub message: Option<String>,
    searched: bool,
}

/// The search address for `query`.
pub fn search_url(query: &str, commercial: bool) -> String {
    let mut url = format!("{SEARCH}?q={}&page_size={PAGE}&mature=false", crate::net::query_escape(query.trim()));
    if commercial {
        url.push_str("&license_type=commercial");
    }
    url
}

/// The results in an Openverse search answer: pictures with a web address, their text capped.
pub fn parse_results(json: &[u8]) -> Result<Vec<Hit>, String> {
    let v: Value = serde_json::from_slice(json).map_err(|_| "the search service gave an answer Doc4Me can't read".to_string())?;
    let results = v.get("results").and_then(Value::as_array).ok_or("the search service gave no results")?;
    let text = |r: &Value, k: &str| r.get(k).and_then(Value::as_str).unwrap_or("").chars().take(300).collect::<String>();
    let web = |r: &Value, k: &str| {
        let u = text(r, k);
        if crate::net::allowed(&u) { u } else { String::new() }
    };
    Ok(results
        .iter()
        .take(PAGE * 2)
        .filter_map(|r| {
            let url = web(r, "url");
            let thumbnail = web(r, "thumbnail");
            (!url.is_empty() || !thumbnail.is_empty()).then(|| Hit {
                title: text(r, "title"),
                creator: text(r, "creator"),
                license: text(r, "license"),
                license_version: text(r, "license_version"),
                license_url: web(r, "license_url"),
                source: web(r, "foreign_landing_url"),
                url,
                thumbnail,
                thumb: Thumb::Failed,
            })
        })
        .collect())
}

/// Open the window (`insert.onlinePicture` without a picture).
pub(crate) fn open(app: &mut WordApp) {
    if app.online_pictures.is_none() {
        app.online_pictures = Some(Browser::default());
    }
}

fn start_search(app: &mut WordApp) {
    let Some(b) = app.online_pictures.as_ref() else { return };
    if b.query.trim().is_empty() {
        return;
    }
    let url = search_url(&b.query, b.commercial);
    let started = crate::net::start(app, FetchRequest::get(url, MAX_JSON));
    let Some(b) = app.online_pictures.as_mut() else { return };
    b.results.clear();
    b.selected = None;
    b.searched = true;
    match started {
        Ok(rx) => {
            b.search = Some(Pending::new(rx));
            b.message = Some(tl!("Searching…").to_string());
        }
        Err(e) => b.message = Some(crate::i18n::fmt(tl!("Couldn't search: {reason}"), &[("reason", &e)])),
    }
}

fn start_download(app: &mut WordApp, i: usize, thumbnail: bool) {
    let Some(hit) = app.online_pictures.as_ref().and_then(|b| b.results.get(i)) else { return };
    let (url, max) = if thumbnail || hit.url.is_empty() { (hit.thumbnail.clone(), MAX_THUMB) } else { (hit.url.clone(), MAX_PICTURE) };
    let thumbnail = thumbnail || hit.url.is_empty();
    let started = crate::net::start(app, FetchRequest::get(url, max));
    let Some(b) = app.online_pictures.as_mut() else { return };
    match started {
        Ok(rx) => {
            b.downloading = Some((i, Pending::new(rx), thumbnail));
            b.message = Some(tl!("Downloading…").to_string());
        }
        Err(e) => b.message = Some(crate::i18n::fmt(tl!("Couldn't download the picture: {reason}"), &[("reason", &e)])),
    }
}

/// A thumbnail texture from picture bytes, or `None` when they aren't a picture we can show.
fn thumb_texture(ctx: &egui::Context, key: &str, bytes: &[u8]) -> Option<egui::TextureHandle> {
    let (w, h) = doc4me_render::image_size(bytes)?;
    if w == 0 || h == 0 || w > MAX_DECODE_SIDE || h > MAX_DECODE_SIDE {
        return None;
    }
    let img = image::load_from_memory(bytes).ok()?.thumbnail(THUMB_PX, THUMB_PX).to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    Some(ctx.load_texture(key, egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw()), egui::TextureOptions::LINEAR))
}

/// Follow the requests in flight (every frame while the window is open).
pub(crate) fn poll(app: &mut WordApp, ctx: &egui::Context) {
    let Some(b) = app.online_pictures.as_mut() else { return };
    let mut busy = false;
    if let Some(p) = &b.search {
        busy = true;
        if let Some(answer) = p.poll() {
            b.search = None;
            match answer.and_then(|json| parse_results(&json)) {
                Ok(hits) => {
                    b.message = hits.is_empty().then(|| tl!("No pictures found.").to_string());
                    b.results = hits;
                }
                Err(e) => b.message = Some(crate::i18n::fmt(tl!("Couldn't search: {reason}"), &[("reason", &e)])),
            }
            // Ask for the thumbnails.
            let urls: Vec<String> = b.results.iter().map(|h| h.thumbnail.clone()).collect();
            for (i, url) in urls.into_iter().enumerate() {
                let thumb = match crate::net::start(app, FetchRequest::get(url, MAX_THUMB)) {
                    Ok(rx) => Thumb::Waiting(Pending::new(rx)),
                    Err(_) => Thumb::Failed,
                };
                if let Some(h) = app.online_pictures.as_mut().and_then(|b| b.results.get_mut(i)) {
                    h.thumb = thumb;
                }
            }
        }
    }
    let Some(b) = app.online_pictures.as_mut() else { return };
    for (i, hit) in b.results.iter_mut().enumerate() {
        if let Thumb::Waiting(p) = &hit.thumb {
            busy = true;
            if let Some(answer) = p.poll() {
                hit.thumb = answer
                    .ok()
                    .and_then(|bytes| thumb_texture(ctx, &format!("online-picture-{i}-{}", hit.thumbnail), &bytes))
                    .map_or(Thumb::Failed, Thumb::Ready);
            }
        }
    }
    let done = match &b.downloading {
        Some((i, p, thumbnail)) => {
            busy = true;
            p.poll().map(|answer| (*i, answer, *thumbnail))
        }
        None => None,
    };
    if let Some((i, answer, thumbnail)) = done {
        b.downloading = None;
        let picture = answer.ok().filter(|bytes| doc4me_render::image_size(bytes).is_some());
        match picture {
            Some(bytes) => insert(app, i, &bytes),
            // The original may be gone or too large: Openverse's own copy is next.
            None if !thumbnail => start_download(app, i, true),
            None => {
                if let Some(b) = app.online_pictures.as_mut() {
                    b.message = Some(crate::i18n::fmt(
                        tl!("Couldn't download the picture: {reason}"),
                        &[("reason", tl!("it isn't a picture Doc4Me can read"))],
                    ));
                }
            }
        }
    }
    if busy {
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

fn insert(app: &mut WordApp, i: usize, bytes: &[u8]) {
    let Some(hit) = app.online_pictures.as_ref().and_then(|b| b.results.get(i)) else { return };
    let params = json!({
        "data": doc4me_engine::cmd::insert::base64_encode(bytes),
        "title": hit.title,
        "creator": hit.creator,
        "license": hit.license,
        "licenseVersion": hit.license_version,
        "licenseUrl": hit.license_url,
        "source": hit.source,
    });
    match app.run("insert.onlinePicture", params) {
        Ok(_) => app.online_pictures = None,
        Err(e) => {
            if let Some(b) = app.online_pictures.as_mut() {
                b.message = Some(crate::i18n::fmt(tl!("Couldn't download the picture: {reason}"), &[("reason", &e)]));
            }
        }
    }
}

/// The window.
pub(crate) fn show(app: &mut WordApp, ctx: &egui::Context) {
    if app.online_pictures.is_none() {
        return;
    }
    let t = Tokens::get(ctx);
    let mut open = true;
    let mut search = false;
    let mut insert_at = None;
    let mut cancel = false;
    egui::Window::new(tl!("Online Pictures"))
        .id(egui::Id::new("online-pictures"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(700.0)
        .anchor(egui::Align2::CENTER_CENTER, vec2(0.0, 0.0))
        .show(ctx, |ui| {
            let Some(b) = app.online_pictures.as_mut() else { return };
            ui.set_width(680.0);
            ui.horizontal(|ui| {
                let edit = ui
                    .add(crate::widgets::text_line(&mut b.query).desired_width(470.0).hint_text(tl!("Search Openverse (Creative Commons pictures)")));
                if !b.searched && b.search.is_none() && b.query.is_empty() {
                    edit.request_focus();
                }
                if edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    search = true;
                }
                if ui.add_enabled(!b.query.trim().is_empty(), crate::widgets::primary_button(ui, tl!("Search"))).clicked() {
                    search = true;
                }
            });
            ui.checkbox(&mut b.commercial, tl!("Only pictures I may use commercially"));
            ui.add_space(6.0);
            egui::ScrollArea::vertical().max_height(360.0).auto_shrink([false, false]).show(ui, |ui| {
                let cell = vec2(160.0, 160.0);
                let columns = 4;
                for (row, chunk) in b.results.chunks(columns).enumerate() {
                    ui.horizontal(|ui| {
                        for (k, hit) in chunk.iter().enumerate() {
                            let i = row * columns + k;
                            let (r, resp) = ui.allocate_exact_size(cell, Sense::click());
                            let selected = b.selected == Some(i);
                            ui.painter().rect_filled(r, 4.0, if selected { t.checked } else { t.panel });
                            match &hit.thumb {
                                Thumb::Ready(tex) => {
                                    let size = tex.size_vec2();
                                    let k = (cell.x - 12.0) / size.x.max(size.y).max(1.0);
                                    let img = egui::Rect::from_center_size(r.center(), size * k);
                                    ui.painter().image(
                                        tex.id(),
                                        img,
                                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                        Color32::WHITE,
                                    );
                                }
                                Thumb::Waiting(_) => {
                                    egui::Spinner::new().paint_at(ui, egui::Rect::from_center_size(r.center(), vec2(20.0, 20.0)));
                                }
                                Thumb::Failed => {
                                    ui.painter().text(r.center(), egui::Align2::CENTER_CENTER, "?", egui::FontId::proportional(24.0), t.text_dim);
                                }
                            }
                            if selected {
                                ui.painter().rect_stroke(r, 4.0, egui::Stroke::new(2.0, t.accent), egui::StrokeKind::Inside);
                            }
                            let tip = format!("{}\n{}", hit.title, licence_line(hit));
                            let resp = resp.on_hover_text(tip);
                            if resp.clicked() {
                                b.selected = Some(i);
                            }
                            if resp.double_clicked() {
                                insert_at = Some(i);
                            }
                        }
                    });
                }
            });
            if let Some(m) = &b.message {
                ui.label(egui::RichText::new(m).color(t.text_dim));
            }
            ui.label(
                egui::RichText::new(tl!("Pictures from Openverse. Respect each picture's licence: Doc4Me adds a credit line under it."))
                    .small()
                    .color(t.text_dim),
            );
            let (ok, no) = crate::dialogs_insert::buttons(ui, "Insert", "Cancel", b.selected.is_some() && b.downloading.is_none());
            if ok {
                insert_at = b.selected;
            }
            cancel = no;
        });
    if search {
        start_search(app);
    }
    if let Some(i) = insert_at {
        start_download(app, i, false);
    }
    if !open || cancel {
        app.online_pictures = None;
    }
}

/// "by Creator · CC BY 2.0", for the tooltip.
fn licence_line(hit: &Hit) -> String {
    let licence = format!("{} {}", hit.license.to_ascii_uppercase(), hit.license_version).trim().to_string();
    if hit.creator.is_empty() { licence } else { format!("{} · {licence}", hit.creator) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searches_ask_openverse_and_results_keep_web_addresses_only() {
        assert_eq!(
            search_url(" แมว & dog ", true),
            "https://api.openverse.org/v1/images/?q=%E0%B9%81%E0%B8%A1%E0%B8%A7%20%26%20dog&page_size=20&mature=false&license_type=commercial"
        );
        let answer = br#"{"results": [
            {"title": "Cat", "creator": "Ann", "license": "by", "license_version": "2.0", "license_url": "https://creativecommons.org/licenses/by/2.0/",
             "foreign_landing_url": "https://www.flickr.com/photos/1", "url": "https://live.staticflickr.com/1.jpg", "thumbnail": "https://api.openverse.org/v1/images/1/thumb/"},
            {"title": "Bad", "url": "javascript:alert(1)", "thumbnail": "file:///etc/passwd", "foreign_landing_url": "javascript:x"},
            {"title": "Only a thumbnail", "url": "http://insecure.example/x.jpg", "thumbnail": "https://api.openverse.org/v1/images/3/thumb/", "license_url": "javascript:x"}
        ]}"#;
        let hits = parse_results(answer).unwrap();
        assert_eq!(hits.len(), 2, "no web address: dropped");
        assert_eq!((hits[0].title.as_str(), hits[0].license_url.as_str()), ("Cat", "https://creativecommons.org/licenses/by/2.0/"));
        assert_eq!((hits[1].url.as_str(), hits[1].license_url.as_str()), ("", ""), "http and scripts dropped");
        assert!(parse_results(b"<html>").is_err());
        assert!(parse_results(b"{}").is_err());
    }

    /// The whole flow with a fake web: search, thumbnails, pick, download, insert with credit.
    #[test]
    fn a_picked_picture_is_inserted_with_its_credit_line() {
        let png = crate::paste_picture::png_from_rgba(8, 6, vec![120; 8 * 6 * 4]).unwrap();
        let answer = br#"{"results": [{"title": "Cat", "creator": "Ann", "license": "by-sa", "license_version": "4.0",
            "license_url": "https://creativecommons.org/licenses/by-sa/4.0/", "foreign_landing_url": "https://commons.wikimedia.org/wiki/File:Cat.jpg",
            "url": "https://upload.wikimedia.org/cat.jpg", "thumbnail": "https://api.openverse.org/v1/images/1/thumb/"}]}"#
            .to_vec();
        let asked = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = asked.clone();
        let services = crate::Services {
            fetch: Some(Box::new(move |req: FetchRequest| {
                seen.borrow_mut().push(req.url.clone());
                let (tx, rx) = std::sync::mpsc::channel();
                let body = if req.url.starts_with(SEARCH) && !req.url.contains("/thumb/") {
                    Ok(answer.clone())
                } else if req.url.contains("wikimedia") {
                    Err("404".to_string())
                } else {
                    Ok(png.clone())
                };
                let _ = tx.send(body);
                rx
            })),
            ..Default::default()
        };
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("Pets")), services);
        let ctx = egui::Context::default();
        app.run("insert.onlinePicture", json!({})).unwrap();
        assert!(app.online_pictures.is_some(), "the window opens");
        app.online_pictures.as_mut().unwrap().query = "cat".into();
        start_search(&mut app);
        poll(&mut app, &ctx);
        poll(&mut app, &ctx);
        let b = app.online_pictures.as_ref().unwrap();
        assert_eq!(b.results.len(), 1);
        assert!(matches!(b.results[0].thumb, Thumb::Ready(_)), "thumbnail shown");
        let undo = app.session.undo_depth();
        start_download(&mut app, 0, false);
        poll(&mut app, &ctx);
        poll(&mut app, &ctx);
        assert!(app.online_pictures.is_none(), "inserted and closed");
        assert!(asked.borrow().iter().any(|u| u.contains("/thumb/")), "fell back to Openverse's copy");
        assert_eq!(app.session.undo_depth(), undo + 1, "one undo step");
        let text = app.session.doc.plain_text(doc4me_doc::StoryRef::Body);
        assert!(text.contains("“Cat” by Ann is licensed under CC BY-SA 4.0."), "{text}");
    }
}
