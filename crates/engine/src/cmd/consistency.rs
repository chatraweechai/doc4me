//! Review › Language › Japanese Consistency Checker (Word's 表記ゆれチェック): words written in
//! more than one way in the body — katakana in full and half width (コンピュータ / ｺﾝﾋﾟｭｰﾀ),
//! with and without a final prolonged sound mark (コンピューター / コンピュータ), and letters and
//! digits in full and half width (ＰＤＦ / PDF). The checker lists each group of spellings with
//! how often each occurs; choosing one replaces the group's other spellings with it, as one undo
//! step (a reviewable revision while Track Changes is on).

use std::collections::{BTreeMap, HashMap};

use doc4me_doc::{Pos, StoryRef};
use serde_json::{Value, json};

use super::find_pattern::{Fuzzy, fold};
use crate::{CmdError, CmdResult, CommandSpec, Session, p};

/// At most this many groups are listed (a long document rarely has more than a few dozen).
const MAX_GROUPS: usize = 500;
/// Words longer than this are not compared (a run of letters this long is not a word).
const MAX_WORD: usize = 64;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("review.consistencyChecker", "Japanese Consistency Checker", "Review › Language", consistency_checker).params(
            r#"{"to"?: string (the spelling to keep), "from"?: string (only this other spelling)} → {"groups": [{"forms": [{"text", "count"}]}]} without `to`; {"replaced": n} with it"#,
        ),
    ]
}

/// What kind of word a character belongs to (`None`: it ends a word).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Class {
    Katakana,
    Alnum,
}

fn class(c: char) -> Option<Class> {
    match c {
        '\u{30A1}'..='\u{30FA}' | '\u{30FC}'..='\u{30FE}' | '\u{FF66}'..='\u{FF9F}' => Some(Class::Katakana),
        'A'..='Z' | 'a'..='z' | '0'..='9' | '\u{FF10}'..='\u{FF19}' | '\u{FF21}'..='\u{FF3A}' | '\u{FF41}'..='\u{FF5A}' => Some(Class::Alnum),
        _ => None,
    }
}

/// Voiced and semi-voiced katakana by base letter, for half-width pairs (ｶﾞ → ガ, ﾊﾟ → パ).
fn voiced(base: char, mark: char) -> Option<char> {
    const VOICED: [(char, char); 21] = [
        ('カ', 'ガ'),
        ('キ', 'ギ'),
        ('ク', 'グ'),
        ('ケ', 'ゲ'),
        ('コ', 'ゴ'),
        ('サ', 'ザ'),
        ('シ', 'ジ'),
        ('ス', 'ズ'),
        ('セ', 'ゼ'),
        ('ソ', 'ゾ'),
        ('タ', 'ダ'),
        ('チ', 'ヂ'),
        ('ツ', 'ヅ'),
        ('テ', 'デ'),
        ('ト', 'ド'),
        ('ハ', 'バ'),
        ('ヒ', 'ビ'),
        ('フ', 'ブ'),
        ('ヘ', 'ベ'),
        ('ホ', 'ボ'),
        ('ウ', 'ヴ'),
    ];
    match mark {
        '゛' => VOICED.iter().find(|(b, _)| *b == base).map(|(_, v)| *v),
        '゜' => "ハヒフヘホ".chars().position(|b| b == base).and_then(|i| "パピプペポ".chars().nth(i)),
        _ => None,
    }
}

/// The spelling-independent form of a word: full width for kana, half width for letters and
/// digits, half-width voiced pairs joined, dashes read as ー, and a final ー dropped from a
/// katakana word of three letters or more.
fn key(word: &str) -> String {
    let f = Fuzzy { on: true, width: true, kana: false, small_kana: false, long_vowel: true };
    let mut out: Vec<char> = Vec::with_capacity(word.len());
    for c in word.chars().map(|c| fold(c, &f)) {
        if let Some(v) = out.last().and_then(|b| voiced(*b, c)) {
            out.pop();
            out.push(v);
        } else {
            out.push(c);
        }
    }
    if out.len() >= 3 && out.last() == Some(&'ー') && out.first().and_then(|c| class(*c)) == Some(Class::Katakana) {
        out.pop();
    }
    out.into_iter().collect()
}

/// Every word in the body that could be spelled another way, in document order: (start, end,
/// text). Text marked deleted by Track Changes ends a word and is not read.
fn words(s: &Session) -> Vec<(Pos, Pos, String)> {
    let mut out = Vec::new();
    for path in s.doc.para_paths(StoryRef::Body) {
        let Some(p) = s.doc.para(StoryRef::Body, &path) else { continue };
        let mut start: Option<(usize, Class)> = None;
        let flush = |from: usize, to: usize, out: &mut Vec<(Pos, Pos, String)>| {
            let w = &p.text[from..to];
            if w.chars().count() <= MAX_WORD {
                out.push((Pos::new(StoryRef::Body, path.clone(), from), Pos::new(StoryRef::Body, path.clone(), to), w.to_string()));
            }
        };
        for (i, c) in p.text.char_indices() {
            let k = class(c).filter(|_| p.props_of_char(i).del.is_none());
            match (start, k) {
                (Some((_, a)), Some(b)) if a == b => {}
                (Some((from, _)), _) => {
                    flush(from, i, &mut out);
                    start = k.map(|k| (i, k));
                }
                (None, Some(k)) => start = Some((i, k)),
                (None, None) => {}
            }
        }
        if let Some((from, _)) = start {
            flush(from, p.text.len(), &mut out);
        }
    }
    out
}

