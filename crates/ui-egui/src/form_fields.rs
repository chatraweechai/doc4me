//! Legacy form fields on the page. In a document protected for filling in forms a click ticks a
//! check box, opens a drop-down's list, or opens a box to type a text field's value (also when
//! Tab reaches a text field); its status bar text shows while it is selected, F1 shows its help.
//! Outside protection a double-click opens the field's Options. Everything ends in a
//! `developer.*` command.

use doc4me_doc::Pos;
use doc4me_doc::form::{FormKind, TextType};
use doc4me_doc::para::OBJ;
use doc4me_layout::DocLayout;
use egui::{Rect, pos2, vec2};
use serde_json::json;

use crate::WordApp;
use crate::theme::Tokens;

/// Most fields tracked at once (a long form).
const MAX_FIELDS: usize = 2000;

/// What the canvas remembers between frames.
#[derive(Default)]
pub struct FormUi {
    /// Form fields shown this frame: (screen rect, where the field is).
    fields: Vec<(Rect, Pos)>,
    popup: Option<Popup>,
    /// The field whose text box was closed with Esc (not opened again until another is selected).
    dismissed: Option<Pos>,
    /// Where the document's form fields are, for the document revision it was found in (a
    /// long document isn't walked every frame).
    found: Option<((u64, u64), Vec<Pos>)>,
}

struct Popup {
    at: Pos,
    anchor: Rect,
    /// `Some`: the text being typed into a text field; `None`: a drop-down's list.
    text: Option<String>,
    /// The text box has had focus (so losing it means the user moved on).
    focused: bool,
}

fn screen(pr: Rect, scale: f32, r: doc4me_geom::Rect) -> Rect {
    Rect::from_min_size(pos2(pr.min.x + r.x * scale, pr.min.y + r.y * scale), vec2(r.w * scale, r.h * scale))
}

fn protected(app: &WordApp) -> bool {
    app.session.doc.settings.protection.as_deref() == Some("forms")
}

fn after(at: &Pos) -> Pos {
    Pos { off: at.off + OBJ.len_utf8(), ..at.clone() }
}

/// Note where the fields are (inside the canvas viewport, after the selection), and open the
/// text box of a text field the selection has reached in a protected form.
pub fn paint(app: &mut WordApp, layout: &DocLayout, rects: &[Rect], scale: f32) {
    app.forms.fields.clear();
    let rev = (app.session.document_id(), app.session.rev());
    if app.forms.found.as_ref().is_none_or(|(r, _)| *r != rev) {
        app.forms.found = Some((rev, doc4me_engine::cmd::forms::all(&app.session)));
    }
    let all = app.forms.found.as_ref().map(|(_, a)| a.clone()).unwrap_or_default();
    for at in all.into_iter().take(MAX_FIELDS) {
        let pieces = layout.selection_rects(&app.session.doc, &at, &after(&at), app.session.page_hint);
        if let Some((pi, r)) = pieces.first()
            && let Some(pr) = rects.get(*pi)
        {
            app.forms.fields.push((screen(*pr, scale, *r), at));
        }
    }
    let sel = (app.session.sel.anchor.clone(), app.session.sel.focus.clone());
    let selected = app.forms.fields.iter().find(|(_, at)| sel == (at.clone(), after(at)) || sel == (after(at), at.clone())).cloned();
    if app.forms.dismissed.as_ref().is_some_and(|d| selected.as_ref().is_none_or(|(_, at)| at != d)) {
        app.forms.dismissed = None;
    }
    if !protected(app) {
        app.forms.popup = None;
        return;
    }
    if let Some((r, at)) = selected {
        // The status bar text of the selected field.
        let field = doc4me_engine::cmd::forms::field_at(&app.session, &at);
        if let Some((f, value)) = field
            && app.forms.popup.as_ref().is_none_or(|p| p.at != at)
            && app.forms.dismissed.as_ref() != Some(&at)
            && f.enabled
            && matches!(f.kind, FormKind::Text { input: TextType::Regular | TextType::Number | TextType::Date, .. })
        {
            let value = if value == doc4me_doc::form::BLANK_TEXT { String::new() } else { value };
            app.forms.popup = Some(Popup { at, anchor: r, text: Some(value), focused: false });
        }
    } else if app.forms.popup.as_ref().is_some_and(|p| p.text.is_some()) {
        app.forms.popup = None;
    }
}

