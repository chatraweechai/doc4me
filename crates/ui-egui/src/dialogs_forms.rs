//! Developer › Legacy Tools dialogs: Form Field (the kind to insert), Text, Check Box and
//! Drop-Down Form Field Options, and Form Field Help Text. OK runs `developer.formField`,
//! `developer.formFieldOptions` and `developer.formFieldHelp`, so scripts and agents get the
//! same result without them. Macros named in a field are kept in the file but never run, so the
//! dialogs don't offer them.

use egui::Ui;
use serde::Serialize;
use serde_json::{Value, json};

use crate::WordApp;
use crate::dialogs::buttons;

/// Text types: the engine's name and the dialog's label.
const TEXT_TYPES: [(&str, &str); 6] = [
    ("regular", "Regular text"),
    ("number", "Number"),
    ("date", "Date"),
    ("currentDate", "Current date"),
    ("currentTime", "Current time"),
    ("calculated", "Calculation"),
];

/// Word's text formats offered for each text type (any other can be typed).
fn formats(text_type: &str) -> &'static [&'static str] {
    match text_type {
        "number" | "calculated" => &["0", "0.00", "#,##0", "#,##0.00", "$#,##0.00;($#,##0.00)", "0%", "0.00%"],
        "date" | "currentDate" => &["M/d/yyyy", "dddd, MMMM d, yyyy", "MMMM d, yyyy", "M/d/yy", "yyyy-MM-dd", "d-MMM-yy", "dd/MM/yyyy"],
        "currentTime" => &["h:mm am/pm", "h:mm:ss am/pm", "H:mm", "H:mm:ss"],
        _ => &["Uppercase", "Lowercase", "First capital", "Title case"],
    }
}

/// One help text being edited: none, an AutoText entry, or typed.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct HelpText {
    /// 0 none, 1 AutoText entry, 2 typed.
    pub mode: u8,
    pub auto_text: String,
    pub own: String,
}

impl HelpText {
    fn read(v: &Value) -> HelpText {
        let text = v.get("text").and_then(Value::as_str).unwrap_or("").to_string();
        match v.get("autoText").and_then(Value::as_bool) {
            _ if text.is_empty() => HelpText::default(),
            Some(true) => HelpText { mode: 1, auto_text: text, own: String::new() },
            _ => HelpText { mode: 2, auto_text: String::new(), own: text },
        }
    }
    fn param(&self) -> Value {
        match self.mode {
            1 if !self.auto_text.trim().is_empty() => json!({"text": self.auto_text.trim(), "autoText": true}),
            2 if !self.own.is_empty() => json!({"text": self.own}),
            _ => Value::Null,
        }
    }
}

/// Status bar and F1 help text.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct HelpForm {
    /// 0 Status Bar, 1 Help Key (F1).
    pub tab: usize,
    pub status: HelpText,
    pub help: HelpText,
}

/// A form field's options as the dialog edits them.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct OptionsForm {
    /// Where the field is (found again there on OK).
    pub at: Value,
    /// text, checkBox or dropDown.
    pub kind: String,
    pub name: String,
    pub enabled: bool,
    pub calc_on_exit: bool,
    pub text_type: String,
    pub default_text: String,
    /// 0: unlimited.
    pub max_length: u32,
    pub format: String,
    pub auto_size: bool,
    pub size: f32,
    pub default_checked: bool,
    pub entries: Vec<String>,
    pub new_entry: String,
    pub selected: Option<usize>,
    pub help: HelpForm,
}

