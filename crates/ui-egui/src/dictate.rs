//! Home › Voice › Dictate.
//!
//! Word sends the microphone to Microsoft's speech service. Doc4Me recognises speech on the
//! computer with whisper.cpp (open source; [`crate::Services::dictate`], the desktop app's
//! `dictate` module): the first time, the free speech model is downloaded from Hugging Face and
//! checked by SHA-256; after that nothing leaves the computer. The host listens, cuts the speech
//! at pauses and sends the text of each stretch; this module types it at the insertion point,
//! as if typed, and shows what dictation is doing.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, TryRecvError};

use serde_json::json;

use crate::WordApp;
use crate::theme::Tokens;

/// What to dictate in.
pub struct DictationRequest {
    /// A Whisper language code (`th`, `en`, `ja`…).
    pub language: String,
}

/// What the host tells the UI while dictating.
#[derive(Clone, Debug, PartialEq)]
pub enum DictationEvent {
    /// Downloading the speech model (bytes so far, of the total).
    Downloading { done: u64, total: u64 },
    /// Loading the speech model.
    Loading,
    /// The microphone is on.
    Listening,
    /// Recognised speech.
    Text(String),
    /// Dictation stopped with this reason.
    Error(String),
    /// Dictation ended (Stop was pressed).
    Stopped,
}

/// A dictation in progress: its events, and the switch that stops it.
pub struct Dictation {
    pub events: Receiver<DictationEvent>,
    pub stop: Arc<AtomicBool>,
}

/// The languages offered: (Whisper code, English name).
pub const LANGUAGES: &[(&str, &str)] = &[
    ("th", "Thai"),
    ("en", "English"),
    ("zh", "Chinese (Simplified)"),
    ("ja", "Japanese"),
    ("ko", "Korean"),
    ("vi", "Vietnamese"),
    ("id", "Indonesian"),
    ("ms", "Malay"),
    ("fr", "French"),
    ("de", "German"),
    ("es", "Spanish"),
    ("it", "Italian"),
    ("pt", "Portuguese (Brazil)"),
    ("ru", "Russian"),
    ("uk", "Ukrainian"),
    ("pl", "Polish"),
    ("cs", "Czech"),
    ("hu", "Hungarian"),
    ("tr", "Turkish"),
    ("et", "Estonian"),
    ("sr", "Serbian"),
];

/// Dictation's state, while it runs.
pub struct Active {
    session: Dictation,
    pub language: String,
    pub state: DictationEvent,
}

/// The Whisper language for a language tag (`th-TH` → `th`), if offered.
pub fn language_for_tag(tag: &str) -> Option<&'static str> {
    let lang = tag.split(['-', '_']).next().unwrap_or("").to_ascii_lowercase();
    LANGUAGES.iter().map(|(c, _)| *c).find(|c| *c == lang)
}

/// `tools.dictate`: start (or `on`), stop (or `off`), or switch.
pub(crate) fn request(app: &mut WordApp, what: &str) {
    let on = match what {
        "on" => true,
        "off" => false,
        _ => app.dictation.is_none(),
    };
    if !on {
        stop(app);
        return;
    }
    if app.dictation.is_some() {
        return;
    }
    let language = app
        .session
        .editing_lang
        .as_deref()
        .and_then(language_for_tag)
        .or_else(|| language_for_tag(crate::i18n::current().code()))
        .unwrap_or("en")
        .to_string();
    start(app, language);
}

fn start(app: &mut WordApp, language: String) {
    let Some(hook) = app.services.dictate.as_ref() else {
        app.status(tl!("Dictation isn't available here.").to_string());
        return;
    };
    match hook(DictationRequest { language: language.clone() }) {
        Ok(session) => app.dictation = Some(Active { session, language, state: DictationEvent::Loading }),
        Err(e) => failed(app, &e),
    }
}

fn stop(app: &mut WordApp) {
    if let Some(a) = &app.dictation {
        a.session.stop.store(true, Ordering::Relaxed);
    }
}

fn failed(app: &mut WordApp, reason: &str) {
    log::warn!("dictation: {reason}");
    app.status(crate::i18n::fmt(tl!("Dictation stopped: {reason}"), &[("reason", reason)]));
}

/// Type `text` at the insertion point, with a space before it when it follows a word (not in
/// languages written without spaces).
fn type_text(app: &mut WordApp, text: &str, language: &str) {
    let text = text.trim();
    if text.is_empty() {
        return;
    }
    let spaced = !matches!(language, "th" | "zh" | "ja");
    let before = app
        .session
        .doc
        .para_at(&app.session.sel.focus)
        .and_then(|p| p.text.get(..app.session.sel.focus.off.min(p.text.len())))
        .and_then(|t| t.chars().last());
    let text = if spaced && before.is_some_and(|c| !c.is_whitespace()) { format!(" {text}") } else { text.to_string() };
    if let Err(e) = app.execute("text.insert", json!({"text": text})) {
        app.status(e);
    }
}

