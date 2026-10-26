//! Word's dialogs for a selected picture, shape or text box: Layout (Position, Text Wrapping and
//! Size: `arrange.layout`), Format Picture (`picture.format`) and Format Shape (`shape.format`).
//! Each opens with the object's settings and its OK runs one command with what changed (one undo
//! step), so scripts and agents get the same result without the dialog. Shown through
//! [`crate::dialogs::Dialog::Object`].

use egui::{Ui, vec2};
use serde::Serialize;
use serde_json::{Map, Value, json};

use crate::WordApp;
use crate::theme::semibold;

/// Horizontal and vertical alignments, and what positions are measured from, with their labels.
const H_ALIGN: [(&str, &str); 5] = [("left", "Left"), ("center", "Centered"), ("right", "Right"), ("inside", "Inside"), ("outside", "Outside")];
const V_ALIGN: [(&str, &str); 5] = [("top", "Top"), ("center", "Centered"), ("bottom", "Bottom"), ("inside", "Inside"), ("outside", "Outside")];
const H_FROM: [(&str, &str); 8] = [
    ("margin", "Margin"),
    ("page", "Page"),
    ("column", "Column"),
    ("character", "Character"),
    ("leftMargin", "Left Margin"),
    ("rightMargin", "Right Margin"),
    ("insideMargin", "Inside Margin"),
    ("outsideMargin", "Outside Margin"),
];
const V_FROM: [(&str, &str); 8] = [
    ("margin", "Margin"),
    ("page", "Page"),
    ("paragraph", "Paragraph"),
    ("line", "Line"),
    ("topMargin", "Top Margin"),
    ("bottomMargin", "Bottom Margin"),
    ("insideMargin", "Inside Margin"),
    ("outsideMargin", "Outside Margin"),
];
const WRAPS: [(&str, &str); 7] = [
    ("inline", "In Line with Text"),
    ("square", "Square"),
    ("tight", "Tight"),
    ("through", "Through"),
    ("topAndBottom", "Top and Bottom"),
    ("behindText", "Behind Text"),
    ("inFrontOfText", "In Front of Text"),
];
const SIDES: [(&str, &str); 4] = [("bothSides", "Both sides"), ("left", "Left only"), ("right", "Right only"), ("largest", "Largest only")];

/// One axis of a position: an alignment (empty: the offset, in the interface unit) and what it is
/// measured from.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Axis {
    pub align: String,
    pub offset: f32,
    pub from: String,
}

/// The Layout dialog's fields (lengths in the interface unit).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutForm {
    pub tab: usize,
    pub horizontal: Axis,
    pub vertical: Axis,
    pub move_with_text: bool,
    pub wrap: String,
    pub side: String,
    /// Distance from the text: top, bottom, left, right.
    pub dist: [f32; 4],
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub lock: bool,
    /// The size as the Size tab last drew it (the locked side follows the other).
    #[serde(skip)]
    pub seen: Option<(f32, f32)>,
}

/// Format Picture's fields.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PictureForm {
    pub tab: usize,
    pub width: f32,
    pub height: f32,
    pub lock: bool,
    #[serde(skip)]
    pub seen: Option<(f32, f32)>,
    pub rotation: f32,
    /// Crop: left, top, right, bottom, percent of the picture.
    pub crop: [f32; 4],
    /// Changes from the picture as it is now.
    pub brightness: f32,
    pub contrast: f32,
    pub transparency: f32,
    pub alt: String,
    pub title: String,
    pub decorative: bool,
}

/// Format Shape's fields (colours as hex, empty for none).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ShapeForm {
    pub tab: usize,
    pub fill: String,
    pub line: String,
    pub line_width: f32,
    pub width: f32,
    pub height: f32,
    pub lock: bool,
    #[serde(skip)]
    pub seen: Option<(f32, f32)>,
    pub rotation: f32,
    pub alt: String,
    pub title: String,
    pub decorative: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "object", rename_all = "camelCase")]
