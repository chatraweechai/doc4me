//! References dialogs (#402): Caption (with New Label and Numbering), Index and Mark Index Entry,
//! Custom Table of Contents (with Options and the TOC styles), Mark Table of Contents Entry, and
//! the Source Manager with Create / Edit Source. Each fills a form from the session and ends in a
//! command (`references.caption`, `references.index`, `references.markEntry`, `references.toc`,
//! `references.markTocEntry`, `references.sources`, `references.citation`), so agents get the
//! same result without the dialog. Shown through [`crate::dialogs::Dialog::Refs`].

use doc4me_doc::{Block, Source, StyleKind};
use doc4me_engine::cmd::references::FieldCode;
use egui::{Sense, Ui, vec2};
use serde::Serialize;
use serde_json::{Value, json};

use crate::WordApp;
use crate::dialogs::{Dialog, buttons};
use crate::theme::{Tokens, semibold};

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "refs", rename_all = "camelCase")]
pub enum RefsDialog {
    Caption(CaptionForm),
    /// Caption › Numbering…; `was` comes back on Cancel.
    CaptionNumbering {
        form: CaptionForm,
        #[serde(skip)]
        was: Box<CaptionForm>,
    },
    /// Caption › New Label….
    NewLabel(CaptionForm),
    /// Caption › AutoCaption…: for each object, whether it gets a caption, the label and above
    /// or below; `back` is the Caption dialog to return to.
    AutoCaption {
        rows: Vec<AutoCaptionRow>,
        labels: Vec<String>,
        #[serde(skip)]
        back: Option<Box<CaptionForm>>,
    },
    Index(IndexForm),
    MarkEntry(MarkForm),
    /// Mark Table of Contents Entry (Alt+Shift+O).
    MarkToc(MarkTocForm),
    Toc(TocForm),
    /// Table of Contents › Options…; `was` comes back on Cancel.
    TocOptions {
        form: TocForm,
        #[serde(skip)]
        was: Box<TocForm>,
    },
    /// Table of Contents › Modify…: the TOC 1–9 styles.
    TocStyles {
        form: TocForm,
        level: u8,
    },
    Sources {
        selected: Option<usize>,
    },
    Source(Box<SourceForm>),
    /// Insert Table of Figures, and its Options… (`was` comes back on Cancel).
    Figures(FiguresForm),
    FiguresOptions {
        form: FiguresForm,
        #[serde(skip)]
        was: Box<FiguresForm>,
    },
    /// Insert Table of Authorities.
    Authorities(AuthoritiesForm),
    /// Mark Citation (Alt+Shift+I), and its Category… (Edit Category; back to Mark Citation).
    MarkCitation(MarkCitationForm),
    EditCategory {
        back: MarkCitationForm,
        selected: usize,
        name: String,
    },
}

impl RefsDialog {
    pub fn name(&self) -> &'static str {
        match self {
            RefsDialog::Caption(_) => "caption",
            RefsDialog::CaptionNumbering { .. } => "captionNumbering",
            RefsDialog::NewLabel(_) => "newLabel",
            RefsDialog::AutoCaption { .. } => "autoCaption",
            RefsDialog::Index(_) => "index",
            RefsDialog::MarkEntry(_) => "markEntry",
            RefsDialog::MarkToc(_) => "markTocEntry",
            RefsDialog::Toc(_) => "toc",
            RefsDialog::TocOptions { .. } => "tocOptions",
            RefsDialog::TocStyles { .. } => "tocStyles",
            RefsDialog::Sources { .. } => "sourceManager",
            RefsDialog::Source(f) if f.edit => "editSource",
            RefsDialog::Source(_) => "createSource",
            RefsDialog::Figures(_) => "tableOfFigures",
            RefsDialog::FiguresOptions { .. } => "tableOfFiguresOptions",
            RefsDialog::Authorities(_) => "tableOfAuthorities",
            RefsDialog::MarkCitation(_) => "markCitation",
            RefsDialog::EditCategory { .. } => "editCategory",
        }
    }

    /// The window title (English; the caller translates it).
    pub fn title(&self) -> &'static str {
        match self {
            RefsDialog::Caption(_) => "Caption",
            RefsDialog::CaptionNumbering { .. } => "Caption Numbering",
            RefsDialog::NewLabel(_) => "New Label",
            RefsDialog::AutoCaption { .. } => "AutoCaption",
            RefsDialog::Index(_) => "Index",
            RefsDialog::MarkEntry(_) => "Mark Index Entry",
            RefsDialog::MarkToc(_) => "Mark Table of Contents Entry",
            RefsDialog::Toc(_) => "Table of Contents",
            RefsDialog::TocOptions { .. } => "Table of Contents Options",
            RefsDialog::TocStyles { .. } => "Table of Contents Styles",
            RefsDialog::Sources { .. } => "Source Manager",
            RefsDialog::Source(f) if f.edit => "Edit Source",
            RefsDialog::Source(_) => "Create Source",
            RefsDialog::Figures(_) => "Table of Figures",
            RefsDialog::FiguresOptions { .. } => "Table of Figures Options",
            RefsDialog::Authorities(_) => "Table of Authorities",
            RefsDialog::MarkCitation(_) => "Mark Citation",
            RefsDialog::EditCategory { .. } => "Edit Category",
        }
    }
}

/// The dialog `ui.dialog` opens by `name`: `caption`, `index`, `markEntry`, `markTocEntry`, `toc`,
/// `sourceManager` or `createSource` (which inserts a citation of the new source).
pub fn open(name: &str, app: &WordApp) -> Option<RefsDialog> {
    Some(match name {
        "caption" => RefsDialog::Caption(CaptionForm::read(app)),
        "index" => RefsDialog::Index(IndexForm::read(app)),
        "markEntry" => RefsDialog::MarkEntry(MarkForm::read(app)),
        "markTocEntry" => RefsDialog::MarkToc(MarkTocForm {
            entry: app.session.selected_text().trim().to_string(),
            identifier: "C".into(),
            level: 1,
            message: String::new(),
        }),
        "toc" => RefsDialog::Toc(TocForm::read(app)),
        "autoCaption" => auto_caption_dialog(app, None),
        "sourceManager" => RefsDialog::Sources { selected: None },
        "createSource" => RefsDialog::Source(Box::new(SourceForm::new(true, false))),
        "tableOfFigures" => RefsDialog::Figures(FiguresForm::default()),
        "tableOfAuthorities" => RefsDialog::Authorities(AuthoritiesForm { category: 0, passim: true, leader: "dot".into() }),
        "markCitation" => RefsDialog::MarkCitation(MarkCitationForm::new(app)),
        _ => return None,
    })
}

/// Draws the dialog; returns true to close it.
pub fn body(app: &mut WordApp, ui: &mut Ui, d: &mut RefsDialog) -> bool {
    let next = match d {
        RefsDialog::Caption(f) => caption_ui(app, ui, f),
        RefsDialog::CaptionNumbering { form, was } => numbering_ui(ui, form, was),
        RefsDialog::NewLabel(f) => new_label_ui(app, ui, f),
        RefsDialog::AutoCaption { rows, labels, back } => auto_caption_ui(app, ui, rows, labels, back),
        RefsDialog::Index(f) => index_ui(app, ui, f),
        RefsDialog::MarkEntry(f) => mark_ui(app, ui, f),
        RefsDialog::MarkToc(f) => mark_toc_ui(app, ui, f),
        RefsDialog::Toc(f) => toc_ui(app, ui, f),
        RefsDialog::TocOptions { form, was } => toc_options_ui(app, ui, form, was),
        RefsDialog::TocStyles { form, level } => toc_styles_ui(app, ui, form, level),
        RefsDialog::Sources { selected } => sources_ui(app, ui, selected),
        RefsDialog::Source(f) => source_ui(app, ui, f),
        RefsDialog::Figures(f) => figures_ui(app, ui, f),
        RefsDialog::FiguresOptions { form, was } => figures_options_ui(app, ui, form, was),
        RefsDialog::Authorities(f) => authorities_ui(app, ui, f),
        RefsDialog::MarkCitation(f) => mark_citation_ui(app, ui, f),
        RefsDialog::EditCategory { back, selected, name } => edit_category_ui(app, ui, back, selected, name),
    };
    match next {
        Next::Stay => false,
        Next::Close => true,
        Next::Go(n) => {
            *d = *n;
            false
        }
    }
}

fn go(d: RefsDialog) -> Next {
    Next::Go(Box::new(d))
}

/// What a dialog does after a frame.
enum Next {
    Stay,
    Close,
    Go(Box<RefsDialog>),
}

fn heading(ui: &mut Ui, s: &str) {
    ui.add_space(4.0);
    ui.label(egui::RichText::new(tl!(s)).font(semibold(12.5)));
}

/// A choice from `options` ((value, label)) in a drop-down.
fn choice<T: PartialEq + Clone>(ui: &mut Ui, id: &str, value: &mut T, options: &[(T, &str)]) {
    let label = options.iter().find(|(v, _)| v == value).map(|(_, l)| tl!(l).to_string()).unwrap_or_default();
    egui::ComboBox::from_id_salt(id).selected_text(label).width(150.0).show_ui(ui, |ui| {
        for (v, l) in options {
            ui.selectable_value(value, v.clone(), tl!(l));
        }
    });
}

