//! File › Options (#485): the settings agents can read and change without the window — user
//! name and initials, typing (replace selection, overtype), measurement units, formatting marks
//! and proofing. The Options page in the app sets them through the same command.

use serde_json::{Value, json};

use crate::{CmdError, CmdResult, CommandSpec, Prefs, Session, p};

/// Options panes, in the order the page lists them (the `pane` param of `file.options`).
pub const PANES: [&str; 10] =
    ["general", "display", "proofing", "save", "typography", "language", "accessibility", "advanced", "trustCenter", "agents"];

/// Longest initials kept, in characters.
pub const MAX_INITIALS_CHARS: usize = 9;
/// Longest word the custom dictionary takes, in characters.
pub const MAX_WORD_CHARS: usize = 64;
/// Most words one call may add or remove.
const MAX_WORDS_PER_CALL: usize = 1000;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("file.options", "Options", "File", options)
            .params(
                r#"{"pane"?: "general|display|proofing|save|typography|language|accessibility|advanced|trustCenter|agents", "author"?: string, "initials"?: string, "units"?: "inches|centimeters|millimeters|points|picas", "typingReplacesSelection"?: bool, "insertKeyOvertype"?: bool, "overtype"?: bool, "marks"?: {"tabs"?: bool, "spaces"?: bool, "paragraphs"?: bool, "hidden"?: bool}, "checkSpelling"?: bool, "markGrammar"?: bool, "ignoreUppercase"?: bool, "ignoreNumbers"?: bool, "ignoreInternet"?: bool, "addWords"?: [string], "removeWords"?: [string], "pasteSame"|"pasteBetween"|"pasteOther"?: "keep|merge|text" (Paste within this document, between documents, from other programs), "logicalCursor"?: bool, "rtlDefault"?: bool (Bidirectional text), "grammarRules"?: {"capitalization"|"repeated"|"wordForms"|"punctuation"|"spacing"?: bool} (Writing Style › Settings)} — with any setting it changes them and opens nothing; without, it opens File › Options (at `pane`). Returns the settings."#,
            )
            .pure(),
        CommandSpec::new("text.overtype", "Overtype", "File › Options › Advanced", |s, v| {
            s.prefs.overtype = p::bool(v, "value").unwrap_or(!s.prefs.overtype);
            Ok(json!({"value": s.prefs.overtype}))
        })
        .params(r#"{"value"?: bool (default: toggle)}"#)
        .pure(),
        CommandSpec::new("file.fileLocations", "File Locations", "File › Options › Advanced", file_locations)
            .params(r#"{"documents"?: folder | null, "pictures"?: folder | null} — the folders Open and Save As, and Insert Picture, start in (null: the system's choice); without either, opens File › Options › Advanced"#)
            .pure(),
        CommandSpec::new("file.bidiOptions", "Bidirectional Text", "File › Options › Advanced", bidi_options)
            .params(r#"{"cursorMovement"?: "logical|visual", "defaultDirection"?: "ltr|rtl" (new blank documents)} — without either, opens File › Options › Advanced"#)
            .pure(),
    ]
}

/// Longest folder path File Locations keeps, in bytes.
const MAX_FOLDER_PATH: usize = 4096;

/// A File Locations folder: `Some(None)` clears it, `Some(Some(path))` sets it to an existing
/// folder (absolute), `None` leaves it.
fn folder_param(v: &Value, k: &str) -> Result<Option<Option<String>>, CmdError> {
    match v.get(k) {
        None => Ok(None),
        Some(Value::Null) => Ok(Some(None)),
        Some(Value::String(t)) if t.trim().is_empty() => Ok(Some(None)),
        Some(Value::String(t)) => {
            let t = t.trim();
            // An agent confined to its folders learns nothing of folders outside them.
            crate::file_access::check_read(t).map_err(CmdError::Failed)?;
            let path = std::path::Path::new(t);
            if t.len() > MAX_FOLDER_PATH || t.contains('\0') || !path.is_absolute() || !path.is_dir() {
                return Err(CmdError::Params(format!("`{k}` must be an existing folder, given as a full path")));
            }
            Ok(Some(Some(t.to_string())))
        }
        Some(_) => Err(CmdError::Params(format!("`{k}` must be a folder path or null"))),
    }
}

/// Word's File Locations: where Open and Save As (Documents) and Insert Picture (Pictures) start.
fn file_locations(s: &mut Session, v: &Value) -> CmdResult {
    let documents = folder_param(v, "documents")?;
    let pictures = folder_param(v, "pictures")?;
    let opened = documents.is_none() && pictures.is_none();
    if let Some(d) = documents {
        s.prefs.documents_dir = d;
    }
    if let Some(p) = pictures {
        s.prefs.pictures_dir = p;
    }
    if opened {
        s.ui_requests.push(json!({"open": "options", "pane": "advanced"}));
    }
    Ok(json!({"documents": s.prefs.documents_dir, "pictures": s.prefs.pictures_dir, "opened": opened}))
}

/// Word's bidirectional (complex scripts) options: Cursor movement and Default direction.
fn bidi_options(s: &mut Session, v: &Value) -> CmdResult {
    let mut set = serde_json::Map::new();
    match p::str(v, "cursorMovement") {
        None => {}
        Some(m @ ("logical" | "visual")) => {
            set.insert("logicalCursor".into(), json!(m == "logical"));
        }
        Some(m) => return Err(CmdError::Params(format!("unknown cursorMovement `{m}`: logical or visual"))),
    }
    match p::str(v, "defaultDirection") {
        None => {}
        Some(d @ ("ltr" | "rtl")) => {
            set.insert("rtlDefault".into(), json!(d == "rtl"));
        }
        Some(d) => return Err(CmdError::Params(format!("unknown defaultDirection `{d}`: ltr or rtl"))),
    }
    if set.is_empty() {
        set.insert("pane".into(), json!("advanced"));
    }
    let r = options(s, &Value::Object(set))?;
    Ok(json!({
        "cursorMovement": if s.prefs.logical_cursor { "logical" } else { "visual" },
        "defaultDirection": if s.prefs.rtl_default { "rtl" } else { "ltr" },
        "opened": r["opened"],
    }))
}

/// The on/off settings by their `file.options` name.
fn flags() -> [(&'static str, fn(&mut Prefs) -> &mut bool); 9] {
    [
        ("typingReplacesSelection", |p| &mut p.typing_replaces_selection),
        ("insertKeyOvertype", |p| &mut p.insert_key_overtype),
        ("overtype", |p| &mut p.overtype),
        ("markGrammar", |p| &mut p.mark_grammar),
        ("ignoreUppercase", |p| &mut p.ignore_uppercase),
        ("ignoreNumbers", |p| &mut p.ignore_numbers),
        ("ignoreInternet", |p| &mut p.ignore_internet),
        ("logicalCursor", |p| &mut p.logical_cursor),
        ("rtlDefault", |p| &mut p.rtl_default),
    ]
}

fn mark_flags() -> [(&'static str, fn(&mut doc4me_layout::display::FormattingMarks) -> &mut bool); 4] {
    [("tabs", |m| &mut m.tabs), ("spaces", |m| &mut m.spaces), ("paragraphs", |m| &mut m.paragraphs), ("hidden", |m| &mut m.hidden)]
}

fn bool_param(v: &Value, k: &str) -> Result<Option<bool>, CmdError> {
    match v.get(k) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Bool(b)) => Ok(Some(*b)),
        Some(_) => Err(CmdError::Params(format!("`{k}` must be true or false"))),
    }
}

/// Words for the custom dictionary: trimmed, single words, at most [`MAX_WORD_CHARS`] long.
fn words_param(v: &Value, k: &str) -> Result<Vec<String>, CmdError> {
    let Some(x) = v.get(k).filter(|x| !x.is_null()) else { return Ok(Vec::new()) };
    let bad = || CmdError::Params(format!("`{k}` must be a list of words"));
    let list = x.as_array().ok_or_else(bad)?;
    if list.len() > MAX_WORDS_PER_CALL {
        return Err(CmdError::Params(format!("`{k}` takes at most {MAX_WORDS_PER_CALL} words")));
    }
    let mut out = Vec::new();
    for w in list {
        let w = w.as_str().ok_or_else(bad)?.trim();
        if w.is_empty() || w.chars().count() > MAX_WORD_CHARS || w.chars().any(|c| c.is_whitespace() || c.is_control()) {
            return Err(CmdError::Params(format!("`{k}`: {w:?} isn't a single word of at most {MAX_WORD_CHARS} characters")));
        }
        out.push(w.to_string());
    }
    Ok(out)
}

fn options(s: &mut Session, v: &Value) -> CmdResult {
    let pane = match p::str(v, "pane") {
        Some(name) if PANES.contains(&name) => name,
        Some(name) => return Err(CmdError::Params(format!("unknown pane `{name}`; one of {}", PANES.join(", ")))),
        None => "general",
    };
    // Read every setting first, so a bad one changes nothing.
    let author = match v.get("author") {
        None | Some(Value::Null) => None,
        Some(Value::String(n)) => Some(n.clone()),
        Some(_) => return Err(CmdError::Params("`author` must be a string".into())),
    };
    let initials = match v.get("initials") {
        None | Some(Value::Null) => None,
        Some(Value::String(i)) => Some(i.trim().chars().filter(|c| !c.is_control()).take(MAX_INITIALS_CHARS).collect::<String>()),
        Some(_) => return Err(CmdError::Params("`initials` must be a string".into())),
    };
    let units = match v.get("units") {
        None | Some(Value::Null) => None,
        Some(u) => Some(
            serde_json::from_value::<doc4me_geom::Unit>(u.clone())
                .map_err(|_| CmdError::Params("`units` is one of inches, centimeters, millimeters, points, picas".into()))?,
        ),
    };
    let mut flag_values = Vec::new();
    for (k, f) in flags() {
        if let Some(b) = bool_param(v, k)? {
            flag_values.push((f, b));
        }
    }
    let mut mark_values = Vec::new();
    match v.get("marks") {
        None | Some(Value::Null) => {}
        Some(m @ Value::Object(_)) => {
            for (k, f) in mark_flags() {
                if let Some(b) = bool_param(m, k)? {
                    mark_values.push((f, b));
                }
            }
        }
        Some(_) => return Err(CmdError::Params("`marks` must be an object of true/false".into())),
    }
    let check_spelling = bool_param(v, "checkSpelling")?;
    let mut paste_values = Vec::new();
    for (k, f) in paste_modes() {
        match v.get(k) {
            None | Some(Value::Null) => {}
            Some(m) => paste_values.push((
                f,
                serde_json::from_value::<crate::session::PasteMode>(m.clone())
                    .map_err(|_| CmdError::Params(format!("`{k}` is one of keep, merge, text")))?,
            )),
        }
    }
    // Writing Style › Settings: {rule: on/off}.
    let mut grammar = Vec::new();
    match v.get("grammarRules") {
        None | Some(Value::Null) => {}
        Some(m @ Value::Object(o)) => {
            if let Some(k) = o.keys().find(|k| !GRAMMAR_RULES.contains(&k.as_str())) {
                return Err(CmdError::Params(format!("unknown grammar rule `{k}`; one of {}", GRAMMAR_RULES.join(", "))));
            }
            for k in GRAMMAR_RULES {
                if let Some(b) = bool_param(m, k)? {
                    grammar.push((k, b));
                }
            }
        }
        Some(_) => return Err(CmdError::Params("`grammarRules` must be an object of true/false".into())),
    }
    let add = words_param(v, "addWords")?;
    let remove = words_param(v, "removeWords")?;

    let setting = author.is_some()
        || initials.is_some()
        || units.is_some()
        || !flag_values.is_empty()
        || !mark_values.is_empty()
        || check_spelling.is_some()
        || !paste_values.is_empty()
        || !grammar.is_empty()
        || !add.is_empty()
        || !remove.is_empty();
    if let Some(name) = author {
        super::file::set_author(s, &json!({"name": name}))?;
    }
    if let Some(i) = initials {
        s.initials = i;
    }
    if let Some(u) = units {
        s.prefs.units = u;
    }
    for (f, b) in flag_values {
        *f(&mut s.prefs) = b;
    }
    for (f, b) in mark_values {
        *f(&mut s.prefs.marks) = b;
    }
    for (f, m) in paste_values {
        *f(&mut s.prefs) = m;
    }
    for (k, on) in grammar {
        s.prefs.grammar_off.retain(|x| x != k);
        if !on {
            s.prefs.grammar_off.push(k.to_string());
        }
    }
    if let Some(b) = check_spelling {
        s.view.proofing = b;
    }
    for w in &add {
        doc4me_proof::add_word(w);
    }
    for w in &remove {
        doc4me_proof::remove_word(w);
    }
    if setting {
        // Marks, hidden text and proofing change what is laid out.
        s.relayout();
    } else {
        s.ui_requests.push(json!({"open": "options", "pane": pane}));
    }
    let mut r = current(s);
    if let Some(o) = r.as_object_mut() {
        o.insert("opened".into(), json!(!setting));
    }
    Ok(r)
}

/// The settings `file.options` reads and sets.
pub fn current(s: &Session) -> Value {
    let pr = &s.prefs;
    json!({
        "author": s.author,
        "initials": s.user_initials(),
        "units": pr.units,
        "typingReplacesSelection": pr.typing_replaces_selection,
        "insertKeyOvertype": pr.insert_key_overtype,
        "overtype": pr.overtype,
        "marks": pr.marks,
        "checkSpelling": s.view.proofing,
        "markGrammar": pr.mark_grammar,
        "ignoreUppercase": pr.ignore_uppercase,
        "ignoreNumbers": pr.ignore_numbers,
        "ignoreInternet": pr.ignore_internet,
        "pasteSame": pr.paste_same,
        "pasteBetween": pr.paste_between,
        "pasteOther": pr.paste_other,
        "logicalCursor": pr.logical_cursor,
        "rtlDefault": pr.rtl_default,
        "grammarRules": GRAMMAR_RULES.iter().map(|k| (k.to_string(), json!(!pr.grammar_off.iter().any(|x| x == k)))).collect::<serde_json::Map<_, _>>(),
        "dictionary": doc4me_proof::user_dictionary(),
    })
}

/// Grammar Settings: the rules Doc4Me's grammar check has, by their `grammarRules` name.
pub const GRAMMAR_RULES: [&str; 5] = ["capitalization", "repeated", "wordForms", "punctuation", "spacing"];

/// Cut, copy, and paste: the `file.options` names of the three paste defaults.
fn paste_modes() -> [(&'static str, fn(&mut Prefs) -> &mut crate::session::PasteMode); 3] {
    [("pasteSame", |p| &mut p.paste_same), ("pasteBetween", |p| &mut p.paste_between), ("pasteOther", |p| &mut p.paste_other)]
}

#[cfg(test)]
mod bidi_tests {
    use doc4me_doc::{Path, Pos, StoryRef};
    use serde_json::json;

    use crate::{Selection, Session};

    #[test]
    fn bidi_options_set_cursor_movement_and_the_default_direction() {
        let mut d = doc4me_doc::Document::from_text("שלום");
        if let Some(b) = d.body.first_mut()
            && let doc4me_doc::Block::Para(p) = std::sync::Arc::make_mut(b)
        {
            p.props.bidi = Some(true);
        }
        let mut s = Session::new(d);
        // Without settings it opens Options at Advanced.
        let r = s.run("file.bidiOptions", &json!({})).unwrap();
        assert_eq!((r["opened"].as_bool(), r["cursorMovement"].as_str()), (Some(true), Some("visual")));
        assert!(s.run("file.bidiOptions", &json!({"cursorMovement": "sideways"})).is_err());
        assert!(s.run("file.bidiOptions", &json!({"defaultDirection": "up"})).is_err());
        // Logical: Left goes on through a right-to-left paragraph, Right goes back.
        let r = s.run("file.bidiOptions", &json!({"cursorMovement": "logical"})).unwrap();
        assert_eq!((r["opened"].as_bool(), r["cursorMovement"].as_str()), (Some(false), Some("logical")));
        s.sel = Selection::caret(Pos::new(StoryRef::Body, Path::top(0), 0));
        s.run("caret.left", &json!({})).unwrap();
        assert_eq!(s.sel.focus.off, "ש".len(), "Left moves on in reading order");
        s.run("caret.right", &json!({})).unwrap();
        assert_eq!(s.sel.focus.off, 0);
        assert_eq!(s.run("file.options", &json!({})).unwrap()["logicalCursor"], json!(true));
        // Default direction: a new blank document reads right to left.
        s.run("file.bidiOptions", &json!({"defaultDirection": "rtl"})).unwrap();
        s.run("file.new", &json!({})).unwrap();
        assert_eq!(s.doc.styles.default_para.bidi, Some(true));
        s.run("file.bidiOptions", &json!({"defaultDirection": "ltr", "cursorMovement": "visual"})).unwrap();
        s.run("file.new", &json!({})).unwrap();
        assert_eq!(s.doc.styles.default_para.bidi, None);
        assert!(!s.prefs.logical_cursor);
    }

    #[test]
    fn file_locations_keep_existing_folders() {
        let mut s = Session::new(doc4me_doc::Document::new());
        let r = s.run("file.fileLocations", &json!({})).unwrap();
        assert_eq!((r["opened"].as_bool(), r["documents"].is_null()), (Some(true), true));
        let dir = std::env::temp_dir();
        let d = dir.to_string_lossy().to_string();
        let r = s.run("file.fileLocations", &json!({"documents": d, "pictures": d})).unwrap();
        assert_eq!((r["documents"].as_str(), r["opened"].as_bool()), (Some(d.as_str()), Some(false)));
        assert_eq!(s.prefs.pictures_dir.as_deref(), Some(d.as_str()));
        // Not a folder, not a full path, not text: nothing changes.
        let file = dir.join("doc4me-file-locations-test.txt");
        std::fs::write(&file, "x").unwrap();
        assert!(s.run("file.fileLocations", &json!({"documents": file.to_string_lossy()})).is_err());
        let _ = std::fs::remove_file(&file);
        assert!(s.run("file.fileLocations", &json!({"documents": "relative/folder"})).is_err());
        assert!(s.run("file.fileLocations", &json!({"pictures": 3})).is_err());
        assert!(s.run("file.fileLocations", &json!({"documents": dir.join("no-such-folder-doc4me").to_string_lossy()})).is_err());
        assert_eq!(s.prefs.documents_dir.as_deref(), Some(d.as_str()));
        // Null or blank: the system's choice again.
        s.run("file.fileLocations", &json!({"documents": null, "pictures": ""})).unwrap();
        assert!(s.prefs.documents_dir.is_none() && s.prefs.pictures_dir.is_none());
    }
}
