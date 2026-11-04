//! Home › Paragraph › Asian Layout: Fit Text (`w:fitText`) and Two Lines in One
//! (`w:eastAsianLayout/@w:combine`), formatting of the selected characters.

use doc4me_doc::props::{CombineBrackets, EastAsianLayout, FitText, MAX_FIT_TEXT};
use serde_json::{Value, json};

use super::sel_result;
use crate::{CmdError, CmdResult, CommandSpec, Session, p};

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("format.fitText", "Fit Text", "Home › Paragraph › Asian Layout", fit_text).params(r#"{"width": number | null}"#),
        CommandSpec::new("format.twoLinesInOne", "Two Lines in One", "Home › Paragraph › Asian Layout", two_lines_in_one)
            .params(r#"{"value": bool, "brackets"?: "none|round|square|angle|curly"}"#),
    ]
}

/// The selection, which Asian Layout formats (it needs characters selected, as in Word).
fn selected(s: &Session) -> Result<(doc4me_doc::Pos, doc4me_doc::Pos), CmdError> {
    let (a, b) = s.sel.ordered();
    if a == b {
        return Err(CmdError::Failed("select the characters first".into()));
    }
    Ok((a, b))
}

/// The selection's Fit Text (`fitText`: width in points, or null) and Two Lines in One
/// (`twoLinesInOne`, `brackets`), as at its start.
pub fn state(s: &Session) -> Value {
    let (a, b) = s.sel.ordered();
    // The formatting of the first selected character (the one after the caret).
    let props = s.doc.para_at(&a).map(|p| {
        let next = p.text.get(a.off..).and_then(|t| t.chars().next()).map_or(a.off, |c| a.off + c.len_utf8());
        p.props_at(next).clone()
    });
    let fit = props.as_ref().and_then(|c| c.fit_text).and_then(FitText::sanitized).map(|f| f.width);
    let ea = props.as_ref().and_then(|c| c.east_asian).unwrap_or_default();
    json!({
        "fitText": fit,
        "twoLinesInOne": ea.combine,
        "brackets": ea.brackets.ooxml(),
        "selected": a != b,
    })
}

/// `width` in points sets the selected text to that width; `null` (or 0) removes Fit Text.
fn fit_text(s: &mut Session, v: &Value) -> CmdResult {
    let width = match v.get("width") {
        None => return Err(CmdError::Params("`width` (points, or null to remove) is required".into())),
        Some(Value::Null) => None,
        Some(_) => {
            let w = p::req_f32(v, "width")?;
            if !w.is_finite() || w < 0.0 {
                return Err(CmdError::Params("`width` must be 0 or more".into()));
            }
            (w > 0.0).then_some(w.min(MAX_FIT_TEXT))
        }
    };
    let (a, b) = selected(s)?;
    // A new piece gets an id of its own, so it never joins fitted text right beside it.
    let id = width.map(|_| next_fit_id(s, &a));
    let fit = width.and_then(|w| FitText { width: w, id }.sanitized());
    s.doc.format_range(&a, &b, &|c| c.fit_text = fit)?;
    sel_result(s)
}

/// One more than the largest Fit Text id in the paragraph at `at`.
fn next_fit_id(s: &Session, at: &doc4me_doc::Pos) -> u32 {
    let Some(p) = s.doc.para_at(at) else { return 1 };
    p.run_ranges().filter_map(|(_, c)| c.fit_text.and_then(|f| f.id)).max().map_or(1, |m| m.saturating_add(1))
}

/// `value` turns Two Lines in One on or off for the selected text; `brackets` encloses the two
/// lines (round, square, angle or curly brackets; none by default).
fn two_lines_in_one(s: &mut Session, v: &Value) -> CmdResult {
    let Some(on) = p::bool(v, "value") else {
        return Err(CmdError::Params("`value` (true or false) is required".into()));
    };
    let brackets = match p::str(v, "brackets") {
        None => CombineBrackets::None,
        Some(b @ ("none" | "round" | "square" | "angle" | "curly")) => CombineBrackets::from_ooxml(b),
        Some(b) => return Err(CmdError::Params(format!("unknown brackets `{b}`: none, round, square, angle or curly"))),
    };
    let (a, b) = selected(s)?;
    s.doc.format_range(&a, &b, &|c| {
        // Horizontal in Vertical, kept from the file, stays as it was.
        let mut l = c.east_asian.unwrap_or_default();
        l.combine = on;
        l.brackets = if on { brackets } else { CombineBrackets::None };
        c.east_asian = (l != EastAsianLayout::default()).then_some(l);
    })?;
    sel_result(s)
}