const LEADERS: [(&str, &str); 4] = [("none", "(none)"), ("dot", "........"), ("hyphen", "--------"), ("underscore", "________")];

fn leader_char(l: &str) -> Option<char> {
    match l {
        "dot" => Some('.'),
        "hyphen" => Some('-'),
        "underscore" => Some('_'),
        _ => None,
    }
}

/// Sample lines on a white card: (indent level, text, page or empty), with `leader` filling up
/// to right-aligned page numbers (or the page right after the text).
fn sample(ui: &mut Ui, lines: &[(usize, String, String)], right: bool, leader: Option<char>) {
    let t = Tokens::get(ui.ctx());
    let row = 17.0;
    let (rect, _) = ui.allocate_exact_size(vec2(260.0, row * lines.len().max(1) as f32 + 10.0), Sense::hover());
    let p = ui.painter_at(rect);
    p.rect_filled(rect, 2.0, egui::Color32::WHITE);
    p.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, t.border_strong), egui::StrokeKind::Inside);
    let font = egui::FontId::proportional(11.0);
    let ink = egui::Color32::BLACK;
    for (i, (level, text, page)) in lines.iter().enumerate() {
        let y = rect.top() + 5.0 + row * i as f32;
        let x = rect.left() + 8.0 + 12.0 * *level as f32;
        let g = p.layout_no_wrap(text.clone(), font.clone(), ink);
        let w = g.size().x;
        p.galley(egui::pos2(x, y), g, ink);
        if page.is_empty() {
            continue;
        }
        let pg = p.layout_no_wrap(page.clone(), font.clone(), ink);
        let pw = pg.size().x;
        let px = if right { rect.right() - 8.0 - pw } else { x + w + 4.0 };
        p.galley(egui::pos2(px, y), pg, ink);
        if right && let Some(c) = leader {
            let dots = p.layout_no_wrap(c.to_string(), font.clone(), egui::Color32::from_gray(90));
            let step = dots.size().x.max(2.0);
            let mut dx = x + w + 4.0;
            while dx + step < px - 3.0 {
                p.galley(egui::pos2(dx, y), dots.clone(), egui::Color32::from_gray(90));
                dx += step;
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Caption

/// Labels every document has.
const BUILT_IN_LABELS: [&str; 3] = ["Equation", "Figure", "Table"];

const NUMBER_FORMATS: [(&str, &str); 5] =
    [("ARABIC", "1, 2, 3, …"), ("alphabetic", "a, b, c, …"), ("ALPHABETIC", "A, B, C, …"), ("roman", "i, ii, iii, …"), ("ROMAN", "I, II, III, …")];

const SEPARATORS: [(&str, &str); 5] =
    [("-", "- (hyphen)"), (".", ". (period)"), (":", ": (colon)"), ("\u{2014}", "\u{2014} (em dash)"), ("\u{2013}", "\u{2013} (en dash)")];

/// The Caption dialog's fields.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptionForm {
    pub label: String,
    /// Built-in labels and the user's own (kept with the interface settings, like Word's
    /// Normal template keeps them).
    pub labels: Vec<String>,
    pub text: String,
    /// `below` or `above`.
    pub position: String,
    pub exclude_label: bool,
    /// `\*` format: `ARABIC`, `alphabetic`, `ALPHABETIC`, `roman`, `ROMAN`.
    pub format: String,
    pub chapter: bool,
    /// Heading level that starts a chapter (1–9).
    pub chapter_level: u8,
    pub separator: String,
    /// New Label's text.
    pub new_label: String,
}

impl CaptionForm {
    pub fn read(app: &WordApp) -> CaptionForm {
        let mut labels: Vec<String> = BUILT_IN_LABELS.iter().map(|l| l.to_string()).collect();
        for l in &app.ui.caption_labels {
            if !labels.contains(l) {
                labels.push(l.clone());
            }
        }
        // In a table the caption is for the table.
        let in_table = app.session.sel.focus.path.depth() > 0
            && app
                .session
                .doc
                .body
                .get(app.session.sel.focus.path.0.first().copied().unwrap_or(0) as usize)
                .is_some_and(|b| matches!(&**b, Block::Table(_)));
        CaptionForm {
            label: if in_table { "Table" } else { "Figure" }.into(),
            labels,
            text: String::new(),
            position: "below".into(),
            exclude_label: false,
            format: "ARABIC".into(),
            chapter: false,
            chapter_level: 1,
            separator: "-".into(),
            new_label: String::new(),
        }
    }

    /// The number as it will show (`Figure 1-A`).
    pub fn preview(&self) -> String {
        let n = match self.format.as_str() {
            "alphabetic" => "a",
            "ALPHABETIC" => "A",
            "roman" => "i",
            "ROMAN" => "I",
            _ => "1",
        };
        let chapter = if self.chapter { format!("1{}", self.separator) } else { String::new() };
        let label = if self.exclude_label { String::new() } else { format!("{} ", self.label) };
        format!("{label}{chapter}{n}")
    }

    /// `references.caption` parameters.
    pub fn params(&self) -> Value {
        let mut v = json!({
            "label": self.label,
            "text": self.text.trim(),
            "position": self.position,
            "excludeLabel": self.exclude_label,
            "format": self.format,
        });
        if self.chapter {
            v["chapter"] = json!(self.chapter_level);
            v["separator"] = json!(self.separator);
        }
        v
    }
}

fn caption_ui(app: &mut WordApp, ui: &mut Ui, f: &mut CaptionForm) -> Next {
    ui.label(tl!("Caption:"));
    ui.horizontal(|ui| {
        ui.label(f.preview());
        ui.add(crate::widgets::text_line(&mut f.text).desired_width(220.0).hint_text(tl!("Caption text")));
    });
    ui.add_space(6.0);
    let mut then = None;
    egui::Grid::new("cap_opts").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
        ui.label(tl!("Label:"));
        let labels: Vec<(String, &str)> = f.labels.iter().map(|l| (l.clone(), l.as_str())).collect();
        egui::ComboBox::from_id_salt("cap_label").selected_text(tl!(&f.label)).width(150.0).show_ui(ui, |ui| {
            for (v, l) in &labels {
                ui.selectable_value(&mut f.label, v.clone(), tl!(l));
            }
        });
        ui.end_row();
        ui.label(tl!("Position:"));
        choice(ui, "cap_pos", &mut f.position, &[("below".to_string(), "Below the item"), ("above".to_string(), "Above the item")]);
        ui.end_row();
    });
    ui.checkbox(&mut f.exclude_label, tl!("Leave the label out of the caption"));
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        if ui.button(tl!("New Label…")).clicked() {
            let mut n = f.clone();
            n.new_label.clear();
            then = Some(RefsDialog::NewLabel(n));
        }
        let custom = !BUILT_IN_LABELS.contains(&f.label.as_str());
        if ui.add_enabled(custom, egui::Button::new(tl!("Delete Label"))).clicked() {
            app.ui.caption_labels.retain(|l| *l != f.label);
            f.labels.retain(|l| *l != f.label);
            f.label = "Figure".into();
        }
        if ui.button(tl!("Numbering…")).clicked() {
            then = Some(RefsDialog::CaptionNumbering { was: Box::new(f.clone()), form: f.clone() });
        }
        if ui.button(tl!("AutoCaption…")).clicked() {
            then = Some(auto_caption_dialog(app, Some(f.clone())));
        }
    });
    if let Some(g) = then {
        return go(g);
    }
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok && let Err(e) = app.run("references.caption", f.params()) {
        app.status(e);
    }
    if ok || cancel { Next::Close } else { Next::Stay }
}

fn numbering_ui(ui: &mut Ui, f: &mut CaptionForm, was: &CaptionForm) -> Next {
    egui::Grid::new("cap_num").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
        ui.label(tl!("Format:"));
        let opts: Vec<(String, &str)> = NUMBER_FORMATS.iter().map(|(v, l)| (v.to_string(), *l)).collect();
        choice(ui, "cap_fmt", &mut f.format, &opts);
        ui.end_row();
    });
    ui.checkbox(&mut f.chapter, tl!("Add the chapter number"));
    ui.add_enabled_ui(f.chapter, |ui| {
        egui::Grid::new("cap_chap").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
            ui.label(tl!("Chapters begin at:"));
            let opts: Vec<(u8, String)> = (1..=9u8).map(|l| (l, crate::i18n::fmt(tl!("Heading {n}"), &[("n", &l.to_string())]))).collect();
            let shown = opts.iter().find(|(l, _)| *l == f.chapter_level).map(|(_, s)| s.clone()).unwrap_or_default();
            egui::ComboBox::from_id_salt("cap_level").selected_text(shown).width(150.0).show_ui(ui, |ui| {
                for (l, s) in &opts {
                    ui.selectable_value(&mut f.chapter_level, *l, s);
                }
            });
            ui.end_row();
            ui.label(tl!("Separator:"));
            let opts: Vec<(String, &str)> = SEPARATORS.iter().map(|(v, l)| (v.to_string(), *l)).collect();
            choice(ui, "cap_sep", &mut f.separator, &opts);
            ui.end_row();
        });
    });
    ui.add_space(4.0);
    ui.label(crate::i18n::fmt(tl!("Example: {sample}"), &[("sample", &f.preview())]));
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok {
        return go(RefsDialog::Caption(f.clone()));
    }
    if cancel {
        return go(RefsDialog::Caption(was.clone()));
    }
    Next::Stay
}

