//! Word's smaller dialogs: Drawing Grid (Align › Grid Settings…), Compatibility Options and Font
//! Substitution (File › Options › Advanced), Insert Rows and Insert Cells (Table Layout › Rows &
//! Columns), Number (Insert › Symbols) and Placeholder Name (Insert Citation › Add New
//! Placeholder…). Each ends by running a command (`layout.drawingGrid`,
//! `file.compatibilityOptions`, `file.fontSubstitution`, `table.insertRows`, `table.insertCells`,
//! `insert.number`, `references.placeholder`), so scripts and agents get the same result without
//! the dialog. Shown through [`crate::dialogs::Dialog::Misc`].

use doc4me_engine::cmd::insert::NUMBER_FORMATS;
use egui::{Ui, vec2};
use serde::Serialize;
use serde_json::{Value, json};

use crate::WordApp;
use crate::theme::semibold;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "form", rename_all = "camelCase")]
pub enum MiscDialog {
    /// Grid spacing in the interface unit, and gridlines on screen.
    DrawingGrid {
        horizontal: f32,
        vertical: f32,
        show: bool,
    },
    /// Don't use HTML paragraph auto spacing; split apart page break and paragraph mark.
    Compatibility {
        mode: u64,
        options: [bool; 2],
    },
    /// The missing fonts and what shows instead; the rows picked for Convert Permanently.
    FontSubstitution {
        fonts: Vec<(String, String)>,
        picked: Vec<bool>,
    },
    InsertRows {
        count: u32,
        below: bool,
    },
    /// `down`, `row` or `column`.
    InsertCells {
        shift: String,
    },
    /// The number as typed and the format (an index into [`NUMBER_FORMATS`]).
    InsertNumber {
        number: String,
        format: usize,
    },
    Placeholder {
        name: String,
    },
    /// Cell margins in the interface unit (top, bottom, left, right) or the table's; Wrap text.
    CellOptions {
        same: bool,
        margins: [f32; 4],
        wrap: bool,
    },
    /// Table Positioning: each axis's alignment (empty: the offset, in the interface unit) and
    /// what it is measured from; the distances from the text around (left, top, right, bottom).
    TablePositioning {
        horizontal: (String, f32, String),
        vertical: (String, f32, String),
        dist: [f32; 4],
        overlap: bool,
    },
}

impl MiscDialog {
    pub fn name(&self) -> &'static str {
        match self {
            MiscDialog::DrawingGrid { .. } => "drawingGrid",
            MiscDialog::Compatibility { .. } => "compatibilityOptions",
            MiscDialog::FontSubstitution { .. } => "fontSubstitution",
            MiscDialog::InsertRows { .. } => "insertRows",
            MiscDialog::InsertCells { .. } => "insertCells",
            MiscDialog::InsertNumber { .. } => "insertNumber",
            MiscDialog::Placeholder { .. } => "placeholderName",
            MiscDialog::CellOptions { .. } => "cellOptions",
            MiscDialog::TablePositioning { .. } => "tablePositioning",
        }
    }

    /// The window title (English; the caller translates it).
    pub fn title(&self) -> &'static str {
        match self {
            MiscDialog::DrawingGrid { .. } => "Drawing Grid",
            MiscDialog::Compatibility { .. } => "Compatibility Options",
            MiscDialog::FontSubstitution { .. } => "Font Substitution",
            MiscDialog::InsertRows { .. } => "Insert Rows",
            MiscDialog::InsertCells { .. } => "Insert Cells",
            MiscDialog::InsertNumber { .. } => "Number",
            MiscDialog::Placeholder { .. } => "Placeholder Name",
            MiscDialog::CellOptions { .. } => "Cell Options",
            MiscDialog::TablePositioning { .. } => "Table Positioning",
        }
    }
}