#[cfg(test)]
mod tests {
    use doc4me_doc::props::{CombineBrackets, EastAsianLayout};
    use doc4me_doc::{Document, Paragraph, StoryRef};
    use serde_json::json;

    use crate::Session;

    fn session(text: &str) -> Session {
        let mut d = Document::new();
        d.body = vec![doc4me_doc::para_block(Paragraph::with_text(text, Default::default()))];
        Session::new(d)
    }

    fn props_of(s: &Session, word: &str) -> doc4me_doc::props::CharProps {
        let p = s.doc.para(StoryRef::Body, &doc4me_doc::Path::top(0)).unwrap();
        let at = p.text.find(word).unwrap() + word.len();
        p.props_at(at).clone()
    }

    #[test]
    fn fit_text_applies_reports_and_removes() {
        let mut s = session("name 東京都 here");
        assert!(s.run("format.fitText", &json!({"width": 72})).is_err(), "nothing selected");
        s.run("select.text", &json!({"text": "東京都"})).unwrap();
        assert!(s.run("format.fitText", &json!({})).is_err(), "a script says the width");
        assert!(s.run("format.fitText", &json!({"width": -3})).is_err());
        s.run("format.fitText", &json!({"width": 72})).unwrap();
        let f = props_of(&s, "東京都").fit_text.unwrap();
        assert_eq!((f.width, f.id), (72.0, Some(1)));
        assert_eq!(super::state(&s)["fitText"], json!(72.0));
        assert!(props_of(&s, "name").fit_text.is_none() && props_of(&s, "here").fit_text.is_none(), "only the selection");
        // Another piece in the paragraph gets its own id, so the two never join.
        s.run("select.text", &json!({"text": "here"})).unwrap();
        s.run("format.fitText", &json!({"width": 50})).unwrap();
        assert_eq!(props_of(&s, "here").fit_text.unwrap().id, Some(2));
        // Removing it, and undoing that.
        s.run("select.text", &json!({"text": "東京都"})).unwrap();
        s.run("format.fitText", &json!({"width": null})).unwrap();
        assert!(props_of(&s, "東京都").fit_text.is_none());
        s.run("edit.undo", &json!({})).unwrap();
        assert_eq!(props_of(&s, "東京都").fit_text.map(|f| f.width), Some(72.0));
    }

    #[test]
    fn two_lines_in_one_sets_brackets_and_keeps_horizontal_in_vertical() {
        let mut s = session("a 株式会社 b");
        assert!(s.run("format.twoLinesInOne", &json!({"value": true})).is_err(), "nothing selected");
        s.run("select.text", &json!({"text": "株式会社"})).unwrap();
        assert!(s.run("format.twoLinesInOne", &json!({"value": true, "brackets": "wavy"})).is_err());
        s.run("format.twoLinesInOne", &json!({"value": true, "brackets": "round"})).unwrap();
        let l = props_of(&s, "株式会社").east_asian.unwrap();
        assert!(l.combine && l.brackets == CombineBrackets::Round, "{l:?}");
        let st = super::state(&s);
        assert_eq!((st["twoLinesInOne"].as_bool(), st["brackets"].as_str()), (Some(true), Some("round")));
        // Horizontal in Vertical from a file stays when Two Lines in One goes.
        s.doc
            .format_range(&s.sel.ordered().0, &s.sel.ordered().1, &|c| {
                c.east_asian = c.east_asian.map(|l| EastAsianLayout { vert: true, ..l });
            })
            .unwrap();
        s.run("format.twoLinesInOne", &json!({"value": false})).unwrap();
        let l = props_of(&s, "株式会社").east_asian.unwrap();
        assert!(!l.combine && l.vert && l.brackets == CombineBrackets::None, "{l:?}");
        // Without either, nothing is left.
        s.doc
            .format_range(&s.sel.ordered().0, &s.sel.ordered().1, &|c| {
                c.east_asian = c.east_asian.map(|l| EastAsianLayout { vert: false, ..l });
            })
            .unwrap();
        s.run("format.twoLinesInOne", &json!({"value": true})).unwrap();
        s.run("format.twoLinesInOne", &json!({"value": false})).unwrap();
        assert!(props_of(&s, "株式会社").east_asian.is_none());
    }
}