/// One object AutoCaption can caption.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCaptionRow {
    /// `table` or `picture`.
    pub object: String,
    pub on: bool,
    pub label: String,
    /// `above` or `below`.
    pub position: String,
}

/// AutoCaption as the user last set it (`references.autoCaption`).
fn auto_caption_dialog(app: &WordApp, back: Option<CaptionForm>) -> RefsDialog {
    let set = &app.session.prefs.auto_caption;
    let rows = doc4me_engine::cmd::references::AUTO_CAPTION_OBJECTS
        .iter()
        .map(|o| match set.iter().find(|e| e.object == *o) {
            Some(e) => AutoCaptionRow { object: o.to_string(), on: true, label: e.label.clone(), position: e.position.clone() },
            None => AutoCaptionRow {
                object: o.to_string(),
                on: false,
                label: if *o == "table" { "Table" } else { "Figure" }.into(),
                position: if *o == "table" { "above" } else { "below" }.into(),
            },
        })
        .collect();
    let labels = back.as_ref().map(|f| f.labels.clone()).unwrap_or_else(|| CaptionForm::read(app).labels);
    RefsDialog::AutoCaption { rows, labels, back: back.map(Box::new) }
}

fn auto_caption_ui(app: &mut WordApp, ui: &mut Ui, rows: &mut [AutoCaptionRow], labels: &[String], back: &Option<Box<CaptionForm>>) -> Next {
    ui.label(tl!("Add caption when inserting:"));
    egui::Grid::new("autocap").num_columns(3).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
        for r in rows.iter_mut() {
            ui.checkbox(&mut r.on, tl!(if r.object == "table" { "Table" } else { "Picture" }));
            ui.add_enabled_ui(r.on, |ui| {
                egui::ComboBox::from_id_salt(("autocap_label", r.object.as_str())).selected_text(tl!(&r.label)).width(130.0).show_ui(ui, |ui| {
                    for l in labels {
                        ui.selectable_value(&mut r.label, l.clone(), tl!(l));
                    }
                });
            });
            ui.add_enabled_ui(r.on, |ui| {
                choice(
                    ui,
                    &format!("autocap_pos_{}", r.object),
                    &mut r.position,
                    &[("above".to_string(), "Above the item"), ("below".to_string(), "Below the item")],
                );
            });
            ui.end_row();
        }
    });
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok {
        let set: Vec<Value> = rows.iter().filter(|r| r.on).map(|r| json!({"object": r.object, "label": r.label, "position": r.position})).collect();
        if let Err(e) = app.run("references.autoCaption", json!({"set": set})) {
            app.status(e);
            return Next::Stay;
        }
    }
    if !(ok || cancel) {
        return Next::Stay;
    }
    match back {
        Some(f) => go(RefsDialog::Caption((**f).clone())),
        None => Next::Close,
    }
}

fn new_label_ui(app: &mut WordApp, ui: &mut Ui, f: &mut CaptionForm) -> Next {
    ui.label(tl!("Label:"));
    let r = ui.add(crate::widgets::text_line(&mut f.new_label).desired_width(260.0));
    r.request_focus();
    let (ok, cancel) = buttons(ui, tl!("OK"));
    let name: String = f.new_label.chars().filter(|c| !matches!(c, '"' | '\\') && !c.is_control()).take(40).collect::<String>().trim().to_string();
    if ok && !name.is_empty() {
        if !f.labels.contains(&name) {
            f.labels.push(name.clone());
        }
        if !BUILT_IN_LABELS.contains(&name.as_str()) && !app.ui.caption_labels.contains(&name) {
            app.ui.caption_labels.push(name.clone());
        }
        f.label = name;
    }
    if ok || cancel { go(RefsDialog::Caption(f.clone())) } else { Next::Stay }
}

// ---------------------------------------------------------------------------------------------
// Index and Mark Index Entry

/// The Index dialog's fields.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexForm {
    /// `indented` or `runIn`.
    pub kind: String,
    pub columns: u32,
    pub right_align: bool,
    pub leader: String,
    pub headings: bool,
}

/// The first field in the body whose name is `name`.
fn body_field(app: &WordApp, name: &str) -> Option<FieldCode> {
    app.session.doc.body.iter().filter_map(|b| b.as_para()).flat_map(|p| p.objects.iter()).find_map(|o| match o {
        doc4me_doc::InlineObject::Field { instr, .. } => Some(FieldCode::parse(instr)).filter(|f| f.name == name && (name != "TOC" || !f.has('c'))),
        _ => None,
    })
}

impl IndexForm {
    /// The index in the document, or Word's defaults for a new one (two columns).
    pub fn read(app: &WordApp) -> IndexForm {
        match body_field(app, "INDEX") {
            Some(f) => IndexForm {
                kind: if f.has('r') { "runIn" } else { "indented" }.into(),
                columns: f.arg('c').and_then(|c| c.trim().parse().ok()).unwrap_or(1).clamp(1, 4),
                right_align: f.arg('e').is_some_and(|e| e.contains('\t')),
                leader: "dot".into(),
                headings: f.has('h'),
            },
            None => IndexForm { kind: "indented".into(), columns: 2, right_align: false, leader: "dot".into(), headings: true },
        }
    }

    /// `references.index` parameters.
    pub fn params(&self) -> Value {
        json!({"type": self.kind, "columns": self.columns, "rightAlign": self.right_align, "tabLeader": self.leader, "headings": self.headings})
    }
}

fn index_ui(app: &mut WordApp, ui: &mut Ui, f: &mut IndexForm) -> Next {
    let page = |p: &str| p.to_string();
    let lines: Vec<(usize, String, String)> = {
        let mut v = Vec::new();
        if f.headings {
            v.push((0, "A".to_string(), String::new()));
        }
        if f.kind == "runIn" {
            v.push((0, format!("{}: {}, 4; {}", tl!("Atmosphere"), tl!("Ionosphere"), tl!("Mesosphere")), page("3")));
        } else {
            v.push((0, tl!("Atmosphere").to_string(), page("1")));
            v.push((1, tl!("Ionosphere").to_string(), page("3")));
            v.push((1, tl!("Mesosphere").to_string(), page("4")));
        }
        v
    };
    sample(ui, &lines, f.right_align, leader_char(&f.leader));
    ui.add_space(6.0);
    egui::Grid::new("idx").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
        ui.label(tl!("Type:"));
        ui.horizontal(|ui| {
            ui.radio_value(&mut f.kind, "indented".to_string(), tl!("Indented"));
            ui.radio_value(&mut f.kind, "runIn".to_string(), tl!("Run-in"));
        });
        ui.end_row();
        ui.label(tl!("Columns:"));
        ui.add(egui::DragValue::new(&mut f.columns).range(1..=4).clamp_existing_to_range(false));
        ui.end_row();
        ui.label("");
        ui.checkbox(&mut f.right_align, tl!("Page numbers at the right margin"));
        ui.end_row();
        ui.label(tl!("Tab leader:"));
        ui.add_enabled_ui(f.right_align, |ui| {
            let opts: Vec<(String, &str)> = LEADERS.iter().map(|(v, l)| (v.to_string(), *l)).collect();
            choice(ui, "idx_leader", &mut f.leader, &opts);
        });
        ui.end_row();
        ui.label("");
        ui.checkbox(&mut f.headings, tl!("Letter headings"));
        ui.end_row();
    });
    ui.add_space(4.0);
    if ui.button(tl!("Mark Entry…")).clicked() {
        return go(RefsDialog::MarkEntry(MarkForm::read(app)));
    }
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok && let Err(e) = app.run("references.index", f.params()) {
        app.status(e);
    }
    if ok || cancel { Next::Close } else { Next::Stay }
}

/// The Mark Index Entry dialog's fields.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkForm {
    pub entry: String,
    pub subentry: String,
    /// `page` (current page), `see` (cross-reference) or `range` (page range bookmark).
    pub option: String,
    pub see: String,
    pub bookmark: String,
    pub bookmarks: Vec<String>,
    pub bold: bool,
    pub italic: bool,
    pub message: String,
}

impl MarkForm {
    pub fn read(app: &WordApp) -> MarkForm {
        let bookmarks: Vec<String> = app.session.doc.bookmarks().into_iter().map(|(n, _)| n).filter(|n| !n.starts_with('_')).collect();
        MarkForm {
            entry: app.session.selected_text().trim().to_string(),
            subentry: String::new(),
            option: "page".into(),
            see: format!("{} ", tl!("See")),
            bookmark: bookmarks.first().cloned().unwrap_or_default(),
            bookmarks,
            bold: false,
            italic: false,
            message: String::new(),
        }
    }

    /// `references.markEntry` parameters (`all` for Mark All).
    pub fn params(&self, all: bool) -> Value {
        let mut v = json!({"entry": self.entry.trim(), "subentry": self.subentry.trim(), "bold": self.bold, "italic": self.italic, "all": all});
        match self.option.as_str() {
            "see" => v["crossReference"] = json!(self.see.trim()),
            "range" => v["bookmark"] = json!(self.bookmark),
            _ => {}
        }
        if all {
            v["text"] = json!(self.entry.trim());
        }
        v
    }
}