pub enum ObjectDialog {
    /// The form, and how it was when the dialog opened (OK sends what changed).
    Layout {
        form: LayoutForm,
        was: Box<LayoutForm>,
    },
    Picture {
        form: PictureForm,
        was: Box<PictureForm>,
    },
    Shape {
        form: ShapeForm,
        was: Box<ShapeForm>,
    },
}

impl ObjectDialog {
    pub fn name(&self) -> &'static str {
        match self {
            ObjectDialog::Layout { .. } => "objectLayout",
            ObjectDialog::Picture { .. } => "formatPicture",
            ObjectDialog::Shape { .. } => "formatShape",
        }
    }

    /// The window title (English; the caller translates it).
    pub fn title(&self) -> &'static str {
        match self {
            ObjectDialog::Layout { .. } => "Layout",
            ObjectDialog::Picture { .. } => "Format Picture",
            ObjectDialog::Shape { .. } => "Format Shape",
        }
    }
}

fn num(v: &Value, k: &str) -> f32 {
    v.get(k).and_then(Value::as_f64).unwrap_or(0.0) as f32
}

fn text(v: &Value, k: &str) -> String {
    v.get(k).and_then(Value::as_str).unwrap_or("").to_string()
}

/// The dialog `ui.dialog` opens by `name`, for the selected object (`None` without one).
pub fn open(name: &str, app: &mut WordApp) -> Option<ObjectDialog> {
    let u = crate::options::unit().pt_per_unit();
    match name {
        "objectLayout" => {
            let v = app.session.run("arrange.layout", &json!({})).ok()?;
            let axis = |at: &str, from: &str| {
                let a = v.get(at).cloned().unwrap_or_default();
                Axis { align: a.as_str().unwrap_or("").to_string(), offset: a.as_f64().unwrap_or(0.0) as f32 / u, from: text(&v, from) }
            };
            let form = LayoutForm {
                tab: 0,
                horizontal: axis("horizontal", "horizontalFrom"),
                vertical: axis("vertical", "verticalFrom"),
                move_with_text: v["moveWithText"].as_bool().unwrap_or(true),
                wrap: text(&v, "wrap"),
                side: text(&v, "wrapSide"),
                dist: [num(&v, "top") / u, num(&v, "bottom") / u, num(&v, "left") / u, num(&v, "right") / u],
                width: num(&v, "width") / u,
                height: num(&v, "height") / u,
                rotation: num(&v, "rotation"),
                lock: true,
                seen: None,
            };
            Some(ObjectDialog::Layout { was: Box::new(form.clone()), form })
        }
        "formatPicture" => {
            let v = app.session.run("picture.format", &json!({})).ok()?;
            let c = |k: &str| v["crop"].get(k).and_then(Value::as_f64).unwrap_or(0.0) as f32 * 100.0;
            let form = PictureForm {
                tab: 0,
                width: num(&v, "width") / u,
                height: num(&v, "height") / u,
                lock: true,
                seen: None,
                rotation: num(&v, "rotation"),
                crop: [c("left"), c("top"), c("right"), c("bottom")],
                brightness: 0.0,
                contrast: 0.0,
                transparency: 0.0,
                alt: text(&v, "alt"),
                title: text(&v, "title"),
                decorative: v["decorative"].as_bool().unwrap_or(false),
            };
            Some(ObjectDialog::Picture { was: Box::new(form.clone()), form })
        }
        "formatShape" => {
            let v = app.session.run("shape.format", &json!({})).ok()?;
            let form = ShapeForm {
                tab: 0,
                fill: text(&v, "fill"),
                line: text(&v, "line"),
                line_width: num(&v, "lineWidth"),
                width: num(&v, "width") / u,
                height: num(&v, "height") / u,
                lock: false,
                seen: None,
                rotation: num(&v, "rotation"),
                alt: text(&v, "alt"),
                title: text(&v, "title"),
                decorative: v["decorative"].as_bool().unwrap_or(false),
            };
            Some(ObjectDialog::Shape { was: Box::new(form.clone()), form })
        }
        _ => None,
    }
}