/// After the canvas handled the pointer: in a protected form a click fills the field in;
/// otherwise a double-click opens its Options.
pub fn clicks(app: &mut WordApp, resp: &egui::Response) {
    let Some(p) = resp.interact_pointer_pos() else { return };
    let Some((r, at)) = app.forms.fields.iter().find(|(r, _)| r.expand(1.0).contains(p)).cloned() else { return };
    let at_json = serde_json::to_value(&at).unwrap_or_default();
    if !protected(app) {
        if resp.double_clicked() {
            app.session.sel = doc4me_engine::Selection { anchor: at.clone(), focus: after(&at) };
            crate::widgets::run_from_ribbon(app, "developer.formFieldOptions", json!({}));
        }
        return;
    }
    if !resp.clicked() {
        return;
    }
    let Some((f, value)) = doc4me_engine::cmd::forms::field_at(&app.session, &at) else { return };
    app.session.sel = doc4me_engine::Selection { anchor: at.clone(), focus: after(&at) };
    if !f.enabled {
        return;
    }
    match f.kind {
        FormKind::CheckBox { .. } => {
            let now = f.checked() != Some(true);
            if let Err(e) = app.run("developer.formFieldValue", json!({"at": at_json, "value": now})) {
                app.status(e);
            }
        }
        FormKind::DropDown { .. } => {
            app.forms.popup = Some(Popup { at, anchor: r, text: None, focused: false });
        }
        FormKind::Text { input, .. } if input.fillable() => {
            let value = if value == doc4me_doc::form::BLANK_TEXT { String::new() } else { value };
            app.forms.popup = Some(Popup { at, anchor: r, text: Some(value), focused: false });
        }
        FormKind::Text { .. } => {}
    }
}

/// The selected field's status bar text, for the status bar (a protected form).
pub fn status_text(app: &WordApp) -> Option<String> {
    if !protected(app) {
        return None;
    }
    let (a, b) = app.session.sel.ordered();
    let at = (b == after(&a)).then_some(a)?;
    let (f, _) = doc4me_engine::cmd::forms::field_at(&app.session, &at)?;
    help_text(app, &f.status)
}

/// Help text as typed, or the text of the AutoText entry it names (none when there's no such entry).
fn help_text(app: &WordApp, h: &doc4me_doc::form::FormHelp) -> Option<String> {
    let text = if h.auto_text { app.session.building_blocks.get(&h.text)?.plain_text() } else { h.text.clone() };
    let text = text.trim().to_string();
    (!text.is_empty()).then_some(text)
}

/// The open list or text box (outside the canvas viewport, above everything), and F1 help.
pub fn popup(app: &mut WordApp, ctx: &egui::Context) {
    // F1 on a selected field shows its help text.
    if protected(app)
        && ctx.input(|i| i.key_pressed(egui::Key::F1))
        && let Some(help) = selected_help(app)
    {
        app.status(help);
    }
    let Some(pop) = app.forms.popup.as_mut() else { return };
    let (at, anchor) = (pop.at.clone(), pop.anchor);
    let Some((f, _)) = doc4me_engine::cmd::forms::field_at(&app.session, &at) else {
        app.forms.popup = None;
        return;
    };
    let t = Tokens::get(ctx);
    let at_json = serde_json::to_value(&at).unwrap_or_default();
    let mut close = false;
    // (commit, then go to the next or previous field)
    let mut commit: Option<Option<bool>> = None;
    let area = egui::Area::new(egui::Id::new("form_field_popup")).order(egui::Order::Foreground).fixed_pos(pos2(anchor.min.x, anchor.max.y + 2.0));
    let shown = area.show(ctx, |ui| {
        egui::Frame::popup(ui.style()).fill(t.menu).show(ui, |ui| match (&f.kind, pop.text.as_mut()) {
            (FormKind::DropDown { entries, .. }, _) => {
                ui.set_min_width(anchor.width().max(160.0));
                egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                    for (i, e) in entries.iter().enumerate() {
                        if ui.selectable_label(f.chosen() == Some(i), e.as_str()).clicked() {
                            commit = Some(None);
                            pop.text = Some(i.to_string());
                        }
                    }
                });
            }
            (FormKind::Text { max_len, .. }, Some(text)) => {
                let mut edit = egui::TextEdit::singleline(text).desired_width(anchor.width().max(220.0));
                if *max_len > 0 {
                    edit = edit.char_limit(usize::from(*max_len));
                }
                let r = ui.add(edit);
                if !pop.focused {
                    r.request_focus();
                    pop.focused = true;
                }
                let (tab, shift, enter) = ui.input(|i| (i.key_pressed(egui::Key::Tab), i.modifiers.shift, i.key_pressed(egui::Key::Enter)));
                if tab {
                    commit = Some(Some(shift));
                } else if enter || (r.lost_focus() && !ui.input(|i| i.key_pressed(egui::Key::Escape))) {
                    commit = Some(Some(false));
                }
            }
            _ => close = true,
        });
    });
    let escape = ctx.input(|i| i.key_pressed(egui::Key::Escape));
    let outside = ctx.input(|i| i.pointer.any_pressed())
        && ctx.input(|i| i.pointer.interact_pos()).is_some_and(|p| !shown.response.rect.contains(p) && !anchor.contains(p));
    if escape {
        app.forms.dismissed = Some(at.clone());
        app.forms.popup = None;
        app.canvas.want_focus = true;
        return;
    }
    let Some(pop) = app.forms.popup.take() else { return };
    let typed = pop.text.clone().unwrap_or_default();
    let is_list = matches!(f.kind, FormKind::DropDown { .. });
    if commit.is_none() && outside && !is_list {
        commit = Some(None);
    }
    if let Some(then) = commit {
        let value = if is_list { json!(typed.parse::<u64>().unwrap_or(0)) } else { json!(typed) };
        match app.run("developer.formFieldValue", json!({"at": at_json, "value": value})) {
            Ok(_) => {
                if let Some(back) = then {
                    let _ = app.run(if back { "text.backTab" } else { "text.tab" }, json!({}));
                }
            }
            Err(e) => {
                // Keep the box open on what was typed so it can be put right.
                app.status(e);
                app.forms.popup = Some(Popup { focused: false, ..pop });
                return;
            }
        }
        app.canvas.want_focus = true;
        return;
    }
    if close || (outside && is_list) {
        app.canvas.want_focus = true;
        return;
    }
    app.forms.popup = Some(pop);
}

