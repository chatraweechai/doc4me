//! Dialogs about data and lists: Recognized Functions (Equation › Equation Options), Insert
//! Database, Filter and Sort (Edit Recipient List) and Table AutoFormat (Table Design › Table
//! Styles). Each ends by running a command (`equation.functions`, `insert.database`,
//! `mailings.query`, `table.style` with `table.look`), so scripts and agents get the same result
//! without the dialog. Shown through [`crate::dialogs::Dialog::Data`].

use egui::{Ui, vec2};
use serde::Serialize;
use serde_json::{Value, json};

use crate::WordApp;
use crate::file_dialogs::{AfterPick, FileDialogRequest};
use crate::theme::semibold;

/// One Filter Records rule: the field, the comparison (`mailings.query`'s `op`), the value, and
/// whether it joins the rule before with Or (else And).
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub field: String,
    pub op: String,
    pub value: String,
    pub or: bool,
}

/// Word offers six filter rules and three sort fields.
const RULES: usize = 6;
const SORT_KEYS: usize = 3;

/// `mailings.query`'s comparisons and their names.
const COMPARISONS: [(&str, &str); 10] = [
    ("eq", "Equal to"),
    ("ne", "Not equal to"),
    ("lt", "Less than"),
    ("gt", "Greater than"),
    ("le", "Less than or equal"),
    ("ge", "Greater than or equal"),
    ("blank", "Is blank"),
    ("notBlank", "Is not blank"),
    ("contains", "Contains"),
    ("notContains", "Does not contain"),
];

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "form", rename_all = "camelCase")]
pub enum DataDialog {
    /// The names an equation sets upright as functions, and the one being typed.
    MathFunctions { functions: Vec<String>, name: String },
    /// The data source, its sheet (when a workbook has several), whether its first row names the
    /// fields, the fields with the ones to insert ticked, and the records (all, or from–to).
    InsertDatabase {
        path: String,
        sheets: Vec<String>,
        sheet: String,
        headers: bool,
        fields: Vec<(String, bool)>,
        records: usize,
        all: bool,
        from: u32,
        to: u32,
        message: String,
    },
    /// Filter Records (tab 0) or Sort Records (tab 1): the rules, and (field, descending) keys.
    FilterSort { tab: u8, fields: Vec<String>, rules: Vec<Rule>, sort: Vec<(String, bool)> },
    /// The table styles (id, name), the one picked and Apply special formats to: heading rows,
    /// first column, last row, last column.
    TableAutoFormat { styles: Vec<(String, String)>, style: usize, look: [bool; 4] },
}

impl DataDialog {
    pub fn name(&self) -> &'static str {
        match self {
            DataDialog::MathFunctions { .. } => "mathFunctions",
            DataDialog::InsertDatabase { .. } => "insertDatabase",
            DataDialog::FilterSort { .. } => "filterSort",
            DataDialog::TableAutoFormat { .. } => "tableAutoFormat",
        }
    }

    /// The window title (English; the caller translates it).
    pub fn title(&self) -> &'static str {
        match self {
            DataDialog::MathFunctions { .. } => "Recognized Functions",
            DataDialog::InsertDatabase { .. } => "Insert Database",
            DataDialog::FilterSort { .. } => "Filter and Sort",
            DataDialog::TableAutoFormat { .. } => "Table AutoFormat",
        }
    }
}

/// `equation.functions`' list.
fn functions(v: &Value) -> Vec<String> {
    v.get("functions").and_then(Value::as_array).into_iter().flatten().filter_map(|n| n.as_str().map(str::to_string)).collect()
}

