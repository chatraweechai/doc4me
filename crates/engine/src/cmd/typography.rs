//! File › Options › Typography: the document's character spacing control (punctuation
//! compression) and first and last characters (kinsoku), as Word for East Asian languages has
//! them. They are document settings (`w:settings`), saved with the file and undone like edits.

use doc4me_doc::{CharSpacing, Kinsoku};
use serde_json::{Value, json};

use crate::{CmdError, CmdResult, CommandSpec, Session, p};

pub fn specs() -> Vec<CommandSpec> {
    vec![CommandSpec::new("file.typography", "Typography", "File › Options", typography).params(
        r#"{"charSpacing"?: "doNotCompress|compressPunctuation|compressPunctuationAndJapaneseKana", "kinsoku"?: "standard|strict|custom", "noLineStart"?: string, "noLineEnd"?: string} — with any setting it changes them; without, it opens File › Options › Typography. Returns the settings (with the lists in effect)."#,
    )]
}

/// The document's typography settings: `charSpacing`, `kinsoku` (standard, strict or custom)
/// and the characters no line may start (`noLineStart`) or end (`noLineEnd`) with.
pub fn state(s: &Session) -> Value {
    let k = &s.doc.settings.kinsoku;
    let level = if k.no_start.is_some() || k.no_end.is_some() {
        "custom"
    } else if k.strict {
        "strict"
    } else {
        "standard"
    };
    let (start, end) = doc4me_layout::kinsoku::standard_lists(k.strict);
    json!({
        "charSpacing": s.doc.settings.char_spacing.ooxml(),
        "kinsoku": level,
        "noLineStart": k.no_start.clone().unwrap_or(start),
        "noLineEnd": k.no_end.clone().unwrap_or(end),
    })
}

fn list_param(v: &Value, k: &str) -> Result<Option<String>, CmdError> {
    match v.get(k) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(t)) => Ok(Some(Kinsoku::clean_list(t).unwrap_or_default())),
        Some(_) => Err(CmdError::Params(format!("`{k}` must be a string of characters"))),
    }
}

fn typography(s: &mut Session, v: &Value) -> CmdResult {
    // Read every setting first, so a bad one changes nothing.
    let spacing = match p::str(v, "charSpacing") {
        None => None,
        Some(x) => Some(CharSpacing::from_ooxml(x).ok_or_else(|| {
            CmdError::Params(format!("unknown charSpacing `{x}`: doNotCompress, compressPunctuation or compressPunctuationAndJapaneseKana"))
        })?),
    };
    let level = match p::str(v, "kinsoku") {
        None => None,
        Some(l @ ("standard" | "strict" | "custom")) => Some(l),
        Some(l) => return Err(CmdError::Params(format!("unknown kinsoku `{l}`: standard, strict or custom"))),
    };
    let (start, end) = (list_param(v, "noLineStart")?, list_param(v, "noLineEnd")?);
    if level.is_some_and(|l| l != "custom") && (start.is_some() || end.is_some()) {
        return Err(CmdError::Params("`noLineStart` and `noLineEnd` set custom lists: give them with kinsoku `custom`".into()));
    }
    if spacing.is_none() && level.is_none() && start.is_none() && end.is_none() {
        s.ui_requests.push(json!({"open": "options", "pane": "typography"}));
        let mut r = state(s);
        if let Some(o) = r.as_object_mut() {
            o.insert("opened".into(), json!(true));
        }
        return Ok(r);
    }
    let st = &mut s.doc.settings;
    if let Some(c) = spacing {
        st.char_spacing = c;
    }
    let level = level.or_else(|| (start.is_some() || end.is_some()).then_some("custom"));
    match level {
        Some("standard") => st.kinsoku = Kinsoku { lang: st.kinsoku.lang.clone(), ..Kinsoku::default() },
        Some("strict") => st.kinsoku = Kinsoku { strict: true, lang: st.kinsoku.lang.clone(), ..Kinsoku::default() },
        Some(_) => {
            // Custom lists start from the ones in effect; an empty list allows any character.
            let (std_start, std_end) = doc4me_layout::kinsoku::standard_lists(st.kinsoku.strict);
            let k = &mut st.kinsoku;
            let no_start = start.or_else(|| k.no_start.clone()).unwrap_or(std_start);
            let no_end = end.or_else(|| k.no_end.clone()).unwrap_or(std_end);
            k.strict = false;
            k.no_start = Some(no_start);
            k.no_end = Some(no_end);
        }
        None => {}
    }
    s.relayout();
    Ok(state(s))
}

#[cfg(test)]
mod tests {
    use doc4me_doc::{CharSpacing, Document};
    use serde_json::json;

    use crate::Session;

    #[test]
    fn typography_settings_change_reopen_and_undo() {
        let mut s = Session::new(Document::from_text("日本語の文章。「引用」"));
        let r = s.run("file.typography", &json!({})).unwrap();
        assert_eq!((r["kinsoku"].as_str(), r["charSpacing"].as_str(), r["opened"].as_bool()), (Some("standard"), Some("doNotCompress"), Some(true)));
        assert!(r["noLineStart"].as_str().unwrap().contains('。'), "the standard list is shown");
        assert!(s.run("file.typography", &json!({"charSpacing": "squeeze"})).is_err());
        assert!(s.run("file.typography", &json!({"kinsoku": "strict", "noLineStart": "ー"})).is_err(), "lists only with custom");
        s.run("file.typography", &json!({"charSpacing": "compressPunctuation", "kinsoku": "strict"})).unwrap();
        assert_eq!(s.doc.settings.char_spacing, CharSpacing::CompressPunctuation);
        assert!(s.doc.settings.kinsoku.strict);
        let r = s.run("file.typography", &json!({"noLineStart": "。。、x\n", "noLineEnd": ""})).unwrap();
        assert_eq!((r["kinsoku"].as_str(), r["noLineStart"].as_str(), r["noLineEnd"].as_str()), (Some("custom"), Some("。、x"), Some("")));
        assert!(!s.doc.settings.kinsoku.strict);
        s.run("file.typography", &json!({"kinsoku": "standard"})).unwrap();
        assert_eq!(s.doc.settings.kinsoku, doc4me_doc::Kinsoku::default());
        s.run("edit.undo", &json!({})).unwrap();
        assert_eq!(s.doc.settings.kinsoku.no_start.as_deref(), Some("。、x"));
    }
}
