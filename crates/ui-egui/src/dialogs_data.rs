//! Dialogs about data and lists: Recognized Functions (Equation › Equation Options), Insert
//! Database, Filter and Sort (Edit Recipient List), Table AutoFormat (Table Design › Table
//! Styles) and the review after AutoFormat. Each ends by running a command
//! (`equation.functions`, `insert.database`, `mailings.query`, `table.style` with `table.look`,
//! `format.autoFormatReview`), so scripts and agents get the same result without the dialog.
//! Shown through [`crate::dialogs::Dialog::Data`].

use egui::{Ui, vec2};
use serde::Serialize;
use serde_json::{Value, json};

use doc4me_engine::cmd::options::GRAMMAR_RULES;

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
    /// After AutoFormat with review: the changed paragraphs (before, after), and whether the
    /// list of changes is open (Review Changes).
    AutoFormatReview { changes: Vec<(String, String)>, reviewing: bool },
    /// Merge (Word's Merge to New Document and Merge to Printer, or the Mail Merge Helper's Merge
    /// with its Merge to choice; `fixed` when the button chose where): to the printer or a new
    /// document, which records (0 all, 1 the current one, 2 from–to), whether blank lines print.
    Merge { fixed: bool, printer: bool, records: u8, from: u32, to: u32, blank_lines: bool },
    /// The Mail Merge Helper: a merge set up in three steps. `picking` says what the next picked
    /// file is (1 a header source, 2 a data source).
    MergeHelper { picking: u8, message: String },
    /// Header Options › Create: the field names (Word's list to start), the name being typed and
    /// the one selected; `saving` once the file name is asked for.
    HeaderSource { fields: Vec<String>, name: String, selected: Option<usize>, saving: bool, message: String },
    /// Writing Style › Settings: each grammar rule on or off, in `GRAMMAR_RULES` order.
    GrammarSettings { rules: Vec<bool> },
}