/// The dialog `ui.dialog` opens by `name`.
pub fn open(name: &str, app: &mut WordApp) -> Option<DataDialog> {
    Some(match name {
        "mathFunctions" => {
            let v = app.session.run("equation.functions", &json!({})).ok()?;
            DataDialog::MathFunctions { functions: functions(&v), name: String::new() }
        }
        "insertDatabase" => DataDialog::InsertDatabase {
            path: String::new(),
            sheets: Vec::new(),
            sheet: String::new(),
            headers: true,
            fields: Vec::new(),
            records: 0,
            all: true,
            from: 1,
            to: 1,
            message: String::new(),
        },
        "filterSort" => {
            let v = app.session.run("mailings.query", &json!({})).ok()?;
            let fields = app.session.merge.headers.clone();
            let q = v.get("query").cloned().unwrap_or_default();
            let s = |r: &Value, k: &str| r.get(k).and_then(Value::as_str).unwrap_or("").to_string();
            let mut rules: Vec<Rule> = q
                .get("filter")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .map(|r| Rule {
                    field: s(r, "field"),
                    op: s(r, "op"),
                    value: s(r, "value"),
                    or: r.get("or").and_then(Value::as_bool).unwrap_or(false),
                })
                .collect();
            rules.resize(RULES, Rule { op: "eq".into(), ..Default::default() });
            let mut sort: Vec<(String, bool)> = q
                .get("sort")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .map(|k| (s(k, "field"), k.get("descending").and_then(Value::as_bool).unwrap_or(false)))
                .collect();
            sort.resize(SORT_KEYS, (String::new(), false));
            DataDialog::FilterSort { tab: 0, fields, rules, sort }
        }
        "tableAutoFormat" => {
            let (tp, _, _) = app.session.sel.focus.path.cell()?;
            let table = app.session.doc.table(app.session.sel.focus.story, &tp)?.clone();
            let mut styles: Vec<(String, String)> = app
                .session
                .doc
                .styles
                .styles
                .iter()
                .filter(|s| s.kind == doc4me_doc::StyleKind::Table && !s.hidden)
                .map(|s| (s.id.clone(), crate::i18n::style_name(s)))
                .collect();
            styles.sort_by_key(|(_, n)| n.to_lowercase());
            let style = table.props.style.as_ref().and_then(|id| styles.iter().position(|(s, _)| s == id)).unwrap_or(0);
            let l = &table.props.look;
            DataDialog::TableAutoFormat { styles, style, look: [l.header_row, l.first_column, l.total_row, l.last_column] }
        }
        _ => return None,
    })
}

/// Draws the dialog; returns true to close it.
pub fn body(app: &mut WordApp, ui: &mut Ui, d: &mut DataDialog) -> bool {
    match d {
        DataDialog::MathFunctions { functions: list, name } => math_functions(app, ui, list, name),
        DataDialog::InsertDatabase { .. } => insert_database(app, ui, d),
        DataDialog::FilterSort { tab, fields, rules, sort } => filter_sort(app, ui, tab, fields, rules, sort),
        DataDialog::TableAutoFormat { styles, style, look } => table_autoformat(app, ui, styles, style, look),
    }
}

fn heading(ui: &mut Ui, s: &str) {
    ui.label(egui::RichText::new(tl!(s)).font(semibold(12.5)));
}

/// OK / Cancel (or another first button); returns (ok, cancel).
fn buttons(ui: &mut Ui, ok: &str, ok_enabled: bool) -> (bool, bool) {
    let mut r = (false, false);
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Cancel")).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                r.1 = true;
            }
            if ui.add_enabled(ok_enabled, crate::widgets::primary_button(ui, tl!(ok))).clicked() {
                r.0 = true;
            }
        });
    });
    r
}

/// Runs a dialog's command; a refusal shows in the status bar.
fn run(app: &mut WordApp, id: &str, params: Value) -> Option<Value> {
    match app.run(id, params) {
        Ok(v) => Some(v),
        Err(e) => {
            app.status(e);
            None
        }
    }
}

/// Recognized Functions: add a name, delete one, or go back to the built-in list. Each change
/// takes effect at once (the list belongs to the user's preferences, not the document).
fn math_functions(app: &mut WordApp, ui: &mut Ui, list: &mut Vec<String>, name: &mut String) -> bool {
    ui.label(tl!("Recognized Functions"));
    let field = ui.add(crate::widgets::text_line(name).desired_width(260.0));
    let typed = name.trim().to_string();
    let listed = list.contains(&typed);
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(260.0);
        egui::ScrollArea::vertical().id_salt("math_functions").max_height(200.0).show(ui, |ui| {
            for n in list.iter() {
                if ui.selectable_label(*n == typed, n.as_str()).clicked() {
                    *name = n.clone();
                }
            }
        });
    });
    let mut change = None;
    ui.horizontal(|ui| {
        let enter = field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        if ui.add_enabled(!typed.is_empty() && !listed, egui::Button::new(tl!("Add"))).clicked() || (enter && !typed.is_empty() && !listed) {
            change = Some(json!({"add": typed}));
        }
        if ui.add_enabled(listed, egui::Button::new(tl!("Delete"))).clicked() {
            change = Some(json!({"remove": typed}));
        }
        if ui.button(tl!("Restore Defaults")).clicked() {
            change = Some(json!({"reset": true}));
        }
    });
    if let Some(params) = change
        && let Some(v) = run(app, "equation.functions", params)
    {
        *list = functions(&v);
        name.clear();
    }
    let (ok, cancel) = buttons(ui, "OK", true);
    ok || cancel
}