fn mark_ui(app: &mut WordApp, ui: &mut Ui, f: &mut MarkForm) -> Next {
    heading(ui, "Index");
    egui::Grid::new("xe").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
        ui.label(tl!("Main entry:"));
        ui.add(crate::widgets::text_line(&mut f.entry).desired_width(220.0));
        ui.end_row();
        ui.label(tl!("Subentry:"));
        ui.add(crate::widgets::text_line(&mut f.subentry).desired_width(220.0));
        ui.end_row();
    });
    heading(ui, "Options");
    ui.horizontal(|ui| {
        ui.radio_value(&mut f.option, "see".to_string(), tl!("Cross-reference:"));
        ui.add_enabled(f.option == "see", crate::widgets::text_line(&mut f.see).desired_width(160.0));
    });
    ui.radio_value(&mut f.option, "page".to_string(), tl!("This page"));
    ui.horizontal(|ui| {
        ui.add_enabled_ui(!f.bookmarks.is_empty(), |ui| {
            ui.radio_value(&mut f.option, "range".to_string(), tl!("Pages of bookmark:"));
        });
        ui.add_enabled_ui(f.option == "range", |ui| {
            let opts: Vec<(String, &str)> = f.bookmarks.iter().map(|b| (b.clone(), b.as_str())).collect();
            egui::ComboBox::from_id_salt("xe_bm").selected_text(f.bookmark.clone()).width(140.0).show_ui(ui, |ui| {
                for (v, l) in &opts {
                    ui.selectable_value(&mut f.bookmark, v.clone(), *l);
                }
            });
        });
    });
    heading(ui, "Page number style");
    ui.horizontal(|ui| {
        ui.checkbox(&mut f.bold, tl!("Bold"));
        ui.checkbox(&mut f.italic, tl!("Italic"));
    });
    if !f.message.is_empty() {
        ui.label(egui::RichText::new(&f.message).small());
    }
    let mut close = false;
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Close")).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
            let can = !f.entry.trim().is_empty();
            if ui.add_enabled(can, egui::Button::new(tl!("Mark All"))).clicked() {
                f.message = match app.run("references.markEntry", f.params(true)) {
                    Ok(v) => {
                        crate::i18n::fmt(tl!("{n} entries marked."), &[("n", &v.get("marked").and_then(Value::as_u64).unwrap_or(0).to_string())])
                    }
                    Err(e) => e,
                };
            }
            if ui.add_enabled(can, crate::widgets::primary_button(ui, tl!("Mark"))).clicked() {
                f.message = match app.run("references.markEntry", f.params(false)) {
                    Ok(_) => tl!("Entry marked.").to_string(),
                    Err(e) => e,
                };
            }
        });
    });
    if close { Next::Close } else { Next::Stay }
}

// ---------------------------------------------------------------------------------------------
// Mark Table of Contents Entry

/// The Mark Table of Contents Entry dialog's fields: the entry, the table it belongs to (C, a
/// table of contents) and its level. It stays open for the next entry, as Word's does.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkTocForm {
    pub entry: String,
    pub identifier: String,
    pub level: u8,
    pub message: String,
}

const TABLE_IDS: [&str; 26] =
    ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

fn mark_toc_ui(app: &mut WordApp, ui: &mut Ui, f: &mut MarkTocForm) -> Next {
    egui::Grid::new("tc").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
        ui.label(tl!("Entry:"));
        ui.add(crate::widgets::text_line(&mut f.entry).desired_width(240.0));
        ui.end_row();
        ui.label(tl!("Table identifier:"));
        egui::ComboBox::from_id_salt("tc_id").selected_text(f.identifier.clone()).width(60.0).show_ui(ui, |ui| {
            for id in TABLE_IDS {
                ui.selectable_value(&mut f.identifier, id.to_string(), id);
            }
        });
        ui.end_row();
        ui.label(tl!("Level:"));
        ui.add(egui::DragValue::new(&mut f.level).range(1..=9).clamp_existing_to_range(false));
        ui.end_row();
    });
    if !f.message.is_empty() {
        ui.label(egui::RichText::new(&f.message).small());
    }
    let mut close = false;
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Close")).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
            let can = !f.entry.trim().is_empty();
            if ui.add_enabled(can, crate::widgets::primary_button(ui, tl!("Mark"))).clicked() {
                let params = json!({"entry": f.entry.trim(), "identifier": f.identifier, "level": f.level});
                f.message = match app.run("references.markTocEntry", params) {
                    Ok(_) => tl!("Entry marked.").to_string(),
                    Err(e) => e,
                };
            }
        });
    });
    if close { Next::Close } else { Next::Stay }
}

// ---------------------------------------------------------------------------------------------
// Table of Figures and Table of Authorities

/// Insert Table of Figures: the caption label, or (Options) a style and TC fields with an
/// identifier; label and number, page numbers and their alignment, the tab leader.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FiguresForm {
    pub label: String,
    pub include_label: bool,
    pub page_numbers: bool,
    pub right_align: bool,
    pub leader: String,
    /// Options: build from a style (`style`) and/or TC fields with `identifier`.
    pub use_style: bool,
    pub style: String,
    pub use_tc: bool,
    pub identifier: String,
}

impl Default for FiguresForm {
    fn default() -> Self {
        FiguresForm {
            label: "Figure".into(),
            include_label: true,
            page_numbers: true,
            right_align: true,
            leader: "dot".into(),
            use_style: false,
            style: "Caption".into(),
            use_tc: false,
            identifier: "F".into(),
        }
    }
}

impl FiguresForm {
    /// `references.tableOfFigures` parameters.
    pub fn params(&self) -> Value {
        let mut v = json!({
            "label": self.label,
            "includeLabel": self.include_label,
            "pages": self.page_numbers,
            "rightAlign": self.right_align,
            "leader": self.leader,
        });
        if self.use_style {
            v["style"] = json!(self.style);
        }
        if self.use_tc {
            v["identifier"] = json!(self.identifier);
        }
        v
    }
}

const FIGURE_LABELS: [&str; 3] = ["Figure", "Table", "Equation"];

fn figures_ui(app: &mut WordApp, ui: &mut Ui, f: &mut FiguresForm) -> Next {
    let from_options = f.use_style || f.use_tc;
    ui.add_enabled_ui(!from_options, |ui| {
        ui.horizontal(|ui| {
            ui.label(tl!("Caption label:"));
            egui::ComboBox::from_id_salt("tof_label").selected_text(tl!(f.label.as_str())).width(120.0).show_ui(ui, |ui| {
                for l in FIGURE_LABELS {
                    ui.selectable_value(&mut f.label, l.to_string(), tl!(l));
                }
            });
        });
        ui.checkbox(&mut f.include_label, tl!("Include label and number"));
    });
    if from_options {
        ui.label(egui::RichText::new(tl!("Built from the style or table entry fields chosen in Options.")).small().weak());
    }
    ui.checkbox(&mut f.page_numbers, tl!("Show page numbers"));
    ui.add_enabled_ui(f.page_numbers, |ui| {
        ui.checkbox(&mut f.right_align, tl!("Right align page numbers"));
        ui.horizontal(|ui| {
            ui.label(tl!("Tab leader:"));
            ui.add_enabled_ui(f.right_align, |ui| {
                let opts: Vec<(String, &str)> = LEADERS.iter().map(|(v, l)| (v.to_string(), *l)).collect();
                choice(ui, "tof_leader", &mut f.leader, &opts);
            });
        });
    });
    ui.add_space(4.0);
    if ui.button(tl!("Options…")).clicked() {
        return go(RefsDialog::FiguresOptions { was: Box::new(f.clone()), form: f.clone() });
    }
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok && let Err(e) = app.run("references.tableOfFigures", f.params()) {
        app.status(e);
    }
    if ok || cancel { Next::Close } else { Next::Stay }
}

/// Table of Figures Options: build the table from a style, and/or from TC fields with an
/// identifier (Mark Table of Contents Entry's).
fn figures_options_ui(app: &mut WordApp, ui: &mut Ui, f: &mut FiguresForm, was: &FiguresForm) -> Next {
    heading(ui, "Build table of figures from:");
    ui.horizontal(|ui| {
        ui.checkbox(&mut f.use_style, tl!("Style:"));
        ui.add_enabled_ui(f.use_style, |ui| {
            let mut names: Vec<String> =
                app.session.doc.styles.styles.iter().filter(|s| s.kind == StyleKind::Paragraph && !s.hidden).map(|s| s.name.clone()).collect();
            names.sort_by_key(|n| n.to_lowercase());
            egui::ComboBox::from_id_salt("tof_style").selected_text(f.style.clone()).width(160.0).show_ui(ui, |ui| {
                egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                    for n in names {
                        ui.selectable_value(&mut f.style, n.clone(), n);
                    }
                });
            });
        });
    });
    ui.horizontal(|ui| {
        ui.checkbox(&mut f.use_tc, tl!("Table entry fields"));
        ui.add_enabled_ui(f.use_tc, |ui| {
            ui.label(tl!("Table identifier:"));
            egui::ComboBox::from_id_salt("tof_id").selected_text(f.identifier.clone()).width(60.0).show_ui(ui, |ui| {
                for id in TABLE_IDS {
                    ui.selectable_value(&mut f.identifier, id.to_string(), id);
                }
            });
        });
    });
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok {
        return go(RefsDialog::Figures(f.clone()));
    }
    if cancel {
        return go(RefsDialog::Figures(was.clone()));
    }
    Next::Stay
}

