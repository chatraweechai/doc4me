//! Review › Chinese Conversion: Traditional ↔ Simplified Chinese, character by character, with
//! OpenCC's character tables (Apache-2.0, `assets/opencc`). Like Word, it converts the selection,
//! or the whole document when nothing is selected; each run keeps its formatting and Track
//! Changes records the change (see [`super::format::map_chars`]). Simplified to Traditional can
//! use the forms usual in Taiwan, Hong Kong SAR and Macao SAR (OpenCC's Taiwan variants).

use std::collections::HashMap;
use std::sync::OnceLock;

use doc4me_doc::{Pos, StoryRef};
use serde_json::{Value, json};

use crate::{CmdError, CmdResult, CommandSpec, Session, p};

const LOC: &str = "Review › Chinese Conversion";

static ST_CHARACTERS: &str = include_str!("../../../../assets/opencc/STCharacters.txt");
static TS_CHARACTERS: &str = include_str!("../../../../assets/opencc/TSCharacters.txt");
static TW_VARIANTS: &str = include_str!("../../../../assets/opencc/TWVariants.txt");

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("review.chineseConversion", "Chinese Conversion", LOC, chinese_conversion).params(
            r#"{"to"?: "simplified|traditional", "variants"?: bool (Taiwan, Hong Kong SAR and Macao SAR forms; to traditional)} → {"changed": n} (the selection, or the whole document when nothing is selected)"#,
        ),
        CommandSpec::new("review.toSimplified", "Traditional to Simplified", LOC, |s, _| convert(s, Direction::Simplified, false)),
        CommandSpec::new("review.toTraditional", "Simplified to Traditional", LOC, |s, _| convert(s, Direction::Traditional, false)),
    ]
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Simplified,
    Traditional,
}

/// A table's character pairs: the first value of each `key<TAB>values` line whose key and value
/// are one character each (OpenCC lists alternatives after the first; the first is its choice).
fn table(src: &str) -> HashMap<char, char> {
    let mut map = HashMap::new();
    for line in src.lines() {
        if line.starts_with('#') {
            continue;
        }
        let Some((key, values)) = line.split_once('\t') else { continue };
        let mut k = key.chars();
        let (Some(from), None) = (k.next(), k.next()) else { continue };
        let Some(first) = values.split(' ').next() else { continue };
        let mut v = first.chars();
        if let (Some(to), None) = (v.next(), v.next()) {
            map.entry(from).or_insert(to);
        }
    }
    map
}

fn to_traditional() -> &'static HashMap<char, char> {
    static T: OnceLock<HashMap<char, char>> = OnceLock::new();
    T.get_or_init(|| table(ST_CHARACTERS))
}

fn to_simplified() -> &'static HashMap<char, char> {
    static T: OnceLock<HashMap<char, char>> = OnceLock::new();
    T.get_or_init(|| table(TS_CHARACTERS))
}

fn taiwan_variants() -> &'static HashMap<char, char> {
    static T: OnceLock<HashMap<char, char>> = OnceLock::new();
    T.get_or_init(|| table(TW_VARIANTS))
}

/// `c` in the other script (`None`: the same in both).
fn convert_char(c: char, to: Direction, variants: bool) -> Option<char> {
    match to {
        Direction::Simplified => to_simplified().get(&c).copied(),
        Direction::Traditional => {
            let t = to_traditional().get(&c).copied().unwrap_or(c);
            let t = if variants { taiwan_variants().get(&t).copied().unwrap_or(t) } else { t };
            (t != c).then_some(t)
        }
    }
}

/// Chinese Conversion: without `to`, nothing changes (the dialog asks which way).
fn chinese_conversion(s: &mut Session, v: &Value) -> CmdResult {
    let to = match p::str(v, "to") {
        None => {
            s.only_read();
            return Ok(json!({"changed": 0}));
        }
        Some("simplified") => Direction::Simplified,
        Some("traditional") => Direction::Traditional,
        Some(other) => return Err(CmdError::Params(format!("to: simplified or traditional, not {other:?}"))),
    };
    convert(s, to, p::bool(v, "variants").unwrap_or(false))
}

fn convert(s: &mut Session, to: Direction, variants: bool) -> CmdResult {
    let (a, b) = s.sel.ordered();
    // Nothing selected: the whole document, as Word does.
    let (a, b) = if a == b { (s.doc.start_of(StoryRef::Body), end_of_body(s)) } else { (a, b) };
    let changed = super::format::map_chars(s, &a, &b, &|c| convert_char(c, to, variants))?;
    if changed == 0 {
        s.only_read();
    }
    Ok(json!({"changed": changed}))
}