/// The dialog `ui.dialog` opens by `name`.
pub fn open(name: &str, app: &mut WordApp) -> Option<MiscDialog> {
    let read = |app: &mut WordApp, id: &str| app.session.run(id, &json!({})).unwrap_or_default();
    Some(match name {
        "drawingGrid" => {
            let v = read(app, "layout.drawingGrid");
            let u = crate::options::unit().pt_per_unit();
            let f = |k: &str| v.get(k).and_then(Value::as_f64).unwrap_or(9.0) as f32 / u;
            MiscDialog::DrawingGrid {
                horizontal: f("horizontal"),
                vertical: f("vertical"),
                show: v.get("show").and_then(Value::as_bool).unwrap_or(false),
            }
        }
        "compatibilityOptions" => {
            let v = read(app, "file.compatibilityOptions");
            let b = |k: &str| v.get(k).and_then(Value::as_bool).unwrap_or(false);
            MiscDialog::Compatibility {
                mode: v.get("mode").and_then(Value::as_u64).unwrap_or(15),
                options: [b("doNotUseHtmlParagraphAutoSpacing"), b("splitPageBreakAndParagraphMark")],
            }
        }
        "fontSubstitution" => {
            let fonts = substitutions(&read(app, "file.fontSubstitution"));
            let picked = vec![false; fonts.len()];
            MiscDialog::FontSubstitution { fonts, picked }
        }
        "insertRows" => MiscDialog::InsertRows { count: 1, below: false },
        "insertCells" => MiscDialog::InsertCells { shift: "down".into() },
        "insertNumber" => MiscDialog::InsertNumber { number: String::new(), format: 0 },
        "placeholderName" => {
            let n = (1..).find(|i| !app.session.doc.sources.iter().any(|s| s.tag == format!("Placeholder{i}"))).unwrap_or(1);
            MiscDialog::Placeholder { name: format!("Placeholder{n}") }
        }
        "cellOptions" => {
            let v = app.session.run("table.cellOptions", &json!({})).ok()?;
            let u = crate::options::unit().pt_per_unit();
            let f = |k: &str| v.get(k).and_then(Value::as_f64).unwrap_or(0.0) as f32 / u;
            MiscDialog::CellOptions {
                same: v.get("sameAsTable").and_then(Value::as_bool).unwrap_or(true),
                margins: [f("top"), f("bottom"), f("left"), f("right")],
                wrap: v.get("wrap").and_then(Value::as_bool).unwrap_or(true),
            }
        }
        "tablePositioning" => {
            let v = app.session.run("table.positioning", &json!({})).ok()?;
            let u = crate::options::unit().pt_per_unit();
            let axis = |k: &str, from: &str| {
                let at = v.get(k).cloned().unwrap_or_default();
                let s = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
                (at.as_str().unwrap_or("").to_string(), at.as_f64().unwrap_or(0.0) as f32 / u, s(from))
            };
            let d = |k: &str| v.get(k).and_then(Value::as_f64).unwrap_or(0.0) as f32 / u;
            MiscDialog::TablePositioning {
                horizontal: axis("horizontal", "horizontalFrom"),
                vertical: axis("vertical", "verticalFrom"),
                dist: [d("left"), d("top"), d("right"), d("bottom")],
                overlap: v.get("overlap").and_then(Value::as_bool).unwrap_or(true),
            }
        }
        _ => return None,
    })
}

/// `file.fontSubstitution`'s (font, substitute) rows.
fn substitutions(v: &Value) -> Vec<(String, String)> {
    let s = |f: &Value, k: &str| f.get(k).and_then(Value::as_str).unwrap_or("").to_string();
    v.get("fonts").and_then(Value::as_array).into_iter().flatten().map(|f| (s(f, "font"), s(f, "substitute"))).collect()
}

