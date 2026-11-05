//! The Translator pane (Review › Language › Translate).
//!
//! Word sends the text to Microsoft Translator; Doc4Me uses free services: MyMemory by default
//! (no account; Translated's free allowance is 5,000 characters a day, 50,000 with an email
//! address) or a LibreTranslate server the user names (open source, and free on one's own
//! machine or a free instance). The engine says what to translate and puts the answers back
//! (`review.translate`, `review.translateDocument`, [`doc4me_engine::cmd::translate`]); this
//! module asks the service through [`crate::net`], one request at a time.
//!
//! - **Selection**: the selected text, translated, with Insert to replace the selection
//!   (`edit.pasteText`: the formatting where it goes stays).
//! - **Document**: every paragraph translated into a new document that replaces this one, as
//!   Mail Merge's Finish does (Doc4Me shows one document a window); unsaved changes are asked
//!   about first.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::WordApp;
use crate::net::{FetchRequest, Pending};
use crate::theme::Tokens;

/// MyMemory's address.
const MYMEMORY: &str = "https://api.mymemory.translated.net/get";
/// Largest answer taken from a service.
const MAX_ANSWER: u64 = 1 << 20;

/// The languages offered: (MyMemory code, LibreTranslate code, English name).
pub const LANGUAGES: &[(&str, &str, &str)] = &[
    ("en", "en", "English"),
    ("th", "th", "Thai"),
    ("zh-CN", "zh-Hans", "Chinese (Simplified)"),
    ("zh-TW", "zh-Hant", "Chinese (Traditional)"),
    ("ja", "ja", "Japanese"),
    ("ko", "ko", "Korean"),
    ("vi", "vi", "Vietnamese"),
    ("id", "id", "Indonesian"),
    ("ms", "ms", "Malay"),
    ("hi", "hi", "Hindi"),
    ("ar", "ar", "Arabic"),
    ("fr", "fr", "French"),
    ("de", "de", "German"),
    ("es", "es", "Spanish"),
    ("it", "it", "Italian"),
    ("pt-BR", "pt-BR", "Portuguese (Brazil)"),
    ("nl", "nl", "Dutch"),
    ("sv", "sv", "Swedish"),
    ("pl", "pl", "Polish"),
    ("cs", "cs", "Czech"),
    ("hu", "hu", "Hungarian"),
    ("et", "et", "Estonian"),
    ("tr", "tr", "Turkish"),
    ("ru", "ru", "Russian"),
    ("uk", "uk", "Ukrainian"),
    ("sr", "sr", "Serbian"),
];

/// The service settings, kept with the other preferences.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    /// `mymemory` or `libretranslate`.
    pub service: String,
    /// MyMemory: an email address for the larger free allowance (optional).
    pub email: String,
    /// LibreTranslate: the server's address and its API key (optional).
    pub server: String,
    pub key: String,
    /// The language translated into (a MyMemory code from [`LANGUAGES`]).
    pub to: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { service: "mymemory".into(), email: String::new(), server: String::new(), key: String::new(), to: String::new() }
    }
}

/// One translation request: the paragraph (document scope), the piece's place, and the text.
struct Job {
    para: usize,
    text: String,
    join: &'static str,
}

/// The pane's state while it is open.
#[derive(Default)]
pub struct Translator {
    pub document: bool,
    pub from: String,
    pub source: String,
    pub result: String,
    /// Document scope: the paragraphs (path, text) and their translations so far.
    paragraphs: Vec<(Value, String)>,
    translated: Vec<String>,
    jobs: std::collections::VecDeque<Job>,
    total: usize,
    pending: Option<(Pending, &'static str, usize)>,
    pub message: Option<String>,
}

/// The MyMemory code for a language tag (`en-US` → `en`, `zh-TW` → `zh-TW`), if offered.
pub fn code_for_tag(tag: &str) -> Option<&'static str> {
    let lower = tag.to_ascii_lowercase();
    let (lang, region) = lower.split_once(['-', '_']).unwrap_or((&lower, ""));
    if lang == "zh" {
        return Some(if matches!(region, "tw" | "hk" | "mo" | "hant") { "zh-TW" } else { "zh-CN" });
    }
    if lang == "pt" {
        return Some("pt-BR");
    }
    LANGUAGES.iter().map(|(c, _, _)| *c).find(|c| c.eq_ignore_ascii_case(lang))
}