impl OptionsForm {
    /// The form for the field at `at` (a position), else at the selection, if there is one.
    pub fn read(app: &mut WordApp, at: Option<&Value>) -> Option<OptionsForm> {
        let params = match at {
            Some(at) => json!({"at": at}),
            None => json!({}),
        };
        let v = app.session.run("developer.formFieldInfo", &params).ok()?;
        let s = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        let b = |k: &str| v.get(k).and_then(Value::as_bool).unwrap_or(false);
        let kind = s("type");
        let size = v.get("size").and_then(Value::as_f64);
        Some(OptionsForm {
            at: v.get("at").cloned().unwrap_or(Value::Null),
            name: s("name"),
            enabled: b("enabled"),
            calc_on_exit: b("calcOnExit"),
            text_type: if kind == "text" { s("textType") } else { String::new() },
            default_text: if kind == "text" { s("default") } else { String::new() },
            max_length: v.get("maxLength").and_then(Value::as_u64).unwrap_or(0) as u32,
            format: s("format"),
            auto_size: size.is_none(),
            size: size.unwrap_or(10.0) as f32,
            default_checked: kind == "checkBox" && b("default"),
            entries: v
                .get("entries")
                .and_then(Value::as_array)
                .map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect())
                .unwrap_or_default(),
            new_entry: String::new(),
            selected: None,
            help: HelpForm { tab: 0, status: HelpText::read(&v["status"]), help: HelpText::read(&v["help"]) },
            kind,
        })
    }

    /// The `developer.formFieldOptions` parameters the form stands for.
    pub fn params(&self) -> Value {
        let mut v = json!({
            "at": self.at,
            "name": self.name.trim(),
            "enabled": self.enabled,
            "calcOnExit": self.calc_on_exit,
            "help": self.help.help.param(),
            "status": self.help.status.param(),
        });
        match self.kind.as_str() {
            "text" => {
                v["textType"] = json!(self.text_type);
                v["default"] = json!(self.default_text);
                v["maxLength"] = json!(self.max_length);
                v["format"] = json!(self.format.trim());
            }
            "checkBox" => {
                v["size"] = if self.auto_size { Value::Null } else { json!(self.size) };
                v["default"] = json!(self.default_checked);
            }
            _ => v["entries"] = json!(self.entries),
        }
        v
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "dialogKind", rename_all = "camelCase")]
pub enum FormDialog {
    /// Form Field: which kind to insert.
    Insert {
        kind: String,
    },
    Options {
        form: OptionsForm,
    },
    /// Help Text: of the field at `at` on its own, or (`back`) for the Options dialog it was
    /// opened from, which it returns to.
    Help {
        at: Value,
        form: HelpForm,
        back: Option<Box<OptionsForm>>,
    },
    /// Section Protection (Restrict Editing › Filling in forms › Select sections…): which
    /// sections stay protected.
    Sections {
        protected: Vec<bool>,
    },
}

impl FormDialog {
    pub fn name(&self) -> &'static str {
        match self {
            FormDialog::Insert { .. } => "insertFormField",
            FormDialog::Options { .. } => "formFieldOptions",
            FormDialog::Help { .. } => "formFieldHelp",
            FormDialog::Sections { .. } => "sectionProtection",
        }
    }

    /// The window title (English; the caller translates it).
    pub fn title(&self) -> &'static str {
        match self {
            FormDialog::Insert { .. } => "Form Field",
            FormDialog::Options { form } => match form.kind.as_str() {
                "text" => "Text Form Field Options",
                "checkBox" => "Check Box Form Field Options",
                _ => "Drop-Down Form Field Options",
            },
            FormDialog::Help { .. } => "Form Field Help Text",
            FormDialog::Sections { .. } => "Section Protection",
        }
    }
}

/// The dialog `ui.dialog` opens by `name` (`None`: not one of these, or no form field at the
/// selection for Options and Help Text).
pub fn open(name: &str, app: &mut WordApp) -> Option<FormDialog> {
    match name {
        "insertFormField" => Some(FormDialog::Insert { kind: "text".into() }),
        "sectionProtection" => {
            let v = app.session.run("review.protectSections", &json!({})).ok()?;
            let protected = v["sections"].as_array().map(|a| a.iter().map(|b| b.as_bool().unwrap_or(true)).collect()).unwrap_or_default();
            Some(FormDialog::Sections { protected })
        }
        "formFieldOptions" | "formFieldHelp" => {
            let Some(form) = OptionsForm::read(app, None) else {
                app.status(tl!("Put the caret on a form field first."));
                return None;
            };
            Some(if name == "formFieldOptions" {
                FormDialog::Options { form }
            } else {
                FormDialog::Help { at: form.at.clone(), form: form.help, back: None }
            })
        }
        _ => None,
    }
}