/// What changed, as the command takes it (lengths in points).
fn moved(a: f32, b: f32) -> bool {
    (a - b).abs() > 1e-4
}

impl LayoutForm {
    pub fn changes(&self, was: &LayoutForm) -> Value {
        let u = crate::options::unit().pt_per_unit();
        let mut m = Map::new();
        let place = |a: &Axis| if a.align.is_empty() { json!(a.offset * u) } else { json!(a.align) };
        if self.horizontal != was.horizontal {
            m.insert("horizontal".into(), place(&self.horizontal));
            m.insert("horizontalFrom".into(), json!(self.horizontal.from));
        }
        if self.vertical != was.vertical {
            m.insert("vertical".into(), place(&self.vertical));
            m.insert("verticalFrom".into(), json!(self.vertical.from));
        }
        if self.move_with_text != was.move_with_text && self.vertical.from == was.vertical.from {
            m.insert("moveWithText".into(), json!(self.move_with_text));
        }
        if self.wrap != was.wrap {
            m.insert("wrap".into(), json!(self.wrap));
        }
        if self.side != was.side {
            m.insert("wrapSide".into(), json!(self.side));
        }
        for (i, k) in ["top", "bottom", "left", "right"].into_iter().enumerate() {
            if moved(self.dist[i], was.dist[i]) {
                m.insert(k.into(), json!(self.dist[i] * u));
            }
        }
        if moved(self.width, was.width) {
            m.insert("width".into(), json!(self.width * u));
        }
        if moved(self.height, was.height) {
            m.insert("height".into(), json!(self.height * u));
        }
        if moved(self.rotation, was.rotation) {
            m.insert("rotation".into(), json!(self.rotation));
        }
        Value::Object(m)
    }
}

impl PictureForm {
    pub fn changes(&self, was: &PictureForm) -> Value {
        let u = crate::options::unit().pt_per_unit();
        let mut m = Map::new();
        if moved(self.width, was.width) || moved(self.height, was.height) {
            m.insert("width".into(), json!(self.width * u));
            m.insert("height".into(), json!(self.height * u));
        }
        if self.crop.iter().zip(&was.crop).any(|(a, b)| moved(*a, *b)) {
            let c = |i: usize| self.crop[i].clamp(0.0, 45.0) / 100.0;
            m.insert("crop".into(), json!({"left": c(0), "top": c(1), "right": c(2), "bottom": c(3)}));
        }
        for (k, v) in [("brightness", self.brightness), ("contrast", self.contrast), ("transparency", self.transparency)] {
            if v != 0.0 {
                m.insert(k.into(), json!(v));
            }
        }
        if moved(self.rotation, was.rotation) {
            m.insert("rotation".into(), json!(self.rotation));
        }
        alt_changes(&mut m, (&self.alt, &self.title, self.decorative), (&was.alt, &was.title, was.decorative));
        Value::Object(m)
    }
}

impl ShapeForm {
    pub fn changes(&self, was: &ShapeForm) -> Value {
        let u = crate::options::unit().pt_per_unit();
        let mut m = Map::new();
        let color = |c: &str| if c.is_empty() { Value::Null } else { json!(c) };
        if self.fill != was.fill {
            m.insert("fill".into(), color(&self.fill));
        }
        if self.line != was.line {
            m.insert("line".into(), color(&self.line));
        }
        if moved(self.line_width, was.line_width) {
            m.insert("lineWidth".into(), json!(self.line_width));
        }
        if moved(self.width, was.width) {
            m.insert("width".into(), json!(self.width * u));
        }
        if moved(self.height, was.height) {
            m.insert("height".into(), json!(self.height * u));
        }
        if moved(self.rotation, was.rotation) {
            m.insert("rotation".into(), json!(self.rotation));
        }
        alt_changes(&mut m, (&self.alt, &self.title, self.decorative), (&was.alt, &was.title, was.decorative));
        Value::Object(m)
    }
}