/// A guess at the language from the script of the text, for text without a language tag.
pub fn guess(text: &str) -> &'static str {
    let mut counts = [0usize; 6];
    for c in text.chars().take(2000) {
        let k = match c as u32 {
            0x0E00..=0x0E7F => 1,
            0x3040..=0x30FF => 2,
            0x4E00..=0x9FFF => 3,
            0xAC00..=0xD7AF => 4,
            0x0400..=0x04FF => 5,
            _ if c.is_alphabetic() => 0,
            _ => continue,
        };
        counts[k] += 1;
    }
    // Kana means Japanese even among many kanji.
    if counts[2] > 0 {
        return "ja";
    }
    let best = counts.iter().enumerate().max_by_key(|(_, n)| **n).map(|(i, _)| i).unwrap_or(0);
    ["en", "th", "ja", "zh-CN", "ko", "ru"][best]
}

/// The request for one piece of text.
pub fn request(settings: &Settings, from: &str, to: &str, text: &str) -> Result<FetchRequest, String> {
    let codes = |c: &str| LANGUAGES.iter().find(|(m, _, _)| *m == c).map(|(m, l, _)| (*m, *l));
    let (from_m, from_l) = codes(from).ok_or("choose the language to translate from")?;
    let (to_m, to_l) = codes(to).ok_or("choose the language to translate into")?;
    if settings.service == "libretranslate" {
        let server = settings.server.trim().trim_end_matches('/');
        if !crate::net::allowed(server) {
            return Err("enter the https address of a LibreTranslate server".into());
        }
        let mut body = json!({"q": text, "source": from_l, "target": to_l, "format": "text"});
        if !settings.key.trim().is_empty() {
            body["api_key"] = json!(settings.key.trim());
        }
        let mut req = FetchRequest::get(format!("{server}/translate"), MAX_ANSWER);
        req.body = Some(("application/json".into(), body.to_string().into_bytes()));
        return Ok(req);
    }
    let esc = crate::net::query_escape;
    let mut url = format!("{MYMEMORY}?q={}&langpair={}%7C{}", esc(text), esc(from_m), esc(to_m));
    let email = settings.email.trim();
    if email.contains('@') && email.len() <= 254 {
        url.push_str(&format!("&de={}", esc(email)));
    }
    Ok(FetchRequest::get(url, MAX_ANSWER))
}

/// The translation in a service's answer, or the service's reason for giving none.
pub fn answer_text(service: &str, body: &[u8]) -> Result<String, String> {
    let v: Value = serde_json::from_slice(body).map_err(|_| "the translation service gave an answer Doc4Me can't read".to_string())?;
    let text = if service == "libretranslate" {
        if let Some(e) = v.get("error").and_then(Value::as_str) {
            return Err(e.chars().take(300).collect());
        }
        v.get("translatedText").and_then(Value::as_str)
    } else {
        let status = v.get("responseStatus").and_then(|s| s.as_u64().or_else(|| s.as_str().and_then(|t| t.parse().ok()))).unwrap_or(0);
        if status != 200 {
            let why = v.get("responseDetails").and_then(Value::as_str).unwrap_or("the service refused the request");
            return Err(why.chars().take(300).collect());
        }
        v.pointer("/responseData/translatedText").and_then(Value::as_str)
    };
    let text = text.ok_or("the translation service gave no translation")?;
    Ok(unescape(text).chars().take(100_000).collect())
}

/// The HTML entities translation services leave in their text.
fn unescape(s: &str) -> String {
    s.replace("&quot;", "\"").replace("&#39;", "'").replace("&#039;", "'").replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&")
}

/// Open the pane (`review.translate` from the ribbon) on the selection.
pub(crate) fn open(app: &mut WordApp) {
    let text = app.session.selected_text();
    let tag = app
        .session
        .run("review.translate", &json!({"scope": "selection"}))
        .ok()
        .and_then(|v| v["language"].as_str().map(str::to_string))
        .unwrap_or_default();
    let from = code_for_tag(&tag).filter(|_| !tag.is_empty()).unwrap_or_else(|| guess(&text)).to_string();
    if app.ui.translator.to.is_empty() {
        let ui_lang = crate::i18n::current().code();
        let to = code_for_tag(ui_lang).filter(|c| *c != from).unwrap_or(if from == "en" { "th" } else { "en" });
        app.ui.translator.to = to.to_string();
    }
    app.translator = Some(Translator { from, source: text, ..Default::default() });
}

