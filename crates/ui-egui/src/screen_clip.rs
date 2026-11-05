//! Insert › Illustrations › Screenshot › Screen Clipping.
//!
//! Word dims the screen and lets the user drag out the part to insert. Doc4Me hands that to the
//! system's own screen clipping ([`crate::Services::screen_clip`]: the host starts it and answers
//! on a channel once the user is done) and inserts the clip as a picture, one undoable step like
//! any inserted picture. Like Word, the window gets out of the way meanwhile: on Windows and macOS
//! it is minimized and comes back with the clip. On Linux and the BSDs it stays put (Wayland can't
//! bring a minimized window back), and the system's clipping covers it. The clip comes from another
//! program, so it is checked like a pasted picture ([`crate::paste_picture`]).

use std::sync::mpsc::{Receiver, TryRecvError};

use egui::{ViewportCommand, ViewportId};

use crate::WordApp;
use crate::paste_picture::ClipboardPicture;

/// The host's answer: the clip, `Ok(None)` when the user cancelled, or why clipping failed.
pub type ScreenClip = Result<Option<ClipboardPicture>, String>;

/// Longest wait for a clip, in milliseconds; after that the user has walked away.
const MAX_WAIT_MS: f64 = 5.0 * 60_000.0;

/// A screen clipping in progress.
pub struct Clipping {
    answer: Receiver<ScreenClip>,
    started: f64,
    /// The window was minimized for it, and comes back when it ends.
    minimized: bool,
    /// The window has lost focus since: focus coming back means the user returned without a clip.
    left: bool,
}

/// Start a screen clipping (the ribbon's Screen Clipping, `insert.screenshot`).
pub(crate) fn start(app: &mut WordApp) {
    let Some(hook) = app.services.screen_clip.as_ref() else {
        app.status(tl!("Screen clipping isn't available here. Copy a picture of the screen, then paste it.").to_string());
        return;
    };
    match hook() {
        Ok(answer) => {
            let minimized = cfg!(any(target_os = "windows", target_os = "macos")) && app.ctx.is_some();
            if minimized && let Some(ctx) = &app.ctx {
                ctx.send_viewport_cmd_to(ViewportId::ROOT, ViewportCommand::Minimized(true));
            }
            app.screen_clip = Some(Clipping { answer, started: crate::now_ms(), minimized, left: false });
            app.status(tl!("Select the part of the screen to insert.").to_string());
        }
        Err(e) => failed(app, &e),
    }
}

/// Insert the clip once the host answers (every frame while a clipping is in progress).
pub(crate) fn poll(app: &mut WordApp, ctx: &egui::Context) {
    let Some(clip) = app.screen_clip.as_mut() else { return };
    let mut answer = match clip.answer.try_recv() {
        Ok(answer) => Some(answer),
        // The host gave up without a word.
        Err(TryRecvError::Disconnected) => Some(Ok(None)),
        Err(TryRecvError::Empty) => None,
    };
    if answer.is_none() && clip.minimized {
        // The user brought the window back before clipping anything (the system's clipping on
        // Windows doesn't say when it was cancelled).
        let focused = ctx.input(|i| i.viewport().focused).unwrap_or(false);
        if !focused {
            clip.left = true;
        } else if clip.left {
            answer = Some(Ok(None));
        }
    }
    if answer.is_none() && crate::now_ms() - clip.started > MAX_WAIT_MS {
        answer = Some(Ok(None));
    }
    let Some(answer) = answer else {
        // The host also wakes the UI when it answers; this is the fallback.
        ctx.request_repaint_after(std::time::Duration::from_millis(400));
        return;
    };
    let minimized = clip.minimized;
    app.screen_clip = None;
    if minimized {
        ctx.send_viewport_cmd_to(ViewportId::ROOT, ViewportCommand::Minimized(false));
        ctx.send_viewport_cmd_to(ViewportId::ROOT, ViewportCommand::Focus);
    }
    match answer {
        Ok(Some(picture)) => match crate::paste_picture::insert(app, picture) {
            Some(Ok(_)) => app.status_msg = None,
            Some(Err(e)) => failed(app, &e),
            None => app.status(tl!("Screen clipping ended without a picture.").to_string()),
        },
        Ok(None) => app.status(tl!("Screen clipping ended without a picture.").to_string()),
        Err(e) => failed(app, &e),
    }
}