/// Insert Database: Get Data picks the source; its fields and records come from
/// `insert.database` with `preview`, and Insert Data puts the chosen ones in a table.
fn insert_database(app: &mut WordApp, ui: &mut Ui, d: &mut DataDialog) -> bool {
    let DataDialog::InsertDatabase { path, sheets, sheet, headers, fields, records, all, from, to, message } = d else { return true };
    let mut reload = false;
    if let Some(p) = app.dialog_pick.take() {
        *path = p;
        sheets.clear();
        sheet.clear();
        reload = true;
    }
    heading(ui, "Data source:");
    ui.horizontal(|ui| {
        let r = ui.add(crate::widgets::text_line(path).desired_width(300.0));
        reload |= r.lost_focus() && !path.trim().is_empty();
        if ui.button(tl!("Get Data…")).clicked() {
            let _ = app.ask_file(FileDialogRequest::Open { purpose: "recipients".into() }, AfterPick::ForDialog);
        }
    });
    if !sheets.is_empty() {
        ui.horizontal(|ui| {
            ui.label(tl!("Sheet:"));
            let before = sheet.clone();
            egui::ComboBox::from_id_salt("database_sheet").selected_text(sheet.as_str()).show_ui(ui, |ui| {
                for s in sheets.iter() {
                    ui.selectable_value(sheet, s.clone(), s.as_str());
                }
            });
            reload |= *sheet != before;
        });
    }
    reload |= ui.checkbox(headers, tl!("First row of data contains column headers")).changed() && !path.is_empty();
    if reload {
        let mut p = json!({"path": path.trim(), "headers": *headers, "preview": true});
        if !sheet.is_empty() {
            p["sheet"] = json!(sheet);
        }
        match app.run("insert.database", p) {
            Ok(v) if v.get("chooseSheet").is_some() => {
                *sheets = v.get("sheets").and_then(Value::as_array).into_iter().flatten().filter_map(|s| s.as_str().map(str::to_string)).collect();
                *sheet = sheets.first().cloned().unwrap_or_default();
                // Read the first sheet's fields at once.
                let v = app.run("insert.database", json!({"path": path.trim(), "headers": *headers, "preview": true, "sheet": sheet}));
                load_fields(v, fields, records, message);
            }
            v => load_fields(v, fields, records, message),
        }
        *to = (*records).max(1) as u32;
    }
    if !fields.is_empty() {
        ui.add_space(4.0);
        heading(ui, "Fields to insert:");
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_width(300.0);
            egui::ScrollArea::vertical().id_salt("database_fields").max_height(160.0).show(ui, |ui| {
                for (f, on) in fields.iter_mut() {
                    ui.checkbox(on, f.as_str());
                }
            });
        });
        ui.add_space(4.0);
        ui.label(crate::i18n::fmt(tl!("{n} records"), &[("n", &records.to_string())]));
        ui.radio_value(all, true, tl!("All records"));
        ui.horizontal(|ui| {
            ui.radio_value(all, false, tl!("From"));
            let n = (*records).max(1) as u32;
            ui.add_enabled(!*all, egui::DragValue::new(from).range(1..=n).clamp_existing_to_range(false));
            ui.label(tl!("To"));
            ui.add_enabled(!*all, egui::DragValue::new(to).range(1..=n).clamp_existing_to_range(false));
        });
    }
    if !message.is_empty() {
        ui.label(egui::RichText::new(message.as_str()).color(crate::theme::Tokens::get(ui.ctx()).red));
    }
    let chosen: Vec<&str> = fields.iter().filter(|(_, on)| *on).map(|(f, _)| f.as_str()).collect();
    let (ok, cancel) = buttons(ui, "Insert Data", !chosen.is_empty());
    if ok {
        let mut p = json!({"path": path.trim(), "headers": *headers, "fields": chosen});
        if !sheet.is_empty() {
            p["sheet"] = json!(sheet);
        }
        if !*all {
            p["from"] = json!(*from);
            p["to"] = json!((*to).max(*from));
        }
        match app.run("insert.database", p) {
            Ok(_) => return true,
            Err(e) => *message = e,
        }
    }
    cancel
}

