//! Legacy form fields through the commands: insert, Options, Help Text, filling in (also in a
//! document protected for filling in forms), Tab between fields, reset, and the file.

use doc4me_doc::form::{BLANK_TEXT, FormKind, TextType};
use doc4me_doc::{InlineObject, StoryRef};
use serde_json::json;

use crate::Session;

fn run(s: &mut Session, id: &str, v: serde_json::Value) -> serde_json::Value {
    s.run(id, &v).unwrap_or_else(|e| panic!("{id}: {e}"))
}

fn all_forms(s: &Session) -> Vec<doc4me_doc::Pos> {
    crate::cmd::forms::all(s)
}

fn text(s: &Session) -> String {
    s.doc.plain_text(StoryRef::Body)
}

/// "Name: [Text1] Agree: [Check1] Size: [Dropdown1]".
fn form() -> Session {
    let mut s = Session::new(doc4me_doc::Document::new());
    run(&mut s, "text.insert", json!({"text": "Name: "}));
    run(&mut s, "developer.formField", json!({"type": "text", "maxLength": 10, "format": "Title case"}));
    run(&mut s, "text.insert", json!({"text": " Agree: "}));
    run(&mut s, "developer.formField", json!({"type": "checkBox"}));
    run(&mut s, "text.insert", json!({"text": " Size: "}));
    run(&mut s, "developer.formField", json!({"type": "dropDown", "entries": ["Small", "Medium", "Large"], "default": "Medium"}));
    s
}

#[test]
fn inserted_form_fields_have_word_s_names_bookmarks_and_defaults() {
    let mut s = form();
    // A blank text field shows five en spaces; the drop-down its default entry.
    assert_eq!(text(&s), format!("Name: {BLANK_TEXT} Agree:  Size: Medium"));
    let names: Vec<String> = s.doc.bookmarks().into_iter().map(|(n, _)| n).collect();
    assert_eq!(names, ["Text1", "Check1", "Dropdown1"]);
    let t = run(&mut s, "developer.formFieldInfo", json!({"field": "Text1"}));
    assert_eq!(
        (t["type"].as_str(), t["maxLength"].as_u64(), t["format"].as_str(), t["value"].as_str()),
        (Some("text"), Some(10), Some("Title case"), Some(""))
    );
    let d = run(&mut s, "developer.formFieldInfo", json!({"field": "dropdown1"}));
    assert_eq!((d["value"].as_u64(), d["text"].as_str()), (Some(1), Some("Medium")));
    // A second text field is Text2; a name already used is refused.
    run(&mut s, "developer.formField", json!({"type": "text"}));
    assert!(s.doc.bookmarks().iter().any(|(n, _)| n == "Text2"));
    assert!(s.run("developer.formField", &json!({"type": "text", "name": "Check1"})).is_err());
    assert!(s.run("developer.formField", &json!({"type": "radio"})).is_err());
}

#[test]
fn options_and_help_text_change_the_field_and_its_bookmark() {
    let mut s = form();
    let t = run(
        &mut s,
        "developer.formFieldOptions",
        json!({"field": "Text1", "name": "FullName", "default": "Ada", "textType": "regular", "help": {"text": "Your full name"}, "calcOnExit": true}),
    );
    assert_eq!((t["name"].as_str(), t["value"].as_str(), t["calcOnExit"].as_bool()), (Some("FullName"), Some("Ada"), Some(true)));
    assert!(text(&s).starts_with("Name: Ada Agree"), "the new default shows");
    assert!(s.doc.bookmarks().iter().any(|(n, _)| n == "FullName"));
    assert!(!s.doc.bookmarks().iter().any(|(n, _)| n == "Text1"));
    let h = run(&mut s, "developer.formFieldHelp", json!({"field": "FullName", "status": {"text": "Type your name"}}));
    assert_eq!((h["status"]["text"].as_str(), h["help"]["text"].as_str()), (Some("Type your name"), Some("Your full name")));
    run(&mut s, "developer.formFieldHelp", json!({"field": "FullName", "help": null}));
    assert!(run(&mut s, "developer.formFieldInfo", json!({"field": "FullName"}))["help"].is_null());
    // A pasted copy keeps the field's name but not its bookmark (bookmark names are unique):
    // renaming the copy gives it a bookmark of its own and leaves the original's alone; taking a
    // name away takes its bookmark away.
    run(&mut s, "developer.formField", json!({"type": "checkBox", "name": "Twin"}));
    run(&mut s, "developer.formField", json!({"type": "checkBox", "name": ""}));
    let copy_at = all_forms(&s).into_iter().last().unwrap();
    if let Some(InlineObject::Field { form: Some(f), .. }) = s.doc.para_mut(copy_at.story, &copy_at.path).unwrap().object_at_mut(copy_at.off) {
        f.name = "Twin".into();
    }
    run(&mut s, "developer.formFieldOptions", json!({"at": copy_at, "name": "Copy"}));
    let count = |s: &Session, n: &str| s.doc.bookmarks().into_iter().filter(|(b, _)| b == n).count();
    assert_eq!((count(&s, "Twin"), count(&s, "Copy")), (1, 1));
    run(&mut s, "developer.formFieldOptions", json!({"field": "Copy", "name": ""}));
    assert_eq!((count(&s, "Twin"), count(&s, "Copy")), (1, 0));
    // Check box size and default; drop-down entries.
    let c = run(&mut s, "developer.formFieldOptions", json!({"field": "Check1", "size": 14, "default": true}));
    assert_eq!((c["size"].as_f64(), c["value"].as_bool()), (Some(14.0), Some(true)));
    let d = run(&mut s, "developer.formFieldOptions", json!({"field": "Dropdown1", "entries": ["Red", "Green"]}));
    assert_eq!(d["text"], "Red");
    assert!(text(&s).ends_with("Size: Red"));
    // Bad settings are refused and change nothing.
    let before = s.doc.clone();
    for bad in [
        json!({"field": "FullName", "maxLength": 2}),
        json!({"field": "FullName", "textType": "money"}),
        json!({"field": "FullName", "name": "has space"}),
        json!({"field": "Check1", "size": 0}),
        json!({"field": "Dropdown1", "entries": [""]}),
        json!({"field": "Dropdown1", "default": "Blue"}),
    ] {
        assert!(s.run("developer.formFieldOptions", &bad).is_err(), "{bad}");
    }
    assert_eq!(s.doc, before);
}

