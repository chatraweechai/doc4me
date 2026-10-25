//! Modify Style › Format: Font…, Paragraph…, Tabs…, Border… and Language… edit a style with the
//! dialogs that format the selection. The dialog works on a scratch document — one paragraph in
//! the style (or, for a character style, one run), all selected — swapped in only while the
//! dialog draws; when it closes, the formatting it set there goes back into Modify Style, which
//! applies it with the rest on OK (`styles.modify`), so Cancel there still changes nothing.

use doc4me_doc::{CharProps, Document, Path, Pos, StoryRef, StyleKind, para_block};
use doc4me_engine::{Selection, Session};
use serde_json::{Map, Value, json};

use crate::WordApp;
use crate::dialogs::Dialog;

/// The Format menu: (label, the dialog `ui.dialog` names, for paragraph styles only).
pub const ITEMS: [(&str, &str, bool); 5] = [
    ("Font…", "font", false),
    ("Paragraph…", "paragraph", true),
    ("Tabs…", "tabs", true),
    ("Border…", "borders", true),
    ("Language…", "language", false),
];

/// The text of the scratch paragraph.
const SAMPLE: &str = "AaBbCcYyZz";

/// A Format dialog's scratch document and the Modify Style dialog to go back to.
pub struct StyleScratch {
    pub session: Box<Session>,
    /// The scratch paragraph's and run's direct formatting when the dialog opened.
    before: (Value, Value),
    resume: Box<Dialog>,
}

/// Whether `name` is one of the Format dialogs (or one they lead to), which draw on the scratch.
pub fn uses_scratch(name: &str) -> bool {
    matches!(name, "font" | "paragraph" | "asianTypography" | "tabs" | "borders" | "pageBorders" | "shading" | "language")
}

/// Opens Format › `which` for the style `md` (a Modify Style dialog) edits.
pub fn open(app: &mut WordApp, which: &str, md: Dialog) {
    let Dialog::ModifyStyle { id, .. } = &md else { return };
    let Some(kind) = app.session.doc.styles.get(id).map(|s| s.kind) else {
        app.dialog = Some(md);
        return;
    };
    let mut session = Box::new(Session::new(scratch_document(&app.session.doc, id, kind)));
    // What Modify Style already holds, so this dialog starts from it.
    let (chr, para) = pending(&md);
    let _ = session.run("styles.modify", &json!({"style": id, "chr": chr, "para": para}));
    let end = session.doc.para(StoryRef::Body, &Path::top(0)).map_or(0, |p| p.len());
    let at = |off| Pos { story: StoryRef::Body, path: Path::top(0), off };
    session.sel = Selection { anchor: at(0), focus: at(end) };
    let before = direct(&session);
    std::mem::swap(&mut app.session, &mut session);
    let dialog = Dialog::open(which, app);
    std::mem::swap(&mut app.session, &mut session);
    match dialog {
        Some(d) => {
            app.dialog = Some(d);
            app.style_scratch = Some(StyleScratch { session, before, resume: Box::new(md) });
        }
        None => app.dialog = Some(md),
    }
}

/// The Format dialog closed: what it set goes into Modify Style, whose fields show it.
pub fn finish(app: &mut WordApp, sc: StyleScratch) {
    let StyleScratch { session, before, resume } = sc;
    let mut md = *resume;
    let after = direct(&session);
    if after != before
        && let Dialog::ModifyStyle { id, extra_chr, extra_para, font, size, bold, italic, color, before: sp_before, after: sp_after, .. } = &mut md
    {
        merge(extra_para, &after.0);
        merge(extra_chr, &after.1);
        let styles = &session.doc.styles;
        let para = session.doc.para(StoryRef::Body, &Path::top(0));
        let props = para.map(|p| p.props.clone()).unwrap_or_default();
        let run = run_props(&session);
        let is_para_style = styles.get(id).is_some_and(|s| s.kind == StyleKind::Paragraph);
        let rc = styles.resolve_char(props.style.as_deref().filter(|_| is_para_style), &run);
        *font = rc.font;
        *size = rc.size;
        *bold = rc.bold;
        *italic = rc.italic;
        *color = match rc.color {
            doc4me_doc::TextColor::Rgb(c) => c.hex(),
            doc4me_doc::TextColor::Auto => "auto".into(),
        };
        let rp = styles.resolve_para(&props);
        *sp_before = rp.space_before;
        *sp_after = rp.space_after;
    }
    app.dialog = Some(md);
}

/// One paragraph of sample text in the style (a run in it for a character style), in a copy of
/// `doc` without its content, tracked changes or protection, so formatting lands as direct
/// formatting there.
fn scratch_document(doc: &Document, id: &str, kind: StyleKind) -> Document {
    let mut d = doc.clone();
    d.parts.clear();
    d.comments.clear();
    d.revisions.clear();
    d.settings.track_changes = false;
    d.settings.protection = None;
    let chr = CharProps { style: (kind == StyleKind::Character).then(|| id.to_string()), ..Default::default() };
    let mut p = doc4me_doc::Paragraph::with_text(SAMPLE, chr);
    if kind == StyleKind::Paragraph {
        p = p.styled(id);
    }
    d.body = vec![para_block(p)];
    d
}