fn alt_changes(m: &mut Map<String, Value>, now: (&str, &str, bool), was: (&str, &str, bool)) {
    if now.0 != was.0 {
        m.insert("alt".into(), json!(now.0));
    }
    if now.1 != was.1 {
        m.insert("title".into(), json!(now.1));
    }
    if now.2 != was.2 {
        m.insert("decorative".into(), json!(now.2));
    }
}

/// Draws the dialog; returns true to close it.
pub fn body(app: &mut WordApp, ui: &mut Ui, d: &mut ObjectDialog) -> bool {
    let (id, params) = match d {
        ObjectDialog::Layout { form, was } => {
            layout_ui(ui, form);
            ("arrange.layout", form.changes(was))
        }
        ObjectDialog::Picture { form, was } => {
            picture_ui(ui, form);
            ("picture.format", form.changes(was))
        }
        ObjectDialog::Shape { form, was } => {
            let theme = app.session.doc.settings.theme_colors.clone();
            shape_ui(ui, form, &theme);
            ("shape.format", form.changes(was))
        }
    };
    let (ok, cancel) = buttons(ui);
    if ok {
        if params.as_object().is_some_and(|m| m.is_empty()) {
            return true;
        }
        return match app.run(id, params) {
            Ok(_) => true,
            Err(e) => {
                app.status(e);
                false
            }
        };
    }
    cancel
}

fn tabs(ui: &mut Ui, tab: &mut usize, names: &[&str]) {
    ui.horizontal(|ui| {
        for (i, n) in names.iter().enumerate() {
            if ui.selectable_label(*tab == i, tl!(n)).clicked() {
                *tab = i;
            }
        }
    });
    ui.separator();
}

fn heading(ui: &mut Ui, s: &str) {
    ui.add_space(4.0);
    ui.label(egui::RichText::new(tl!(s)).font(semibold(12.5)));
}

/// A length in the interface unit (`lo`–`hi` points).
fn length(v: &mut f32, lo: f32, hi: f32) -> egui::DragValue<'_> {
    let unit = crate::options::unit();
    let k = unit.pt_per_unit();
    egui::DragValue::new(v).speed(0.01).range(lo / k..=hi / k).clamp_existing_to_range(false).suffix(unit.suffix()).max_decimals(2)
}

fn choice(ui: &mut Ui, id: &str, value: &mut String, options: &[(&str, &str)]) {
    let shown = options.iter().find(|(v, _)| *v == value.as_str()).map_or("", |(_, l)| *l);
    egui::ComboBox::from_id_salt(id).selected_text(tl!(shown)).width(150.0).show_ui(ui, |ui| {
        for (v, l) in options {
            ui.selectable_value(value, v.to_string(), tl!(l));
        }
    });
}

/// One axis of Position: Alignment or Absolute position, and what it's measured from.
fn axis_ui(ui: &mut Ui, id: &str, a: &mut Axis, aligns: &[(&str, &str)], froms: &[(&str, &str)]) {
    egui::Grid::new(id).num_columns(4).spacing(vec2(8.0, 6.0)).show(ui, |ui| {
        let mut aligned = !a.align.is_empty();
        if ui.radio(aligned, tl!("Alignment")).clicked() && !aligned {
            aligned = true;
            a.align = aligns[0].0.to_string();
        }
        ui.add_enabled_ui(aligned, |ui| choice(ui, &format!("{id}_al"), &mut a.align, aligns));
        ui.label(tl!("relative to"));
        choice(ui, &format!("{id}_from"), &mut a.from, froms);
        ui.end_row();
        if ui.radio(!aligned, tl!("Absolute position")).clicked() {
            a.align.clear();
        }
        ui.add_enabled(a.align.is_empty(), length(&mut a.offset, -1584.0, 1584.0));
        ui.label(tl!("relative to"));
        ui.label("");
        ui.end_row();
    });
}