#[test]
fn filling_in_checks_types_and_follows_formats() {
    let mut s = form();
    run(&mut s, "developer.formFieldValue", json!({"field": "Text1", "value": "ada byron"}));
    assert!(text(&s).starts_with("Name: Ada Byron Agree"));
    assert!(s.run("developer.formFieldValue", &json!({"field": "Text1", "value": "far too long a name"})).is_err());
    run(&mut s, "developer.formFieldValue", json!({"field": "Check1", "value": true}));
    assert!(text(&s).contains("Agree:  Size"), "a check box adds no text");
    assert_eq!(run(&mut s, "developer.formFieldInfo", json!({"field": "Check1"}))["value"], true);
    run(&mut s, "developer.formFieldValue", json!({"field": "Dropdown1", "value": 2}));
    assert!(text(&s).ends_with("Size: Large"));
    assert!(s.run("developer.formFieldValue", &json!({"field": "Dropdown1", "value": "Huge"})).is_err());
    // A number field formats what's typed.
    run(&mut s, "developer.formFieldOptions", json!({"field": "Text1", "textType": "number", "format": "#,##0.00", "maxLength": 0}));
    run(&mut s, "developer.formFieldValue", json!({"field": "Text1", "value": "1234.5"}));
    assert!(text(&s).starts_with("Name: 1,234.50 "));
    assert!(s.run("developer.formFieldValue", &json!({"field": "Text1", "value": "lots"})).is_err());
    // Reset Form Fields brings back the defaults.
    assert_eq!(run(&mut s, "developer.resetFormFields", json!({}))["reset"], 3);
    assert_eq!(text(&s), format!("Name: {BLANK_TEXT} Agree:  Size: Medium"));
    assert_eq!(run(&mut s, "developer.formFieldInfo", json!({"field": "Check1"}))["value"], false);
}

#[test]
fn a_form_protected_document_only_lets_its_fields_change() {
    let mut s = form();
    run(&mut s, "text.newParagraph", json!({}));
    run(&mut s, "developer.plainText", json!({"tag": "notes"}));
    run(&mut s, "developer.formFieldOptions", json!({"field": "Dropdown1", "enabled": false}));
    run(&mut s, "review.restrict", json!({"mode": "forms"}));
    // Tab goes from field to field (the disabled drop-down is skipped), round to the first.
    s.sel = crate::Selection::caret(doc4me_doc::Pos::new(StoryRef::Body, doc4me_doc::Path::top(0), 0));
    let a = run(&mut s, "developer.nextFormField", json!({}));
    let b = run(&mut s, "developer.nextFormField", json!({}));
    let c = run(&mut s, "developer.nextFormField", json!({}));
    assert_eq!((a["name"].as_str(), b["name"].as_str(), c["name"].as_str()), (Some("Text1"), Some("Check1"), Some("Text1")));
    let back = run(&mut s, "developer.nextFormField", json!({"back": true}));
    assert_eq!(back["name"], "Check1");
    // Tab and Shift+Tab do the same outside a content control.
    assert_eq!(run(&mut s, "text.tab", json!({}))["name"], "Text1");
    assert_eq!(run(&mut s, "text.backTab", json!({}))["name"], "Check1");
    // Filling in works; the text around the fields can't change.
    run(&mut s, "developer.formFieldValue", json!({"field": "Text1", "value": "ada"}));
    run(&mut s, "developer.formFieldValue", json!({"field": "Check1", "value": true}));
    assert!(s.run("developer.formFieldValue", &json!({"field": "Dropdown1", "value": 0})).is_err(), "disabled");
    let before = text(&s);
    s.sel = crate::Selection::caret(doc4me_doc::Pos::new(StoryRef::Body, doc4me_doc::Path::top(0), 0));
    assert!(s.run("text.insert", &json!({"text": "x"})).is_err());
    assert!(s.run("format.bold", &json!({})).is_err());
    assert!(s.run("developer.formFieldOptions", &json!({"field": "Text1", "name": "Other"})).is_err());
    assert!(s.run("developer.resetFormFields", &json!({})).is_err());
    assert_eq!(text(&s), before);
    // A content control's content can be typed in.
    let r = s.doc.control_ranges(StoryRef::Body)[0].clone();
    s.sel = crate::Selection::caret(r.content_start());
    run(&mut s, "text.insert", json!({"text": "See attached"}));
    assert!(text(&s).ends_with("See attached"));
    // Typing just outside the control (at the paragraph's end) is refused.
    let end = s.doc.para(StoryRef::Body, &doc4me_doc::Path::top(1)).unwrap().len();
    s.sel = crate::Selection::caret(doc4me_doc::Pos::new(StoryRef::Body, doc4me_doc::Path::top(1), end));
    assert!(s.run("text.insert", &json!({"text": "!"})).is_err());
    assert!(text(&s).ends_with("See attached"));
}