impl DataDialog {
    pub fn name(&self) -> &'static str {
        match self {
            DataDialog::MathFunctions { .. } => "mathFunctions",
            DataDialog::InsertDatabase { .. } => "insertDatabase",
            DataDialog::FilterSort { .. } => "filterSort",
            DataDialog::TableAutoFormat { .. } => "tableAutoFormat",
            DataDialog::AutoFormatReview { .. } => "autoFormatReview",
            DataDialog::Merge { fixed: false, .. } => "merge",
            DataDialog::Merge { printer: false, .. } => "mergeToNew",
            DataDialog::Merge { .. } => "mergeToPrinter",
            DataDialog::MergeHelper { .. } => "mailMergeHelper",
            DataDialog::HeaderSource { .. } => "createHeaderSource",
            DataDialog::GrammarSettings { .. } => "grammarSettings",
        }
    }

    /// The window title (English; the caller translates it).
    pub fn title(&self) -> &'static str {
        match self {
            DataDialog::MathFunctions { .. } => "Recognized Functions",
            DataDialog::InsertDatabase { .. } => "Insert Database",
            DataDialog::FilterSort { .. } => "Filter and Sort",
            DataDialog::TableAutoFormat { .. } => "Table AutoFormat",
            DataDialog::AutoFormatReview { .. } => "AutoFormat",
            DataDialog::Merge { fixed: false, .. } => "Merge",
            DataDialog::Merge { printer: false, .. } => "Merge to New Document",
            DataDialog::Merge { .. } => "Merge to Printer",
            DataDialog::MergeHelper { .. } => "Mail Merge Helper",
            DataDialog::HeaderSource { .. } => "Create Header Source",
            DataDialog::GrammarSettings { .. } => "Grammar Settings",
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
        "autoFormatReview" => {
            let v = app.session.run("format.autoFormatReview", &json!({})).ok()?;
            DataDialog::AutoFormatReview { changes: review_changes(&v), reviewing: false }
        }
        "merge" | "mergeToNew" | "mergeToPrinter" => {
            let n = app.session.merge.rows.len().max(1) as u32;
            DataDialog::Merge { fixed: name != "merge", printer: name == "mergeToPrinter", records: 0, from: 1, to: n, blank_lines: false }
        }
        "mailMergeHelper" => DataDialog::MergeHelper { picking: 0, message: String::new() },
        "grammarSettings" => {
            let off = &app.session.prefs.grammar_off;
            DataDialog::GrammarSettings { rules: GRAMMAR_RULES.iter().map(|r| !off.iter().any(|x| x == r)).collect() }
        }
        "createHeaderSource" => DataDialog::HeaderSource {
            fields: doc4me_engine::cmd::mailings::DEFAULT_FIELDS.iter().map(|f| f.to_string()).collect(),
            name: String::new(),
            selected: None,
            saving: false,
            message: String::new(),
        },
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
        DataDialog::AutoFormatReview { changes, reviewing } => autoformat_review(app, ui, changes, reviewing),
        DataDialog::Merge { fixed, printer, records, from, to, blank_lines } => merge(app, ui, *fixed, printer, records, from, to, blank_lines),
        DataDialog::MergeHelper { picking, message } => merge_helper(app, ui, picking, message),
        DataDialog::HeaderSource { fields, name, selected, saving, message } => header_source(app, ui, fields, name, selected, saving, message),
        DataDialog::GrammarSettings { rules } => grammar_settings(app, ui, rules),
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

/// `format.autoFormatReview`'s changes as (before, after).
fn review_changes(v: &Value) -> Vec<(String, String)> {
    let s = |c: &Value, k: &str| c.get(k).and_then(Value::as_str).unwrap_or("").to_string();
    v.get("changes").and_then(Value::as_array).into_iter().flatten().map(|c| (s(c, "before"), s(c, "after"))).collect()
}

/// Runs a review action and takes the changes left; a refusal shows in the status bar.
fn review_action(app: &mut WordApp, changes: &mut Vec<(String, String)>, action: &str) -> bool {
    match app.run("format.autoFormatReview", json!({"action": action})) {
        Ok(v) => {
            *changes = review_changes(&v);
            true
        }
        Err(e) => {
            app.status(e);
            false
        }
    }
}

/// Word's "Formatting completed" dialog after AutoFormat with review: Accept All, Reject All, or
/// Review Changes (find each changed paragraph and reject it). Close keeps what is left.
fn autoformat_review(app: &mut WordApp, ui: &mut Ui, changes: &mut Vec<(String, String)>, reviewing: &mut bool) -> bool {
    ui.label(tl!("Formatting completed. You can now:"));
    let n = changes.len().to_string();
    ui.label(egui::RichText::new(crate::i18n::fmt(tl!("{count} paragraphs were changed."), &[("count", &n)])).small().weak());
    let mut close = false;
    ui.horizontal(|ui| {
        if ui.button(tl!("Accept All")).clicked() {
            review_action(app, changes, "acceptAll");
            close = true;
        }
        if ui.button(tl!("Reject All")).clicked() {
            review_action(app, changes, "rejectAll");
            close = true;
        }
        if ui.selectable_label(*reviewing, tl!("Review Changes…")).clicked() {
            *reviewing = !*reviewing;
            if *reviewing {
                review_action(app, changes, "next");
            }
        }
    });
    if close {
        return true;
    }
    if *reviewing {
        ui.add_space(4.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_width(320.0);
            egui::ScrollArea::vertical().id_salt("autoformat_changes").max_height(180.0).show(ui, |ui| {
                let short = |t: &str| if t.chars().count() > 60 { format!("{}…", t.chars().take(60).collect::<String>()) } else { t.to_string() };
                for (before, after) in changes.iter() {
                    ui.label(egui::RichText::new(short(before)).small().strikethrough().weak());
                    ui.label(short(after));
                    ui.separator();
                }
            });
        });
        ui.horizontal(|ui| {
            if ui.button(tl!("← Find")).clicked() {
                review_action(app, changes, "previous");
            }
            if ui.button(tl!("Find →")).clicked() {
                review_action(app, changes, "next");
            }
            if ui.button(tl!("Reject")).clicked() && review_action(app, changes, "reject") && !changes.is_empty() {
                review_action(app, changes, "next");
            }
        });
        ui.label(egui::RichText::new(tl!("Find selects the next changed paragraph; Reject puts the selected one back as it was.")).small().weak());
    }
    // Nothing left to review: done.
    if changes.is_empty() {
        return true;
    }
    ui.add_space(8.0);
    let mut done = ui.input(|i| i.key_pressed(egui::Key::Escape));
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        done |= ui.button(tl!("Close")).clicked();
    });
    done
}