/// Insert Table of Authorities: a category (0 = All), passim, the tab leader.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoritiesForm {
    pub category: u8,
    pub passim: bool,
    pub leader: String,
}

/// The category names, from the engine (the user's, or Word's in the interface language).
fn categories(app: &mut WordApp) -> Vec<String> {
    app.session
        .run("references.toaCategory", &json!({}))
        .ok()
        .and_then(|v| {
            v.get("categories").and_then(Value::as_array).map(|a| a.iter().filter_map(|n| n.as_str().map(|n| tl!(n).to_string())).collect())
        })
        .unwrap_or_default()
}

fn category_combo(ui: &mut Ui, id: &str, value: &mut u8, names: &[String], all: bool) {
    let shown = if *value == 0 { tl!("All").to_string() } else { names.get(*value as usize - 1).cloned().unwrap_or_default() };
    egui::ComboBox::from_id_salt(id).selected_text(shown).width(200.0).show_ui(ui, |ui| {
        if all {
            ui.selectable_value(value, 0, tl!("All"));
        }
        for (i, n) in names.iter().enumerate() {
            ui.selectable_value(value, i as u8 + 1, n.as_str());
        }
    });
}

fn authorities_ui(app: &mut WordApp, ui: &mut Ui, f: &mut AuthoritiesForm) -> Next {
    let names = categories(app);
    ui.horizontal(|ui| {
        ui.label(tl!("Category:"));
        category_combo(ui, "toa_category", &mut f.category, &names, true);
    });
    ui.checkbox(&mut f.passim, tl!("Use passim (five or more pages)"));
    ui.horizontal(|ui| {
        ui.label(tl!("Tab leader:"));
        let opts: Vec<(String, &str)> = LEADERS.iter().map(|(v, l)| (v.to_string(), *l)).collect();
        choice(ui, "toa_leader", &mut f.leader, &opts);
    });
    ui.add_space(4.0);
    if ui.button(tl!("Mark Citation…")).clicked() {
        return go(RefsDialog::MarkCitation(MarkCitationForm::new(app)));
    }
    let (ok, cancel) = buttons(ui, tl!("OK"));
    // The headings in the interface language, for the categories the user hasn't renamed.
    let local: Vec<&str> = doc4me_engine::cmd::citations::TOA_CATEGORIES.iter().map(|n| tl!(n)).collect();
    if ok
        && let Err(e) =
            app.run("references.tableOfAuthorities", json!({"category": f.category, "passim": f.passim, "leader": f.leader, "names": local}))
    {
        app.status(e);
    }
    if ok || cancel { Next::Close } else { Next::Stay }
}

/// Mark Citation: the long citation (the selection), the short one, the category.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkCitationForm {
    pub long: String,
    pub short: String,
    pub category: u8,
    pub message: String,
}

impl MarkCitationForm {
    fn new(app: &WordApp) -> MarkCitationForm {
        let t = app.session.selected_text().trim().replace(['\n', '\r'], " ");
        MarkCitationForm { short: t.clone(), long: t, category: 1, message: String::new() }
    }
}

fn mark_citation_ui(app: &mut WordApp, ui: &mut Ui, f: &mut MarkCitationForm) -> Next {
    let names = categories(app);
    egui::Grid::new("ta").num_columns(2).spacing(vec2(10.0, 6.0)).show(ui, |ui| {
        ui.label(tl!("Long citation:"));
        ui.add(crate::widgets::text_line(&mut f.long).desired_width(260.0));
        ui.end_row();
        ui.label(tl!("Category:"));
        ui.horizontal(|ui| {
            category_combo(ui, "ta_category", &mut f.category, &names, false);
        });
        ui.end_row();
        ui.label(tl!("Short citation:"));
        ui.add(crate::widgets::text_line(&mut f.short).desired_width(260.0));
        ui.end_row();
    });
    if ui.button(tl!("Category…")).clicked() {
        let selected = f.category.max(1) as usize - 1;
        let name = names.get(selected).cloned().unwrap_or_default();
        return go(RefsDialog::EditCategory { back: f.clone(), selected, name });
    }
    if !f.message.is_empty() {
        ui.label(egui::RichText::new(&f.message).small());
    }
    let mut close = false;
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Close")).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                close = true;
            }
            if ui.add_enabled(!f.long.trim().is_empty(), crate::widgets::primary_button(ui, tl!("Mark"))).clicked() {
                let params = json!({"entry": f.long.trim(), "short": f.short.trim(), "category": f.category.max(1)});
                f.message = match app.run("references.markCitation", params) {
                    Ok(_) => tl!("Citation marked.").to_string(),
                    Err(e) => e,
                };
            }
        });
    });
    if close { Next::Close } else { Next::Stay }
}

/// Edit Category: rename a category (Replace); OK goes back to Mark Citation.
fn edit_category_ui(app: &mut WordApp, ui: &mut Ui, back: &mut MarkCitationForm, selected: &mut usize, name: &mut String) -> Next {
    let names = categories(app);
    ui.label(tl!("Category:"));
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(240.0);
        egui::ScrollArea::vertical().id_salt("toa_categories").max_height(200.0).show(ui, |ui| {
            for (i, n) in names.iter().enumerate() {
                if ui.selectable_label(*selected == i, format!("{}  {n}", i + 1)).clicked() {
                    *selected = i;
                    *name = n.clone();
                }
            }
        });
    });
    ui.horizontal(|ui| {
        ui.label(tl!("Replace with:"));
        ui.add(crate::widgets::text_line(name).desired_width(170.0).char_limit(40));
        if ui.button(tl!("Replace")).clicked()
            && let Err(e) = app.run("references.toaCategory", json!({"category": *selected + 1, "name": name.trim()}))
        {
            app.status(e);
        }
    });
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok || cancel {
        back.message.clear();
        return go(RefsDialog::MarkCitation(back.clone()));
    }
    Next::Stay
}

// ---------------------------------------------------------------------------------------------
// Table of Contents

/// The Custom Table of Contents dialog's fields (with its Options).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TocForm {
    pub page_numbers: bool,
    pub right_align: bool,
    pub leader: String,
    pub hyperlinks: bool,
    pub levels: u8,
    /// Options: heading styles (`\o`), outline levels (`\u`), TC fields (`\f`).
    pub heading_styles: bool,
    pub outline_levels: bool,
    pub tc_fields: bool,
    /// Options: other paragraph styles with a TOC level (0 = not in the table), by style name.
    pub style_levels: Vec<(String, u8)>,
}