/// Modify Style's pending `chr` and `para`: what Format set, then its own fields.
pub fn pending(md: &Dialog) -> (Value, Value) {
    let Dialog::ModifyStyle { font, size, bold, italic, color, before, after, extra_chr, extra_para, .. } = md else {
        return (json!({}), json!({}));
    };
    let mut chr = extra_chr.clone();
    chr.insert("font".into(), json!(font));
    chr.insert("size".into(), json!(*size));
    chr.insert("bold".into(), json!(*bold));
    chr.insert("italic".into(), json!(*italic));
    if let Some(c) = doc4me_doc::Rgb::parse(color) {
        chr.insert("color".into(), json!({"Rgb": [c.0, c.1, c.2]}));
    }
    let mut para = extra_para.clone();
    para.insert("spaceBefore".into(), json!(*before));
    para.insert("spaceAfter".into(), json!(*after));
    (Value::Object(chr), Value::Object(para))
}

/// The run formatting of the scratch's sample text.
fn run_props(s: &Session) -> CharProps {
    s.doc.para(StoryRef::Body, &Path::top(0)).and_then(|p| p.run_ranges().next().map(|(_, c)| c.clone())).unwrap_or_default()
}

/// The scratch paragraph's and its text's direct formatting (styles aside), as `styles.modify`
/// takes them.
fn direct(s: &Session) -> (Value, Value) {
    let mut para = s.doc.para(StoryRef::Body, &Path::top(0)).map(|p| p.props.clone()).unwrap_or_default();
    para.style = None;
    para.fmt_change = None;
    let mut chr = run_props(s);
    chr.style = None;
    chr.fmt_change = None;
    (serde_json::to_value(&para).unwrap_or_default(), serde_json::to_value(&chr).unwrap_or_default())
}

/// Sets every property `from` has (not null) in `into`.
fn merge(into: &mut Map<String, Value>, from: &Value) {
    for (k, v) in from.as_object().into_iter().flatten() {
        if !v.is_null() {
            into.insert(k.clone(), v.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use doc4me_doc::props::Align;
    use serde_json::json;

    use super::*;
    use crate::Services;

    /// Format › Paragraph… opens the Paragraph dialog on a scratch paragraph in the style; what it
    /// sets there (alignment, a tab, and a font from Format › Font…) shows in Modify Style when it
    /// closes, leaves the document alone, and goes into the style on OK. Language… opens too.
    #[test]
    fn modify_style_format_dialogs_edit_the_style() {
        let mut a = WordApp::new(Session::new(Document::from_text("Kilns\nPresses")), Services::default());
        let before = a.session.doc.clone();
        // Modify Style without a style: the caret paragraph's.
        assert_eq!(a.run("styles.modify", json!({})).unwrap(), json!({"pending": "modifyStyle"}));
        assert!(matches!(&a.dialog, Some(Dialog::ModifyStyle { id, .. }) if id == "Normal"), "{:?}", a.dialog);
        let md = Dialog::modify_style(&a, "Heading1").unwrap();
        open(&mut a, "paragraph", md);
        assert!(matches!(a.dialog, Some(Dialog::Paragraph { .. })), "{:?}", a.dialog);
        let mut sc = a.style_scratch.take().unwrap();
        assert_eq!(sc.session.doc.para(StoryRef::Body, &Path::top(0)).unwrap().props.style.as_deref(), Some("Heading1"));
        // What the dialogs' OK runs, on the scratch.
        std::mem::swap(&mut a.session, &mut sc.session);
        a.run("para.align", json!({"value": "center"})).unwrap();
        a.run("para.tabs", json!({"set": [{"pos": 144}]})).unwrap();
        a.run("format.set", json!({"props": {"font": "Courier New", "italic": true}})).unwrap();
        std::mem::swap(&mut a.session, &mut sc.session);
        a.dialog = None;
        finish(&mut a, sc);
        let Some(Dialog::ModifyStyle { font, italic, .. }) = &a.dialog else { panic!("{:?}", a.dialog) };
        assert_eq!((font.as_str(), *italic), ("Courier New", true));
        assert_eq!(a.session.doc, before, "nothing changes before OK");
        // Another Format dialog starts from what Modify Style holds.
        let md = a.dialog.take().unwrap();
        open(&mut a, "language", md);
        assert!(matches!(a.dialog, Some(Dialog::Layout(_))), "{:?}", a.dialog);
        let sc = a.style_scratch.take().unwrap();
        let p = sc.session.doc.para(StoryRef::Body, &Path::top(0)).unwrap();
        assert_eq!(sc.session.doc.styles.resolve_para(&p.props).align, Align::Center);
        a.dialog = None;
        finish(&mut a, sc);
        // OK: the style takes it all.
        let (chr, para) = pending(a.dialog.as_ref().unwrap());
        a.run("styles.modify", json!({"style": "Heading1", "chr": chr, "para": para})).unwrap();
        let st = a.session.doc.styles.get("Heading1").unwrap();
        assert_eq!((st.para.align, st.chr.font.as_deref(), st.chr.italic), (Some(Align::Center), Some("Courier New"), Some(true)));
        assert!(st.para.tabs.as_ref().is_some_and(|t| t.iter().any(|t| (t.pos - 144.0).abs() < 0.1)), "{:?}", st.para.tabs);
        // A character style has no paragraph to format.
        assert!(ITEMS.iter().any(|(_, w, para_only)| *w == "paragraph" && *para_only));
    }
}