/// Draws the dialog; returns true to close it.
pub fn body(app: &mut WordApp, ui: &mut Ui, d: &mut FormDialog) -> bool {
    match d {
        FormDialog::Sections { protected } => {
            ui.label(tl!("Protected sections:"));
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(240.0);
                egui::ScrollArea::vertical().id_salt("section_protection").max_height(220.0).show(ui, |ui| {
                    for (i, on) in protected.iter_mut().enumerate() {
                        ui.checkbox(on, crate::i18n::fmt(tl!("Section {n}"), &[("n", &(i + 1).to_string())]));
                    }
                });
            });
            let (ok, cancel) = buttons(ui, tl!("OK"));
            if ok {
                match app.run("review.protectSections", json!({"protected": protected})) {
                    Ok(_) => return true,
                    Err(e) => app.status(e),
                }
            }
            cancel
        }
        FormDialog::Insert { kind } => {
            ui.label(tl!("Type:"));
            for (k, label) in [("text", "Text"), ("checkBox", "Check box"), ("dropDown", "Drop-down")] {
                ui.radio_value(kind, k.to_string(), tl!(label));
            }
            let mut options = false;
            ui.add_space(4.0);
            if ui.button(tl!("Options…")).clicked() {
                options = true;
            }
            let (ok, cancel) = buttons(ui, tl!("OK"));
            if ok || options {
                let at = match app.run("developer.formField", json!({"type": kind.as_str()})) {
                    Ok(v) => v.get("at").cloned(),
                    Err(e) => {
                        app.status(e);
                        return true;
                    }
                };
                // Options… inserts the field and goes on to its options.
                if options && let Some(form) = OptionsForm::read(app, at.as_ref()) {
                    *d = FormDialog::Options { form };
                    return false;
                }
                return true;
            }
            cancel
        }
        FormDialog::Options { form } => {
            let mut help = false;
            match form.kind.as_str() {
                "text" => text_options(ui, form),
                "checkBox" => check_box_options(ui, form),
                _ => drop_down_options(ui, form),
            }
            ui.add_space(6.0);
            ui.separator();
            egui::Grid::new("ff_settings").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
                ui.label(tl!("Bookmark"));
                ui.add(crate::widgets::text_line(&mut form.name).char_limit(doc4me_doc::form::MAX_NAME).desired_width(200.0));
                ui.end_row();
            });
            let enabled = match form.kind.as_str() {
                "text" => "Fill-in enabled",
                "checkBox" => "Check box enabled",
                _ => "Drop-down enabled",
            };
            ui.checkbox(&mut form.enabled, tl!(enabled));
            ui.checkbox(&mut form.calc_on_exit, tl!("Calculate on exit"));
            ui.add_space(4.0);
            if ui.button(tl!("Add Help Text…")).clicked() {
                help = true;
            }
            if help {
                let back = Box::new(form.clone());
                *d = FormDialog::Help { at: back.at.clone(), form: back.help.clone(), back: Some(back) };
                return false;
            }
            let (ok, cancel) = buttons(ui, tl!("OK"));
            if ok {
                match app.run("developer.formFieldOptions", form.params()) {
                    Ok(_) => return true,
                    Err(e) => app.status(e),
                }
            }
            cancel
        }
        FormDialog::Help { at, form, back } => {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut form.tab, 0, tl!("Status Bar"));
                ui.selectable_value(&mut form.tab, 1, tl!("Help Key (F1)"));
            });
            ui.separator();
            let limit = if form.tab == 0 { doc4me_doc::form::MAX_STATUS } else { doc4me_doc::form::MAX_HELP };
            let h = if form.tab == 0 { &mut form.status } else { &mut form.help };
            ui.radio_value(&mut h.mode, 0, tl!("None"));
            ui.horizontal(|ui| {
                ui.radio_value(&mut h.mode, 1, tl!("AutoText entry:"));
                ui.add_enabled(h.mode == 1, crate::widgets::text_line(&mut h.auto_text).desired_width(180.0));
            });
            ui.radio_value(&mut h.mode, 2, tl!("Type your own:"));
            ui.add_enabled(h.mode == 2, egui::TextEdit::multiline(&mut h.own).char_limit(limit).desired_rows(4).desired_width(320.0));
            let (ok, cancel) = buttons(ui, tl!("OK"));
            if let Some(b) = back.as_mut()
                && (ok || cancel)
            {
                // Back to the Options dialog, with the help text kept (OK) or as it was.
                let mut options = (**b).clone();
                if ok {
                    options.help = form.clone();
                }
                *d = FormDialog::Options { form: options };
                return false;
            }
            if ok {
                match app.run("developer.formFieldHelp", json!({"at": at, "status": form.status.param(), "help": form.help.param()})) {
                    Ok(_) => return true,
                    Err(e) => app.status(e),
                }
            }
            cancel
        }
    }
}

