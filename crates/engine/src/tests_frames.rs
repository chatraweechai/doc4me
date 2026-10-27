//! Text frames through the commands: Insert Frame, Format Frame (size, wrapping, position,
//! removing it), a style's frame, the layout and the file.

use doc4me_doc::para::{Anchor, FloatAlign};
use doc4me_doc::props::FrameWrap;
use doc4me_doc::{Path, Pos, StoryRef};
use doc4me_layout::Placed;
use serde_json::json;

use crate::{Selection, Session};

fn run(s: &mut Session, id: &str, v: serde_json::Value) -> serde_json::Value {
    s.run(id, &v).unwrap_or_else(|e| panic!("{id}: {e}"))
}

/// Body paragraph `i` on page 1: where its first line's text starts (x), the line's top, and
/// where its text ends.
fn placed(s: &mut Session, i: u32) -> (f32, f32, f32) {
    let l = s.layout();
    for it in &l.pages[0].items {
        if let Placed::Lines { story: StoryRef::Body, path, para, l0, x, y, .. } = it
            && path.0 == [i]
            && *l0 == 0
            && let Some(line) = para.lines.first()
        {
            let (a, b) = (line.xs.first().copied().unwrap_or(0.0), line.xs.last().copied().unwrap_or(0.0));
            return (x + a, *y, x + b);
        }
    }
    panic!("paragraph {i} isn't on page 1");
}

/// "One" / "Framed" / "Three", the caret in "Framed".
fn three() -> Session {
    let mut s = Session::new(doc4me_doc::Document::from_text("One"));
    run(&mut s, "caret.docEnd", json!({}));
    for t in ["Framed", "Three"] {
        run(&mut s, "text.newParagraph", json!({}));
        run(&mut s, "text.insert", json!({"text": t}));
    }
    s.sel = Selection::caret(Pos::new(StoryRef::Body, Path::top(1), 0));
    s
}

#[test]
fn a_frame_stands_out_of_the_text_flow() {
    let mut s = three();
    let before = placed(&mut s, 2);
    assert!(run(&mut s, "format.frame", json!({}))["frame"].is_null());
    // Insert Frame: as wide as its text, at the column's left edge where it stood; "Three" moves
    // up into its place and wraps around it.
    let f = run(&mut s, "developer.insertFrame", json!({}));
    assert_eq!(
        (f["frame"]["wrap"].as_str(), f["frame"]["hRel"].as_str(), f["frame"]["vRel"].as_str()),
        (Some("around"), Some("column"), Some("paragraph"))
    );
    let (framed, after) = (placed(&mut s, 1), placed(&mut s, 2));
    assert!(after.1 < before.1 - 5.0, "the text after it moves up: {after:?} was {before:?}");
    assert!((after.1 - framed.1).abs() < 0.5, "the frame stands at the top of the paragraph after it: {framed:?} {after:?}");
    assert!(after.0 >= framed.2 - 0.5, "wrapped beside it: {after:?} {framed:?}");
    // Format Frame: an inch wide at the right margin, over the text.
    let sect = s.doc.sections()[0].1.clone();
    run(&mut s, "format.frame", json!({"w": 72, "hRel": "margin", "hAlign": "end", "wrap": "none"}));
    let (framed, after) = (placed(&mut s, 1), placed(&mut s, 2));
    let right = sect.page_w - sect.margin_right;
    assert!(framed.0 >= right - 72.5 && framed.0 < right - 30.0, "{framed:?} at the right margin ({right})");
    assert!((after.0 - (sect.margin_left + sect.gutter)).abs() < 0.5, "the text runs under it: {after:?}");
    // Not beside: the text goes on below it.
    run(&mut s, "format.frame", json!({"wrap": "notBeside", "h": 50, "hRule": "exact"}));
    let (framed, after) = (placed(&mut s, 1), placed(&mut s, 2));
    assert!(after.1 >= framed.1 + 49.5, "{after:?} below the 50 pt frame at {framed:?}");
    // A measurement replaces the alignment.
    let f = run(&mut s, "format.frame", json!({"x": 36}));
    assert!(f["frame"]["hAlign"].is_null() && f["frame"]["x"].as_f64() == Some(36.0), "{f}");
    // Saved and opened again.
    let back = doc4me_docx::read(&doc4me_docx::write(&s.doc).unwrap()).unwrap();
    let frame = back.para(StoryRef::Body, &Path::top(1)).and_then(|p| p.props.frame).expect("the frame is saved");
    assert_eq!((frame.w, frame.h, frame.wrap, frame.h_rel, frame.x), (Some(72.0), Some(50.0), FrameWrap::NotBeside, Anchor::Margin, 36.0));
    assert!(back.para(StoryRef::Body, &Path::top(2)).is_some_and(|p| p.props.frame.is_none()));
    // Remove Frame: back in the text, one undo puts it back.
    assert!(run(&mut s, "format.frame", json!({"remove": true}))["frame"].is_null());
    assert!((placed(&mut s, 2).1 - before.1).abs() < 0.01);
    run(&mut s, "edit.undo", json!({}));
    assert!(run(&mut s, "format.frame", json!({}))["frame"]["w"].as_f64() == Some(72.0));
    // Bad values are refused.
    assert!(s.run("format.frame", &json!({"wrap": "sideways"})).is_err());
}