impl TocForm {
    /// The table in the document, or the defaults for a new one.
    pub fn read(app: &WordApp) -> TocForm {
        let f = body_field(app, "TOC").unwrap_or_else(|| FieldCode::parse(r#"TOC \o "1-3" \h \z \u"#));
        let levels = f.arg('o').and_then(doc4me_engine::cmd::references::level_range).map_or(3, |r| r.1);
        let listed: Vec<(String, u8)> = f
            .arg('t')
            .map(|t| {
                let parts: Vec<&str> = t.split(',').map(str::trim).collect();
                parts.chunks(2).filter_map(|c| Some((c.first()?.to_string(), c.get(1)?.parse::<u8>().ok()?.clamp(1, 9)))).collect()
            })
            .unwrap_or_default();
        let style_levels = app
            .session
            .doc
            .styles
            .styles
            .iter()
            .filter(|s| s.kind == StyleKind::Paragraph && !s.hidden && !s.id.starts_with("Heading") && !s.id.starts_with("TOC"))
            .map(|s| (s.name.clone(), listed.iter().find(|(n, _)| n.eq_ignore_ascii_case(&s.name)).map_or(0, |(_, l)| *l)))
            .collect();
        // Right-aligned page numbers: no `\p` separator, or a tab one.
        let leader = app
            .session
            .doc
            .body
            .iter()
            .filter_map(|b| b.as_para())
            .find(|p| p.props.style.as_deref().is_some_and(|s| s.starts_with("TOC") && s != "TOCHeading"))
            .and_then(|p| p.props.tabs.as_ref()?.last().map(|t| t.leader));
        TocForm {
            page_numbers: !f.has('n'),
            right_align: f.arg('p').is_none_or(|p| p.contains('\t')),
            leader: match leader {
                Some(doc4me_doc::props::TabLeader::None) => "none",
                Some(doc4me_doc::props::TabLeader::Hyphen) => "hyphen",
                Some(doc4me_doc::props::TabLeader::Underscore) => "underscore",
                _ => "dot",
            }
            .into(),
            hyperlinks: f.has('h'),
            levels: levels.clamp(1, 9),
            heading_styles: f.has('o'),
            outline_levels: f.has('u'),
            tc_fields: f.has('f'),
            style_levels,
        }
    }

    /// `references.toc` parameters.
    pub fn params(&self) -> Value {
        let styles: serde_json::Map<String, Value> = self.style_levels.iter().filter(|(_, l)| *l > 0).map(|(n, l)| (n.clone(), json!(l))).collect();
        json!({
            "levels": self.levels,
            "pageNumbers": self.page_numbers,
            "rightAlign": self.right_align,
            "tabLeader": self.leader,
            "hyperlinks": self.hyperlinks,
            "headingStyles": self.heading_styles,
            "outlineLevels": self.outline_levels,
            "tcFields": self.tc_fields,
            "styleLevels": styles,
        })
    }
}

fn toc_ui(app: &mut WordApp, ui: &mut Ui, f: &mut TocForm) -> Next {
    let lines: Vec<(usize, String, String)> = (1..=f.levels.min(3))
        .map(|l| {
            let name = crate::i18n::fmt(tl!("Heading {n}"), &[("n", &l.to_string())]);
            (l as usize - 1, name, if f.page_numbers { (l * 2 - 1).to_string() } else { String::new() })
        })
        .collect();
    sample(ui, &lines, f.right_align, leader_char(&f.leader));
    ui.add_space(6.0);
    ui.checkbox(&mut f.page_numbers, tl!("Include page numbers"));
    ui.add_enabled_ui(f.page_numbers, |ui| {
        ui.checkbox(&mut f.right_align, tl!("Page numbers at the right margin"));
        ui.horizontal(|ui| {
            ui.label(tl!("Tab leader:"));
            ui.add_enabled_ui(f.right_align, |ui| {
                let opts: Vec<(String, &str)> = LEADERS.iter().map(|(v, l)| (v.to_string(), *l)).collect();
                choice(ui, "toc_leader", &mut f.leader, &opts);
            });
        });
    });
    ui.checkbox(&mut f.hyperlinks, tl!("Link entries to their headings"));
    ui.horizontal(|ui| {
        ui.label(tl!("Levels:"));
        ui.add(egui::DragValue::new(&mut f.levels).range(1..=9).clamp_existing_to_range(false));
    });
    ui.add_space(4.0);
    let mut then = None;
    ui.horizontal(|ui| {
        if ui.button(tl!("Options…")).clicked() {
            then = Some(RefsDialog::TocOptions { was: Box::new(f.clone()), form: f.clone() });
        }
        if ui.button(tl!("Modify…")).clicked() {
            then = Some(RefsDialog::TocStyles { form: f.clone(), level: 1 });
        }
    });
    if let Some(g) = then {
        return go(g);
    }
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok && let Err(e) = app.run("references.toc", f.params()) {
        app.status(e);
    }
    if ok || cancel { Next::Close } else { Next::Stay }
}

fn toc_options_ui(_app: &mut WordApp, ui: &mut Ui, f: &mut TocForm, was: &TocForm) -> Next {
    heading(ui, "Collect entries from:");
    ui.checkbox(&mut f.heading_styles, crate::i18n::fmt(tl!("Heading styles (levels 1–{n})"), &[("n", &f.levels.to_string())]));
    ui.label(tl!("Other styles and their TOC level:"));
    egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
        egui::Grid::new("toc_styles").num_columns(2).spacing(vec2(10.0, 3.0)).show(ui, |ui| {
            for (name, level) in f.style_levels.iter_mut() {
                ui.label(name.as_str());
                ui.add(
                    egui::DragValue::new(level)
                        .range(0..=9)
                        .clamp_existing_to_range(false)
                        .custom_formatter(|v, _| if v < 0.5 { String::new() } else { format!("{v:.0}") }),
                );
                ui.end_row();
            }
        });
    });
    ui.checkbox(&mut f.outline_levels, tl!("Outline levels"));
    ui.checkbox(&mut f.tc_fields, tl!("TC entry fields"));
    ui.add_space(4.0);
    if ui.button(tl!("Reset")).clicked() {
        f.heading_styles = true;
        f.outline_levels = true;
        f.tc_fields = false;
        for (_, l) in f.style_levels.iter_mut() {
            *l = 0;
        }
    }
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok {
        return go(RefsDialog::Toc(f.clone()));
    }
    if cancel {
        return go(RefsDialog::Toc(was.clone()));
    }
    Next::Stay
}

/// Modify… in Table of Contents: pick a TOC level style and edit it in Modify Style, which
/// comes back here when it closes.
fn toc_styles_ui(app: &mut WordApp, ui: &mut Ui, f: &mut TocForm, level: &mut u8) -> Next {
    ui.label(tl!("Styles:"));
    egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
        for l in 1..=9u8 {
            let name = app.session.doc.styles.get(&format!("TOC{l}")).map(|s| s.name.clone()).unwrap_or_else(|| format!("TOC {l}"));
            ui.selectable_value(level, l, name);
        }
    });
    ui.add_space(6.0);
    if ui.button(tl!("Modify…")).clicked()
        && let Some(mut d) = Dialog::modify_style(app, &format!("TOC{level}"))
    {
        if let Dialog::ModifyStyle { resume, .. } = &mut d {
            *resume = Some(Box::new(Dialog::Refs(Box::new(RefsDialog::TocStyles { form: f.clone(), level: *level }))));
        }
        app.dialog = Some(d);
        return Next::Close;
    }
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok || cancel { go(RefsDialog::Toc(f.clone())) } else { Next::Stay }
}

// ---------------------------------------------------------------------------------------------
// Sources

/// Source types with their fields: (kind, label, recommended fields). A field is (key, label).
const SOURCE_TYPES: [(&str, &str, &[(&str, &str)]); 9] = [
    ("book", "Book", &[("author", "Author"), ("title", "Title"), ("year", "Year"), ("city", "City"), ("publisher", "Publisher")]),
    (
        "bookSection",
        "Chapter in a book",
        &[
            ("author", "Author"),
            ("title", "Title"),
            ("journal", "Book title"),
            ("year", "Year"),
            ("pages", "Pages"),
            ("city", "City"),
            ("publisher", "Publisher"),
        ],
    ),
    (
        "article",
        "Journal article",
        &[("author", "Author"), ("title", "Title"), ("journal", "Journal"), ("year", "Year"), ("volume", "Volume"), ("pages", "Pages")],
    ),
    (
        "periodical",
        "Magazine or newspaper article",
        &[
            ("author", "Author"),
            ("title", "Title"),
            ("journal", "Periodical"),
            ("year", "Year"),
            ("month", "Month"),
            ("day", "Day"),
            ("pages", "Pages"),
        ],
    ),
    (
        "conference",
        "Conference paper",
        &[
            ("author", "Author"),
            ("title", "Title"),
            ("journal", "Conference"),
            ("year", "Year"),
            ("pages", "Pages"),
            ("city", "City"),
            ("publisher", "Publisher"),
        ],
    ),
    ("report", "Report", &[("author", "Author"), ("title", "Title"), ("year", "Year"), ("publisher", "Institution"), ("city", "City")]),
    (
        "website",
        "Website",
        &[
            ("author", "Author"),
            ("title", "Page title"),
            ("journal", "Site name"),
            ("year", "Year"),
            ("month", "Month"),
            ("day", "Day"),
            ("url", "URL"),
        ],
    ),
    ("film", "Film", &[("author", "Author"), ("title", "Title"), ("publisher", "Studio"), ("year", "Year")]),
    ("other", "Other", &[("author", "Author"), ("title", "Title"), ("year", "Year"), ("city", "City"), ("publisher", "Publisher")]),
];

/// Every field "show all bibliography fields" adds, after the type's own.
const ALL_FIELDS: [(&str, &str); 17] = [
    ("author", "Author"),
    ("editor", "Editor"),
    ("title", "Title"),
    ("journal", "Container Title"),
    ("year", "Year"),
    ("month", "Month"),
    ("day", "Day"),
    ("city", "City"),
    ("publisher", "Publisher"),
    ("edition", "Edition"),
    ("volume", "Volume"),
    ("issue", "Issue"),
    ("pages", "Pages"),
    ("url", "URL"),
    ("doi", "DOI"),
    ("accessed", "Accessed on"),
    ("tag", "Tag"),
];

fn field_mut<'a>(s: &'a mut Source, key: &str) -> Option<&'a mut String> {
    Some(match key {
        "author" => &mut s.author,
        "editor" => &mut s.editor,
        "title" => &mut s.title,
        "journal" => &mut s.journal,
        "year" => &mut s.year,
        "month" => &mut s.month,
        "day" => &mut s.day,
        "city" => &mut s.city,
        "publisher" => &mut s.publisher,
        "edition" => &mut s.edition,
        "volume" => &mut s.volume,
        "issue" => &mut s.issue,
        "pages" => &mut s.pages,
        "url" => &mut s.url,
        "doi" => &mut s.doi,
        "accessed" => &mut s.accessed,
        "tag" => &mut s.tag,
        _ => return None,
    })
}

/// Create Source / Edit Source.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceForm {
    pub source: Source,
    pub show_all: bool,
    /// Editing a source that is there (its tag stays: citations point at it).
    pub edit: bool,
    /// Insert a citation of the source on OK (Insert Citation › Add New Source…).
    pub cite: bool,
    /// Opened from the Source Manager, which comes back when this closes.
    pub back: bool,
}

impl SourceForm {
    pub fn new(cite: bool, back: bool) -> SourceForm {
        SourceForm { source: Source { kind: "book".into(), ..Default::default() }, show_all: false, edit: false, cite, back }
    }

    /// The fields shown: the type's own, or every field.
    pub fn fields(&self) -> Vec<(&'static str, &'static str)> {
        let own = SOURCE_TYPES.iter().find(|(k, ..)| *k == self.source.kind).map(|(_, _, f)| *f).unwrap_or(SOURCE_TYPES[0].2);
        let mut v: Vec<(&str, &str)> = own.to_vec();
        if self.show_all {
            v.extend(ALL_FIELDS.iter().filter(|(k, _)| !own.iter().any(|(o, _)| o == k) && *k != "tag").copied());
        }
        v.push(("tag", "Tag"));
        v
    }
}