fn end_of_body(s: &Session) -> Pos {
    s.doc.end_of(StoryRef::Body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_read_one_character_pairs() {
        assert_eq!(convert_char('汉', Direction::Traditional, false), Some('漢'));
        assert_eq!(convert_char('漢', Direction::Simplified, false), Some('汉'));
        assert_eq!(convert_char('中', Direction::Traditional, false), None, "the same in both");
        assert_eq!(convert_char('a', Direction::Simplified, false), None);
        // Taiwan's forms: 伪 → 僞 (OpenCC's standard), 偽 in Taiwan.
        assert_eq!(convert_char('伪', Direction::Traditional, false), Some('僞'));
        assert_eq!(convert_char('伪', Direction::Traditional, true), Some('偽'));
        assert!(to_traditional().len() > 3_000 && to_simplified().len() > 3_000);
    }

    fn run(s: &mut Session, id: &str, v: Value) -> Value {
        s.run(id, &v).unwrap_or_else(|e| panic!("{id}: {e}"))
    }

    fn text(s: &Session) -> String {
        s.doc.plain_text(StoryRef::Body)
    }

    #[test]
    fn the_selection_or_the_whole_document_converts_keeping_its_formatting() {
        let mut s = Session::new(doc4me_doc::Document::from_text("汉语 简体\n后来"));
        // "简体" bold, then selected and converted: only it changes, and stays bold.
        let at = |path: usize, off: usize| Pos::new(StoryRef::Body, doc4me_doc::Path::top(path), off);
        s.sel = crate::Selection { anchor: at(0, "汉语 ".len()), focus: at(0, "汉语 简体".len()) };
        run(&mut s, "format.bold", json!({}));
        assert_eq!(run(&mut s, "review.toTraditional", json!({}))["changed"], json!(2));
        assert_eq!(text(&s), "汉语 簡體\n后来");
        let p = s.doc.para(StoryRef::Body, &doc4me_doc::Path::top(0)).unwrap();
        assert_eq!(p.props_of_char("汉语 ".len()).bold, Some(true), "the converted text keeps its run's formatting");
        assert_eq!(s.doc.copy_range(&s.sel.anchor, &s.sel.focus).plain_text(), "簡體", "the selection still covers it");
        // Nothing selected: the whole document.
        s.sel = crate::Selection::caret(at(1, 0));
        assert_eq!(run(&mut s, "review.toTraditional", json!({}))["changed"], json!(4));
        assert_eq!(text(&s), "漢語 簡體\n後來");
        run(&mut s, "edit.undo", json!({}));
        assert_eq!(text(&s), "汉语 簡體\n后来", "one undo takes it back");
        // And back to Simplified; the dialog without a direction changes nothing.
        assert_eq!(run(&mut s, "review.chineseConversion", json!({}))["changed"], json!(0));
        run(&mut s, "review.chineseConversion", json!({"to": "simplified"}));
        assert_eq!(text(&s), "汉语 简体\n后来");
        assert!(s.run("review.chineseConversion", &json!({"to": "klingon"})).is_err());
        // Taiwan, Hong Kong SAR and Macao SAR forms.
        let mut s = Session::new(doc4me_doc::Document::from_text("伪造"));
        run(&mut s, "review.chineseConversion", json!({"to": "traditional", "variants": true}));
        assert_eq!(text(&s), "偽造");
    }

    #[test]
    fn tracked_conversion_is_a_deletion_and_an_insertion() {
        let mut s = Session::new(doc4me_doc::Document::from_text("汉字"));
        s.doc.settings.track_changes = true;
        run(&mut s, "review.toTraditional", json!({}));
        let p = s.doc.para(StoryRef::Body, &doc4me_doc::Path::top(0)).unwrap();
        assert!(p.runs.iter().any(|r| r.props.del.is_some()) && p.runs.iter().any(|r| r.props.ins.is_some()), "{:?}", p.runs);
        // 字 is the same in both: only 汉 changes, marked deleted, with 漢 inserted after it.
        assert_eq!(p.text, "汉漢字");
        assert!(p.props_of_char(0).del.is_some() && p.props_of_char("汉".len()).ins.is_some() && p.props_of_char("汉漢".len()).ins.is_none());
    }
}
