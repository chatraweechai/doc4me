//! Keyboard: typing, editing keys and command shortcuts (the user's custom keys from Customize
//! Keyboard first, then the registry's `shortcut` fields).

use egui::{Key, Modifiers};
use serde_json::{Value, json};

use crate::WordApp;

/// The registry's name for a key.
pub fn key_name(k: Key) -> Option<&'static str> {
    Some(match k {
        Key::ArrowLeft => "Left",
        Key::ArrowRight => "Right",
        Key::ArrowUp => "Up",
        Key::ArrowDown => "Down",
        Key::Home => "Home",
        Key::End => "End",
        Key::PageUp => "PageUp",
        Key::PageDown => "PageDown",
        Key::Enter => "Enter",
        Key::Tab => "Tab",
        Key::Backspace => "Backspace",
        Key::Delete => "Delete",
        Key::Escape => "Escape",
        Key::Space => "Space",
        Key::Insert => "Insert",
        Key::Equals | Key::Plus => "=",
        Key::Minus => "-",
        Key::Period => ".",
        Key::Comma => ",",
        Key::OpenBracket => "[",
        Key::CloseBracket => "]",
        Key::Num0 => "0",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Num5 => "5",
        Key::Num6 => "6",
        Key::Num7 => "7",
        Key::Num8 => "8",
        Key::Num9 => "9",
        Key::F1 => "F1",
        Key::F2 => "F2",
        Key::F3 => "F3",
        Key::F4 => "F4",
        Key::F5 => "F5",
        Key::F6 => "F6",
        Key::F7 => "F7",
        Key::F8 => "F8",
        Key::F9 => "F9",
        Key::F10 => "F10",
        Key::F11 => "F11",
        Key::F12 => "F12",
        Key::A => "A",
        Key::B => "B",
        Key::C => "C",
        Key::D => "D",
        Key::E => "E",
        Key::F => "F",
        Key::G => "G",
        Key::H => "H",
        Key::I => "I",
        Key::J => "J",
        Key::K => "K",
        Key::L => "L",
        Key::M => "M",
        Key::N => "N",
        Key::O => "O",
        Key::P => "P",
        Key::Q => "Q",
        Key::R => "R",
        Key::S => "S",
        Key::T => "T",
        Key::U => "U",
        Key::V => "V",
        Key::W => "W",
        Key::X => "X",
        Key::Y => "Y",
        Key::Z => "Z",
        _ => return None,
    })
}

/// A key with its modifiers as the registry writes shortcuts (`Mod+Shift+K`).
pub fn combo(m: Modifiers, key: &str, with_shift: bool) -> String {
    let mut s = String::new();
    if m.command {
        s.push_str("Mod+");
    }
    if m.ctrl && !m.command {
        s.push_str("Ctrl+");
    }
    if m.mac_cmd && m.ctrl {
        s.push_str("Ctrl+");
    }
    if m.alt {
        s.push_str("Alt+");
    }
    if m.shift && with_shift {
        s.push_str("Shift+");
    }
    s.push_str(key);
    s
}

/// Find and run the command bound to this key. Returns true if handled.
fn dispatch(app: &mut WordApp, key: Key, m: Modifiers) -> bool {
    let Some(name) = key_name(key) else { return false };
    // Alt+Shift+Minus is Nonbreaking Hyphen in Microsoft's list but still collapses a heading in
    // Outline view.
    if app.session.view.mode == doc4me_engine::layout::ViewMode::Outline && combo(m, name, true) == "Alt+Shift+-" {
        let _ = app.run("outline.collapse", json!({}));
        return true;
    }
    let reg = app.session.registry.clone();
    if let Some(spec) = app.session.keymap.resolve(&reg, &combo(m, name, true)) {
        let _ = app.run(spec.id, json!({}));
        return true;
    }
    // Shift extends caret movement; Shift+Backspace still erases (#707).
    if m.shift
        && let Some(spec) = app.session.keymap.resolve(&reg, &combo(m, name, false))
        && let Some(params) = shift_variant_params(spec.id)
    {
        let _ = app.run(spec.id, params);
        return true;
    }
    false
}

