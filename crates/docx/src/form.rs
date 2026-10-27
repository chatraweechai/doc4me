//! Legacy form field data ↔ `w:ffData` (ECMA-376 §17.16.17), the child of a form field's
//! `w:fldChar w:fldCharType="begin"`. See [`doc4me_doc::form`].

use doc4me_doc::form::{FormField, FormHelp, FormKind, TextType};

use crate::xml::{El, W};

fn on(e: Option<&El>) -> Option<bool> {
    e.map(|e| e.attr("w:val").is_none_or(|v| !matches!(v, "0" | "false" | "off")))
}

fn help(e: Option<&El>) -> FormHelp {
    match e {
        Some(e) => FormHelp { auto_text: e.attr("w:type") == Some("autoText"), text: e.attr("w:val").unwrap_or("").to_string() },
        None => FormHelp::default(),
    }
}

fn u16_of(v: &str) -> Option<u16> {
    v.trim().parse::<i64>().ok().map(|n| n.clamp(0, i64::from(u16::MAX)) as u16)
}

/// The form field data in `w:ffData`, for a field whose code is `instr_word` (`FORMTEXT`,
/// `FORMCHECKBOX`, `FORMDROPDOWN`); `None` when the data is for another kind than the code
/// (the code decides what the field is).
pub fn read(ff: &El, instr_word: &str) -> Option<FormField> {
    let kind = match instr_word {
        "FORMTEXT" => {
            let t = ff.child("w:textInput");
            if t.is_none() && (ff.child("w:checkBox").is_some() || ff.child("w:ddList").is_some()) {
                return None;
            }
            FormKind::Text {
                input: t.and_then(|t| t.child_val("w:type")).map(TextType::from_ooxml).unwrap_or_default(),
                default: t.and_then(|t| t.child_val("w:default")).unwrap_or("").to_string(),
                max_len: t.and_then(|t| t.child_val("w:maxLength")).and_then(u16_of).unwrap_or(0),
                format: t.and_then(|t| t.child_val("w:format")).unwrap_or("").to_string(),
            }
        }
        "FORMCHECKBOX" => {
            let c = ff.child("w:checkBox")?;
            let size = match c.child("w:sizeAuto") {
                Some(_) => None,
                None => c.child_val("w:size").and_then(u16_of).filter(|h| *h > 0).map(|h| f32::from(h) / 2.0),
            };
            FormKind::CheckBox { size, default: on(c.child("w:default")).unwrap_or(false), checked: on(c.child("w:checked")) }
        }
        "FORMDROPDOWN" => {
            let d = ff.child("w:ddList")?;
            FormKind::DropDown {
                entries: d.children("w:listEntry").take(doc4me_doc::form::MAX_ENTRIES).filter_map(|e| e.attr("w:val")).map(str::to_string).collect(),
                default: d.child_val("w:default").and_then(u16_of).unwrap_or(0),
                result: d.child_val("w:result").and_then(u16_of),
            }
        }
        _ => return None,
    };
    let mut f = FormField {
        name: ff.child_val("w:name").unwrap_or("").to_string(),
        enabled: on(ff.child("w:enabled")).unwrap_or(true),
        calc_on_exit: on(ff.child("w:calcOnExit")).unwrap_or(false),
        entry_macro: ff.child_val("w:entryMacro").unwrap_or("").to_string(),
        exit_macro: ff.child_val("w:exitMacro").unwrap_or("").to_string(),
        help: help(ff.child("w:helpText")),
        status: help(ff.child("w:statusText")),
        kind,
    };
    f.clamp();
    Some(f)
}