fn text_options(ui: &mut Ui, f: &mut OptionsForm) {
    egui::Grid::new("ff_text").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
        ui.label(tl!("Type:"));
        let shown = TEXT_TYPES.iter().find(|(k, _)| *k == f.text_type).map(|(_, l)| *l).unwrap_or("Regular text");
        egui::ComboBox::from_id_salt("ff_text_type").width(200.0).selected_text(tl!(shown)).show_ui(ui, |ui| {
            for (k, label) in TEXT_TYPES {
                if ui.selectable_label(f.text_type == k, tl!(label)).clicked() && f.text_type != k {
                    f.text_type = k.to_string();
                    f.format.clear();
                }
            }
        });
        ui.end_row();
        ui.label(tl!("Default"));
        let fills = matches!(f.text_type.as_str(), "regular" | "number" | "date" | "calculated");
        ui.add_enabled(fills, crate::widgets::text_line(&mut f.default_text).char_limit(doc4me_doc::form::MAX_TEXT).desired_width(200.0));
        ui.end_row();
        ui.label(tl!("Maximum length:"));
        ui.horizontal(|ui| {
            let mut unlimited = f.max_length == 0;
            if ui.checkbox(&mut unlimited, tl!("Unlimited")).changed() {
                f.max_length = if unlimited { 0 } else { f.default_text.chars().count().max(10) as u32 };
            }
            if !unlimited {
                ui.add(egui::DragValue::new(&mut f.max_length).range(1..=32767).clamp_existing_to_range(false));
            }
        });
        ui.end_row();
        ui.label(tl!("Format:"));
        let current = f.format.clone();
        let items: Vec<String> = formats(&f.text_type).iter().map(|s| s.to_string()).collect();
        if let Some(v) = crate::widgets::combo(ui, "ff_text_format", 200.0, &current, &items, None) {
            f.format = v;
        }
        ui.end_row();
    });
}

fn check_box_options(ui: &mut Ui, f: &mut OptionsForm) {
    ui.label(tl!("Check box size:"));
    ui.horizontal(|ui| {
        ui.radio_value(&mut f.auto_size, true, tl!("Auto"));
        ui.radio_value(&mut f.auto_size, false, tl!("Exactly"));
        ui.add_enabled(!f.auto_size, egui::DragValue::new(&mut f.size).range(1.0..=1638.0).clamp_existing_to_range(false).suffix(" pt"));
    });
    ui.add_space(4.0);
    ui.label(tl!("Default value:"));
    ui.horizontal(|ui| {
        ui.radio_value(&mut f.default_checked, false, tl!("Not checked"));
        ui.radio_value(&mut f.default_checked, true, tl!("Checked"));
    });
}