/// How a command bound to a plain key runs when Shift is also held and nothing is bound to the
/// shifted combo: caret moves extend the selection, Backspace erases as usual (#707).
fn shift_variant_params(id: &str) -> Option<Value> {
    if id.starts_with("caret.") {
        Some(json!({"extend": true}))
    } else if id == "text.backspace" {
        Some(json!({}))
    } else {
        None
    }
}

/// F1–F12: shortcuts on these work without the canvas focused.
fn is_function_key(key: Key) -> bool {
    matches!(key, Key::F1 | Key::F2 | Key::F3 | Key::F4 | Key::F5 | Key::F6 | Key::F7 | Key::F8 | Key::F9 | Key::F10 | Key::F11 | Key::F12)
}

/// Keys while editing inside an equation. Returns true when handled.
fn equation_key(app: &mut WordApp, key: Key, m: Modifiers) -> bool {
    if m.command || m.alt || (m.ctrl && !cfg!(target_os = "macos")) {
        return false;
    }
    let (id, params) = match key {
        Key::ArrowLeft => ("equation.move", json!({"dir": "left"})),
        Key::ArrowRight => ("equation.move", json!({"dir": "right"})),
        Key::ArrowUp => ("equation.move", json!({"dir": "up"})),
        Key::ArrowDown => ("equation.move", json!({"dir": "down"})),
        Key::Home => ("equation.move", json!({"dir": "home"})),
        Key::End => ("equation.move", json!({"dir": "end"})),
        Key::Tab => ("equation.move", json!({"dir": if m.shift { "prev" } else { "next" }})),
        Key::Backspace => ("equation.backspace", json!({})),
        Key::Delete => ("equation.delete", json!({})),
        Key::Enter => ("equation.enter", json!({})),
        Key::Escape => ("equation.exit", json!({})),
        _ => return false,
    };
    let _ = app.run(id, params);
    true
}

/// Whether releasing `key` with `modifiers` is the plain Paste shortcut (Mod+V or the Paste
/// key) — not Mod+Shift+V (Paste Formatting) or Mod+Alt+V (Paste Special) — so a clipboard
/// picture may be pasted on it (#45).
fn is_paste_release(key: Key, modifiers: egui::Modifiers) -> bool {
    (modifiers.command || key == Key::Paste) && !modifiers.alt && !modifiers.shift
}

/// The accents Word for Windows types with Ctrl and a punctuation key, then a letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accent {
    Grave,
    Acute,
    Circumflex,
    Tilde,
    Umlaut,
    Ring,
    Ligature,
    Cedilla,
    Stroke,
}

/// What Ctrl (with Shift or Alt) and this key do in Word for Windows: start an accent, or type
/// ¿ and ¡ straight away. Shifted keys are read as US keyboards place them (Shift+6 is ^).
pub enum AccentKey {
    Dead(Accent),
    Char(char),
}

pub fn accent_key(key: Key, m: Modifiers) -> Option<AccentKey> {
    let ctrl = m.ctrl || (m.command && !cfg!(target_os = "macos"));
    if !ctrl || cfg!(target_os = "macos") {
        return None;
    }
    use AccentKey::{Char, Dead};
    Some(match (key, m.shift, m.alt) {
        (Key::Backtick, false, false) => Dead(Accent::Grave),
        (Key::Quote, false, false) => Dead(Accent::Acute),
        (Key::Num6, true, false) => Dead(Accent::Circumflex),
        (Key::Backtick, true, false) => Dead(Accent::Tilde),
        (Key::Semicolon | Key::Colon, true, false) => Dead(Accent::Umlaut),
        (Key::Num2, true, false) => Dead(Accent::Ring),
        (Key::Num7, true, false) => Dead(Accent::Ligature),
        (Key::Comma, false, false) => Dead(Accent::Cedilla),
        (Key::Slash, false, false) => Dead(Accent::Stroke),
        (Key::Slash | Key::Questionmark, true, true) => Char('¿'),
        (Key::Num1 | Key::Exclamationmark, true, true) => Char('¡'),
        _ => return None,
    })
}