/// Draws the dialog; returns true to close it.
pub fn body(app: &mut WordApp, ui: &mut Ui, d: &mut MiscDialog) -> bool {
    match d {
        MiscDialog::DrawingGrid { horizontal, vertical, show } => {
            let unit = crate::options::unit();
            egui::Grid::new("dgrid").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
                for (label, v) in [("Horizontal spacing:", &mut *horizontal), ("Vertical spacing:", &mut *vertical)] {
                    ui.label(tl!(label));
                    ui.add(
                        egui::DragValue::new(v)
                            .speed(0.01)
                            .range(0.5 / unit.pt_per_unit()..=100.0 / unit.pt_per_unit())
                            .clamp_existing_to_range(false)
                            .suffix(unit.suffix())
                            .max_decimals(2),
                    );
                    ui.end_row();
                }
            });
            ui.checkbox(show, tl!("Display gridlines on screen"));
            let (ok, cancel) = buttons(ui, true);
            if ok {
                let u = unit.pt_per_unit();
                return run(app, "layout.drawingGrid", json!({"horizontal": *horizontal * u, "vertical": *vertical * u, "show": *show}));
            }
            cancel
        }
        MiscDialog::Compatibility { mode, options } => {
            let version = match *mode {
                15.. => "Word 2013 and later",
                14 => "Word 2010",
                12 => "Word 2007",
                _ => "Word 97–2003",
            };
            ui.label(crate::i18n::fmt(tl!("Compatibility mode: {version}"), &[("version", tl!(version))]));
            heading(ui, "Layout options for this document");
            ui.checkbox(&mut options[0], tl!("Don't use HTML paragraph auto spacing"));
            ui.checkbox(&mut options[1], tl!("Split apart page break and paragraph mark"));
            let (ok, cancel) = buttons(ui, true);
            if ok {
                let p = json!({"doNotUseHtmlParagraphAutoSpacing": options[0], "splitPageBreakAndParagraphMark": options[1]});
                return run(app, "file.compatibilityOptions", p);
            }
            cancel
        }
        MiscDialog::FontSubstitution { fonts, picked } => {
            if fonts.is_empty() {
                ui.label(tl!("Every font this document uses is installed."));
            } else {
                egui::Grid::new("fsub").num_columns(2).striped(true).spacing(vec2(16.0, 4.0)).show(ui, |ui| {
                    ui.label(egui::RichText::new(tl!("Missing document font")).font(semibold(12.5)));
                    ui.label(egui::RichText::new(tl!("Substituted font")).font(semibold(12.5)));
                    ui.end_row();
                    for ((font, sub), on) in fonts.iter().zip(picked.iter_mut()) {
                        ui.checkbox(on, font.as_str());
                        ui.label(sub.as_str());
                        ui.end_row();
                    }
                });
            }
            let mut close = false;
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tl!("Close")).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        close = true;
                    }
                    let names: Vec<&str> = fonts.iter().zip(picked.iter()).filter(|(_, on)| **on).map(|((f, _), _)| f.as_str()).collect();
                    if ui.add_enabled(!names.is_empty(), egui::Button::new(tl!("Convert Permanently"))).clicked() {
                        match app.run("file.fontSubstitution", json!({"convert": names})) {
                            Ok(v) => {
                                *fonts = substitutions(&v);
                                *picked = vec![false; fonts.len()];
                            }
                            Err(e) => app.status(e),
                        }
                    }
                });
            });
            close
        }
        MiscDialog::InsertRows { count, below } => {
            ui.horizontal(|ui| {
                ui.label(tl!("Number of rows:"));
                ui.add(egui::DragValue::new(count).range(1..=100).clamp_existing_to_range(false));
            });
            ui.radio_value(below, false, tl!("Insert Above"));
            ui.radio_value(below, true, tl!("Insert Below"));
            let (ok, cancel) = buttons(ui, (1..=100).contains(count));
            if ok {
                return run(app, "table.insertRows", json!({"count": *count, "below": *below}));
            }
            cancel
        }
        MiscDialog::InsertCells { shift } => {
            ui.add_enabled_ui(false, |ui| ui.radio(false, tl!("Shift cells right")))
                .inner
                .on_disabled_hover_text(tl!("A table's rows keep the same columns here."));
            for (k, label) in [("down", "Shift cells down"), ("row", "Insert entire row"), ("column", "Insert entire column")] {
                ui.radio_value(shift, k.to_string(), tl!(label));
            }
            let (ok, cancel) = buttons(ui, true);
            if ok {
                return run(app, "table.insertCells", json!({"shift": shift}));
            }
            cancel
        }
        MiscDialog::InsertNumber { number, format } => {
            ui.horizontal(|ui| {
                ui.label(tl!("Number:"));
                ui.add(crate::widgets::text_line(number).desired_width(120.0));
            });
            let n = number.trim().parse::<u32>().ok().filter(|n| *n <= 999_999);
            ui.label(tl!("Number type:"));
            egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                for (i, (_, fmt)) in NUMBER_FORMATS.iter().enumerate() {
                    // Each type shows how it writes 1, 2, 3.
                    let sample = (1..=3).map(|k| fmt.format(k)).collect::<Vec<_>>().join(", ");
                    ui.radio_value(format, i, sample);
                }
            });
            let (switch, _) = NUMBER_FORMATS.get(*format).copied().unwrap_or(NUMBER_FORMATS[0]);
            let ok_input = n.is_some_and(|n| n > 0 || !matches!(switch, "alphabetic" | "ALPHABETIC" | "roman" | "ROMAN" | "ThaiLetter"));
            let (ok, cancel) = buttons(ui, ok_input);
            if ok && let Some(n) = n {
                return run(app, "insert.number", json!({"number": n, "format": switch}));
            }
            cancel
        }
        MiscDialog::Placeholder { name } => {
            ui.label(tl!("Type the tag name of the source. You can add more information to this source later by editing the source."));
            ui.add(crate::widgets::text_line(name).desired_width(260.0));
            let valid = !name.trim().is_empty() && name.trim().chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-');
            let (ok, cancel) = buttons(ui, valid);
            if ok {
                return run(app, "references.placeholder", json!({"name": name.trim()}));
            }
            cancel
        }
        MiscDialog::CellOptions { same, margins, wrap } => {
            heading(ui, "Cell margins");
            ui.checkbox(same, tl!("Same as the whole table"));
            egui::Grid::new("cellopt").num_columns(4).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
                let [top, bottom, left, right] = margins;
                for (i, (label, v)) in [("Top:", top), ("Left:", left), ("Bottom:", bottom), ("Right:", right)].into_iter().enumerate() {
                    ui.add_enabled(!*same, egui::Label::new(tl!(label)));
                    ui.add_enabled(!*same, length(v, 0.0, 144.0));
                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
            heading(ui, "Options");
            ui.checkbox(wrap, tl!("Wrap text"));
            ui.add_enabled_ui(false, |ui| ui.checkbox(&mut false, tl!("Fit text")))
                .inner
                .on_disabled_hover_text(tl!("Text isn't squeezed to fit its cell yet."));
            let (ok, cancel) = buttons(ui, true);
            if ok {
                let u = crate::options::unit().pt_per_unit();
                let [top, bottom, left, right] = margins.map(|m| m * u);
                let p = if *same {
                    json!({"sameAsTable": true, "wrap": *wrap})
                } else {
                    json!({"sameAsTable": false, "top": top, "bottom": bottom, "left": left, "right": right, "wrap": *wrap})
                };
                return run(app, "table.cellOptions", p);
            }
            cancel
        }
        MiscDialog::TablePositioning { horizontal, vertical, dist, overlap } => {
            let place = |a: &(String, f32, String)| if a.0.is_empty() { json!(a.1 * crate::options::unit().pt_per_unit()) } else { json!(a.0) };
            heading(ui, "Horizontal");
            egui::Grid::new("tpos_h").num_columns(5).spacing(vec2(8.0, 6.0)).show(ui, |ui| {
                let aligns = [("left", "Left"), ("right", "Right"), ("center", "Center"), ("inside", "Inside"), ("outside", "Outside")];
                axis(ui, "tpos_h", horizontal, &aligns, &[("margin", "Margin"), ("page", "Page"), ("column", "Column")]);
            });
            heading(ui, "Vertical");
            egui::Grid::new("tpos_v").num_columns(5).spacing(vec2(8.0, 6.0)).show(ui, |ui| {
                let aligns = [("top", "Top"), ("bottom", "Bottom"), ("center", "Center"), ("inside", "Inside"), ("outside", "Outside")];
                axis(ui, "tpos_v", vertical, &aligns, &[("margin", "Margin"), ("page", "Page"), ("paragraph", "Paragraph")]);
            });
            heading(ui, "Distance from surrounding text");
            egui::Grid::new("tpos_d").num_columns(4).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
                let [left, top, right, bottom] = dist;
                for (i, (label, v)) in [("Top:", top), ("Left:", left), ("Bottom:", bottom), ("Right:", right)].into_iter().enumerate() {
                    ui.label(tl!(label));
                    ui.add(length(v, 0.0, 1584.0));
                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
            heading(ui, "Options");
            // Move with text: measured from the paragraph the table stands in.
            let mut moves = vertical.2 == "paragraph";
            if ui.checkbox(&mut moves, tl!("Move with text")).changed() {
                vertical.2 = if moves { "paragraph" } else { "margin" }.into();
            }
            ui.checkbox(overlap, tl!("Allow overlap"));
            let mut remove = false;
            ui.add_space(4.0);
            if ui.button(tl!("Remove Positioning")).clicked() {
                remove = true;
            }
            let (ok, cancel) = buttons(ui, true);
            if remove {
                return run(app, "table.positioning", json!({"around": false}));
            }
            if ok {
                let u = crate::options::unit().pt_per_unit();
                let [left, top, right, bottom] = dist.map(|d| d * u);
                let p = json!({
                    "around": true,
                    "horizontal": place(horizontal), "horizontalFrom": horizontal.2,
                    "vertical": place(vertical), "verticalFrom": vertical.2,
                    "left": left, "top": top, "right": right, "bottom": bottom, "overlap": *overlap,
                });
                return run(app, "table.positioning", p);
            }
            cancel
        }
    }
}