/// `w:ffData` for a form field, in the schema's order (name, enabled, calcOnExit, entryMacro,
/// exitMacro, helpText, statusText, then the kind's own element), as Word writes it.
pub fn write(w: &mut W, f: &FormField) {
    w.open("w:ffData", &[]);
    w.val("w:name", &f.name);
    if f.enabled {
        w.empty("w:enabled", &[]);
    } else {
        w.val("w:enabled", "0");
    }
    w.val("w:calcOnExit", if f.calc_on_exit { "1" } else { "0" });
    if !f.entry_macro.is_empty() {
        w.val("w:entryMacro", &f.entry_macro);
    }
    if !f.exit_macro.is_empty() {
        w.val("w:exitMacro", &f.exit_macro);
    }
    for (tag, h) in [("w:helpText", &f.help), ("w:statusText", &f.status)] {
        if !h.is_empty() {
            w.empty(tag, &[("w:type", if h.auto_text { "autoText" } else { "text" }), ("w:val", &h.text)]);
        }
    }
    match &f.kind {
        FormKind::Text { input, default, max_len, format } => {
            w.open("w:textInput", &[]);
            if let Some(t) = input.ooxml() {
                w.val("w:type", t);
            }
            if !default.is_empty() {
                w.val("w:default", default);
            }
            if *max_len > 0 {
                w.val("w:maxLength", &max_len.to_string());
            }
            if !format.is_empty() {
                w.val("w:format", format);
            }
            w.close("w:textInput");
        }
        FormKind::CheckBox { size, default, checked } => {
            w.open("w:checkBox", &[]);
            match size {
                Some(pt) => w.val("w:size", &((pt * 2.0).round().clamp(2.0, 3276.0) as u32).to_string()),
                None => w.empty("w:sizeAuto", &[]),
            }
            w.val("w:default", if *default { "1" } else { "0" });
            if let Some(c) = checked {
                w.val("w:checked", if *c { "1" } else { "0" });
            }
            w.close("w:checkBox");
        }
        FormKind::DropDown { entries, default, result } => {
            w.open("w:ddList", &[]);
            if let Some(r) = result {
                w.val("w:result", &r.to_string());
            }
            if *default > 0 {
                w.val("w:default", &default.to_string());
            }
            for e in entries {
                w.val("w:listEntry", e);
            }
            w.close("w:ddList");
        }
    }
    w.close("w:ffData");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn el(xml: &str) -> El {
        let doc = format!(
            r#"<w:r xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:fldChar w:fldCharType="begin">{xml}</w:fldChar></w:r>"#
        );
        let r = crate::xml::parse(doc.as_bytes()).unwrap();
        r.child("w:fldChar").unwrap().child("w:ffData").unwrap().clone()
    }

    fn round(f: &FormField) -> FormField {
        let mut w = W::new();
        write(&mut w, f);
        let xml = String::from_utf8(w.into_bytes()).unwrap();
        let xml = xml.trim_start_matches(|c| c != '<');
        let xml = xml.find("<w:ffData").map(|i| &xml[i..]).unwrap();
        read(&el(xml), f.instr()).unwrap()
    }

    #[test]
    fn word_s_form_fields_read_and_write_back() {
        // As Word writes them (from a GOV.UK form in the corpus).
        let t = read(
            &el(r#"<w:ffData><w:name w:val="IADate"/><w:enabled/><w:calcOnExit w:val="0"/><w:helpText w:type="text" w:val="dd/mm/yyyy"/><w:textInput><w:type w:val="date"/><w:default w:val="01/01/2011"/><w:format w:val="DD/MM/YYYY"/></w:textInput></w:ffData>"#),
            "FORMTEXT",
        )
        .unwrap();
        assert_eq!((t.name.as_str(), t.enabled, t.help.text.as_str()), ("IADate", true, "dd/mm/yyyy"));
        assert!(
            matches!(&t.kind, FormKind::Text { input: TextType::Date, default, format, max_len: 0 } if default == "01/01/2011" && format == "DD/MM/YYYY")
        );
        assert_eq!(round(&t), t);
        let c = read(&el(r#"<w:ffData><w:name w:val="Check1"/><w:enabled w:val="0"/><w:calcOnExit w:val="0"/><w:checkBox><w:size w:val="24"/><w:default w:val="0"/><w:checked/></w:checkBox></w:ffData>"#), "FORMCHECKBOX").unwrap();
        assert_eq!((c.enabled, c.checked()), (false, Some(true)));
        assert!(matches!(c.kind, FormKind::CheckBox { size: Some(12.0), .. }));
        assert_eq!(round(&c), c);
        let d = read(
            &el(r#"<w:ffData><w:name w:val="IARPC"/><w:enabled/><w:calcOnExit w:val="1"/><w:statusText w:type="autoText" w:val="Hint"/><w:ddList><w:result w:val="2"/><w:listEntry w:val="RED"/><w:listEntry w:val="AMBER"/><w:listEntry w:val="GREEN"/></w:ddList></w:ffData>"#),
            "FORMDROPDOWN",
        )
        .unwrap();
        assert_eq!((d.calc_on_exit, d.status.auto_text, d.chosen_text()), (true, true, "GREEN"));
        assert_eq!(round(&d), d);
        // Data for another kind than the code is not taken; hostile values are bounded.
        assert!(read(&el(r#"<w:ffData><w:checkBox><w:sizeAuto/></w:checkBox></w:ffData>"#), "FORMTEXT").is_none());
        assert!(read(&el(r#"<w:ffData><w:textInput/></w:ffData>"#), "PAGE").is_none());
        let long = format!(
            r#"<w:ffData><w:name w:val="{}"/><w:ddList><w:result w:val="99999999"/>{}</w:ddList></w:ffData>"#,
            "n".repeat(500),
            r#"<w:listEntry w:val="e"/>"#.repeat(100)
        );
        let d = read(&el(&long), "FORMDROPDOWN").unwrap();
        assert_eq!(d.name.len(), doc4me_doc::form::MAX_NAME);
        assert_eq!(d.chosen(), Some(doc4me_doc::form::MAX_ENTRIES - 1));
    }
}