/// The letter `c` with the accent, as Word composes it; `None` when the pair has no character.
pub fn compose(accent: Accent, c: char) -> Option<char> {
    let pick = |from: &str, to: &str| from.chars().position(|x| x == c).and_then(|i| to.chars().nth(i));
    match accent {
        Accent::Grave => pick("aeiouAEIOU", "àèìòùÀÈÌÒÙ"),
        Accent::Acute => pick("aeiouyAEIOUYdD", "áéíóúýÁÉÍÓÚÝðÐ"),
        Accent::Circumflex => pick("aeiouAEIOU", "âêîôûÂÊÎÔÛ"),
        Accent::Tilde => pick("anoANO", "ãñõÃÑÕ"),
        Accent::Umlaut => pick("aeiouyAEIOUY", "äëïöüÿÄËÏÖÜŸ"),
        Accent::Ring => pick("aA", "åÅ"),
        Accent::Ligature => pick("aAoOs", "æÆœŒß"),
        Accent::Cedilla => pick("cC", "çÇ"),
        Accent::Stroke => pick("oO", "øØ"),
    }
}

/// The command a paste with these modifiers runs: Word for Windows (Microsoft 365) pastes text only
/// on Mod+Shift+V and formatting on Mod+Alt+V; Word for Mac pastes formatting on ⌘⇧V and opens
/// Paste Special on ⌘⌥V.
fn paste_command(alt: bool, shift: bool) -> &'static str {
    match (alt, shift) {
        (true, true) => "edit.pasteText",
        (false, true) if cfg!(target_os = "macos") => "edit.pasteFormat",
        (false, true) => "edit.pasteText",
        (true, false) if cfg!(target_os = "macos") => "edit.pasteSpecial",
        (true, false) => "edit.pasteFormat",
        _ => "edit.paste",
    }
}

