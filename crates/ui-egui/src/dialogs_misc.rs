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
    }
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
    }
}