fn selected_help(app: &WordApp) -> Option<String> {
    let (a, b) = app.session.sel.ordered();
    let at = (b == after(&a)).then_some(a)?;
    let (f, _) = doc4me_engine::cmd::forms::field_at(&app.session, &at)?;
    help_text(app, &f.help)
}

#[cfg(test)]
mod tests {
    use egui_kittest::Harness;
    use serde_json::json;

    use crate::{Services, WordApp};

    fn harness(a: WordApp) -> Harness<'static, WordApp> {
        let mut h = Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        h
    }

    fn key(h: &mut Harness<'_, WordApp>, key: egui::Key) {
        h.event(egui::Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers: Default::default() });
        h.event(egui::Event::Key { key, physical_key: None, pressed: false, repeat: false, modifiers: Default::default() });
    }

    fn value(h: &mut Harness<'_, WordApp>, name: &str) -> serde_json::Value {
        h.state_mut().session.run("developer.formFieldInfo", &json!({"field": name})).unwrap()["value"].clone()
    }

    #[test]
    fn a_protected_form_is_filled_in_on_the_page() {
        let mut app = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("Name: ")), Services::default());
        app.session.run("caret.docEnd", &json!({})).unwrap();
        app.run("developer.formField", json!({"type": "text", "status": {"text": "Your name"}, "help": {"text": "As on your passport"}})).unwrap();
        app.run("text.insert", json!({"text": " Agree: "})).unwrap();
        app.run("developer.formField", json!({"type": "checkBox"})).unwrap();
        app.run("review.restrict", json!({"mode": "forms"})).unwrap();
        app.session.sel = doc4me_engine::Selection::caret(doc4me_doc::Pos::new(doc4me_doc::StoryRef::Body, doc4me_doc::Path::top(0), 0));
        let mut h = harness(app);
        // Tab selects the text field and opens its box; its status bar text shows.
        h.state_mut().run("text.tab", json!({})).unwrap();
        h.run_steps(3);
        assert!(h.state().forms.popup.as_ref().is_some_and(|p| p.text.as_deref() == Some("")), "the text box is open");
        assert_eq!(super::status_text(h.state()).as_deref(), Some("Your name"));
        assert_eq!(super::selected_help(h.state()).as_deref(), Some("As on your passport"));
        h.event(egui::Event::Text("Ada".into()));
        h.run_steps(1);
        key(&mut h, egui::Key::Enter);
        h.run_steps(3);
        assert_eq!(value(&mut h, "Text1"), "Ada");
        // Enter moved on to the check box; a click on it ticks it.
        let rect = h
            .state()
            .forms
            .fields
            .iter()
            .find(|(_, at)| doc4me_engine::cmd::forms::field_at(&h.state().session, at).is_some_and(|(f, _)| f.name == "Check1"))
            .map(|(r, _)| *r)
            .unwrap();
        let at = rect.center();
        h.event(egui::Event::PointerMoved(at));
        h.event(egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed: true, modifiers: Default::default() });
        h.step();
        h.event(egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed: false, modifiers: Default::default() });
        h.run_steps(3);
        assert_eq!(value(&mut h, "Check1"), true);
        // The text around the fields stays as it was.
        let text = h.state().session.doc.plain_text(doc4me_doc::StoryRef::Body);
        assert!(text.starts_with("Name: Ada Agree: "), "{text:?}");
    }
}