fn layout_ui(ui: &mut Ui, f: &mut LayoutForm) {
    tabs(ui, &mut f.tab, &["Position", "Text Wrapping", "Size"]);
    match f.tab {
        0 => {
            heading(ui, "Horizontal");
            axis_ui(ui, "lay_h", &mut f.horizontal, &H_ALIGN, &H_FROM);
            heading(ui, "Vertical");
            axis_ui(ui, "lay_v", &mut f.vertical, &V_ALIGN, &V_FROM);
            heading(ui, "Options");
            if ui.checkbox(&mut f.move_with_text, tl!("Move object with text")).changed() {
                f.vertical.from = if f.move_with_text { "paragraph" } else { "margin" }.into();
            }
        }
        1 => {
            heading(ui, "Wrapping style");
            ui.horizontal_wrapped(|ui| {
                for (v, l) in WRAPS {
                    ui.radio_value(&mut f.wrap, v.to_string(), tl!(l));
                }
            });
            let sided = matches!(f.wrap.as_str(), "square" | "tight" | "through");
            heading(ui, "Wrap text");
            ui.add_enabled_ui(sided, |ui| {
                ui.horizontal(|ui| {
                    for (v, l) in SIDES {
                        ui.radio_value(&mut f.side, v.to_string(), tl!(l));
                    }
                });
            });
            heading(ui, "Distance from text");
            let floats = f.wrap != "inline";
            egui::Grid::new("lay_dist").num_columns(4).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
                for (i, l) in ["Top:", "Left:", "Bottom:", "Right:"].into_iter().enumerate() {
                    let k = [0, 2, 1, 3][i];
                    ui.add_enabled(floats, egui::Label::new(tl!(l)));
                    ui.add_enabled(floats, length(&mut f.dist[k], 0.0, 1584.0));
                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
        }
        _ => {
            size_ui(ui, "lay_size", (&mut f.width, &mut f.height, &mut f.lock, &mut f.seen), &mut f.rotation);
        }
    }
}

/// A Size tab's width, height, aspect-ratio lock and the size it drew last.
type Size<'a> = (&'a mut f32, &'a mut f32, &'a mut bool, &'a mut Option<(f32, f32)>);

/// Height and width (aspect ratio locked or not) and rotation. A side changed since the last frame
/// (here or by a script) takes the other along while the ratio is locked.
fn size_ui(ui: &mut Ui, id: &str, (w, h, lock, seen): Size<'_>, rotation: &mut f32) {
    let (w0, h0) = seen.unwrap_or((*w, *h));
    egui::Grid::new(id).num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
        ui.label(tl!("Height:"));
        ui.add(length(h, 1.0, 1584.0));
        ui.end_row();
        ui.label(tl!("Width:"));
        ui.add(length(w, 1.0, 1584.0));
        ui.end_row();
        ui.label(tl!("Rotation:"));
        ui.add(egui::DragValue::new(rotation).range(-360.0..=360.0).clamp_existing_to_range(false).suffix("°").max_decimals(1));
        ui.end_row();
    });
    ui.checkbox(lock, tl!("Lock aspect ratio"));
    // Locked: the other side follows.
    if *lock && w0 > 0.0 && h0 > 0.0 {
        if moved(*w, w0) {
            *h = h0 * *w / w0;
        } else if moved(*h, h0) {
            *w = w0 * *h / h0;
        }
    }
    *seen = Some((*w, *h));
}

fn alt_ui(ui: &mut Ui, (alt, title, decorative): (&mut String, &mut String, &mut bool)) {
    ui.label(tl!("Title:"));
    ui.add(crate::widgets::text_line(title).desired_width(320.0));
    ui.label(tl!("Description:"));
    ui.add_enabled(!*decorative, crate::widgets::text_box(alt).desired_rows(3).desired_width(320.0));
    ui.checkbox(decorative, tl!("Mark as decorative"));
}