fn source_ui(app: &mut WordApp, ui: &mut Ui, f: &mut SourceForm) -> Next {
    egui::Grid::new("src").num_columns(2).spacing(vec2(10.0, 5.0)).show(ui, |ui| {
        ui.label(tl!("Source type:"));
        let shown = SOURCE_TYPES.iter().find(|(k, ..)| *k == f.source.kind).map(|(_, l, _)| tl!(l).to_string()).unwrap_or_default();
        egui::ComboBox::from_id_salt("src_kind").selected_text(shown).width(200.0).show_ui(ui, |ui| {
            for (k, l, _) in SOURCE_TYPES {
                ui.selectable_value(&mut f.source.kind, k.to_string(), tl!(l));
            }
        });
        ui.end_row();
        let edit = f.edit;
        for (key, label) in f.fields() {
            // The person who edited a book, not the spelling Editor.
            let name = if key == "editor" { crate::i18n::tc("bibliography", label) } else { tl!(label) };
            ui.label(format!("{name}:"));
            if let Some(v) = field_mut(&mut f.source, key) {
                let hint = match key {
                    "author" => tl!("Last, First; Last, First"),
                    "tag" => tl!("Made from the author and year when empty"),
                    _ => "",
                };
                ui.add_enabled(!(edit && key == "tag"), crate::widgets::text_line(v).desired_width(260.0).hint_text(hint));
            }
            ui.end_row();
        }
    });
    ui.checkbox(&mut f.show_all, tl!("Show every bibliography field"));
    let (ok, cancel) = buttons(ui, tl!("OK"));
    if ok {
        let src = serde_json::to_value(&f.source).unwrap_or(Value::Null);
        let r = if f.cite { app.run("references.citation", json!({"source": src})) } else { app.run("references.sources", json!({"add": src})) };
        if let Err(e) = r {
            app.status(e);
        }
    }
    if (ok || cancel) && f.back {
        return go(RefsDialog::Sources { selected: None });
    }
    if ok || cancel { Next::Close } else { Next::Stay }
}

fn sources_ui(app: &mut WordApp, ui: &mut Ui, selected: &mut Option<usize>) -> Next {
    let t = Tokens::get(ui.ctx());
    let list: Vec<String> = app
        .session
        .doc
        .sources
        .iter()
        .map(|s| {
            let parts: Vec<&str> = [s.author.as_str(), s.title.as_str(), s.year.as_str()].into_iter().filter(|x| !x.is_empty()).collect();
            format!("{} — {}", s.tag, parts.join("; "))
        })
        .collect();
    ui.label(tl!("Sources in this document:"));
    egui::Frame::new().stroke(egui::Stroke::new(1.0, t.border_strong)).inner_margin(4.0).show(ui, |ui| {
        ui.set_min_size(vec2(380.0, 160.0));
        egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
            if list.is_empty() {
                ui.label(egui::RichText::new(tl!("No sources yet.")).color(t.text_dim));
            }
            for (i, l) in list.iter().enumerate() {
                if ui.selectable_label(*selected == Some(i), l).clicked() {
                    *selected = Some(i);
                }
            }
        });
    });
    let src = selected.and_then(|i| app.session.doc.sources.get(i)).cloned();
    let mut next = Next::Stay;
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        if ui.button(tl!("New…")).clicked() {
            next = go(RefsDialog::Source(Box::new(SourceForm::new(false, true))));
        }
        if ui.add_enabled(src.is_some(), egui::Button::new(tl!("Edit…"))).clicked()
            && let Some(s) = &src
        {
            next = go(RefsDialog::Source(Box::new(SourceForm { source: s.clone(), show_all: false, edit: true, cite: false, back: true })));
        }
        if ui.add_enabled(src.is_some(), egui::Button::new(tl!("Delete"))).clicked()
            && let Some(s) = &src
        {
            if let Err(e) = app.run("references.sources", json!({"remove": s.tag})) {
                app.status(e);
            }
            *selected = None;
        }
        if ui.add_enabled(src.is_some(), egui::Button::new(tl!("Insert Citation"))).clicked()
            && let Some(s) = &src
        {
            if let Err(e) = app.run("references.citation", json!({"tag": s.tag})) {
                app.status(e);
            }
            next = Next::Close;
        }
    });
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Close")).clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                next = Next::Close;
            }
        });
    });
    next
}

#[cfg(test)]
mod tests {
    use doc4me_doc::{InlineObject, StoryRef};
    use doc4me_engine::Session;
    use serde_json::json;

    use super::*;
    use crate::Services;

    fn app(text: &str) -> WordApp {
        WordApp::new(Session::new(doc4me_doc::Document::from_text(text)), Services::default())
    }

    fn form(a: &WordApp) -> RefsDialog {
        match &a.dialog {
            Some(Dialog::Refs(d)) => (**d).clone(),
            d => panic!("no references dialog: {d:?}"),
        }
    }

    fn instrs(a: &WordApp) -> Vec<String> {
        let doc = &a.session.doc;
        doc.para_paths(StoryRef::Body)
            .iter()
            .filter_map(|p| doc.para(StoryRef::Body, p))
            .flat_map(|p| p.objects.iter())
            .filter_map(|o| match o {
                InlineObject::Field { instr, .. } => Some(instr.clone()),
                _ => None,
            })
            .collect()
    }

    /// #402: Insert Caption opens the dialog; a new label is kept with the interface settings,
    /// and Numbering's chapter options become `STYLEREF` and `SEQ … \s` fields.
    #[test]
    fn caption_dialog_numbers_with_chapters() {
        let mut a = app("Results\nA chart");
        a.run("select.text", json!({"text": "Results"})).unwrap();
        a.run("styles.apply", json!({"style": "Heading 1"})).unwrap();
        a.run("select.text", json!({"text": "A chart"})).unwrap();
        assert_eq!(a.run("references.caption", json!({})).unwrap(), json!({"pending": "caption"}));
        let RefsDialog::Caption(mut f) = form(&a) else { panic!() };
        assert_eq!((f.label.as_str(), f.labels.len()), ("Figure", 3));
        // New Label…, then Numbering… with a chapter number.
        f.new_label = "Plate".into();
        a.dialog = None;
        a.ui.caption_labels.push(f.new_label.clone());
        f.label = "Plate".into();
        f.format = "ALPHABETIC".into();
        f.chapter = true;
        f.separator = ".".into();
        f.text = "Growth".into();
        assert_eq!(f.preview(), "Plate 1.A");
        a.run("references.caption", f.params()).unwrap();
        assert!(instrs(&a).contains(&"SEQ Plate \\* ALPHABETIC \\s 1".to_string()), "{:?}", instrs(&a));
        assert!(a.session.doc.plain_text(StoryRef::Body).contains("Plate 1.A: Growth"));
        // The label is offered next time and saved with the preferences.
        assert!(CaptionForm::read(&a).labels.contains(&"Plate".to_string()));
        assert_eq!(a.prefs().caption_labels, vec!["Plate".to_string()]);
    }