/// The source's fields (all ticked) and record count from a preview, or why it can't be read.
fn load_fields(v: Result<Value, String>, fields: &mut Vec<(String, bool)>, records: &mut usize, message: &mut String) {
    match v {
        Ok(v) => {
            *fields =
                v.get("fields").and_then(Value::as_array).into_iter().flatten().filter_map(|f| f.as_str().map(|f| (f.to_string(), true))).collect();
            *records = v.get("records").and_then(Value::as_u64).unwrap_or(0) as usize;
            message.clear();
        }
        Err(e) => {
            fields.clear();
            *records = 0;
            *message = e;
        }
    }
}

/// A field picker; "(none)" is the empty choice.
fn field_combo(ui: &mut Ui, id: (&str, usize), fields: &[String], value: &mut String) {
    let shown = if value.is_empty() { tl!("(none)").to_string() } else { value.clone() };
    egui::ComboBox::from_id_salt(id).width(150.0).selected_text(shown).show_ui(ui, |ui| {
        ui.selectable_value(value, String::new(), tl!("(none)"));
        for f in fields {
            ui.selectable_value(value, f.clone(), f.as_str());
        }
    });
}

/// Filter and Sort: Filter Records and Sort Records tabs; OK runs `mailings.query`, Clear All
/// empties both.
fn filter_sort(app: &mut WordApp, ui: &mut Ui, tab: &mut u8, fields: &[String], rules: &mut [Rule], sort: &mut [(String, bool)]) -> bool {
    ui.horizontal(|ui| {
        ui.selectable_value(tab, 0, tl!("Filter Records"));
        ui.selectable_value(tab, 1, tl!("Sort Records"));
    });
    ui.separator();
    if *tab == 0 {
        egui::Grid::new("filter_rules").num_columns(4).spacing(vec2(8.0, 6.0)).show(ui, |ui| {
            ui.label("");
            ui.label(tl!("Field"));
            ui.label(tl!("Comparison"));
            ui.label(tl!("Compare to"));
            ui.end_row();
            for (i, r) in rules.iter_mut().enumerate() {
                if i == 0 {
                    ui.label("");
                } else {
                    egui::ComboBox::from_id_salt(("rule_join", i)).width(60.0).selected_text(tl!(if r.or { "Or" } else { "And" })).show_ui(
                        ui,
                        |ui| {
                            ui.selectable_value(&mut r.or, false, tl!("And"));
                            ui.selectable_value(&mut r.or, true, tl!("Or"));
                        },
                    );
                }
                field_combo(ui, ("rule_field", i), fields, &mut r.field);
                let name = COMPARISONS.iter().find(|(k, _)| *k == r.op).map_or("Equal to", |(_, n)| n);
                egui::ComboBox::from_id_salt(("rule_op", i)).width(150.0).selected_text(tl!(name)).show_ui(ui, |ui| {
                    for (k, n) in COMPARISONS {
                        ui.selectable_value(&mut r.op, k.to_string(), tl!(n));
                    }
                });
                let needs_value = !matches!(r.op.as_str(), "blank" | "notBlank");
                ui.add_enabled(needs_value && !r.field.is_empty(), crate::widgets::text_line(&mut r.value).desired_width(150.0));
                ui.end_row();
            }
        });
    } else {
        egui::Grid::new("sort_keys").num_columns(3).spacing(vec2(8.0, 6.0)).show(ui, |ui| {
            for (i, (field, desc)) in sort.iter_mut().enumerate() {
                ui.label(tl!(if i == 0 { "Sort by" } else { "Then by" }));
                field_combo(ui, ("sort_field", i), fields, field);
                ui.horizontal(|ui| {
                    ui.radio_value(desc, false, tl!("Ascending"));
                    ui.radio_value(desc, true, tl!("Descending"));
                });
                ui.end_row();
            }
        });
    }
    ui.add_space(6.0);
    if ui.button(tl!("Clear All")).clicked() {
        for r in rules.iter_mut() {
            *r = Rule { op: "eq".into(), ..Default::default() };
        }
        for k in sort.iter_mut() {
            *k = (String::new(), false);
        }
    }
    let (ok, cancel) = buttons(ui, "OK", true);
    if ok {
        let filter: Vec<Value> = rules
            .iter()
            .filter(|r| !r.field.is_empty())
            .enumerate()
            .map(|(i, r)| json!({"field": r.field, "op": r.op, "value": r.value, "or": i > 0 && r.or}))
            .collect();
        let keys: Vec<Value> = sort.iter().filter(|(f, _)| !f.is_empty()).map(|(f, d)| json!({"field": f, "descending": d})).collect();
        let params = if filter.is_empty() && keys.is_empty() { json!({"clear": true}) } else { json!({"filter": filter, "sort": keys}) };
        if let Some(v) = run(app, "mailings.query", params) {
            let of = v.get("of").and_then(Value::as_u64).unwrap_or(0).to_string();
            let n = v.get("records").and_then(Value::as_u64).unwrap_or(0).to_string();
            app.status(crate::i18n::fmt(tl!("{n} of {of} recipients"), &[("n", &n), ("of", &of)]));
            return true;
        }
    }
    cancel
}

