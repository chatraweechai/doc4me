//! Text frames (Word 97–2003 frames, `w:framePr`): Developer › Controls › Legacy Tools › Insert
//! Frame puts the selected paragraphs in a frame, and Format Frame (Word's Frame dialog) sets its
//! size, wrapping and position or removes it. Adjacent paragraphs with the same frame are one
//! frame; the layout places it out of the text flow.

use std::collections::HashSet;

use doc4me_doc::ParaProps;
use doc4me_doc::props::FrameProps;
use serde_json::{Value, json};

use crate::{CmdError, CmdResult, CommandSpec, Session, p};

const LOC: &str = "Developer › Controls › Legacy Tools";

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("developer.insertFrame", "Insert Frame", LOC, insert_frame).params(r#"{} → {"frame": Frame}"#),
        CommandSpec::new("format.frame", "Format Frame", LOC, format_frame).params(
            r#"{"w"?: n | null (null: as wide as the text), "h"?: n | null, "hRule"?: "auto|atLeast|exact", "wrap"?: "around|none|notBeside|auto|tight|through", "hRel"?: "column|margin|page", "x"?: n, "hAlign"?: "start|center|end|inside|outside" | null, "hSpace"?: n, "vRel"?: "paragraph|margin|page", "y"?: n, "vAlign"?: "start|center|end|inside|outside" | null, "vInline"?: bool, "vSpace"?: n, "anchorLock"?: bool, "remove"?: bool} → {"frame": Frame | null} (no parameters: the frame at the selection)"#,
        ),
    ]
}

/// The frame of the paragraph the selection starts in, as its style and direct formatting give it.
fn current(s: &Session) -> Option<FrameProps> {
    let (a, _) = s.sel.ordered();
    let para = s.doc.para_at(&a)?;
    s.doc.styles.resolve_para(&para.props).frame
}

fn result(s: &Session) -> CmdResult {
    Ok(json!({"frame": current(s)}))
}

/// Insert Frame: the selected paragraphs (the caret's) in a frame as wide and tall as their text,
/// at the column's left edge where they stand, the text after it wrapped around it.
fn insert_frame(s: &mut Session, _: &Value) -> CmdResult {
    if current(s).is_some() {
        s.only_read();
        return result(s);
    }
    super::para::fmt(s, &|p| p.frame = Some(FrameProps::default()))?;
    result(s)
}

/// Format Frame: the parameters given change the selected paragraphs' frame (they go in one when
/// they have none); `remove` takes them out of it. Without parameters: the frame, unchanged.
fn format_frame(s: &mut Session, v: &Value) -> CmdResult {
    let Some(given) = v.as_object().filter(|o| !o.is_empty()) else {
        s.only_read();
        return result(s);
    };
    if p::bool(v, "remove").unwrap_or(false) {
        // A frame from the paragraph style is cancelled; a frame of the paragraph's own goes.
        let framed = framed_styles(s);
        super::para::fmt(s, &|p| {
            let style = p.style.clone().unwrap_or_else(|| "Normal".into());
            p.frame = framed.contains(&style).then(|| FrameProps { off: true, ..Default::default() });
        })?;
        return result(s);
    }
    let mut frame = serde_json::to_value(current(s).unwrap_or_default()).map_err(|e| CmdError::Failed(e.to_string()))?;
    let Some(obj) = frame.as_object_mut() else { return Err(CmdError::Failed("frame".into())) };
    for (k, val) in given {
        if k == "remove" {
            continue;
        }
        obj.insert(k.clone(), val.clone());
    }
    // A position given as a measurement replaces an alignment (and an alignment a measurement).
    if given.contains_key("x") && !given.contains_key("hAlign") {
        obj.remove("hAlign");
    }
    if given.contains_key("y") && !given.contains_key("vAlign") {
        obj.remove("vAlign");
    }
    if given.contains_key("y") || given.get("vAlign").is_some_and(|a| !a.is_null()) {
        obj.remove("vInline");
    }
    obj.remove("off");
    let f: FrameProps = serde_json::from_value(frame).map_err(|e| CmdError::Params(format!("frame: {e}")))?;
    let f = f.clamped();
    super::para::fmt(s, &|p| p.frame = Some(f))?;
    result(s)
}

/// The paragraph styles that put a paragraph in a frame.
fn framed_styles(s: &Session) -> HashSet<String> {
    s.doc
        .styles
        .styles
        .iter()
        .filter(|st| s.doc.styles.resolve_para(&ParaProps { style: Some(st.id.clone()), ..Default::default() }).frame.is_some())
        .map(|st| st.id.clone())
        .collect()
}