/// Grammar Settings (File › Options › Proofing › Writing Style › Settings…): the kinds of rule
/// Doc4Me's grammar check has, each on or off, in English and Polish alike.
fn grammar_settings(app: &mut WordApp, ui: &mut Ui, rules: &mut [bool]) -> bool {
    const LABELS: [(&str, &str); 5] = [
        ("Capitalization", "The first word of a sentence starts with a capital."),
        ("Repeated words", "The same word twice in a row."),
        ("Word forms", "\"a\" or \"an\" before the next word; Polish \"w\"/\"we\", \"z\"/\"ze\"."),
        ("Punctuation", "A space before punctuation, doubled marks, Polish commas before conjunctions."),
        ("Spacing", "Extra spaces between words."),
    ];
    heading(ui, "Grammar");
    for ((label, why), on) in LABELS.iter().zip(rules.iter_mut()) {
        ui.checkbox(on, tl!(label)).on_hover_text(tl!(why));
        ui.label(egui::RichText::new(tl!(why)).small().weak());
    }
    ui.add_space(4.0);
    if ui.button(tl!("Reset All")).clicked() {
        rules.iter_mut().for_each(|r| *r = true);
    }
    let (ok, cancel) = buttons(ui, "OK", true);
    if ok {
        let set: serde_json::Map<String, Value> = GRAMMAR_RULES.iter().zip(rules.iter()).map(|(k, on)| (k.to_string(), json!(on))).collect();
        return run(app, "file.options", json!({"grammarRules": set})).is_some();
    }
    cancel
}

/// Merge (Merge to New Document, Merge to Printer): which records, and whether a line whose
/// merge fields are all empty prints. Check Errors… and Query Options… open their dialogs.
#[allow(clippy::too_many_arguments)]
fn merge(
    app: &mut WordApp,
    ui: &mut Ui,
    fixed: bool,
    printer: &mut bool,
    records: &mut u8,
    from: &mut u32,
    to: &mut u32,
    blank_lines: &mut bool,
) -> bool {
    let n = app.session.merge.rows.len();
    let last = n.max(1) as u32;
    if !fixed {
        ui.horizontal(|ui| {
            ui.label(tl!("Merge to:"));
            egui::ComboBox::from_id_salt("merge_to").selected_text(if *printer { tl!("Printer") } else { tl!("New document") }).show_ui(ui, |ui| {
                ui.selectable_value(printer, false, tl!("New document"));
                ui.selectable_value(printer, true, tl!("Printer"));
            });
        });
    }
    heading(ui, "Merge records");
    ui.radio_value(records, 0, tl!("All"));
    ui.radio_value(records, 1, tl!("Current record"));
    ui.horizontal(|ui| {
        ui.radio_value(records, 2, tl!("From:"));
        ui.add(egui::DragValue::new(from).range(1..=last).clamp_existing_to_range(false));
        ui.label(tl!("To:"));
        ui.add(egui::DragValue::new(to).range(1..=last).clamp_existing_to_range(false));
    });
    *from = (*from).clamp(1, last);
    *to = (*to).clamp(*from, last);
    heading(ui, "When merging records");
    ui.radio_value(blank_lines, false, tl!("Don't print blank lines when data fields are empty"));
    ui.radio_value(blank_lines, true, tl!("Print blank lines when data fields are empty"));
    let mut other = None;
    ui.horizontal(|ui| {
        if ui.button(tl!("Check Errors…")).clicked() {
            other = Some(("mailings.checkErrors", json!({})));
        }
        if ui.button(tl!("Query Options…")).clicked() {
            other = Some(("ui.dialog", json!({"name": "filterSort"})));
        }
    });
    if let Some((id, p)) = other {
        // Its dialog takes this one's place.
        run(app, id, p);
        return true;
    }
    let (ok, cancel) = buttons(ui, "Merge", n > 0);
    if ok {
        let mut p = json!({"blankLines": *blank_lines});
        match *records {
            1 => p["current"] = json!(true),
            2 => {
                p["from"] = json!(*from);
                p["to"] = json!(*to);
            }
            _ => {}
        }
        // The merge replaces the document: the window asks to save it first.
        return run(app, if *printer { "mailings.print" } else { "mailings.finish" }, p).is_some();
    }
    cancel
}