fn start(app: &mut WordApp) {
    let Some(t) = app.translator.as_mut() else { return };
    t.result.clear();
    t.message = None;
    t.jobs.clear();
    t.translated.clear();
    t.paragraphs.clear();
    if t.document {
        let r = app.session.run("review.translate", &json!({"scope": "document"})).unwrap_or_default();
        let Some(t) = app.translator.as_mut() else { return };
        for p in r["paragraphs"].as_array().into_iter().flatten() {
            if let (Some(text), path) = (p["text"].as_str(), p["path"].clone()) {
                t.paragraphs.push((path, text.to_string()));
            }
        }
        t.translated = vec![String::new(); t.paragraphs.len()];
        for (i, (_, text)) in t.paragraphs.iter().enumerate() {
            for (piece, join) in doc4me_engine::cmd::translate::pieces(text, doc4me_engine::cmd::translate::MAX_PIECE) {
                t.jobs.push_back(Job { para: i, text: piece, join });
            }
        }
    } else {
        let source = app.session.selected_text();
        let Some(t) = app.translator.as_mut() else { return };
        t.source = source;
        for (piece, join) in doc4me_engine::cmd::translate::pieces(&t.source, doc4me_engine::cmd::translate::MAX_PIECE) {
            t.jobs.push_back(Job { para: 0, text: piece, join });
        }
        t.translated = vec![String::new()];
    }
    let Some(t) = app.translator.as_mut() else { return };
    t.total = t.jobs.len();
    if t.total == 0 {
        t.message = Some(tl!("There's no text to translate.").to_string());
        return;
    }
    next(app);
}

/// Send the next piece.
fn next(app: &mut WordApp) {
    let Some(t) = app.translator.as_mut() else { return };
    let Some(job) = t.jobs.pop_front() else { return };
    let req = request(&app.ui.translator, &t.from, &app.ui.translator.to, &job.text);
    let started = req.and_then(|r| crate::net::start(app, r));
    let Some(t) = app.translator.as_mut() else { return };
    match started {
        Ok(rx) => t.pending = Some((Pending::new(rx), job.join, job.para)),
        Err(e) => {
            t.jobs.clear();
            t.message = Some(crate::i18n::fmt(tl!("Couldn't translate: {reason}"), &[("reason", &e)]));
        }
    }
}

/// Follow the request in flight (every frame while the pane is open).
pub(crate) fn poll(app: &mut WordApp, ctx: &egui::Context) {
    let Some(t) = app.translator.as_mut() else { return };
    let Some((pending, join, para)) = &t.pending else { return };
    ctx.request_repaint_after(std::time::Duration::from_millis(100));
    let Some(answer) = pending.poll() else { return };
    let (join, para) = (*join, *para);
    t.pending = None;
    let service = app.ui.translator.service.clone();
    match answer.and_then(|body| answer_text(&service, &body)) {
        Ok(text) => {
            if let Some(slot) = t.translated.get_mut(para) {
                slot.push_str(&text);
                slot.push_str(join);
            }
            if t.jobs.is_empty() {
                finish(app);
            } else {
                next(app);
            }
        }
        Err(e) => {
            t.jobs.clear();
            // MyMemory answers 429 once the day's free allowance is used.
            let e = if e.contains("429") || e.contains("FREE TRANSLATIONS") {
                tl!("the free daily allowance is used up (try again tomorrow, or give an email address)").to_string()
            } else {
                e
            };
            t.message = Some(crate::i18n::fmt(tl!("Couldn't translate: {reason}"), &[("reason", &e)]));
        }
    }
}

fn finish(app: &mut WordApp) {
    let Some(t) = app.translator.as_mut() else { return };
    if !t.document {
        t.result = t.translated.first().cloned().unwrap_or_default();
        return;
    }
    let paragraphs: Vec<Value> = t.paragraphs.iter().zip(&t.translated).map(|((path, _), text)| json!({"path": path, "text": text})).collect();
    let to = app.ui.translator.to.clone();
    t.message = None;
    match app.run("review.translateDocument", json!({"paragraphs": paragraphs, "language": to})) {
        Ok(_) => app.translator = None,
        Err(e) => {
            if let Some(t) = app.translator.as_mut() {
                t.message = Some(crate::i18n::fmt(tl!("Couldn't translate: {reason}"), &[("reason", &e)]));
            }
        }
    }
}

fn language_combo(ui: &mut egui::Ui, id: &str, value: &mut String) {
    let shown = LANGUAGES.iter().find(|(c, _, _)| c == value).map(|(_, _, n)| tl!(n).to_string()).unwrap_or_default();
    egui::ComboBox::from_id_salt(id).selected_text(shown).width(190.0).show_ui(ui, |ui| {
        for (code, _, name) in LANGUAGES {
            ui.selectable_value(value, (*code).to_string(), tl!(name));
        }
    });
}