    /// Caption › AutoCaption… opens with what the preferences hold; OK sets which inserted
    /// objects get a caption (kept with the preferences) and goes back to Caption.
    #[test]
    fn auto_caption_dialog_sets_the_preference() {
        use egui_kittest::kittest::Queryable;
        let mut a = app("Text");
        a.run("references.autoCaption", json!({})).unwrap();
        let RefsDialog::AutoCaption { rows, labels, back } = form(&a) else { panic!() };
        assert!(back.is_none() && rows.iter().all(|r| !r.on) && labels.contains(&"Figure".to_string()));
        a.run("references.caption", json!({})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(3);
        h.get_by_label("AutoCaption…").click();
        h.run_steps(3);
        if let Some(Dialog::Refs(d)) = &mut h.state_mut().dialog
            && let RefsDialog::AutoCaption { rows, .. } = &mut **d
        {
            rows[1].on = true;
            rows[1].label = "Table".into();
        } else {
            panic!("no AutoCaption: {:?}", h.state().dialog);
        }
        h.run_steps(2);
        h.get_by_label("OK").click();
        h.run_steps(3);
        let a = h.state();
        assert!(matches!(&a.dialog, Some(Dialog::Refs(d)) if matches!(**d, RefsDialog::Caption(_))), "back to Caption: {:?}", a.dialog);
        let want = doc4me_engine::cmd::references::AutoCaption { object: "picture".into(), label: "Table".into(), position: "below".into() };
        assert_eq!(a.prefs().editing.auto_caption, vec![want]);
    }

    /// #402: Mark Entry opens with the selection; Insert Index opens with Word's defaults and
    /// reads the index back; both write their switches.
    #[test]
    fn index_and_mark_entry_dialogs_write_switches() {
        let mut a = app("Kilns are hot.\nMore text.");
        a.run("select.text", json!({"text": "Kilns"})).unwrap();
        assert_eq!(a.run("references.markEntry", json!({})).unwrap(), json!({"pending": "markEntry"}));
        let RefsDialog::MarkEntry(mut m) = form(&a) else { panic!() };
        assert_eq!(m.entry, "Kilns");
        m.subentry = "firing".into();
        m.italic = true;
        a.run("references.markEntry", m.params(false)).unwrap();
        assert!(instrs(&a).contains(&"XE \"Kilns:firing\" \\i".to_string()), "{:?}", instrs(&a));
        a.run("caret.docEnd", json!({})).unwrap();
        assert_eq!(a.run("references.index", json!({})).unwrap(), json!({"pending": "index"}));
        let RefsDialog::Index(mut f) = form(&a) else { panic!() };
        assert_eq!((f.columns, f.kind.as_str(), f.headings), (2, "indented", true));
        f.kind = "runIn".into();
        f.right_align = true;
        a.run("references.index", f.params()).unwrap();
        assert!(instrs(&a).contains(&"INDEX \\h \"A\" \\c \"2\" \\e \"\t\" \\r".to_string()), "{:?}", instrs(&a));
        assert_eq!(IndexForm::read(&a), f, "the dialog shows the index's settings");
    }

    /// #402: Custom Table of Contents and its Options make the TOC switches, and the dialog
    /// opens again with them.
    #[test]
    fn custom_table_of_contents_dialog() {
        let mut a = app("Intro\nNote");
        a.run("select.text", json!({"text": "Intro"})).unwrap();
        a.run("styles.apply", json!({"style": "Heading 1"})).unwrap();
        a.run("select.text", json!({"text": "Note"})).unwrap();
        a.run("styles.apply", json!({"style": "Quote"})).unwrap();
        a.run("caret.docStart", json!({})).unwrap();
        a.run("ui.dialog", json!({"name": "toc"})).unwrap();
        let RefsDialog::Toc(mut f) = form(&a) else { panic!() };
        assert_eq!((f.levels, f.hyperlinks, f.right_align, f.heading_styles, f.outline_levels), (3, true, true, true, true));
        f.levels = 2;
        f.hyperlinks = false;
        f.leader = "hyphen".into();
        // Options…: the Quote style at level 2.
        f.style_levels.iter_mut().find(|(n, _)| n == "Quote").unwrap().1 = 2;
        a.run("references.toc", f.params()).unwrap();
        assert_eq!(instrs(&a)[0], "TOC \\o \"1-2\" \\z \\u \\t \"Quote,2\"");
        let text = a.session.doc.plain_text(StoryRef::Body);
        assert!(text.contains("Intro\t1") && text.contains("Note\t1"), "{text}");
        assert_eq!(TocForm::read(&a), f);
    }

    /// #402: Insert Citation without a source opens Create Source; the type picks the fields,
    /// "show all" adds the rest, and Edit Source from the Source Manager updates the citation.
    #[test]
    fn create_and_edit_source() {
        let mut a = app("Studios matter ");
        a.run("caret.docEnd", json!({})).unwrap();
        assert_eq!(a.run("references.citation", json!({})).unwrap(), json!({"pending": "createSource"}));
        let RefsDialog::Source(mut f) = form(&a) else { panic!() };
        let f = &mut *f;
        f.source.kind = "website".into();
        let shown = f.fields().len();
        assert!(f.fields().iter().any(|(k, l)| *k == "url" && *l == "URL"));
        f.show_all = true;
        assert!(f.fields().len() > shown && f.fields().iter().any(|(k, _)| *k == "doi"));
        for (k, v) in [("author", "Rivera, Alex"), ("title", "Kilns Online"), ("year", "2024"), ("url", "https://example.org"), ("doi", "10.1/x")] {
            *field_mut(&mut f.source, k).unwrap() = v.into();
        }
        a.run("references.citation", json!({"source": serde_json::to_value(&f.source).unwrap()})).unwrap();
        assert!(a.session.doc.plain_text(StoryRef::Body).contains("(Rivera, 2024)"));
        assert_eq!(a.session.doc.sources[0].doi, "10.1/x");
        // Manage Sources › Edit…: a new year shows in the citation.
        assert_eq!(a.run("references.sources", json!({})).unwrap(), json!({"pending": "sourceManager"}));
        let mut src = a.session.doc.sources[0].clone();
        src.year = "2025".into();
        a.run("references.sources", json!({"add": src})).unwrap();
        assert!(a.session.doc.plain_text(StoryRef::Body).contains("(Rivera, 2025)"), "{}", a.session.doc.plain_text(StoryRef::Body));
        assert_eq!(a.session.doc.sources.len(), 1);
    }

    /// Mark Table of Contents Entry opens with the selection (Alt+Shift+O without an entry);
    /// Custom Table of Contents and Type a New List are commands that open their dialogs.
    #[test]
    fn mark_toc_entry_and_custom_toc_open_from_commands() {
        let mut a = app("Kilns\nPresses");
        a.run("select.text", json!({"text": "Presses"})).unwrap();
        assert_eq!(a.run("references.markTocEntry", json!({})).unwrap(), json!({"pending": "markTocEntry"}));
        let RefsDialog::MarkToc(f) = form(&a) else { panic!() };
        assert_eq!((f.entry.as_str(), f.identifier.as_str(), f.level), ("Presses", "C", 1));
        a.run("references.markTocEntry", json!({"entry": f.entry, "identifier": f.identifier, "level": 3})).unwrap();
        assert!(instrs(&a).contains(&"TC \"Presses\" \\f C \\l \"3\"".to_string()), "{:?}", instrs(&a));
        a.run("references.customToc", json!({})).unwrap();
        assert!(matches!(form(&a), RefsDialog::Toc(_)));
        a.run("mailings.newList", json!({})).unwrap();
        assert_eq!(a.dialog.as_ref().map(|d| d.name()), Some("recipientList"));
    }

    /// dlg-165 and dlg-029: Insert Table of Figures › Options… builds from a style; Mark Citation
    /// › Category… renames a category, the citation is filed under it, and Insert Table of
    /// Authorities lists it under the new name.
    #[test]
    fn figures_options_and_authority_categories() {
        use egui_kittest::kittest::Queryable;
        let mut a = app("Quoted words\nSmith v. Jones held it.\nEnd");
        a.run("select.text", json!({"text": "Quoted words"})).unwrap();
        a.run("styles.apply", json!({"style": "Quote"})).unwrap();
        a.run("select.text", json!({"text": "Smith v. Jones"})).unwrap();
        assert_eq!(a.run("references.markCitation", json!({})).unwrap(), json!({"pending": "markCitation"}));
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(3);
        assert!(matches!(form(h.state()), RefsDialog::MarkCitation(f) if f.long == "Smith v. Jones" && f.category == 1));
        h.get_by_label("Category…").click();
        h.run_steps(3);
        if let Some(Dialog::Refs(d)) = &mut h.state_mut().dialog
            && let RefsDialog::EditCategory { selected, name, .. } = &mut **d
        {
            *selected = 8;
            *name = "Treaties".into();
        } else {
            panic!("no Edit Category: {:?}", h.state().dialog);
        }
        h.run_steps(2);
        h.query_all_by_label("Replace").last().expect("Replace").click();
        h.run_steps(2);
        h.get_by_label("OK").click();
        h.run_steps(3);
        if let Some(Dialog::Refs(d)) = &mut h.state_mut().dialog
            && let RefsDialog::MarkCitation(f) = &mut **d
        {
            f.category = 9;
        } else {
            panic!("back to Mark Citation: {:?}", h.state().dialog);
        }
        h.run_steps(2);
        h.get_by_label("Mark").click();
        h.run_steps(2);
        assert!(instrs(h.state()).contains(&"TA \\l \"Smith v. Jones\" \\s \"Smith v. Jones\" \\c 9".to_string()), "{:?}", instrs(h.state()));
        h.get_by_label("Close").click();
        h.run_steps(2);
        let a = h.state_mut();
        a.run("caret.docEnd", json!({})).unwrap();
        a.run("text.newParagraph", json!({})).unwrap();
        assert_eq!(a.run("references.tableOfAuthorities", json!({})).unwrap(), json!({"pending": "tableOfAuthorities"}));
        let RefsDialog::Authorities(f) = form(a) else { panic!() };
        a.run("references.tableOfAuthorities", json!({"category": f.category, "passim": f.passim, "leader": f.leader})).unwrap();
        let t = a.session.doc.plain_text(StoryRef::Body);
        assert!(t.contains("Treaties") && t.contains("Smith v. Jones\t1"), "{t}");
        assert_eq!(a.prefs().editing.toa_categories.get(8).map(String::as_str), Some("Treaties"));
        // Table of Figures › Options…: from the Quote style.
        assert_eq!(a.run("references.tableOfFigures", json!({})).unwrap(), json!({"pending": "tableOfFigures"}));
        h.run_steps(3);
        h.get_by_label("Options…").click();
        h.run_steps(3);
        if let Some(Dialog::Refs(d)) = &mut h.state_mut().dialog
            && let RefsDialog::FiguresOptions { form, .. } = &mut **d
        {
            form.use_style = true;
            form.style = "Quote".into();
        } else {
            panic!("no Options: {:?}", h.state().dialog);
        }
        h.run_steps(2);
        h.get_by_label("OK").click();
        h.run_steps(3);
        assert!(matches!(form(h.state()), RefsDialog::Figures(f) if f.use_style));
        h.get_by_label("OK").click();
        h.run_steps(3);
        assert!(instrs(h.state()).iter().any(|i| i.starts_with("TOC \\t \"Quote,1\" \\c")), "{:?}", instrs(h.state()));
        assert!(h.state().session.doc.plain_text(StoryRef::Body).contains("Quoted words\t1"));
    }
}
