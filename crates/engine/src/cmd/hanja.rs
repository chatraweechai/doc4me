//! Review › Language › Hangul Hanja Conversion: Korean text between Hangul and Hanja, character
//! by character, with the Unicode Character Database's Korean readings of CJK ideographs
//! (`kHangul` from Unihan 16.0.0, Unicode License v3, `assets/unihan`).
//!
//! Hanja to Hangul converts the selection (or the whole document when nothing is selected) to
//! each character's usual reading: the one in Korea's list of basic Hanja for education (`E`),
//! else the first. Hangul to Hanja lists, for the selected Hangul (or the word at the caret), the
//! Hanja read as each syllable, and writes the ones chosen. Each run keeps its formatting and Track
//! Changes records the change.

use std::collections::HashMap;
use std::sync::OnceLock;

use doc4me_doc::{Pos, StoryRef};
use serde_json::{Value, json};

use crate::{CmdError, CmdResult, CommandSpec, Selection, Session, p};

static K_HANGUL: &str = include_str!("../../../../assets/unihan/kHangul.txt");

/// Most Hanja listed for one syllable.
const MAX_CANDIDATES: usize = 60;
/// Longest Hangul word the conversion looks at, in syllables.
const MAX_WORD: usize = 32;

pub fn specs() -> Vec<CommandSpec> {
    vec![CommandSpec::new("review.hangulHanja", "Hangul Hanja Conversion", "Review › Language", hangul_hanja).params(
        r#"{"to"?: "hangul|hanja", "hanja"?: string (to hanja: one Hanja for each selected Hangul syllable)} — to hangul converts the selection (or the whole document); to hanja writes the Hanja given for the selected Hangul (or the word at the caret); without `to`, returns {"text", "candidates": [[hanja]]} for the selected Hangul"#,
    )]
}

/// Readings by ideograph: (syllable, educational) in the database's order.
fn readings() -> &'static HashMap<char, Vec<(char, bool)>> {
    static R: OnceLock<HashMap<char, Vec<(char, bool)>>> = OnceLock::new();
    R.get_or_init(|| {
        let mut map = HashMap::new();
        for line in K_HANGUL.lines().filter(|l| !l.starts_with('#')) {
            let Some((cp, values)) = line.split_once('\t') else { continue };
            let Some(c) = cp.strip_prefix("U+").and_then(|h| u32::from_str_radix(h, 16).ok()).and_then(char::from_u32) else { continue };
            let list: Vec<(char, bool)> = values
                .split(' ')
                .filter_map(|v| {
                    let (syl, src) = v.split_once(':').unwrap_or((v, ""));
                    let mut it = syl.chars();
                    match (it.next(), it.next()) {
                        (Some(h), None) if is_hangul(h) => Some((h, src.contains('E'))),
                        _ => None,
                    }
                })
                .collect();
            if !list.is_empty() {
                map.insert(c, list);
            }
        }
        map
    })
}

/// Ideographs by syllable: those with an educational reading first, then the rest, each in code
/// point order.
fn by_syllable() -> &'static HashMap<char, Vec<char>> {
    static B: OnceLock<HashMap<char, Vec<char>>> = OnceLock::new();
    B.get_or_init(|| {
        let mut map: HashMap<char, Vec<(bool, char)>> = HashMap::new();
        for (c, list) in readings() {
            for (syl, edu) in list {
                map.entry(*syl).or_default().push((!edu, *c));
            }
        }
        map.into_iter()
            .map(|(k, mut v)| {
                v.sort();
                v.dedup_by_key(|x| x.1);
                (k, v.into_iter().map(|x| x.1).collect())
            })
            .collect()
    })
}

/// A precomposed Hangul syllable.
fn is_hangul(c: char) -> bool {
    ('\u{AC00}'..='\u{D7A3}').contains(&c)
}

/// `c`'s usual Hangul reading (`None`: not a Hanja the database reads).
fn to_hangul(c: char) -> Option<char> {
    let list = readings().get(&c)?;
    list.iter().find(|(_, edu)| *edu).or_else(|| list.first()).map(|(h, _)| *h)
}

/// The Hanja read as `syllable`.
fn candidates(syllable: char) -> &'static [char] {
    by_syllable().get(&syllable).map_or(&[], Vec::as_slice)
}

/// The selected text, or the run of Hangul syllables around the caret (which is then selected).
fn hangul_word(s: &mut Session) -> Result<(Pos, Pos, String), CmdError> {
    let (a, b) = s.sel.ordered();
    let para = s.doc.para_at(&a).ok_or_else(|| CmdError::Failed("no paragraph at the selection".into()))?;
    if a != b {
        if a.path != b.path || a.story != b.story {
            return Err(CmdError::Failed("select Korean text in one paragraph".into()));
        }
        let text = para.text.get(a.off..b.off).unwrap_or("").to_string();
        return Ok((a, b, text));
    }
    let t = &para.text;
    let mut start = a.off;
    while let Some(c) = t[..start].chars().next_back().filter(|c| is_hangul(*c)) {
        start -= c.len_utf8();
    }
    let mut end = a.off;
    while let Some(c) = t[end..].chars().next().filter(|c| is_hangul(*c)) {
        end += c.len_utf8();
    }
    if start == end {
        return Err(CmdError::Failed("put the caret in Korean text, or select it".into()));
    }
    let (sa, sb) = (Pos { off: start, ..a.clone() }, Pos { off: end, ..a });
    let text = t[start..end].to_string();
    s.sel = Selection { anchor: sa.clone(), focus: sb.clone() };
    Ok((sa, sb, text))
}