/// Events for the focused canvas: text, editing keys, clipboard, IME. While keytips are showing
/// every key belongs to them (letters pick badges, Escape cancels), so nothing here runs.
pub fn canvas_events(app: &mut WordApp, ctx: &egui::Context) {
    if app.ui.keytips != crate::keytips::Phase::Off {
        return;
    }
    let events = ctx.input(|i| i.events.clone());
    if app.session.view.read_mode {
        // Read Mode is for navigation and selection, not typing into the document. The ribbon's
        // Edit Document button or Escape returns to Print Layout before editing.
        for e in &events {
            match e {
                egui::Event::Key { key: Key::Escape, pressed: true, .. } => {
                    let _ = app.run("view.printLayout", json!({}));
                    break;
                }
                egui::Event::Key { key: Key::PageDown | Key::ArrowRight, pressed: true, modifiers, .. } if modifiers.is_none() => {
                    crate::canvas::turn_read_page(app, 1.0)
                }
                egui::Event::Key { key: Key::PageUp | Key::ArrowLeft, pressed: true, modifiers, .. } if modifiers.is_none() => {
                    crate::canvas::turn_read_page(app, -1.0)
                }
                egui::Event::Copy => {
                    let selected = app.session.selected_text();
                    if !selected.is_empty() {
                        ctx.copy_text(selected);
                    }
                }
                _ => {}
            }
        }
        return;
    }
    for e in events {
        // An Alt shortcut (Alt+Shift+D, Alt+F7…) may also send its key's text; it isn't typed.
        match &e {
            egui::Event::Key { pressed: true, .. } => app.canvas.swallow_text = false,
            egui::Event::Text(_) if std::mem::take(&mut app.canvas.swallow_text) => continue,
            _ => {}
        }
        if let egui::Event::Paste(t) = &e {
            app.canvas.pasted = true;
            // Copied picture files arrive as their paths: insert the pictures (#45).
            if app.session.math.is_none() && !ctx.input(|i| i.modifiers.alt) && crate::paste_picture::paste_named_files(app, t) {
                continue;
            }
        }
        // Desktop: Mod+V with no text on the clipboard sends no paste event, only the key's
        // release; paste the clipboard's picture then (#45). A paste event before the release
        // means it was text. (The web reads pasted pictures itself.)
        if let egui::Event::Key { key: key @ (Key::V | Key::Paste), pressed: false, modifiers, .. } = &e {
            let paste_keys = is_paste_release(*key, *modifiers);
            let pasted = std::mem::take(&mut app.canvas.pasted);
            if !pasted && paste_keys && app.session.math.is_none() && app.services.clipboard_picture.is_some() {
                let _ = app.run("edit.paste", json!({}));
            }
            continue;
        }
        // Editing an equation: text and editing keys go into it.
        if app.session.math.is_some() {
            match &e {
                egui::Event::Text(t) => {
                    let m = ctx.input(|i| i.modifiers);
                    if !(m.command || t.is_empty() || (m.ctrl && !cfg!(target_os = "macos"))) && t.chars().all(|c| !c.is_control()) {
                        let _ = app.run("equation.type", json!({"text": t}));
                    }
                    continue;
                }
                egui::Event::Ime(egui::ImeEvent::Commit(text)) => {
                    app.canvas.ime_preedit.clear();
                    if !text.is_empty() {
                        let _ = app.run("equation.type", json!({"text": text}));
                    }
                    continue;
                }
                egui::Event::Paste(t) => {
                    let _ = app.run("equation.type", json!({"text": t}));
                    continue;
                }
                egui::Event::Key { key, pressed: true, modifiers, .. } if equation_key(app, *key, *modifiers) => continue,
                _ => {}
            }
        }
        match e {
            egui::Event::Text(t) => {
                let m = ctx.input(|i| i.modifiers);
                if m.command || (m.ctrl && !cfg!(target_os = "macos")) {
                    continue;
                }
                if t.chars().all(|c| !c.is_control()) && !t.is_empty() {
                    // Typing with a text box selected types into it.
                    crate::objects::enter_text_box(app);
                    // After a Ctrl+accent key the letter takes the accent (or comes as typed).
                    let t = match app.canvas.dead_accent.take() {
                        Some(a) => {
                            let mut cs = t.chars();
                            let first = cs.next().and_then(|c| compose(a, c));
                            first.map_or(t.clone(), |c| std::iter::once(c).chain(cs).collect())
                        }
                        None => t,
                    };
                    let _ = app.run("text.insert", json!({"text": t}));
                }
            }
            egui::Event::Ime(egui::ImeEvent::Preedit { text, .. }) => app.canvas.ime_preedit = text,
            egui::Event::Ime(egui::ImeEvent::Commit(text)) => {
                app.canvas.ime_preedit.clear();
                if !text.is_empty() {
                    let _ = app.run("text.insert", json!({"text": text}));
                }
            }
            egui::Event::Paste(t) => {
                // Mod+Shift+V and Mod+Alt+V arrive as a paste too (the shell reads the clipboard).
                let m = ctx.input(|i| i.modifiers);
                let id = paste_command(m.alt, m.shift);
                let params = if id == "edit.pasteFormat" { json!({}) } else { json!({"text": t}) };
                let _ = app.run(id, params);
            }
            egui::Event::Copy | egui::Event::Cut => {
                // Mod+Alt+C (Word for Windows) or Mod+Shift+C arrive as a copy: Copy Formatting.
                let m = ctx.input(|i| i.modifiers);
                if matches!(e, egui::Event::Copy) && (m.shift || (m.alt && !cfg!(target_os = "macos"))) {
                    let _ = app.run("edit.copyFormat", json!({}));
                    continue;
                }
                let id = if matches!(e, egui::Event::Cut) { "edit.cut" } else { "edit.copy" };
                if let Ok(r) = app.run(id, json!({}))
                    && let Some(t) = r.get("text").and_then(Value::as_str)
                    && !t.is_empty()
                {
                    ctx.copy_text(t.to_string());
                }
            }
            egui::Event::Key { key, pressed: true, modifiers, .. } => {
                // Floating shape navigation (Ctrl+Alt+5): Tab goes from shape to shape.
                if key == Key::Tab && app.session.shape_nav && !(modifiers.command || modifiers.ctrl || modifiers.alt) {
                    let _ = app.run("select.floatingShapes", json!({"next": true, "back": modifiers.shift}));
                    continue;
                }
                if key == Key::Escape && app.session.painter.is_some() {
                    app.session.painter = None;
                    continue;
                }
                // The lasso selection and Ink Replay take Delete and Escape first.
                if crate::ink::key(app, key) {
                    continue;
                }
                // Escape puts the pen or eraser down (back to selecting).
                if key == Key::Escape && app.session.view.draw.mode != doc4me_engine::cmd::draw::DrawMode::Select {
                    app.canvas.ink = None;
                    let _ = app.run("draw.select", json!({}));
                    continue;
                }
                if key == Key::Escape && crate::table_pen::stop(app) {
                    continue;
                }
                // File › Options › Advanced: the Insert key switches overtype only when asked to.
                if key == Key::Insert && modifiers.is_none() {
                    if app.session.prefs.insert_key_overtype {
                        let _ = app.run("text.overtype", json!({}));
                    }
                    continue;
                }
                if crate::objects::key(app, key, modifiers) {
                    continue;
                }
                // Word's Ctrl+accent keys.
                match accent_key(key, modifiers) {
                    Some(AccentKey::Dead(a)) => {
                        app.canvas.dead_accent = Some(a);
                        continue;
                    }
                    Some(AccentKey::Char(c)) => {
                        app.canvas.swallow_text = true;
                        let _ = app.run("text.insert", json!({"text": c.to_string()}));
                        continue;
                    }
                    None => {}
                }
                if !matches!(key, Key::Escape) && key_name(key).is_some_and(|n| n.len() > 1) {
                    // A named key (arrows, Enter…) cancels a waiting accent; letters use it.
                    app.canvas.dead_accent = None;
                }
                // Escape leaves a text box to just after it in the body.
                if key == Key::Escape
                    && crate::canvas::in_text_box(app)
                    && app.session.sel.is_collapsed()
                    && let doc4me_doc::StoryRef::Part(id) = app.session.sel.focus.story
                    && let Some(pos) = app.session.doc.text_box_anchor(id)
                {
                    let _ = app.run("caret.set", json!({"pos": pos}));
                    continue;
                }
                if key == Key::Escape && matches!(app.session.sel.focus.story, doc4me_doc::StoryRef::Part(_)) && app.session.sel.is_collapsed() {
                    let _ = app.run("insert.closeHeader", json!({}));
                    continue;
                }
                app.canvas.swallow_text = dispatch(app, key, modifiers) && modifiers.alt;
            }
            _ => {}
        }
    }
}