/// Table AutoFormat: a table style and which special formats it applies.
fn table_autoformat(app: &mut WordApp, ui: &mut Ui, styles: &[(String, String)], style: &mut usize, look: &mut [bool; 4]) -> bool {
    heading(ui, "Table styles:");
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(280.0);
        egui::ScrollArea::vertical().id_salt("autoformat_styles").max_height(220.0).show(ui, |ui| {
            for (i, (_, name)) in styles.iter().enumerate() {
                if ui.selectable_label(*style == i, name.as_str()).clicked() {
                    *style = i;
                }
            }
        });
    });
    ui.add_space(4.0);
    heading(ui, "Apply special formats to");
    egui::Grid::new("autoformat_look").num_columns(2).show(ui, |ui| {
        ui.checkbox(&mut look[0], tl!("Header Row"));
        ui.checkbox(&mut look[1], tl!("First Column"));
        ui.end_row();
        ui.checkbox(&mut look[2], tl!("Total Row"));
        ui.checkbox(&mut look[3], tl!("Last Column"));
        ui.end_row();
    });
    let (ok, cancel) = buttons(ui, "OK", !styles.is_empty());
    if ok
        && let Some((id, _)) = styles.get(*style)
        && run(app, "table.style", json!({"style": id})).is_some()
        && {
            // One undo step for the style and its options.
            app.session.join_next_undo();
            true
        }
        && run(app, "table.look", json!({"headerRow": look[0], "firstColumn": look[1], "totalRow": look[2], "lastColumn": look[3]})).is_some()
    {
        return true;
    }
    cancel
}