fn hangul_hanja(s: &mut Session, v: &Value) -> CmdResult {
    match p::str(v, "to") {
        None => {
            s.only_read();
            let (_, _, text) = hangul_word(s)?;
            if text.chars().count() > MAX_WORD {
                return Err(CmdError::Params(format!("select at most {MAX_WORD} syllables")));
            }
            let cands: Vec<Vec<String>> = text
                .chars()
                .map(|c| if is_hangul(c) { candidates(c).iter().take(MAX_CANDIDATES).map(|h| h.to_string()).collect() } else { Vec::new() })
                .collect();
            Ok(json!({"text": text, "candidates": cands}))
        }
        Some("hangul") => {
            let (a, b) = s.sel.ordered();
            let (a, b) = if a == b { (s.doc.start_of(StoryRef::Body), s.doc.end_of(StoryRef::Body)) } else { (a, b) };
            let changed = super::format::map_chars(s, &a, &b, &to_hangul)?;
            if changed == 0 {
                s.only_read();
            }
            Ok(json!({"changed": changed}))
        }
        Some("hanja") => {
            let hanja = p::req_str(v, "hanja")?.to_string();
            let (_, _, text) = hangul_word(s)?;
            let syllables: Vec<char> = text.chars().collect();
            let chosen: Vec<char> = hanja.chars().collect();
            if syllables.len() > MAX_WORD || syllables.len() != chosen.len() {
                return Err(CmdError::Params(format!("give one Hanja for each of the {} selected characters", syllables.len())));
            }
            // Each Hanja must read as its syllable (a character that isn't Hangul stays itself).
            for (syl, h) in syllables.iter().zip(&chosen) {
                let fits = if is_hangul(*syl) { readings().get(h).is_some_and(|l| l.iter().any(|(r, _)| r == syl)) } else { syl == h };
                if !fits {
                    return Err(CmdError::Params(format!("`{h}` is not read as `{syl}`")));
                }
            }
            if hanja == text {
                s.only_read();
                return Ok(json!({"changed": 0}));
            }
            super::paste_text(s, &hanja)?;
            Ok(json!({"changed": syllables.iter().zip(&chosen).filter(|(a, b)| a != b).count()}))
        }
        Some(other) => Err(CmdError::Params(format!("to: hangul or hanja, not {other:?}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &mut Session, id: &str, v: Value) -> Value {
        s.run(id, &v).unwrap_or_else(|e| panic!("{id}: {e}"))
    }

    #[test]
    fn readings_and_candidates_come_from_the_database() {
        assert_eq!(to_hangul('漢'), Some('한'));
        assert_eq!(to_hangul('樂'), Some('락'), "the educational reading, not the first listed (낙)");
        assert_eq!(to_hangul('a'), None);
        let han = candidates('한');
        assert!(han.contains(&'漢') && han.contains(&'韓'), "{han:?}");
        assert!(readings().len() > 8_000);
    }

    #[test]
    fn hanja_to_hangul_and_back() {
        let mut s = Session::new(doc4me_doc::Document::from_text("大韓民國 만세"));
        assert_eq!(run(&mut s, "review.hangulHanja", json!({"to": "hangul"}))["changed"], json!(4));
        assert_eq!(s.doc.plain_text(StoryRef::Body), "대한민국 만세");
        run(&mut s, "edit.undo", json!({}));
        assert_eq!(s.doc.plain_text(StoryRef::Body), "大韓民國 만세", "one undo step");
        // Hangul to Hanja: the word at the caret, its candidates, then the chosen Hanja.
        let at = |off: usize| Pos::new(StoryRef::Body, doc4me_doc::Path::top(0), off);
        s.sel = Selection::caret(at("大韓民國 만".len()));
        let r = run(&mut s, "review.hangulHanja", json!({}));
        assert_eq!(r["text"], json!("만세"));
        assert!(r["candidates"][0].as_array().unwrap().contains(&json!("萬")), "{r}");
        assert!(s.run("review.hangulHanja", &json!({"to": "hanja", "hanja": "萬"})).is_err(), "one for each syllable");
        assert!(s.run("review.hangulHanja", &json!({"to": "hanja", "hanja": "漢歲"})).is_err(), "漢 is not read 만");
        run(&mut s, "review.hangulHanja", json!({"to": "hanja", "hanja": "萬歲"}));
        assert_eq!(s.doc.plain_text(StoryRef::Body), "大韓民國 萬歲");
        s.sel = Selection::caret(at(0));
        assert!(s.run("review.hangulHanja", &json!({})).is_err(), "no Hangul at the caret");
        assert!(s.run("review.hangulHanja", &json!({"to": "latin"})).is_err());
    }
}