/// The pane.
pub(crate) fn show(app: &mut WordApp, ctx: &egui::Context) {
    if app.translator.is_none() {
        return;
    }
    let t_ = Tokens::get(ctx);
    let mut open = true;
    let mut go = false;
    let mut insert = None;
    let right_top =
        app.canvas.canvas_rect.map_or(ctx.content_rect().right_top() + egui::vec2(-24.0, 160.0), |r| r.right_top() + egui::vec2(-24.0, 16.0));
    let selection_now = app.session.selected_text();
    egui::Window::new(tl!("Translator"))
        .id(egui::Id::new("translator-pane"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .pivot(egui::Align2::RIGHT_TOP)
        .default_pos(right_top)
        .show(ctx, |ui| {
            let settings = &mut app.ui.translator;
            let Some(t) = app.translator.as_mut() else { return };
            ui.set_width(320.0);
            let busy = t.pending.is_some();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut t.document, false, tl!("Selection"));
                ui.selectable_value(&mut t.document, true, tl!("Document"));
            });
            ui.add_space(4.0);
            egui::Grid::new("translator-langs").num_columns(2).show(ui, |ui| {
                ui.label(tl!("From"));
                language_combo(ui, "translator-from", &mut t.from);
                ui.end_row();
                ui.label(tl!("To"));
                language_combo(ui, "translator-to", &mut settings.to);
                ui.end_row();
            });
            ui.add_space(4.0);
            if !t.document {
                if !busy && selection_now != t.source {
                    t.source = selection_now.clone();
                    t.result.clear();
                }
                let mut shown: String = t.source.chars().take(600).collect();
                if shown.is_empty() {
                    shown = tl!("Select text in the document to translate it.").to_string();
                }
                ui.label(egui::RichText::new(shown).color(t_.text_dim));
                ui.separator();
                if !t.result.is_empty() {
                    ui.add(egui::TextEdit::multiline(&mut t.result).desired_rows(4).desired_width(f32::INFINITY));
                }
            } else {
                ui.label(
                    egui::RichText::new(tl!("The whole document is translated into a new document, which replaces this one. Save your work first."))
                        .small()
                        .color(t_.text_dim),
                );
            }
            if busy && t.total > 0 {
                let done = t.total.saturating_sub(t.jobs.len() + 1);
                ui.add(
                    egui::ProgressBar::new(done as f32 / t.total as f32).text(crate::i18n::fmt(
                        tl!("Translating {done} of {total}"),
                        &[("done", &done.to_string()), ("total", &t.total.to_string())],
                    )),
                );
            }
            if let Some(m) = &t.message {
                ui.colored_label(ui.visuals().warn_fg_color, m);
            }
            ui.horizontal(|ui| {
                let can = !busy && (t.document || !t.source.trim().is_empty());
                if ui.add_enabled(can, crate::widgets::primary_button(ui, tl!("Translate"))).clicked() {
                    go = true;
                }
                if !t.document && !t.result.is_empty() && ui.button(tl!("Insert")).clicked() {
                    insert = Some(t.result.clone());
                }
            });
            egui::CollapsingHeader::new(tl!("Translation service")).id_salt("translator-service").show(ui, |ui| {
                ui.radio_value(&mut settings.service, "mymemory".to_string(), tl!("MyMemory (free, no account)"));
                if settings.service == "mymemory" {
                    ui.label(egui::RichText::new(tl!("5,000 characters a day; give an email address for 50,000.")).small().color(t_.text_dim));
                    ui.add(crate::widgets::text_line(&mut settings.email).hint_text(tl!("Email address (optional)")));
                }
                ui.radio_value(&mut settings.service, "libretranslate".to_string(), tl!("LibreTranslate server (open source)"));
                if settings.service == "libretranslate" {
                    ui.add(crate::widgets::text_line(&mut settings.server).hint_text(tl!("Server address (https://…)")));
                    ui.add(crate::widgets::text_line(&mut settings.key).password(true).hint_text(tl!("API key (if the server needs one)")));
                }
            });
            let service = if settings.service == "libretranslate" {
                tl!("the LibreTranslate server").to_string()
            } else {
                "MyMemory (translated.net)".to_string()
            };
            ui.label(
                egui::RichText::new(crate::i18n::fmt(tl!("The text is sent to {service} to be translated."), &[("service", &service)]))
                    .small()
                    .color(t_.text_dim),
            );
        });
    if go {
        start(app);
    }
    if let Some(text) = insert {
        let _ = app.run("edit.pasteText", json!({"text": text}));
    }
    if !open {
        app.translator = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages_are_guessed_and_requests_built_per_service() {
        assert_eq!(code_for_tag("en-US"), Some("en"));
        assert_eq!(code_for_tag("zh-HK"), Some("zh-TW"));
        assert_eq!(code_for_tag("th-TH"), Some("th"));
        assert_eq!(code_for_tag("xx"), None);
        assert_eq!(guess("สวัสดีครับ hello"), "th");
        assert_eq!(guess("日本語のテキスト"), "ja");
        assert_eq!(guess("中文文本"), "zh-CN");
        assert_eq!(guess("Hello there"), "en");
        let mut s = Settings { email: "me@example.com".into(), ..Settings::default() };
        let r = request(&s, "en", "th", "Hello & bye").unwrap();
        assert_eq!(r.url, "https://api.mymemory.translated.net/get?q=Hello%20%26%20bye&langpair=en%7Cth&de=me%40example.com");
        assert!(r.body.is_none());
        s.service = "libretranslate".into();
        s.server = "http://localhost:5000".into();
        assert!(request(&s, "en", "th", "x").is_err(), "https servers only");
        s.server = "https://translate.example.org/".into();
        s.key = "k".into();
        let r = request(&s, "zh-TW", "en", "x").unwrap();
        assert_eq!(r.url, "https://translate.example.org/translate");
        let body: Value = serde_json::from_slice(&r.body.unwrap().1).unwrap();
        assert_eq!((body["source"].as_str(), body["target"].as_str(), body["api_key"].as_str()), (Some("zh-Hant"), Some("en"), Some("k")));
        assert!(request(&s, "en", "xx", "x").is_err());
    }

    #[test]
    fn service_answers_and_refusals_are_read() {
        assert_eq!(
            answer_text("mymemory", r#"{"responseData":{"translatedText":"สวัสดี &quot;โลก&quot;"},"responseStatus":200}"#.as_bytes()).unwrap(),
            "สวัสดี \"โลก\""
        );
        let quota = br#"{"responseData":{"translatedText":"MYMEMORY WARNING"},"responseStatus":429,"responseDetails":"YOU USED ALL AVAILABLE FREE TRANSLATIONS FOR TODAY"}"#;
        assert!(answer_text("mymemory", quota).is_err_and(|e| e.contains("FREE TRANSLATIONS")));
        assert_eq!(answer_text("libretranslate", br#"{"translatedText":"Hola"}"#).unwrap(), "Hola");
        assert!(answer_text("libretranslate", br#"{"error":"Invalid API key"}"#).is_err_and(|e| e == "Invalid API key"));
        assert!(answer_text("mymemory", b"<html>").is_err());
    }

    /// The whole flow against a fake service: the selection translated and inserted, then the
    /// document translated into a new one.
    #[test]
    fn selection_and_document_are_translated() {
        let services = crate::Services {
            fetch: Some(Box::new(|req: FetchRequest| {
                let (tx, rx) = std::sync::mpsc::channel();
                let q = req.url.split("q=").nth(1).and_then(|r| r.split('&').next()).unwrap_or("").to_string();
                let _ = tx.send(Ok(format!(r#"{{"responseData":{{"translatedText":"[th:{q}]"}},"responseStatus":200}}"#).into_bytes()));
                rx
            })),
            ..Default::default()
        };
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("Hello\nWorld")), services);
        let ctx = egui::Context::default();
        app.run("select.all", json!({})).unwrap();
        app.run("review.translate", json!({})).unwrap();
        assert!(app.translator.is_some(), "the pane opens");
        app.ui.translator.to = "th".into();
        start(&mut app);
        for _ in 0..4 {
            poll(&mut app, &ctx);
        }
        let result = app.translator.as_ref().unwrap().result.clone();
        assert!(result.starts_with("[th:Hello"), "{result}");
        app.translator.as_mut().unwrap().document = true;
        start(&mut app);
        for _ in 0..6 {
            poll(&mut app, &ctx);
        }
        assert!(app.translator.is_none(), "done: the pane closes");
        let text = app.session.doc.plain_text(doc4me_doc::StoryRef::Body);
        assert!(text.contains("[th:Hello]") && text.contains("[th:World]"), "{text}");
        assert!(app.session.dirty);
    }
}