#[test]
fn a_style_s_frame_and_removing_it_from_one_paragraph() {
    let mut s = three();
    run(
        &mut s,
        "styles.modify",
        json!({"style": "Quote", "para": {"frame": {"hRel": "page", "hAlign": "center", "vRel": "margin", "vAlign": "end"}}}),
    );
    let (a, b) = (Pos::new(StoryRef::Body, Path::top(1), 0), Pos::new(StoryRef::Body, Path::top(2), 5));
    s.doc.format_paragraphs(&a, &b, &|p| p.style = Some("Quote".into())).unwrap();
    s.touch();
    // Both paragraphs are in the style's frame: one frame, centred near the bottom margin.
    let f = run(&mut s, "format.frame", json!({}));
    assert_eq!((f["frame"]["hAlign"].as_str(), f["frame"]["vAlign"].as_str()), (Some("center"), Some("end")));
    let sect = s.doc.sections()[0].1.clone();
    let (one, two) = (placed(&mut s, 1), placed(&mut s, 2));
    assert!(two.1 > one.1 + 5.0, "stacked in one frame: {one:?} {two:?}");
    assert!(two.1 > sect.page_h - sect.margin_bottom - 40.0, "at the bottom margin: {two:?}");
    // Remove Frame on "Three" cancels the style's frame there (and nothing is written for it).
    s.sel = Selection::caret(b.clone());
    run(&mut s, "format.frame", json!({"remove": true}));
    assert!(run(&mut s, "format.frame", json!({}))["frame"].is_null());
    s.sel = Selection::caret(a);
    assert!(!run(&mut s, "format.frame", json!({}))["frame"].is_null());
    let back = doc4me_docx::read(&doc4me_docx::write(&s.doc).unwrap()).unwrap();
    let quote = back.styles.get("Quote").and_then(|st| st.para.frame).expect("the style's frame is saved");
    assert_eq!((quote.h_rel, quote.h_align, quote.v_align), (Anchor::Page, Some(FloatAlign::Center), Some(FloatAlign::End)));
    // The style's frame cancelled (Modify Style › Format › Frame… › Remove Frame): no frame, and
    // none in the file.
    run(&mut s, "styles.modify", json!({"style": "Quote", "para": {"frame": {"off": true}}}));
    s.sel = Selection::caret(Pos::new(StoryRef::Body, Path::top(1), 0));
    assert!(run(&mut s, "format.frame", json!({}))["frame"].is_null());
    let back = doc4me_docx::read(&doc4me_docx::write(&s.doc).unwrap()).unwrap();
    assert!(back.styles.get("Quote").is_some_and(|st| st.para.frame.is_none()));
}
