//! Developer › Legacy Tools: legacy form fields (see [`doc4me_doc::form`]) — insert text, check
//! box and drop-down form fields, their Options and Help Text, fill them in (also when the
//! document is protected for filling in forms), go from one to the next, reset them, and Form
//! Field Shading.

use doc4me_doc::form::{self, BLANK_TEXT, FormField, FormHelp, FormKind, TextType};
use doc4me_doc::para::OBJ;
use doc4me_doc::{InlineObject, Pos, StoryRef};
use serde_json::{Value, json};

use super::{delete_selection, sel_result};
use crate::{CmdError, CmdResult, CommandSpec, Selection, Session, p};

const LOC: &str = "Developer › Controls › Legacy Tools";

/// The settings a form field takes (Insert Form Field, Form Field Options).
macro_rules! options {
    () => {
        r#""name"?: bookmark (up to 20 characters), "enabled"?: bool, "calcOnExit"?: bool, "entryMacro"?: string, "exitMacro"?: string, "textType"?: "regular|number|date|currentDate|currentTime|calculated", "default"?: string (text) | bool (check box) | index or entry (drop-down), "maxLength"?: 0-32767 (0: unlimited), "format"?: "0.00" | "M/d/yyyy" | "Uppercase|Lowercase|First capital|Title case", "size"?: points | null (auto; check box), "entries"?: [string] (drop-down, up to 25), "help"?: {"text", "autoText"?: bool} | null, "status"?: {"text", "autoText"?: bool} | null"#
    };
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("developer.formField", "Insert Form Field", LOC, insert)
            .params(concat!(r#"{"type": "text|checkBox|dropDown", "#, options!(), r#"} — at the caret, with a bookmark of its name (Text1, Check1, Dropdown1… when none is given)"#)),
        CommandSpec::new("developer.formFieldOptions", "Form Field Options", LOC, options).params(concat!(
            r#"{"field"?: bookmark name, "at"?: pos, "#,
            options!(),
            r#"} — the form field named, at `at`, or at the caret; returns it"#
        )),
        CommandSpec::new("developer.formFieldHelp", "Form Field Help Text", LOC, help_text).params(
            r#"{"field"?: bookmark name, "at"?: pos, "status"?: {"text", "autoText"?: bool} | null (status bar), "help"?: {"text", "autoText"?: bool} | null (F1)}"#,
        ),
        CommandSpec::new("developer.formFieldInfo", "Form Field at Selection", LOC, info).params(r#"{"field"?: bookmark name, "at"?: pos}"#).pure(),
        CommandSpec::new("developer.formFieldValue", "Fill In Form Field", LOC, set_value).params(
            r#"{"field"?: bookmark name, "at"?: pos, "value": string (text) | bool (check box) | index or entry (drop-down)} — what filling in the form does (allowed when the document is protected for filling in forms)"#,
        ),
        CommandSpec::new("developer.nextFormField", "Next Form Field", LOC, next)
            .params(r#"{"back"?: bool} — selects the next (previous) enabled form field (Tab and Shift+Tab in a protected form)"#)
            .pure(),
        CommandSpec::new("developer.resetFormFields", "Reset Form Fields", LOC, reset_all),
        CommandSpec::new("developer.formFieldShading", "Form Field Shading", LOC, |s, v| {
            s.view.form_shading = p::bool(v, "value").unwrap_or(!s.view.form_shading);
            s.relayout();
            Ok(json!({"value": s.view.form_shading}))
        })
        .params(r#"{"value"?: bool}"#)
        .pure(),
    ]
}

// ---------------------------------------------------------------------------------------------
// Finding form fields

/// Every form field in the document: the body first, then headers, footers, notes and boxes.
pub fn all(s: &Session) -> Vec<Pos> {
    let mut out = Vec::new();
    let mut parts: Vec<u32> = s.doc.parts.keys().copied().collect();
    parts.sort_unstable();
    for story in std::iter::once(StoryRef::Body).chain(parts.into_iter().map(StoryRef::Part)) {
        for path in s.doc.para_paths(story) {
            let Some(para) = s.doc.para(story, &path) else { continue };
            for off in para.object_offsets() {
                if matches!(para.object_at(off), Some(InlineObject::Field { form: Some(_), .. })) {
                    out.push(Pos { story, path: path.clone(), off });
                }
            }
        }
    }
    out
}

/// The form field object at `at`.
pub fn field_at(s: &Session, at: &Pos) -> Option<(FormField, String)> {
    match s.doc.para_at(at)?.object_at(at.off)? {
        InlineObject::Field { form: Some(f), result, .. } => Some(((**f).clone(), result.clone())),
        _ => None,
    }
}

/// The form field at `p` or just before it (a caret after the field, a click beside it).
pub fn near(s: &Session, at: &Pos) -> Option<Pos> {
    if field_at(s, at).is_some() {
        return Some(at.clone());
    }
    let w = OBJ.len_utf8();
    let before = Pos { off: at.off.checked_sub(w)?, ..at.clone() };
    field_at(s, &before).map(|_| before)
}

/// The form field a command acts on: `field` (its bookmark name), else the one at `at`, else
/// the one at the selection.
fn target(s: &Session, v: &Value) -> Result<Pos, CmdError> {
    if let Some(name) = p::str(v, "field") {
        return all(s)
            .into_iter()
            .find(|at| field_at(s, at).is_some_and(|(f, _)| f.name.eq_ignore_ascii_case(name)))
            .ok_or_else(|| CmdError::Failed(format!("no form field is called {name:?}")));
    }
    if let Some(at) = v.get("at").and_then(super::parse_pos) {
        return near(s, &at).ok_or_else(|| CmdError::Disabled("there's no form field there".into()));
    }
    let (a, b) = s.sel.ordered();
    near(s, &a).or_else(|| (a != b).then(|| near(s, &b)).flatten()).ok_or_else(|| CmdError::Disabled("the selection isn't on a form field".into()))
}

/// Replace the form field at `at` (its data and result).
fn put(s: &mut Session, at: &Pos, f: FormField, result: String) -> Result<(), CmdError> {
    let para = s.doc.para_mut(at.story, &at.path)?;
    match para.object_at_mut(at.off) {
        Some(InlineObject::Field { form, result: r, .. }) => {
            *form = Some(Box::new(f));
            *r = result;
            Ok(())
        }
        _ => Err(CmdError::Failed("the form field moved".into())),
    }
}

/// What a command or dialog needs to know about a form field.
pub fn describe(at: &Pos, f: &FormField, result: &str) -> Value {
    let help = |h: &FormHelp| if h.is_empty() { Value::Null } else { json!({"text": h.text, "autoText": h.auto_text}) };
    let mut out = json!({
        "type": f.kind_name(),
        "name": f.name,
        "enabled": f.enabled,
        "calcOnExit": f.calc_on_exit,
        "entryMacro": f.entry_macro,
        "exitMacro": f.exit_macro,
        "help": help(&f.help),
        "status": help(&f.status),
        "at": super::pos_json(at),
    });
    match &f.kind {
        FormKind::Text { input, default, max_len, format } => {
            out["textType"] = json!(input.name());
            out["default"] = json!(default);
            out["maxLength"] = json!(max_len);
            out["format"] = json!(format);
            out["value"] = json!(if result == BLANK_TEXT { "" } else { result });
        }
        FormKind::CheckBox { size, default, .. } => {
            out["size"] = json!(size);
            out["default"] = json!(default);
            out["value"] = json!(f.checked() == Some(true));
        }
        FormKind::DropDown { entries, default, .. } => {
            out["entries"] = json!(entries);
            out["default"] = json!(default);
            out["value"] = json!(f.chosen());
            out["text"] = json!(f.chosen_text());
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Options

fn help_param(v: &Value, k: &str) -> Option<FormHelp> {
    match v.get(k)? {
        Value::Null => Some(FormHelp::default()),
        Value::String(t) => Some(FormHelp { auto_text: false, text: t.clone() }),
        h => Some(FormHelp {
            auto_text: h.get("autoText").and_then(Value::as_bool).unwrap_or(false),
            text: h.get("text").and_then(Value::as_str).unwrap_or("").to_string(),
        }),
    }
}

fn clean(t: &str) -> String {
    t.chars().filter(|c| !c.is_control() && *c != OBJ).collect()
}

/// Apply the settings in `v` to `f`; returns whether the shown value should follow a new default.
fn apply_options(f: &mut FormField, v: &Value) -> Result<bool, CmdError> {
    if let Some(n) = p::str(v, "name") {
        let n = n.trim();
        if n.chars().count() > form::MAX_NAME || n.chars().next().is_some_and(|c| c.is_ascii_digit()) || n.contains(char::is_whitespace) {
            return Err(CmdError::Params("a form field's bookmark name has up to 20 characters, no spaces, and doesn't start with a digit".into()));
        }
        f.name = clean(n);
    }
    if let Some(b) = p::bool(v, "enabled") {
        f.enabled = b;
    }
    if let Some(b) = p::bool(v, "calcOnExit") {
        f.calc_on_exit = b;
    }
    if let Some(m) = p::str(v, "entryMacro") {
        f.entry_macro = clean(m);
    }
    if let Some(m) = p::str(v, "exitMacro") {
        f.exit_macro = clean(m);
    }
    if let Some(h) = help_param(v, "help") {
        f.help = h;
    }
    if let Some(h) = help_param(v, "status") {
        f.status = h;
    }
    let mut follow = false;
    match &mut f.kind {
        FormKind::Text { input, default, max_len, format } => {
            if let Some(t) = p::str(v, "textType") {
                *input = TextType::from_name(t).ok_or_else(|| CmdError::Params(format!("unknown textType {t:?}")))?;
            }
            if let Some(n) = v.get("maxLength") {
                let n = n.as_u64().filter(|n| *n <= 32767).ok_or_else(|| CmdError::Params("maxLength: 0 (unlimited) to 32767".into()))?;
                *max_len = n as u16;
            }
            if let Some(t) = p::str(v, "format") {
                *format = clean(t.trim());
            }
            if let Some(d) = v.get("default") {
                let d = d.as_str().ok_or_else(|| CmdError::Params("a text form field's default is text".into()))?;
                *default = clean(d);
                follow = true;
            }
            if *max_len > 0 && default.chars().count() > usize::from(*max_len) {
                return Err(CmdError::Params(format!("the default text is longer than the maximum length ({max_len})")));
            }
        }
        FormKind::CheckBox { size, default, .. } => {
            match v.get("size") {
                Some(Value::Null) => *size = None,
                Some(n) => {
                    *size = Some(
                        n.as_f64()
                            .filter(|n| (1.0..=1638.0).contains(n))
                            .ok_or_else(|| CmdError::Params("size: 1–1638 points, or null for auto".into()))? as f32,
                    )
                }
                None => {}
            }
            if let Some(d) = v.get("default") {
                *default = d.as_bool().ok_or_else(|| CmdError::Params("a check box's default is true or false".into()))?;
            }
        }
        FormKind::DropDown { entries, default, result } => {
            if let Some(list) = v.get("entries") {
                let list = list.as_array().ok_or_else(|| CmdError::Params("entries: [string]".into()))?;
                if list.len() > form::MAX_ENTRIES {
                    return Err(CmdError::Params("a drop-down form field has up to 25 entries".into()));
                }
                let mut new = Vec::new();
                for e in list {
                    let e = clean(e.as_str().ok_or_else(|| CmdError::Params("entries: [string]".into()))?.trim());
                    if e.is_empty() || e.chars().count() > form::MAX_ENTRY {
                        return Err(CmdError::Params("each drop-down entry has 1–50 characters".into()));
                    }
                    new.push(e);
                }
                *entries = new;
                *default = 0;
                *result = None;
                follow = true;
            }
            if let Some(d) = v.get("default") {
                *default = entry_index(entries, d)?;
                *result = None;
                follow = true;
            }
        }
    }
    f.clamp();
    Ok(follow)
}

/// An entry given by its index or its text.
fn entry_index(entries: &[String], v: &Value) -> Result<u16, CmdError> {
    let i = match v {
        Value::Number(n) => n.as_u64().map(|n| n as usize),
        Value::String(t) => entries.iter().position(|e| e == t),
        _ => None,
    };
    i.filter(|i| *i < entries.len()).map(|i| i as u16).ok_or_else(|| CmdError::Params("no such entry in the list".into()))
}

/// The start and end of the bookmark called `name` right around the field at `at` (only markers
/// between them and the field), as Word writes a form field's bookmark. Others of the same name
/// (a copied field keeps its name) are left alone.
fn own_bookmark(para: &doc4me_doc::Paragraph, at: usize, name: &str) -> Option<(usize, usize)> {
    let offs = para.object_offsets();
    let i = offs.iter().position(|o| *o == at)?;
    let w = OBJ.len_utf8();
    let is = |off: usize, start: bool| match para.object_at(off) {
        Some(InlineObject::BookmarkStart { name: n }) if start => n == name,
        Some(InlineObject::BookmarkEnd { name: n }) if !start => n == name,
        _ => false,
    };
    // Markers only, each just before the next.
    let mut start = None;
    let mut next = at;
    for &off in offs[..i].iter().rev() {
        if off + w != next || !para.object_at(off).is_some_and(InlineObject::is_marker) {
            break;
        }
        if is(off, true) {
            start = Some(off);
            break;
        }
        next = off;
    }
    let mut end = None;
    let mut prev = at;
    for &off in &offs[i + 1..] {
        if off != prev + w || !para.object_at(off).is_some_and(InlineObject::is_marker) {
            break;
        }
        if is(off, false) {
            end = Some(off);
            break;
        }
        prev = off;
    }
    Some((start?, end?))
}

/// The bookmark of a form field's name around it, renamed with it (or added, or removed);
/// returns where the field is then.
fn rename_bookmark(s: &mut Session, at: &Pos, old: &str, new: &str) -> Result<Pos, CmdError> {
    let w = OBJ.len_utf8();
    if old == new {
        return Ok(at.clone());
    }
    let para = s.doc.para_mut(at.story, &at.path)?;
    let own = if old.is_empty() { None } else { own_bookmark(para, at.off, old) };
    if let Some((a, b)) = own {
        if new.is_empty() {
            // No name: its bookmark goes (the start was before the field).
            para.delete(b, b + w)?;
            para.delete(a, a + w)?;
            return Ok(Pos { off: at.off - w, ..at.clone() });
        }
        for off in [a, b] {
            if let Some(InlineObject::BookmarkStart { name } | InlineObject::BookmarkEnd { name }) = para.object_at_mut(off) {
                *name = new.to_string();
            }
        }
        return Ok(at.clone());
    }
    if new.is_empty() {
        return Ok(at.clone());
    }
    s.doc.insert_object(&Pos { off: at.off + w, ..at.clone() }, InlineObject::BookmarkEnd { name: new.to_string() }, &Default::default())?;
    s.doc.insert_object(at, InlineObject::BookmarkStart { name: new.to_string() }, &Default::default())?;
    Ok(Pos { off: at.off + w, ..at.clone() })
}

fn options(s: &mut Session, v: &Value) -> CmdResult {
    let at = target(s, v)?;
    let (mut f, mut result) = field_at(s, &at).ok_or_else(|| CmdError::Failed("no form field".into()))?;
    let old_name = f.name.clone();
    if let Some(n) = p::str(v, "name")
        && !n.trim().is_empty()
        && !n.trim().eq_ignore_ascii_case(&old_name)
        && taken(s, n.trim())
    {
        return Err(CmdError::Params(format!("there's already a bookmark called {:?}", n.trim())));
    }
    let follow = apply_options(&mut f, v)?;
    if follow {
        result = f.reset();
    }
    if let FormKind::Text { input: TextType::CurrentDate | TextType::CurrentTime, .. } = f.kind {
        result = now_text(&f);
    }
    let new_name = f.name.clone();
    put(s, &at, f, result)?;
    let at = rename_bookmark(s, &at, &old_name, &new_name)?;
    let (f, result) = field_at(s, &at).ok_or_else(|| CmdError::Failed("no form field".into()))?;
    Ok(describe(&at, &f, &result))
}

fn help_text(s: &mut Session, v: &Value) -> CmdResult {
    let at = target(s, v)?;
    let (mut f, result) = field_at(s, &at).ok_or_else(|| CmdError::Failed("no form field".into()))?;
    if let Some(h) = help_param(v, "help") {
        f.help = h;
    }
    if let Some(h) = help_param(v, "status") {
        f.status = h;
    }
    f.clamp();
    put(s, &at, f.clone(), result.clone())?;
    Ok(describe(&at, &f, &result))
}

fn info(s: &mut Session, v: &Value) -> CmdResult {
    let at = target(s, v)?;
    let (f, result) = field_at(s, &at).ok_or_else(|| CmdError::Failed("no form field".into()))?;
    Ok(describe(&at, &f, &result))
}

// ---------------------------------------------------------------------------------------------
// Insert

/// Is `name` a bookmark of the document already?
fn taken(s: &Session, name: &str) -> bool {
    s.doc.bookmarks().iter().any(|(n, _)| n.eq_ignore_ascii_case(name))
}

/// Word's next free name for a new form field: Text1, Check2, Dropdown3…
fn next_name(s: &Session, stem: &str) -> String {
    (1..).map(|i| format!("{stem}{i}")).find(|n| !taken(s, n)).unwrap_or_default()
}

/// A current date or time field's text, in its format (Word's short date or time without one).
fn now_text(f: &FormField) -> String {
    let FormKind::Text { input, format, .. } = &f.kind else { return String::new() };
    let pic = match (input, format.is_empty()) {
        (_, false) => word_picture(format),
        (TextType::CurrentTime, true) => "h:mm am/pm".to_string(),
        _ => "M/d/yyyy".to_string(),
    };
    super::insert::format_date(&pic)
}

/// Word's date pictures take `D` and `Y` for `d` and `y`.
fn word_picture(fmt: &str) -> String {
    fmt.chars()
        .map(|c| {
            if c == 'D' {
                'd'
            } else if c == 'Y' {
                'y'
            } else {
                c
            }
        })
        .collect()
}

fn insert(s: &mut Session, v: &Value) -> CmdResult {
    let ty = p::req_str(v, "type")?;
    let (mut f, stem) = match ty {
        "text" => (FormField::text(), "Text"),
        "checkBox" => (FormField::check_box(), "Check"),
        "dropDown" => (FormField::drop_down(), "Dropdown"),
        _ => return Err(CmdError::Params("type: text, checkBox or dropDown".into())),
    };
    f.name = next_name(s, stem);
    if let Some(n) = p::str(v, "name").map(str::trim).filter(|n| !n.is_empty())
        && taken(s, n)
    {
        return Err(CmdError::Params(format!("there's already a bookmark called {n:?}")));
    }
    apply_options(&mut f, v)?;
    let result = match f.kind {
        FormKind::Text { input: TextType::CurrentDate | TextType::CurrentTime, .. } => now_text(&f),
        _ => f.reset(),
    };
    let props = s.typing_props();
    let at = delete_selection(s)?;
    let at = s.doc.typing_pos(&at);
    let name = f.name.clone();
    let instr = f.instr().to_string();
    let mut pos = at.clone();
    if !name.is_empty() {
        pos = s.doc.insert_object(&pos, InlineObject::BookmarkStart { name: name.clone() }, &Default::default())?;
    }
    let field_at = pos.clone();
    pos = s.doc.insert_object(&pos, InlineObject::Field { instr, result, locked: false, code: false, form: Some(Box::new(f)) }, &props)?;
    if !name.is_empty() {
        pos = s.doc.insert_object(&pos, InlineObject::BookmarkEnd { name }, &Default::default())?;
    }
    s.sel = Selection::caret(pos);
    s.goal_x = None;
    let (f, result) = field_at_or(s, &field_at)?;
    let mut out = describe(&field_at, &f, &result);
    out["selection"] = sel_result(s)?;
    Ok(out)
}

fn field_at_or(s: &Session, at: &Pos) -> Result<(FormField, String), CmdError> {
    field_at(s, at).ok_or_else(|| CmdError::Failed("no form field".into()))
}

// ---------------------------------------------------------------------------------------------
// Filling in

/// The value a text form field keeps for what was typed: checked against its type, cut to
/// nothing longer than its maximum, in its format.
pub fn text_value(f: &FormField, typed: &str) -> Result<String, CmdError> {
    let FormKind::Text { input, max_len, format, .. } = &f.kind else { return Err(CmdError::Params("not a text form field".into())) };
    let typed: String = typed.chars().filter(|c| !c.is_control() || *c == '\t').filter(|c| *c != OBJ).collect();
    if !input.fillable() {
        return Err(CmdError::Disabled("this form field fills itself in".into()));
    }
    let out = match input {
        _ if typed.trim().is_empty() => String::new(),
        TextType::Number => {
            let n = parse_number(&typed).ok_or_else(|| CmdError::Params(format!("{:?} isn't a valid number", typed.trim())))?;
            if format.is_empty() { typed.trim().to_string() } else { form::format_number(n, format) }
        }
        TextType::Date => {
            let (y, m, d) = parse_date(&typed, format).ok_or_else(|| CmdError::Params(format!("{:?} isn't a valid date", typed.trim())))?;
            if format.is_empty() {
                typed.trim().to_string()
            } else {
                super::insert::format_date_parts(&word_picture(format), y, m as usize, d, (0, 0, 0), super::controls::days_from_civil(y, m, d))
            }
        }
        _ => form::apply_text_case(format, &typed),
    };
    if *max_len > 0 && out.chars().count() > usize::from(*max_len) {
        return Err(CmdError::Params(format!("this form field takes at most {max_len} characters")));
    }
    Ok(if out.is_empty() { BLANK_TEXT.to_string() } else { out })
}

/// A number as typed: digits with a decimal point, thousands commas, a currency sign, a minus
/// or parentheses for negatives, a trailing percent sign.
fn parse_number(t: &str) -> Option<f64> {
    let t = t.trim();
    let (neg, t) = match t.strip_prefix('(').and_then(|x| x.strip_suffix(')')) {
        Some(inner) => (true, inner),
        None => (false, t),
    };
    let pct = t.ends_with('%');
    let clean: String = t.chars().filter(|c| !matches!(c, ',' | ' ' | '$' | '£' | '€' | '¥' | '%' | '\u{A0}')).collect();
    let n: f64 = clean.parse().ok().filter(|n: &f64| n.is_finite())?;
    let n = if pct { n / 100.0 } else { n };
    Some(if neg { -n } else { n })
}

/// A date as typed: `YYYY-MM-DD`, numbers in the order of the field's format (day first when
/// the format starts with the day, else month first), or with a month name.
fn parse_date(t: &str, format: &str) -> Option<(i64, u32, u32)> {
    let t = t.trim();
    if let Some(d) = super::controls::parse_date(t) {
        return Some(d);
    }
    const MONTHS: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
    let words: Vec<String> = t.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_ascii_lowercase).collect();
    let month_word = words.iter().position(|w| w.len() >= 3 && MONTHS.iter().any(|m| w.starts_with(m)));
    let nums: Vec<i64> = words.iter().filter_map(|w| w.parse().ok()).collect();
    let (y, m, d) = match month_word {
        Some(i) => {
            let m = MONTHS.iter().position(|mm| words[i].starts_with(mm))? as u32 + 1;
            let (a, b) = (*nums.first()?, *nums.get(1)?);
            if a > 31 { (a, m, b as u32) } else { (b, m, a as u32) }
        }
        None if nums.len() == 3 => {
            let day_first = format.trim_start().starts_with(['d', 'D']);
            if nums[0] > 31 {
                (nums[0], nums[1] as u32, nums[2] as u32)
            } else if day_first {
                (nums[2], nums[1] as u32, nums[0] as u32)
            } else {
                (nums[2], nums[0] as u32, nums[1] as u32)
            }
        }
        None => return None,
    };
    let y = if (0..100).contains(&y) { 2000 + y } else { y };
    super::controls::parse_date(&format!("{y:04}-{m:02}-{d:02}"))
}

fn set_value(s: &mut Session, v: &Value) -> CmdResult {
    let at = target(s, v)?;
    let (mut f, mut result) = field_at_or(s, &at)?;
    if !f.enabled && s.doc.settings.protection.is_some() {
        return Err(CmdError::Disabled("this form field can't be filled in".into()));
    }
    let val = v.get("value").ok_or_else(|| CmdError::Params("value".into()))?;
    match &mut f.kind {
        FormKind::Text { .. } => {
            let t = val.as_str().ok_or_else(|| CmdError::Params("a text form field's value is text".into()))?;
            result = text_value(&f, t)?;
        }
        FormKind::CheckBox { default, checked, .. } => {
            let b = val.as_bool().ok_or_else(|| CmdError::Params("a check box's value is true or false".into()))?;
            *checked = (b != *default).then_some(b);
        }
        FormKind::DropDown { entries, default, result: r } => {
            let i = entry_index(entries, val)?;
            *r = (i != *default).then_some(i);
            result = entries[usize::from(i)].clone();
        }
    }
    put(s, &at, f.clone(), result.clone())?;
    let w = OBJ.len_utf8();
    s.sel = Selection { anchor: at.clone(), focus: Pos { off: at.off + w, ..at.clone() } };
    Ok(describe(&at, &f, &result))
}

pub(crate) fn next(s: &mut Session, v: &Value) -> CmdResult {
    let back = p::bool(v, "back").unwrap_or(false);
    let fields: Vec<Pos> = all(s).into_iter().filter(|at| field_at(s, at).is_some_and(|(f, _)| f.enabled)).collect();
    if fields.is_empty() {
        return Err(CmdError::Disabled("the document has no form fields to fill in".into()));
    }
    let (a, b) = s.sel.ordered();
    let at = if back { fields.iter().rev().find(|p| **p < a).or(fields.last()) } else { fields.iter().find(|p| **p >= b).or(fields.first()) };
    let at = at.cloned().unwrap_or_else(|| fields[0].clone());
    let w = OBJ.len_utf8();
    s.sel = Selection { anchor: at.clone(), focus: Pos { off: at.off + w, ..at.clone() } };
    s.goal_x = None;
    let (f, result) = field_at_or(s, &at)?;
    let mut out = describe(&at, &f, &result);
    out["statusText"] = json!(f.status.text);
    Ok(out)
}

fn reset_all(s: &mut Session, _v: &Value) -> CmdResult {
    let mut n = 0;
    for at in all(s) {
        let Some((mut f, _)) = field_at(s, &at) else { continue };
        let result = match f.kind {
            FormKind::Text { input: TextType::CurrentDate | TextType::CurrentTime, .. } => now_text(&f),
            _ => f.reset(),
        };
        put(s, &at, f, result)?;
        n += 1;
    }
    Ok(json!({"reset": n}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_values_follow_the_field() {
        let mut f = FormField::text();
        assert_eq!(text_value(&f, "").unwrap(), BLANK_TEXT);
        f.kind = FormKind::Text { input: TextType::Number, default: String::new(), max_len: 12, format: "#,##0.00".into() };
        assert_eq!(text_value(&f, "1234.5").unwrap(), "1,234.50");
        assert_eq!(text_value(&f, "($12)").unwrap(), "-12.00");
        assert!(text_value(&f, "twelve").is_err());
        assert!(text_value(&f, "123456789012").is_err(), "longer than the maximum once formatted");
        f.kind = FormKind::Text { input: TextType::Date, default: String::new(), max_len: 0, format: "DD/MM/YYYY".into() };
        assert_eq!(text_value(&f, "3/4/2026").unwrap(), "03/04/2026", "day first, as the format");
        assert_eq!(text_value(&f, "2026-10-27").unwrap(), "27/10/2026");
        assert_eq!(text_value(&f, "27 October 2026").unwrap(), "27/10/2026");
        assert!(text_value(&f, "31/02/2026").is_err());
        f.kind = FormKind::Text { input: TextType::Regular, default: String::new(), max_len: 3, format: "Uppercase".into() };
        assert_eq!(text_value(&f, "abc").unwrap(), "ABC");
        assert!(text_value(&f, "abcd").is_err());
        f.kind = FormKind::Text { input: TextType::CurrentDate, default: String::new(), max_len: 0, format: String::new() };
        assert!(text_value(&f, "x").is_err());
    }
}