fn failed(app: &mut WordApp, reason: &str) {
    log::warn!("screen clipping: {reason}");
    app.status(crate::i18n::fmt(tl!("Screen clipping didn't work: {reason}"), &[("reason", reason)]));
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;
    use crate::Services;
    use doc4me_engine::Session;

    fn png() -> Vec<u8> {
        crate::paste_picture::png_from_rgba(3, 2, vec![90; 3 * 2 * 4]).unwrap()
    }

    fn app_with(hook: impl Fn() -> Result<Receiver<ScreenClip>, String> + 'static) -> WordApp {
        let services = Services { screen_clip: Some(Box::new(hook)), ..Default::default() };
        WordApp::new(Session::new(doc4me_doc::Document::new()), services)
    }

    fn pictures(app: &mut WordApp) -> usize {
        app.session.run("arrange.selectionPane", &serde_json::json!({})).unwrap().as_array().map(|x| x.len()).unwrap_or(0)
    }

    #[test]
    fn a_clip_is_inserted_as_one_undoable_picture() {
        let (tx, rx) = mpsc::channel();
        let rx = std::cell::RefCell::new(Some(rx));
        let mut app = app_with(move || rx.borrow_mut().take().ok_or_else(|| "busy".to_string()));
        let ctx = egui::Context::default();
        let undo = app.session.undo_depth();
        // Starting changes nothing: no undo step, no unsaved change.
        app.execute("insert.screenshot", serde_json::json!({})).unwrap();
        assert!(app.screen_clip.is_some(), "waiting for the clip");
        assert_eq!(app.session.undo_depth(), undo, "starting adds no undo step");
        assert!(!app.session.dirty, "starting changes nothing");
        poll(&mut app, &ctx);
        assert!(app.screen_clip.is_some(), "still waiting");
        tx.send(Ok(Some(ClipboardPicture::Encoded(png())))).unwrap();
        poll(&mut app, &ctx);
        assert!(app.screen_clip.is_none());
        assert_eq!(pictures(&mut app), 1, "the clip is in the document");
        assert_eq!(app.session.undo_depth(), undo + 1, "one undo step");
        app.execute("edit.undo", serde_json::json!({})).unwrap();
        assert_eq!(pictures(&mut app), 0, "undo takes it out");
    }

    #[test]
    fn cancelled_failed_and_unavailable_clippings_say_so() {
        // Cancelled: nothing inserted.
        let (tx, rx) = mpsc::channel();
        let rx = std::cell::RefCell::new(Some(rx));
        let mut app = app_with(move || rx.borrow_mut().take().ok_or_else(|| "busy".to_string()));
        let ctx = egui::Context::default();
        start(&mut app);
        tx.send(Ok(None)).unwrap();
        poll(&mut app, &ctx);
        assert_eq!(pictures(&mut app), 0);
        assert!(app.status_msg.as_ref().is_some_and(|(m, _)| m.contains("without a picture")), "{:?}", app.status_msg);
        // The host hands over something that isn't a picture.
        let (tx, rx) = mpsc::channel();
        let rx = std::cell::RefCell::new(Some(rx));
        let mut app = app_with(move || rx.borrow_mut().take().ok_or_else(|| "busy".to_string()));
        start(&mut app);
        tx.send(Ok(Some(ClipboardPicture::Encoded(b"not a picture".to_vec())))).unwrap();
        poll(&mut app, &ctx);
        assert_eq!(pictures(&mut app), 0);
        assert!(app.status_msg.as_ref().is_some_and(|(m, _)| m.contains("didn't work")), "{:?}", app.status_msg);
        // The host couldn't start one.
        let mut app = app_with(|| Err("no screen clipping tool".into()));
        start(&mut app);
        assert!(app.screen_clip.is_none());
        assert!(app.status_msg.as_ref().is_some_and(|(m, _)| m.contains("no screen clipping tool")), "{:?}", app.status_msg);
        // The host dropped the channel without answering.
        let mut app = app_with(|| Ok(mpsc::channel().1));
        start(&mut app);
        poll(&mut app, &ctx);
        assert!(app.screen_clip.is_none(), "stops waiting");
        // No host service (the web).
        let mut app = WordApp::new(Session::new(doc4me_doc::Document::new()), Services::default());
        start(&mut app);
        assert!(app.screen_clip.is_none());
        assert!(app.status_msg.as_ref().is_some_and(|(m, _)| m.contains("isn't available")), "{:?}", app.status_msg);
    }
}