fn picture_ui(ui: &mut Ui, f: &mut PictureForm) {
    tabs(ui, &mut f.tab, &["Picture", "Size", "Crop", "Alt Text"]);
    match f.tab {
        0 => {
            heading(ui, "Picture Corrections");
            egui::Grid::new("fp_adj").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
                ui.label(tl!("Brightness:"));
                ui.add(egui::Slider::new(&mut f.brightness, -100.0..=100.0).suffix("%"));
                ui.end_row();
                ui.label(tl!("Contrast:"));
                ui.add(egui::Slider::new(&mut f.contrast, -100.0..=100.0).suffix("%"));
                ui.end_row();
                ui.label(tl!("Transparency:"));
                ui.add(egui::Slider::new(&mut f.transparency, 0.0..=100.0).suffix("%"));
                ui.end_row();
            });
            ui.label(egui::RichText::new(tl!("Brightness, contrast and transparency change the picture from how it looks now.")).small());
        }
        1 => size_ui(ui, "fp_size", (&mut f.width, &mut f.height, &mut f.lock, &mut f.seen), &mut f.rotation),
        2 => {
            heading(ui, "Crop");
            egui::Grid::new("fp_crop").num_columns(4).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
                for (i, l) in ["Left:", "Top:", "Right:", "Bottom:"].into_iter().enumerate() {
                    ui.label(tl!(l));
                    ui.add(egui::DragValue::new(&mut f.crop[i]).range(0.0..=45.0).clamp_existing_to_range(false).suffix("%").max_decimals(1));
                    if i % 2 == 1 {
                        ui.end_row();
                    }
                }
            });
        }
        _ => alt_ui(ui, (&mut f.alt, &mut f.title, &mut f.decorative)),
    }
}

fn shape_ui(ui: &mut Ui, f: &mut ShapeForm, theme: &[doc4me_doc::Rgb]) {
    tabs(ui, &mut f.tab, &["Fill & Line", "Size", "Alt Text"]);
    match f.tab {
        0 => {
            egui::Grid::new("fs_fill").num_columns(2).spacing(vec2(10.0, 8.0)).show(ui, |ui| {
                ui.label(tl!("Fill:"));
                crate::dialogs::color_field(ui, theme, &mut f.fill, "No Fill", "");
                ui.end_row();
                ui.label(tl!("Line:"));
                crate::dialogs::color_field(ui, theme, &mut f.line, "No Line", "");
                ui.end_row();
                ui.label(tl!("Line width:"));
                ui.add(
                    egui::DragValue::new(&mut f.line_width)
                        .range(0.0..=72.0)
                        .clamp_existing_to_range(false)
                        .speed(0.25)
                        .suffix(" pt")
                        .max_decimals(2),
                );
                ui.end_row();
            });
        }
        1 => size_ui(ui, "fs_size", (&mut f.width, &mut f.height, &mut f.lock, &mut f.seen), &mut f.rotation),
        _ => alt_ui(ui, (&mut f.alt, &mut f.title, &mut f.decorative)),
    }
}