/// Groups of two or more spellings of one word: each spelling with its count, groups in the
/// order their first word appears, spellings most frequent first.
fn groups(words: &[(Pos, Pos, String)]) -> Vec<Vec<(String, usize)>> {
    let mut order: Vec<String> = Vec::new();
    let mut by_key: HashMap<String, BTreeMap<String, usize>> = HashMap::new();
    for (_, _, w) in words {
        let k = key(w);
        let forms = by_key.entry(k.clone()).or_insert_with(|| {
            order.push(k);
            BTreeMap::new()
        });
        *forms.entry(w.clone()).or_default() += 1;
    }
    order
        .into_iter()
        .filter_map(|k| {
            let forms = by_key.remove(&k)?;
            if forms.len() < 2 {
                return None;
            }
            let mut v: Vec<(String, usize)> = forms.into_iter().collect();
            v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            Some(v)
        })
        .take(MAX_GROUPS)
        .collect()
}

fn consistency_checker(s: &mut Session, v: &Value) -> CmdResult {
    let found = words(s);
    let Some(to) = p::str(v, "to") else {
        s.only_read();
        let groups = groups(&found);
        return Ok(json!({
            "groups": groups.iter().map(|g| json!({
                "forms": g.iter().map(|(t, n)| json!({"text": t, "count": n})).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
        }));
    };
    let to = to.to_string();
    let target = key(&to);
    if to.is_empty() || target.chars().any(|c| class(c).is_none()) {
        return Err(CmdError::Params("`to` is a word in katakana, or in letters and digits".into()));
    }
    let from = p::str(v, "from");
    if let Some(f) = from
        && key(f) != target
    {
        return Err(CmdError::Params(format!("`{f}` is not another spelling of `{to}`")));
    }
    let edits: Vec<(Pos, Pos, String)> = found
        .into_iter()
        .filter(|(_, _, w)| *w != to && from.is_none_or(|f| f == w.as_str()) && key(w) == target)
        .map(|(a, b, _)| (a, b, to.clone()))
        .collect();
    let n = edits.len();
    if n == 0 {
        s.only_read();
    } else {
        super::edit::replace_each(s, edits)?;
    }
    Ok(json!({"replaced": n}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &mut Session, id: &str, v: Value) -> Value {
        s.run(id, &v).unwrap_or_else(|e| panic!("{id}: {e}"))
    }

    #[test]
    fn keys_join_spellings_of_one_word() {
        assert_eq!(key("ｺﾝﾋﾟｭｰﾀ"), key("コンピュータ"));
        assert_eq!(key("コンピューター"), key("コンピュータ"));
        assert_eq!(key("ﾃﾞｰﾀ"), key("データ"));
        assert_eq!(key("ＰＤＦ"), key("PDF"));
        assert_ne!(key("PDF"), key("pdf"), "case is the writer's choice");
        assert_ne!(key("キャ"), key("キヤ"), "small kana are spelling, not variation");
        // A short word keeps its ー: ルー is not ル.
        assert_ne!(key("ルー"), key("ル"));
    }

    #[test]
    fn lists_groups_and_replaces_the_other_spellings() {
        let mut s = Session::new(doc4me_doc::Document::from_text(
            "コンピューターとｺﾝﾋﾟｭｰﾀ、コンピューター。\nＰＤＦとPDFとPDF、データだけ",
        ));
        let v = run(&mut s, "review.consistencyChecker", json!({}));
        let g = v["groups"].as_array().unwrap();
        assert_eq!(g.len(), 2, "{v}");
        assert_eq!(g[0]["forms"][0], json!({"text": "コンピューター", "count": 2}));
        assert_eq!(g[0]["forms"][1], json!({"text": "ｺﾝﾋﾟｭｰﾀ", "count": 1}));
        assert_eq!(g[1]["forms"][0], json!({"text": "PDF", "count": 2}));
        assert!(s.run("review.consistencyChecker", &json!({"to": "コンピューター", "from": "PDF"})).is_err());
        assert!(s.run("review.consistencyChecker", &json!({"to": "東京"})).is_err());
        assert_eq!(run(&mut s, "review.consistencyChecker", json!({"to": "コンピューター"}))["replaced"], json!(1));
        assert_eq!(run(&mut s, "review.consistencyChecker", json!({"to": "PDF"}))["replaced"], json!(1));
        assert_eq!(s.doc.plain_text(StoryRef::Body), "コンピューターとコンピューター、コンピューター。\nPDFとPDFとPDF、データだけ");
        assert!(run(&mut s, "review.consistencyChecker", json!({}))["groups"].as_array().unwrap().is_empty());
        run(&mut s, "edit.undo", json!({}));
        assert_eq!(s.doc.plain_text(StoryRef::Body), "コンピューターとコンピューター、コンピューター。\nＰＤＦとPDFとPDF、データだけ", "one undo step");
    }

    #[test]
    fn tracked_replacement_is_a_revision_and_deleted_text_is_skipped() {
        let mut s = Session::new(doc4me_doc::Document::from_text("ﾃﾞｰﾀとデータ"));
        s.doc.settings.track_changes = true;
        assert_eq!(run(&mut s, "review.consistencyChecker", json!({"to": "データ"}))["replaced"], json!(1));
        let p = s.doc.para(StoryRef::Body, &doc4me_doc::Path::top(0)).unwrap();
        assert!(p.runs.iter().any(|r| r.props.del.is_some()) && p.runs.iter().any(|r| r.props.ins.is_some()), "{:?}", p.runs);
        // The deleted ﾃﾞｰﾀ is no longer a spelling in use.
        assert!(run(&mut s, "review.consistencyChecker", json!({}))["groups"].as_array().unwrap().is_empty());
    }
}