/// A length in the interface unit, 0–`hi` points.
fn length(v: &mut f32, lo: f32, hi: f32) -> egui::DragValue<'_> {
    let unit = crate::options::unit();
    let k = unit.pt_per_unit();
    egui::DragValue::new(v).speed(0.01).range(lo / k..=hi / k).clamp_existing_to_range(false).suffix(unit.suffix()).max_decimals(2)
}

/// One axis of Table Positioning: the position (an alignment or an offset) and what it's
/// measured from.
fn axis(ui: &mut Ui, id: &str, a: &mut (String, f32, String), aligns: &[(&str, &'static str)], froms: &[(&str, &'static str)]) {
    let (at, off, from) = a;
    let label = |k: &str, list: &[(&str, &'static str)]| -> &'static str { list.iter().find(|(v, _)| *v == k).map_or("Custom", |(_, l)| *l) };
    ui.label(tl!("Position:"));
    egui::ComboBox::from_id_salt((id, "at")).selected_text(tl!(label(at, aligns))).show_ui(ui, |ui| {
        for (v, l) in aligns {
            ui.selectable_value(at, v.to_string(), tl!(l));
        }
        ui.selectable_value(at, String::new(), tl!("Custom"));
    });
    ui.add_enabled(at.is_empty(), length(off, -1584.0, 1584.0));
    ui.label(tl!("Relative to:"));
    egui::ComboBox::from_id_salt((id, "from")).selected_text(tl!(label(from, froms))).show_ui(ui, |ui| {
        for (v, l) in froms {
            ui.selectable_value(from, v.to_string(), tl!(l));
        }
    });
    ui.end_row();
}

fn heading(ui: &mut Ui, s: &str) {
    ui.add_space(4.0);
    ui.label(egui::RichText::new(tl!(s)).font(semibold(12.5)));
}

/// OK / Cancel; returns (ok, cancel). `ok_enabled` greys OK out.
fn buttons(ui: &mut Ui, ok_enabled: bool) -> (bool, bool) {
    let mut r = (false, false);
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Cancel")).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                r.1 = true;
            }
            if ui.add_enabled(ok_enabled, crate::widgets::primary_button(ui, tl!("OK"))).clicked() {
                r.0 = true;
            }
        });
    });
    r
}