/// The Mail Merge Helper: 1 the main document (its type), 2 the data source (create, open, a
/// header source, edit), 3 the merge (Merge…, Query Options…).
fn merge_helper(app: &mut WordApp, ui: &mut Ui, picking: &mut u8, message: &mut String) -> bool {
    // The file picked for Header Options › Open or Get Data › Open Data Source.
    if let Some(path) = app.dialog_pick.take() {
        match std::mem::take(picking) {
            1 => match app.run("mailings.headerSource", json!({"path": path})) {
                Ok(_) => {
                    app.header_source = Some(path);
                    message.clear();
                }
                Err(e) => *message = e,
            },
            2 => {
                let mut p = json!({"path": path});
                if let Some(h) = &app.header_source {
                    p["headerSource"] = json!(h);
                }
                match app.load_recipients(p) {
                    Ok(_) => message.clear(),
                    Err(e) => *message = e,
                }
                // A workbook with several sheets asks which one first.
                if app.dialog.is_some() {
                    return true;
                }
            }
            _ => {}
        }
    }
    let mut open = None;
    ui.label(tl!("The Mail Merge Helper sets up a merge in three steps."));
    ui.add_space(4.0);
    heading(ui, "1. Main document");
    let kind = app.session.merge.kind.clone();
    const TYPES: [(&str, &str); 5] = [
        ("letters", "Form Letters"),
        ("emails", "E-mail Messages"),
        ("envelopes", "Envelopes"),
        ("labels", "Mailing Labels"),
        ("directory", "Catalog"),
    ];
    ui.horizontal(|ui| {
        ui.menu_button(tl!("Create"), |ui| {
            for (k, label) in TYPES {
                if ui.button(tl!(label)).clicked() {
                    run(app, "mailings.start", json!({"kind": k}));
                    ui.close();
                }
            }
            ui.separator();
            if ui.button(tl!("Restore to Normal Word Document")).clicked() {
                run(app, "mailings.start", json!({"kind": "normal"}));
                ui.close();
            }
        });
        let shown = TYPES.iter().find(|(k, _)| *k == kind).map_or(tl!("Normal Word Document"), |(_, l)| tl!(l));
        ui.label(crate::i18n::fmt(tl!("Merge type: {type}"), &[("type", shown)]));
    });
    ui.add_space(4.0);
    heading(ui, "2. Data source");
    ui.horizontal(|ui| {
        ui.menu_button(tl!("Get Data"), |ui| {
            if ui.button(tl!("Create Data Source…")).clicked() {
                open = Some(("mailings.newList", json!({})));
                ui.close();
            }
            if ui.button(tl!("Open Data Source…")).clicked() {
                *picking = 2;
                let _ = app.ask_file(FileDialogRequest::Open { purpose: "recipients".into() }, AfterPick::ForDialog);
                ui.close();
            }
        });
        let has_list = !app.session.merge.headers.is_empty();
        if ui.add_enabled(has_list, egui::Button::new(tl!("Edit…"))).clicked() {
            open = Some(("mailings.editRecipients", json!({})));
        }
    });
    let m = &app.session.merge;
    if m.headers.is_empty() {
        ui.label(egui::RichText::new(tl!("No data source yet.")).small().weak());
    } else {
        let n = m.rows.len().to_string();
        let fields = m.headers.join(", ");
        ui.label(
            egui::RichText::new(crate::i18n::fmt(tl!("{count} records; fields: {fields}"), &[("count", &n), ("fields", &fields)])).small().weak(),
        );
    }
    ui.horizontal(|ui| {
        ui.label(tl!("Header Options:"));
        if ui.button(tl!("Create…")).clicked() {
            open = Some(("ui.dialog", json!({"name": "createHeaderSource"})));
        }
        if ui.button(tl!("Open…")).clicked() {
            *picking = 1;
            let _ = app.ask_file(FileDialogRequest::Open { purpose: "recipients".into() }, AfterPick::ForDialog);
        }
        if ui.add_enabled(app.header_source.is_some(), egui::Button::new(tl!("Remove"))).clicked() {
            app.header_source = None;
        }
    });
    if let Some(h) = &app.header_source {
        let file = std::path::Path::new(h).file_name().map_or(h.clone(), |f| f.to_string_lossy().to_string());
        ui.label(
            egui::RichText::new(crate::i18n::fmt(
                tl!("Header source: {file} (the next data source opened here takes its field names)"),
                &[("file", &file)],
            ))
            .small()
            .weak(),
        );
    }
    ui.add_space(4.0);
    heading(ui, "3. Merge the data with the document");
    ui.horizontal(|ui| {
        if ui.add_enabled(!app.session.merge.rows.is_empty(), egui::Button::new(tl!("Merge…"))).clicked() {
            open = Some(("ui.dialog", json!({"name": "merge"})));
        }
        if ui.add_enabled(!app.session.merge.headers.is_empty(), egui::Button::new(tl!("Query Options…"))).clicked() {
            open = Some(("ui.dialog", json!({"name": "filterSort"})));
        }
    });
    if !message.is_empty() {
        ui.colored_label(ui.visuals().error_fg_color, message.as_str());
    }
    if let Some((id, p)) = open {
        // That step's dialog takes this one's place.
        run(app, id, p);
        return true;
    }
    ui.add_space(8.0);
    let mut done = ui.input(|i| i.key_pressed(egui::Key::Escape));
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        done |= ui.button(tl!("Close")).clicked();
    });
    done
}