/// OK / Cancel; returns (ok, cancel).
fn buttons(ui: &mut Ui) -> (bool, bool) {
    let mut r = (false, false);
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Cancel")).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                r.1 = true;
            }
            if ui.add(crate::widgets::primary_button(ui, tl!("OK"))).clicked() {
                r.0 = true;
            }
        });
    });
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dialogs::Dialog;
    use egui_kittest::kittest::Queryable;

    fn app_with_picture() -> WordApp {
        let mut a = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), Default::default());
        a.run("text.insert", json!({"text": "Text before the picture "})).unwrap();
        let mut png = Vec::new();
        let img = image::RgbaImage::from_pixel(40, 20, image::Rgba([200, 30, 30, 255]));
        image::DynamicImage::ImageRgba8(img).write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png).unwrap();
        a.run("insert.picture", json!({"data": doc4me_engine::cmd::insert::base64_encode(&png), "width": 160})).unwrap();
        a
    }

    fn harness(a: WordApp) -> egui_kittest::Harness<'static, WordApp> {
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(3);
        h
    }

    fn object<'h>(h: &'h mut egui_kittest::Harness<'static, WordApp>) -> &'h mut ObjectDialog {
        match &mut h.state_mut().dialog {
            Some(Dialog::Object(d)) => d,
            d => panic!("no object dialog: {d:?}"),
        }
    }

    /// The ribbon's Layout, Format Picture and Format Shape open their dialogs with the object's
    /// settings; OK runs one command with what changed (one undo step), and OK with nothing
    /// changed leaves the document alone.
    #[test]
    fn layout_and_format_dialogs_apply_what_changed() {
        let mut a = app_with_picture();
        a.run("arrange.layout", json!({})).unwrap();
        let mut h = harness(a);
        let ObjectDialog::Layout { form, .. } = object(&mut h) else { panic!() };
        assert_eq!((form.wrap.as_str(), form.horizontal.align.as_str()), ("inline", ""));
        form.wrap = "square".into();
        form.horizontal = Axis { align: "center".into(), offset: 0.0, from: "margin".into() };
        h.run_steps(2);
        let undo = h.state().session.undo_labels().len();
        h.get_by_label("OK").click();
        h.run_steps(3);
        assert!(h.state().dialog.is_none());
        assert_eq!(h.state().session.undo_labels().len(), undo + 1, "one undo step");
        let got = h.state_mut().session.run("arrange.layout", &json!({})).unwrap();
        assert_eq!((&got["wrap"], &got["horizontal"], &got["horizontalFrom"]), (&json!("square"), &json!("center"), &json!("margin")));

        // Format Picture: a width with the aspect ratio locked, and alt text.
        h.state_mut().run("picture.format", json!({})).unwrap();
        h.run_steps(2);
        let ObjectDialog::Picture { form, .. } = object(&mut h) else { panic!() };
        let (w, ratio) = (form.width, form.height / form.width);
        form.tab = 1;
        h.run_steps(2);
        let ObjectDialog::Picture { form, .. } = object(&mut h) else { panic!() };
        form.width = w / 2.0;
        form.alt = "A red block".into();
        h.run_steps(2);
        let ObjectDialog::Picture { form, .. } = object(&mut h) else { panic!() };
        assert!((form.height - form.width * ratio).abs() < 0.01, "the height follows a locked width");
        h.get_by_label("OK").click();
        h.run_steps(3);
        let got = h.state_mut().session.run("picture.format", &json!({})).unwrap();
        assert!((got["width"].as_f64().unwrap() - 80.0).abs() < 0.1, "{got}");
        assert_eq!(got["alt"], json!("A red block"));

        // Nothing changed: no undo step.
        let undo = h.state().session.undo_labels().len();
        h.state_mut().run("picture.format", json!({})).unwrap();
        h.run_steps(2);
        h.get_by_label("OK").click();
        h.run_steps(3);
        assert!(h.state().dialog.is_none());
        assert_eq!(h.state().session.undo_labels().len(), undo);
    }

    #[test]
    fn format_shape_sets_fill_and_line() {
        let mut a = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::new()), Default::default());
        a.run("insert.shape", json!({"kind": "rectangle"})).unwrap();
        a.run("shape.format", json!({})).unwrap();
        let mut h = harness(a);
        let ObjectDialog::Shape { form, .. } = object(&mut h) else { panic!() };
        form.fill = "00AA00".into();
        form.line.clear();
        h.run_steps(2);
        h.get_by_label("OK").click();
        h.run_steps(3);
        let got = h.state_mut().session.run("shape.format", &json!({})).unwrap();
        assert_eq!((&got["fill"], &got["line"]), (&json!("00AA00"), &Value::Null));
    }
}