/// Command shortcuts when the canvas isn't focused but no text field is either.
pub fn global_shortcuts(app: &mut WordApp, ctx: &egui::Context) {
    // Customize Keyboard records the keys pressed; they never run commands meanwhile.
    if crate::windows::any_canvas_focused(app)
        || ctx.egui_wants_keyboard_input()
        || matches!(app.dialog, Some(crate::dialogs::Dialog::CustomizeKeyboard { .. }))
    {
        return;
    }
    let events = ctx.input(|i| i.events.clone());
    for e in events {
        if let egui::Event::Key { key, pressed: true, modifiers, .. } = e
            && (modifiers.command || is_function_key(key))
        {
            dispatch(app, key, modifiers);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paste_modifiers_follow_word_for_windows() {
        if cfg!(target_os = "macos") {
            return;
        }
        assert_eq!(paste_command(false, false), "edit.paste");
        assert_eq!(paste_command(false, true), "edit.pasteText");
        assert_eq!(paste_command(true, false), "edit.pasteFormat");
        assert_eq!(paste_command(true, true), "edit.pasteText");
    }

    #[test]
    fn ctrl_alt_c_and_v_copy_and_paste_formatting() {
        if cfg!(target_os = "macos") {
            return;
        }
        let mut a = crate::WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("Bold plain")), crate::Services::default());
        a.run("select.range", json!({"anchor": {"block": 0, "off": 0}, "focus": {"block": 0, "off": 4}})).unwrap();
        a.run("format.bold", json!({})).unwrap();
        let ctx = egui::Context::default();
        let alt_ctrl = Modifiers { alt: true, ctrl: true, command: true, ..Default::default() };
        let frame = |a: &mut crate::WordApp, modifiers: Modifiers, e: egui::Event| {
            ctx.run_ui(egui::RawInput { events: vec![egui::Event::ModifiersChanged(modifiers), e], ..Default::default() }, |ui| {
                canvas_events(a, ui.ctx())
            })
            .drop_without_applying_deltas();
        };
        frame(&mut a, alt_ctrl, egui::Event::Copy);
        a.run("select.range", json!({"anchor": {"block": 0, "off": 5}, "focus": {"block": 0, "off": 10}})).unwrap();
        frame(&mut a, alt_ctrl, egui::Event::Paste("clipboard text".into()));
        let p = a.session.doc.para_at(&doc4me_doc::Pos::body(0, 0)).unwrap().clone();
        assert_eq!(p.text, "Bold plain", "formatting only, no text pasted");
        assert_eq!(p.props_of_char(6).bold, Some(true), "Ctrl+Alt+V pasted the bold");
        // Ctrl+Shift+V pastes the clipboard's text only.
        a.run("caret.docEnd", json!({})).unwrap();
        frame(&mut a, Modifiers { shift: true, ctrl: true, command: true, ..Default::default() }, egui::Event::Paste(" more".into()));
        assert_eq!(a.session.doc.plain_text(doc4me_doc::StoryRef::Body), "Bold plain more");
    }

    #[test]
    fn ctrl_accent_keys_compose_like_word() {
        if cfg!(target_os = "macos") {
            return;
        }
        let mut a = crate::WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), crate::Services::default());
        let ctx = egui::Context::default();
        let ctrl = |shift, alt| Modifiers { ctrl: true, command: true, shift, alt, ..Default::default() };
        let key = |key, modifiers| egui::Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers };
        let type_after = |a: &mut crate::WordApp, k: Key, m: Modifiers, letter: &str| {
            ctx.run_ui(egui::RawInput { events: vec![egui::Event::ModifiersChanged(m), key(k, m)], ..Default::default() }, |ui| {
                canvas_events(a, ui.ctx())
            })
            .drop_without_applying_deltas();
            if !letter.is_empty() {
                ctx.run_ui(
                    egui::RawInput {
                        events: vec![egui::Event::ModifiersChanged(Modifiers::NONE), egui::Event::Text(letter.into())],
                        ..Default::default()
                    },
                    |ui| canvas_events(a, ui.ctx()),
                )
                .drop_without_applying_deltas();
            }
        };
        for (k, m, letter) in [
            (Key::Backtick, ctrl(false, false), "e"),
            (Key::Quote, ctrl(false, false), "E"),
            (Key::Num6, ctrl(true, false), "o"),
            (Key::Backtick, ctrl(true, false), "n"),
            (Key::Semicolon, ctrl(true, false), "u"),
            (Key::Num2, ctrl(true, false), "a"),
            (Key::Num7, ctrl(true, false), "o"),
            (Key::Num7, ctrl(true, false), "s"),
            (Key::Comma, ctrl(false, false), "c"),
            (Key::Quote, ctrl(false, false), "d"),
            (Key::Slash, ctrl(false, false), "O"),
            (Key::Slash, ctrl(true, true), ""),
            (Key::Num1, ctrl(true, true), ""),
            (Key::Backtick, ctrl(false, false), "x"),
        ] {
            type_after(&mut a, k, m, letter);
        }
        assert_eq!(a.session.doc.plain_text(doc4me_doc::StoryRef::Body), "èÉôñüåœßçðØ¿¡x");
    }

    #[test]
    fn shift_backspace_still_erases() {
        assert_eq!(shift_variant_params("text.backspace"), Some(json!({})));
        assert_eq!(shift_variant_params("caret.left"), Some(json!({"extend": true})));
        assert_eq!(shift_variant_params("text.delete"), None);
    }

    #[test]
    fn only_plain_paste_releases_paste_a_clipboard_picture() {
        let m = |command, shift, alt| egui::Modifiers { command, shift, alt, ..Default::default() };
        assert!(is_paste_release(Key::V, m(true, false, false)), "Mod+V");
        assert!(is_paste_release(Key::Paste, m(false, false, false)), "the Paste key");
        assert!(!is_paste_release(Key::V, m(true, true, false)), "Mod+Shift+V is Paste Formatting");
        assert!(!is_paste_release(Key::V, m(true, false, true)), "Mod+Alt+V is Paste Special");
        assert!(!is_paste_release(Key::V, m(false, false, false)), "a plain V");
    }
}