/// Header Options › Create: the field names, saved as a header source; back to the Helper.
#[allow(clippy::too_many_arguments)]
fn header_source(
    app: &mut WordApp,
    ui: &mut Ui,
    fields: &mut Vec<String>,
    name: &mut String,
    selected: &mut Option<usize>,
    saving: &mut bool,
    message: &mut String,
) -> bool {
    let back = |app: &mut WordApp| {
        let _ = app.run("ui.dialog", json!({"name": "mailMergeHelper"}));
    };
    if *saving && let Some(path) = app.dialog_pick.take() {
        *saving = false;
        match app.run("mailings.headerSource", json!({"path": path, "fields": fields})) {
            Ok(_) => {
                app.header_source = Some(path);
                back(app);
                return true;
            }
            Err(e) => *message = e,
        }
    }
    ui.label(tl!("A header source holds the field names for a data source without a header row."));
    ui.horizontal(|ui| {
        ui.label(tl!("Field name:"));
        let r = ui.add(crate::widgets::text_line(name).desired_width(160.0).char_limit(40));
        let typed = name.trim().to_string();
        let known = fields.iter().any(|f| f.eq_ignore_ascii_case(&typed));
        let add = ui.add_enabled(!typed.is_empty() && !known, egui::Button::new(tl!("Add Field Name"))).clicked()
            || (r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) && !typed.is_empty() && !known);
        if add {
            fields.push(typed);
            name.clear();
            *selected = Some(fields.len() - 1);
        }
    });
    ui.horizontal_top(|ui| {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_width(200.0);
            egui::ScrollArea::vertical().id_salt("header_fields").max_height(180.0).show(ui, |ui| {
                for (i, f) in fields.iter().enumerate() {
                    if ui.selectable_label(*selected == Some(i), f.as_str()).clicked() {
                        *selected = Some(i);
                    }
                }
            });
        });
        ui.vertical(|ui| {
            let sel = selected.filter(|i| *i < fields.len());
            if ui.add_enabled(sel.is_some(), egui::Button::new(tl!("Remove Field Name"))).clicked()
                && let Some(i) = sel
            {
                *name = fields.remove(i);
                *selected = None;
            }
            if ui.add_enabled(sel.is_some_and(|i| i > 0), egui::Button::new(tl!("Move Up"))).clicked()
                && let Some(i) = sel
            {
                fields.swap(i, i - 1);
                *selected = Some(i - 1);
            }
            if ui.add_enabled(sel.is_some_and(|i| i + 1 < fields.len()), egui::Button::new(tl!("Move Down"))).clicked()
                && let Some(i) = sel
            {
                fields.swap(i, i + 1);
                *selected = Some(i + 1);
            }
        });
    });
    if !message.is_empty() {
        ui.colored_label(ui.visuals().error_fg_color, message.as_str());
    }
    let (ok, cancel) = buttons(ui, "OK", !fields.is_empty() && !*saving);
    if ok {
        *saving = true;
        let _ = app.ask_file(FileDialogRequest::Save { name: "Header.csv".into() }, AfterPick::ForDialog);
    }
    if cancel {
        back(app);
        return true;
    }
    false
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

    /// dlg-203 and dlg-150: AutoCorrect Options › AutoFormat sets what AutoFormat changes, and
    /// AutoFormat and Review Changes opens Word's "Formatting completed" dialog, which finds and
    /// rejects one change and closes keeping the rest.
    #[test]
    fn autoformat_tab_and_review_dialog() {
        use crate::dialogs_review::ReviewDialog;
        let mut h = harness(app("SUMMARY\nIt's \"fine\" -- really.\nThe end of it."));
        h.state_mut().run("tools.autocorrect", json!({})).unwrap();
        h.run_steps(3);
        match h.state_mut().dialog.as_mut() {
            Some(Dialog::Review(d)) => match &mut **d {
                ReviewDialog::AutoCorrect(f) => {
                    f.tab = 3;
                    f.options.autoformat.dashes = false;
                }
                other => panic!("{other:?}"),
            },
            other => panic!("{other:?}"),
        }
        h.run_steps(2);
        h.get_by_label("AutoFormat and Review Changes…").click();
        h.run_steps(3);
        assert!(!h.state().session.prefs.autocorrect.autoformat.dashes, "the tab's options are kept");
        let text = |h: &Harness<'_, WordApp>| h.state().session.doc.plain_text(doc4me_doc::StoryRef::Body);
        assert!(text(&h).contains("It\u{2019}s \u{201C}fine\u{201D} -- really."), "{}", text(&h));
        let changes = match form(&mut h) {
            DataDialog::AutoFormatReview { changes, .. } => changes.len(),
            other => panic!("{other:?}"),
        };
        assert_eq!(changes, 3, "heading, quotes, body text");
        h.get_by_label("Review Changes…").click();
        h.run_steps(2);
        // Review Changes finds the first change: the heading; Reject puts it back.
        assert_eq!(h.state().session.sel.focus.path.0[0], 0);
        h.get_by_label("Reject").click();
        h.run_steps(2);
        let para0 = |h: &Harness<'_, WordApp>| h.state().session.doc.body[0].as_para().and_then(|p| p.props.style.clone());
        assert_eq!(para0(&h), None);
        assert!(matches!(form(&mut h), DataDialog::AutoFormatReview { changes, .. } if changes.len() == 2));
        h.get_by_label("Close").click();
        h.run_steps(2);
        assert!(h.state().dialog.is_none());
        assert!(text(&h).contains("\u{201C}fine\u{201D}"), "the other changes stay");
    }

    /// dlg-117: Edit Individual Documents shows Merge to New Document (records, blank lines);
    /// dlg-124: the Mail Merge Helper; dlg-120 and dlg-135: Header Options › Create writes a
    /// header source and the next data source opened there takes its names.
    #[test]
    fn merge_dialog_helper_and_header_sources() {
        let dir = std::env::temp_dir().join(format!("doc4me-merge-ui-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut h = harness(app(""));
        {
            let a = h.state_mut();
            a.run("mailings.recipients", json!({"csv": "Name,City\nAda,London\nAlan,"})).unwrap();
            a.run("mailings.insertField", json!({"field": "Name"})).unwrap();
            a.run("text.newParagraph", json!({})).unwrap();
            a.run("mailings.insertField", json!({"field": "City"})).unwrap();
            a.run("text.newParagraph", json!({})).unwrap();
            a.run("text.insert", json!({"text": "End"})).unwrap();
            a.session.dirty = false;
            assert_eq!(a.run("mailings.finish", json!({})).unwrap()["pending"], "mergeToNew");
        }
        h.run_steps(3);
        match form(&mut h) {
            DataDialog::Merge { fixed, printer, records, from, to, blank_lines } => {
                assert!(*fixed && !*printer && !*blank_lines && *to == 2);
                *records = 2;
                *from = 2;
            }
            other => panic!("{other:?}"),
        }
        h.run_steps(1);
        h.get_by_label("Merge").click();
        h.run_steps(2);
        let paras: Vec<String> = h.state().session.doc.body.iter().filter_map(|b| b.as_para()).map(|p| p.plain_text()).collect();
        assert_eq!(paras, ["Alan", "End"], "record 2 only, without the blank City line");
        // The Helper: Create… opens Create Header Source with Word's fields.
        h.state_mut().run("ui.dialog", json!({"name": "mailMergeHelper"})).unwrap();
        h.run_steps(3);
        h.get_by_label("Create…").click();
        h.run_steps(3);
        let header = dir.join("names.csv").to_string_lossy().to_string();
        match form(&mut h) {
            DataDialog::HeaderSource { fields, saving, .. } => {
                assert_eq!(fields[0], "Title");
                *fields = vec!["Guest".into(), "Town".into()];
                // OK asked for a file name; the answer comes.
                *saving = true;
            }
            other => panic!("{other:?}"),
        }
        h.state_mut().dialog_pick = Some(header.clone());
        h.run_steps(3);
        assert_eq!(std::fs::read_to_string(&header).unwrap(), "Guest,Town\r\n");
        assert_eq!(h.state().header_source.as_deref(), Some(header.as_str()));
        // Back in the Helper: Open Data Source takes a list without a header row.
        let data = dir.join("guests.csv");
        std::fs::write(&data, "Grace,Arlington\nKath,\n").unwrap();
        match form(&mut h) {
            DataDialog::MergeHelper { picking, .. } => *picking = 2,
            other => panic!("{other:?}"),
        }
        h.state_mut().dialog_pick = Some(data.to_string_lossy().to_string());
        h.run_steps(3);
        let m = &h.state().session.merge;
        assert_eq!(m.headers, ["Guest", "Town"]);
        assert_eq!(m.rows.len(), 2);
        // Merge… opens Merge with its Merge to choice.
        h.get_by_label("Merge…").click();
        h.run_steps(3);
        assert!(matches!(form(&mut h), DataDialog::Merge { fixed: false, .. }));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// dlg-193: File › Options › Proofing › Writing Style › Settings… (Grammar Settings) turns a
    /// rule off with OK and back on with Reset All.
    #[test]
    fn grammar_settings_dialog() {
        let mut h = harness(app("this is is it."));
        h.state_mut().run("ui.dialog", json!({"name": "grammarSettings"})).unwrap();
        h.run_steps(3);
        match form(&mut h) {
            DataDialog::GrammarSettings { rules } => {
                assert_eq!(rules, &vec![true; 5]);
                rules[1] = false;
            }
            other => panic!("{other:?}"),
        }
        h.run_steps(1);
        h.get_by_label("OK").click();
        h.run_steps(2);
        assert_eq!(h.state().session.prefs.grammar_off, ["repeated"]);
        assert!(h.state().dialog.is_none());
        h.state_mut().run("ui.dialog", json!({"name": "grammarSettings"})).unwrap();
        h.run_steps(3);
        assert!(matches!(form(&mut h), DataDialog::GrammarSettings { rules } if !rules[1]));
        h.get_by_label("Reset All").click();
        h.run_steps(1);
        h.get_by_label("OK").click();
        h.run_steps(2);
        assert!(h.state().session.prefs.grammar_off.is_empty());
    }
}