#[cfg(test)]
mod tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use serde_json::json;

    use super::{DataDialog, Rule};
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

    fn app(text: &str) -> WordApp {
        WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text(text)), Services::default())
    }

    fn form<'a>(h: &'a mut Harness<'_, WordApp>) -> &'a mut DataDialog {
        match h.state_mut().dialog.as_mut() {
            Some(Dialog::Data(d)) => d,
            _ => panic!("no data dialog"),
        }
    }

    /// dlg-146: Recognized Functions adds a name, then Restore Defaults.
    #[test]
    fn recognized_functions_dialog_adds_and_restores() {
        let mut h = harness(app(""));
        h.state_mut().run("ui.dialog", json!({"name": "mathFunctions"})).unwrap();
        h.run_steps(3);
        if let DataDialog::MathFunctions { name, .. } = form(&mut h) {
            *name = "erf".into();
        }
        h.run_steps(1);
        h.get_by_label("Add").click();
        h.run_steps(2);
        assert!(h.state().session.prefs.math_functions.as_ref().is_some_and(|f| f.iter().any(|n| n == "erf")));
        assert!(h.query_by_label("erf").is_some(), "listed");
        h.get_by_label("Restore Defaults").click();
        h.run_steps(2);
        assert_eq!(h.state().session.prefs.math_functions, None);
        h.get_by_label("OK").click();
        h.run_steps(2);
        assert!(h.state().dialog.is_none());
    }

    /// dlg-092: Insert Database: Get Data reads the file's fields; Insert Data puts the ticked
    /// ones in a table.
    #[test]
    fn insert_database_dialog_inserts_the_chosen_fields() {
        let dir = std::env::temp_dir().join(format!("doc4me-ui-database-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let csv = dir.join("guests.csv");
        std::fs::write(&csv, "Name,City\nAda,London\nAlan,Wilmslow\n").unwrap();
        let picked = csv.to_string_lossy().to_string();
        let services = Services { pick_open: Some(Box::new(move |_| Some(picked.clone()))), ..Default::default() };
        let a = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("List:")), services);
        let mut h = harness(a);
        h.state_mut().run("caret.docEnd", json!({})).unwrap();
        h.state_mut().run("ui.dialog", json!({"name": "insertDatabase"})).unwrap();
        h.run_steps(3);
        h.get_by_label("Get Data…").click();
        h.run_steps(3);
        assert!(h.query_by_label("2 records").is_some(), "the source's records");
        // City only.
        h.get_by_label("Name").click();
        h.run_steps(2);
        h.get_by_label("Insert Data").click();
        h.run_steps(3);
        assert!(h.state().dialog.is_none());
        let text = h.state().session.doc.plain_text(doc4me_doc::StoryRef::Body);
        assert!(text.contains("City") && text.contains("Wilmslow") && !text.contains("Ada"), "{text}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// dlg-136: Filter and Sort from the dialog: the rules and sort keys become the query.
    #[test]
    fn filter_and_sort_dialog_runs_the_query() {
        let mut h = harness(app(""));
        h.state_mut().run("mailings.recipients", json!({"csv": "Name,Age\nAda,36\nAlan,41\nGrace,85"})).unwrap();
        h.state_mut().run("ui.dialog", json!({"name": "filterSort"})).unwrap();
        h.run_steps(3);
        if let DataDialog::FilterSort { rules, sort, .. } = form(&mut h) {
            rules[0] = Rule { field: "Age".into(), op: "gt".into(), value: "40".into(), or: false };
            sort[0] = ("Name".into(), true);
        }
        h.run_steps(1);
        h.get_by_label("Sort Records").click();
        h.run_steps(2);
        assert_eq!(h.query_all_by_label("Descending").count(), 3, "three sort fields");
        h.get_by_label("OK").click();
        h.run_steps(2);
        assert!(h.state().dialog.is_none());
        let names: Vec<String> = h.state().session.merge.rows.iter().map(|r| r[0].clone()).collect();
        assert_eq!(names, ["Grace", "Alan"]);
        // Opened again, it shows the query in effect; Clear All and OK bring every record back.
        h.state_mut().run("ui.dialog", json!({"name": "filterSort"})).unwrap();
        h.run_steps(3);
        assert!(matches!(form(&mut h), DataDialog::FilterSort { rules, .. } if rules[0].field == "Age"));
        h.get_by_label("Clear All").click();
        h.run_steps(1);
        h.get_by_label("OK").click();
        h.run_steps(2);
        assert_eq!(h.state().session.merge.rows.len(), 3);
    }

    /// dlg-156: Table AutoFormat applies a table style and its special formats in one undo step.
    #[test]
    fn table_autoformat_applies_a_style_and_its_options() {
        let mut h = harness(app(""));
        h.state_mut().run("insert.table", json!({"rows": 3, "cols": 3})).unwrap();
        h.state_mut().run("ui.dialog", json!({"name": "tableAutoFormat"})).unwrap();
        h.run_steps(3);
        let (id, picked) = match form(&mut h) {
            DataDialog::TableAutoFormat { styles, style, look } => {
                *style = styles.iter().position(|(id, _)| id.contains("GridTable4")).unwrap_or(1);
                *look = [true, false, true, false];
                (styles[*style].0.clone(), *style)
            }
            other => panic!("not Table AutoFormat: {other:?}"),
        };
        assert!(picked > 0);
        h.run_steps(1);
        h.get_by_label("OK").click();
        h.run_steps(2);
        let s = &h.state().session;
        let (tp, _, _) = s.sel.focus.path.cell().expect("in the table");
        let t = s.doc.table(s.sel.focus.story, &tp).unwrap();
        assert_eq!(t.props.style.as_deref(), Some(id.as_str()));
        assert!(t.props.look.header_row && t.props.look.total_row && !t.props.look.first_column);
        h.state_mut().run("edit.undo", json!({})).unwrap();
        let s = &h.state().session;
        let (tp, _, _) = s.sel.focus.path.cell().expect("in the table");
        assert_ne!(s.doc.table(s.sel.focus.story, &tp).unwrap().props.style.as_deref(), Some(id.as_str()), "one undo step");
        // Outside a table there is nothing to format.
        let mut b = harness(app("No table here"));
        assert!(b.state_mut().run("ui.dialog", json!({"name": "tableAutoFormat"})).unwrap()["dialog"].is_null());
    }
}