fn drop_down_options(ui: &mut Ui, f: &mut OptionsForm) {
    ui.label(tl!("Drop-down item:"));
    ui.horizontal(|ui| {
        let r = ui.add(crate::widgets::text_line(&mut f.new_entry).char_limit(doc4me_doc::form::MAX_ENTRY).desired_width(200.0));
        let can_add = !f.new_entry.trim().is_empty() && f.entries.len() < doc4me_doc::form::MAX_ENTRIES;
        let enter = r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        if (ui.add_enabled(can_add, egui::Button::new(tl!("Add >>"))).clicked() || (enter && can_add)) && can_add {
            f.entries.push(f.new_entry.trim().to_string());
            f.new_entry.clear();
            f.selected = Some(f.entries.len() - 1);
        }
    });
    ui.add_space(4.0);
    ui.label(tl!("Items in drop-down list:"));
    ui.horizontal(|ui| {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_size(egui::vec2(200.0, 120.0));
            egui::ScrollArea::vertical().id_salt("ff_entries").max_height(160.0).show(ui, |ui| {
                for (i, e) in f.entries.iter().enumerate() {
                    if ui.selectable_label(f.selected == Some(i), e.as_str()).clicked() {
                        f.selected = Some(i);
                    }
                }
            });
        });
        ui.vertical(|ui| {
            let sel = f.selected.filter(|i| *i < f.entries.len());
            if ui.add_enabled(sel.is_some_and(|i| i > 0), egui::Button::new(tl!("Move Up"))).clicked()
                && let Some(i) = sel
            {
                f.entries.swap(i, i - 1);
                f.selected = Some(i - 1);
            }
            if ui.add_enabled(sel.is_some_and(|i| i + 1 < f.entries.len()), egui::Button::new(tl!("Move Down"))).clicked()
                && let Some(i) = sel
            {
                f.entries.swap(i, i + 1);
                f.selected = Some(i + 1);
            }
            if ui.add_enabled(sel.is_some(), egui::Button::new(tl!("Remove"))).clicked()
                && let Some(i) = sel
            {
                // Word puts a removed entry back in the box, ready to be edited and added again.
                f.new_entry = f.entries.remove(i);
                f.selected = None;
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use serde_json::json;

    use super::FormDialog;
    use crate::dialogs::Dialog;
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

    fn form<'a>(h: &'a mut Harness<'_, WordApp>) -> &'a mut FormDialog {
        match h.state_mut().dialog.as_mut() {
            Some(Dialog::Forms(d)) => d,
            other => panic!("{:?}", other.map(|d| d.name())),
        }
    }

    fn info(h: &mut Harness<'_, WordApp>, name: &str) -> serde_json::Value {
        h.state_mut().session.run("developer.formFieldInfo", &json!({"field": name})).unwrap()
    }

    #[test]
    fn form_field_dialogs_insert_set_options_and_help() {
        let mut h = harness(WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("Size: ")), Services::default()));
        h.state_mut().session.run("caret.docEnd", &json!({})).unwrap();
        // Form Field (Developer › Legacy Tools from the command, without a kind): a drop-down, then Options….
        h.state_mut().run("developer.formField", json!({})).unwrap();
        h.run_steps(3);
        assert_eq!(form(&mut h).name(), "insertFormField");
        h.get_by_label("Drop-down").click();
        h.run_steps(1);
        h.get_by_label("Options…").click();
        h.run_steps(3);
        match form(&mut h) {
            FormDialog::Options { form } => {
                assert_eq!((form.kind.as_str(), form.name.as_str()), ("dropDown", "Dropdown1"));
                form.new_entry = "Small".into();
            }
            other => panic!("{other:?}"),
        }
        h.run_steps(1);
        h.get_by_label("Add >>").click();
        h.run_steps(1);
        if let FormDialog::Options { form } = form(&mut h) {
            form.new_entry = "Large".into();
        }
        h.run_steps(1);
        h.get_by_label("Add >>").click();
        h.run_steps(1);
        // Add Help Text… and back.
        h.get_by_label("Add Help Text…").click();
        h.run_steps(2);
        match form(&mut h) {
            FormDialog::Help { form, back, .. } => {
                assert!(back.is_some());
                form.status.mode = 2;
                form.status.own = "Pick a size".into();
            }
            other => panic!("{other:?}"),
        }
        h.run_steps(1);
        h.get_by_label("OK").click();
        // The Help Text window fades out over the Options window it went back to.
        h.run_steps(12);
        assert!(matches!(form(&mut h), FormDialog::Options { form } if form.help.status.own == "Pick a size"));
        h.get_by_label("OK").click();
        h.run_steps(2);
        assert!(h.state().dialog.is_none(), "{:?} {:?}", h.state().status_msg, h.state().dialog.as_ref().map(|d| d.name()));
        let d = info(&mut h, "Dropdown1");
        assert_eq!(d["entries"], json!(["Small", "Large"]));
        assert_eq!(d["status"]["text"], "Pick a size");
        assert_eq!(h.state().session.doc.plain_text(doc4me_doc::StoryRef::Body), "Size: Small");
        // Help Text on its own (the field selected).
        let at = d["at"].clone();
        h.state_mut().session.run("developer.formFieldInfo", &json!({"at": at})).unwrap();
        let at: doc4me_doc::Pos = serde_json::from_value(at).unwrap();
        let end = doc4me_doc::Pos { off: at.off + doc4me_doc::para::OBJ.len_utf8(), ..at.clone() };
        h.state_mut().session.sel = doc4me_engine::Selection { anchor: at, focus: end };
        h.state_mut().run("developer.formFieldHelp", json!({})).unwrap();
        h.run_steps(3);
        if let FormDialog::Help { form, back, .. } = form(&mut h) {
            assert!(back.is_none());
            form.tab = 1;
            form.help.mode = 2;
            form.help.own = "Small fits a pocket".into();
        }
        h.run_steps(1);
        h.get_by_label("OK").click();
        h.run_steps(2);
        assert!(h.state().dialog.is_none());
        assert_eq!(info(&mut h, "Dropdown1")["help"]["text"], "Small fits a pocket");
    }

    #[test]
    fn section_protection_dialog() {
        let mut h = harness(WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("One")), Services::default()));
        h.state_mut().session.run("caret.docEnd", &json!({})).unwrap();
        h.state_mut().run("layout.break", json!({"kind": "nextPage"})).unwrap();
        h.state_mut().run("review.protectSections", json!({})).unwrap();
        h.run_steps(3);
        assert!(matches!(form(&mut h), FormDialog::Sections { protected } if protected == &vec![true, true]));
        h.get_by_label("Section 2").click();
        h.run_steps(1);
        h.get_by_label("OK").click();
        h.run_steps(2);
        assert!(h.state().dialog.is_none());
        assert_eq!(h.state_mut().session.run("review.protectSections", &json!({})).unwrap()["sections"], json!([true, false]));
    }

    #[test]
    fn text_and_check_box_options() {
        let mut h = harness(WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), Services::default()));
        h.state_mut().run("developer.formField", json!({"type": "text"})).unwrap();
        h.state_mut().run("developer.formField", json!({"type": "checkBox"})).unwrap();
        h.state_mut().session.run("caret.left", &json!({"extend": true})).unwrap();
        // The check box is selected: its options.
        h.state_mut().run("developer.formFieldOptions", json!({})).unwrap();
        h.run_steps(3);
        match form(&mut h) {
            FormDialog::Options { form } => {
                assert_eq!(form.kind, "checkBox");
                assert!(form.auto_size);
                form.auto_size = false;
                form.size = 14.0;
                form.default_checked = true;
                form.name = "Agree".into();
            }
            other => panic!("{other:?}"),
        }
        h.run_steps(1);
        h.get_by_label("OK").click();
        h.run_steps(2);
        let c = info(&mut h, "Agree");
        assert_eq!((c["size"].as_f64(), c["value"].as_bool()), (Some(14.0), Some(true)));
        // The text field: a number with a format, a default and a maximum length.
        let t = info(&mut h, "Text1");
        let at: doc4me_doc::Pos = serde_json::from_value(t["at"].clone()).unwrap();
        let end = doc4me_doc::Pos { off: at.off + doc4me_doc::para::OBJ.len_utf8(), ..at.clone() };
        h.state_mut().session.sel = doc4me_engine::Selection { anchor: at, focus: end };
        h.state_mut().run("developer.formFieldOptions", json!({})).unwrap();
        h.run_steps(3);
        if let FormDialog::Options { form } = form(&mut h) {
            assert_eq!((form.kind.as_str(), form.text_type.as_str()), ("text", "regular"));
            form.text_type = "number".into();
            form.format = "0.00".into();
            form.default_text = "5".into();
            form.max_length = 6;
        }
        h.run_steps(1);
        h.get_by_label("OK").click();
        h.run_steps(2);
        let t = info(&mut h, "Text1");
        assert_eq!(
            (t["textType"].as_str(), t["format"].as_str(), t["maxLength"].as_u64(), t["value"].as_str()),
            (Some("number"), Some("0.00"), Some(6), Some("5"))
        );
        // Options with nothing selected says what to do.
        h.state_mut().session.run("select.all", &json!({})).unwrap();
        h.state_mut().session.run("select.collapse", &json!({})).unwrap();
        h.state_mut().run("developer.formFieldOptions", json!({})).ok();
        h.run_steps(2);
    }
}