/// Runs a dialog's command; a refusal shows in the status bar and keeps the dialog open.
fn run(app: &mut WordApp, id: &str, params: Value) -> bool {
    match app.run(id, params) {
        Ok(_) => true,
        Err(e) => {
            app.status(e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use doc4me_engine::Session;
    use egui_kittest::kittest::Queryable;
    use serde_json::json;

    use super::*;
    use crate::dialogs::Dialog;
    use crate::{Services, WordApp};

    fn form(a: &mut WordApp) -> MiscDialog {
        let Some(Dialog::Misc(d)) = a.dialog.take() else { panic!("no dialog: {:?}", a.dialog) };
        *d
    }

    /// Each command without its input opens its dialog, which shows the current settings.
    #[test]
    fn commands_without_input_open_their_dialogs() {
        let mut a = WordApp::new(Session::new(doc4me_doc::Document::from_text("Kilns")), Services::default());
        assert_eq!(a.run("layout.drawingGrid", json!({})).unwrap(), json!({"pending": "drawingGrid"}));
        let MiscDialog::DrawingGrid { show, .. } = form(&mut a) else { panic!() };
        assert!(!show);
        a.run("file.compatibilityOptions", json!({})).unwrap();
        assert_eq!(form(&mut a), MiscDialog::Compatibility { mode: 15, options: [false, false] });
        a.run("file.fontSubstitution", json!({})).unwrap();
        assert!(matches!(form(&mut a), MiscDialog::FontSubstitution { .. }));
        a.run("insert.number", json!({})).unwrap();
        assert!(matches!(form(&mut a), MiscDialog::InsertNumber { format: 0, .. }));
        a.run("references.placeholder", json!({})).unwrap();
        assert_eq!(form(&mut a), MiscDialog::Placeholder { name: "Placeholder1".into() });
        a.run("references.placeholder", json!({"name": "Placeholder1"})).unwrap();
        a.run("references.placeholder", json!({})).unwrap();
        assert_eq!(form(&mut a), MiscDialog::Placeholder { name: "Placeholder2".into() }, "the next free name");
        a.run("insert.table", json!({"rows": 2, "cols": 2})).unwrap();
        a.run("table.insertRows", json!({})).unwrap();
        assert_eq!(form(&mut a), MiscDialog::InsertRows { count: 1, below: false });
        a.run("table.insertCells", json!({})).unwrap();
        assert_eq!(form(&mut a), MiscDialog::InsertCells { shift: "down".into() });
        a.run("table.cellOptions", json!({})).unwrap();
        let MiscDialog::CellOptions { same, margins, wrap } = form(&mut a) else { panic!() };
        assert!(same && wrap);
        assert!((margins[2] * crate::options::unit().pt_per_unit() - 5.4).abs() < 1e-3, "{margins:?}");
        a.run("table.positioning", json!({})).unwrap();
        let MiscDialog::TablePositioning { horizontal, vertical, overlap, .. } = form(&mut a) else { panic!() };
        assert_eq!((horizontal.2.as_str(), vertical.2.as_str(), overlap), ("column", "paragraph", true), "Word's defaults");
    }

    /// Table Properties: Text wrapping Around with Positioning… wraps the table and opens Table
    /// Positioning, whose Remove Positioning puts it back in the text; the Cell tab's Options…
    /// opens Cell Options, whose OK sets the cell's margins and Wrap text.
    #[test]
    fn table_properties_leads_to_positioning_and_cell_options() {
        let mut a = WordApp::new(Session::new(doc4me_doc::Document::from_text("Kilns")), Services::default());
        a.run("insert.table", json!({"rows": 2, "cols": 2})).unwrap();
        a.run("table.properties", json!({})).unwrap();
        if let Some(Dialog::TableProperties { form, .. }) = &mut a.dialog {
            form.wrap_around = true;
        }
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(3);
        h.get_by_label("Positioning…").click();
        h.run_steps(3);
        let float = |a: &WordApp| a.session.doc.body.iter().find_map(|b| b.as_table().map(|t| t.props.float)).flatten();
        assert!(float(h.state()).is_some(), "Around wraps the table");
        assert!(matches!(h.state().dialog.as_ref().map(Dialog::name), Some("tablePositioning")), "{:?}", h.state().dialog);
        h.get_by_label("Remove Positioning").click();
        h.run_steps(3);
        assert!(float(h.state()).is_none() && h.state().dialog.is_none());
        h.state_mut().run("table.properties", json!({})).unwrap();
        h.run_steps(3);
        h.get_by_label("Options…").click();
        h.run_steps(3);
        if let Some(Dialog::Misc(d)) = &mut h.state_mut().dialog
            && let MiscDialog::CellOptions { same, wrap, .. } = &mut **d
        {
            (*same, *wrap) = (false, false);
        } else {
            panic!("no Cell Options: {:?}", h.state().dialog);
        }
        h.run_steps(2);
        h.get_by_label("OK").click();
        h.run_steps(3);
        let a = h.state();
        let (tp, r, c) = a.session.sel.focus.path.cell().unwrap();
        let cell = &a.session.doc.table(a.session.sel.focus.story, &tp).unwrap().rows[r].cells[c];
        assert_eq!((cell.props.margins, cell.props.no_wrap), (Some([0.0, 5.4, 0.0, 5.4]), true));
    }
}