#[test]
fn form_fields_save_and_open_with_their_data() {
    let mut s = form();
    run(&mut s, "developer.formFieldValue", json!({"field": "Text1", "value": "ada"}));
    run(&mut s, "developer.formFieldValue", json!({"field": "Check1", "value": true}));
    run(&mut s, "developer.formFieldValue", json!({"field": "Dropdown1", "value": "Large"}));
    run(&mut s, "developer.formFieldHelp", json!({"field": "Check1", "help": {"text": "Tick to agree"}}));
    let bytes = doc4me_docx::write(&s.doc).unwrap();
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(&bytes)).unwrap();
    let mut xml = String::new();
    std::io::Read::read_to_string(&mut zip.by_name("word/document.xml").unwrap(), &mut xml).unwrap();
    assert!(xml.contains("<w:ffData>") && xml.contains(r#"<w:listEntry w:val="Large"/>"#) && xml.contains(r#"<w:checked w:val="1"/>"#), "{xml}");
    let back = doc4me_docx::read(&bytes).unwrap();
    assert_eq!(back.plain_text(StoryRef::Body), s.doc.plain_text(StoryRef::Body));
    let forms: Vec<_> = back
        .para(StoryRef::Body, &doc4me_doc::Path::top(0))
        .unwrap()
        .objects
        .iter()
        .filter_map(|o| match o {
            InlineObject::Field { form: Some(f), .. } => Some((**f).clone()),
            _ => None,
        })
        .collect();
    assert_eq!(forms.len(), 3);
    assert!(matches!(&forms[0].kind, FormKind::Text { input: TextType::Regular, max_len: 10, format, .. } if format == "Title case"));
    assert_eq!((forms[1].checked(), forms[1].help.text.as_str()), (Some(true), "Tick to agree"));
    assert_eq!(forms[2].chosen_text(), "Large");
    // Form field shading is a view setting (on, as Word), never saved.
    assert_eq!(run(&mut s, "developer.formFieldShading", json!({}))["value"], false);
}

#[test]
fn sections_left_unprotected_edit_as_usual_in_a_protected_form() {
    let mut s = form();
    run(&mut s, "layout.break", json!({"kind": "continuous"}));
    run(&mut s, "text.insert", json!({"text": "Notes"}));
    assert_eq!(run(&mut s, "review.protectSections", json!({}))["sections"], json!([true, true]));
    assert!(s.run("review.protectSections", &json!({"protected": [true]})).is_err(), "one per section");
    assert_eq!(run(&mut s, "review.protectSections", json!({"protected": [true, false]}))["sections"], json!([true, false]));
    run(&mut s, "review.restrict", json!({"mode": "forms"}));
    assert!(s.run("review.protectSections", &json!({"protected": [false, false]})).is_err(), "not while protected");
    // The second section types and formats as usual.
    run(&mut s, "text.insert", json!({"text": " and more"}));
    let end = s.doc.end_of(StoryRef::Body);
    let last = end.path.clone();
    s.sel = crate::Selection { anchor: doc4me_doc::Pos::new(StoryRef::Body, last.clone(), 0), focus: end };
    run(&mut s, "format.bold", json!({}));
    assert!(text(&s).ends_with("Notes and more"));
    // The first still takes only its fields.
    s.sel = crate::Selection::caret(doc4me_doc::Pos::new(StoryRef::Body, doc4me_doc::Path::top(0), 0));
    assert!(s.run("text.insert", &json!({"text": "x"})).is_err());
    run(&mut s, "developer.formFieldValue", json!({"field": "Text1", "value": "ada"}));
    // The file keeps which sections are protected.
    let back = doc4me_docx::read(&doc4me_docx::write(&s.doc).unwrap()).unwrap();
    let secs: Vec<bool> = back.sections().iter().map(|(_, sp)| sp.form_unprotected).collect();
    assert_eq!(secs, [false, true]);
}