/// Follow the host's events (every frame while dictating).
pub(crate) fn poll(app: &mut WordApp, ctx: &egui::Context) {
    let Some(active) = app.dictation.as_mut() else { return };
    ctx.request_repaint_after(std::time::Duration::from_millis(150));
    let mut texts = Vec::new();
    let mut end = None;
    loop {
        match active.session.events.try_recv() {
            Ok(DictationEvent::Text(t)) => texts.push(t),
            Ok(DictationEvent::Error(e)) => {
                end = Some(Some(e));
                break;
            }
            Ok(DictationEvent::Stopped) => {
                end = Some(None);
                break;
            }
            Ok(state) => active.state = state,
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => {
                end = Some(None);
                break;
            }
        }
    }
    let language = active.language.clone();
    for t in texts {
        type_text(app, &t, &language);
    }
    if let Some(reason) = end {
        app.dictation = None;
        match reason {
            Some(e) => failed(app, &e),
            None => app.status(tl!("Dictation stopped.").to_string()),
        }
    }
}

/// The small window that shows what dictation is doing.
pub(crate) fn show(app: &mut WordApp, ctx: &egui::Context) {
    let Some(active) = app.dictation.as_ref() else { return };
    let t = Tokens::get(ctx);
    let (what, progress) = match &active.state {
        DictationEvent::Downloading { done, total } => {
            let f = if *total > 0 { *done as f32 / *total as f32 } else { 0.0 };
            (crate::i18n::fmt(tl!("Downloading the speech model ({mb} MB, once)…"), &[("mb", &(total / 1_000_000).to_string())]), Some(f))
        }
        DictationEvent::Loading => (tl!("Getting ready…").to_string(), None),
        _ => (tl!("Listening… speak now").to_string(), None),
    };
    let mut language = active.language.clone();
    let mut stop_now = false;
    let mut open = true;
    let right_top =
        app.canvas.canvas_rect.map_or(ctx.content_rect().right_top() + egui::vec2(-24.0, 160.0), |r| r.right_top() + egui::vec2(-24.0, 16.0));
    egui::Window::new(tl!("Dictate"))
        .id(egui::Id::new("dictation"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .pivot(egui::Align2::RIGHT_TOP)
        .default_pos(right_top)
        .show(ctx, |ui| {
            ui.set_width(260.0);
            ui.label(&what);
            if let Some(f) = progress {
                ui.add(egui::ProgressBar::new(f));
            }
            ui.horizontal(|ui| {
                ui.label(tl!("Language"));
                let shown = LANGUAGES.iter().find(|(c, _)| *c == language).map(|(_, n)| tl!(n).to_string()).unwrap_or_default();
                egui::ComboBox::from_id_salt("dictation-language").selected_text(shown).show_ui(ui, |ui| {
                    for (code, name) in LANGUAGES {
                        ui.selectable_value(&mut language, (*code).to_string(), tl!(name));
                    }
                });
            });
            if ui.add(crate::widgets::primary_button(ui, tl!("Stop"))).clicked() {
                stop_now = true;
            }
            ui.label(
                egui::RichText::new(tl!("Speech is recognised on this computer (whisper.cpp); nothing is sent online.")).small().color(t.text_dim),
            );
        });
    if stop_now || !open {
        stop(app);
    } else if language != active.language {
        // Another language: start again in it.
        stop(app);
        app.dictation = None;
        start(app, language);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictated_text_is_typed_where_the_caret_is() {
        let (tx, rx) = std::sync::mpsc::channel();
        let rx = std::cell::RefCell::new(Some(rx));
        let stopped = Arc::new(AtomicBool::new(false));
        let flag = stopped.clone();
        let services = crate::Services {
            dictate: Some(Box::new(move |req: DictationRequest| {
                assert_eq!(req.language, "en");
                Ok(Dictation { events: rx.borrow_mut().take().ok_or("busy")?, stop: flag.clone() })
            })),
            ..Default::default()
        };
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("Hello")), services);
        app.session.editing_lang = Some("en-US".into());
        let ctx = egui::Context::default();
        app.run("caret.docEnd", json!({})).unwrap();
        app.run("tools.dictate", json!({})).unwrap();
        assert!(app.dictation.is_some());
        tx.send(DictationEvent::Listening).unwrap();
        tx.send(DictationEvent::Text(" world. ".into())).unwrap();
        tx.send(DictationEvent::Text("How are you?".into())).unwrap();
        poll(&mut app, &ctx);
        assert_eq!(app.session.doc.plain_text(doc4me_doc::StoryRef::Body), "Hello world. How are you?");
        assert_eq!(app.dictation.as_ref().unwrap().state, DictationEvent::Listening);
        // The button again stops it; the host ends with Stopped.
        app.run("tools.dictate", json!({})).unwrap();
        assert!(stopped.load(Ordering::Relaxed));
        tx.send(DictationEvent::Stopped).unwrap();
        poll(&mut app, &ctx);
        assert!(app.dictation.is_none());
    }

    #[test]
    fn errors_and_missing_service_are_reported() {
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), crate::Services::default());
        app.run("tools.dictate", json!({})).unwrap();
        assert!(app.dictation.is_none());
        assert!(app.status_msg.as_ref().is_some_and(|(m, _)| m.contains("isn't available")));
        let services = crate::Services { dictate: Some(Box::new(|_| Err("no microphone".into()))), ..Default::default() };
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), services);
        app.run("tools.dictate", json!({"value": true})).unwrap();
        assert!(app.status_msg.as_ref().is_some_and(|(m, _)| m.contains("no microphone")));
        assert_eq!(language_for_tag("th-TH"), Some("th"));
        assert_eq!(language_for_tag("xx"), None);
    }
}
