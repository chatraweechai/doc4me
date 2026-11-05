//! Thai proofing. Precision first: a mark means the text is certainly wrong, never just a word
//! the checker doesn't know (names, new loanwords and technical terms are left alone). Two kinds
//! of mistakes are found, each with the correction as its suggestion:
//!
//! - **Typing order and look-alike characters** ([`orthography`]): characters in an order that
//!   no Thai word has, which usually look right on screen but break search, sorting and
//!   spelling: `เ` typed twice for `แ`, `ํ` + `า` for `ำ`, a tone mark or ์ typed before the
//!   vowel above or below its consonant (`ก่ี` for `กี่`), a tone mark typed after `ำ` (`นำ้` for
//!   `น้ำ`), and the same mark twice.
//! - **Common misspellings** ([`misspellings`]): words often written wrongly, with the spelling
//!   of the Royal Society of Thailand (ราชบัณฑิตยสภา, its dictionary พจนานุกรมฉบับ
//!   ราชบัณฑิตยสถาน พ.ศ. 2554 and transliteration rules): `อนุญาติ` → `อนุญาต`, `กระเพรา` →
//!   `กะเพรา`, `อีเมล์` → `อีเมล`. A misspelling is only marked where the dictionary word
//!   segmenter sees a word that begins and ends where it does, so the same letters inside a
//!   longer correct word, or across two words, are not.
//!
//! Thai text is checked wherever it is, whatever language its run is tagged with: no other
//! language is written in Thai script. Messages are in Thai.

use crate::{Issue, IssueKind, ProofLang};

/// Our list of common misspellings: `wrong<TAB>right` per line, `#` comments.
const MISSPELLINGS: &str = include_str!("../../../../assets/spelling/th-misspellings.tsv");

/// Is `c` a Thai character (U+0E00–U+0E7F)?
pub fn is_thai(c: char) -> bool {
    ('\u{0E00}'..='\u{0E7F}').contains(&c)
}

fn is_tone(c: char) -> bool {
    ('\u{0E48}'..='\u{0E4B}').contains(&c)
}

/// Vowels written above or below the consonant: ั ิ ี ึ ื ุ ู ฺ.
fn is_above_below_vowel(c: char) -> bool {
    c == '\u{0E31}' || ('\u{0E34}'..='\u{0E3A}').contains(&c)
}

/// Marks written above or below a consonant: those vowels, ็, the tone marks, ์, ํ, ๎.
fn is_mark(c: char) -> bool {
    is_above_below_vowel(c) || ('\u{0E47}'..='\u{0E4E}').contains(&c)
}

/// The misspellings: (wrong, right), wrong forms unique.
pub fn misspellings() -> &'static [(&'static str, &'static str)] {
    static LIST: std::sync::OnceLock<Vec<(&'static str, &'static str)>> = std::sync::OnceLock::new();
    LIST.get_or_init(|| {
        let mut v: Vec<(&str, &str)> = MISSPELLINGS
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .filter_map(|l| l.split_once('\t'))
            .map(|(w, r)| (w.trim(), r.trim()))
            .filter(|(w, r)| !w.is_empty() && !r.is_empty() && w != r)
            .collect();
        v.sort_unstable();
        v.dedup_by(|a, b| a.0 == b.0);
        v
    })
}

/// One fix of a character sequence no Thai word has: the bytes it covers and what replaces them
/// (`None` when the intended text can't be told, such as two different tone marks).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix {
    pub start: usize,
    pub end: usize,
    pub replacement: Option<String>,
    pub message: &'static str,
}

/// Character sequences in `text` that no correctly typed Thai has, with their fixes, in order.
pub fn orthography(text: &str) -> Vec<Fix> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let end_of = |k: usize| chars.get(k).map_or(text.len(), |(i, _)| *i);
    let mut out = Vec::new();
    let mut k = 0;
    while let Some(&(i, c)) = chars.get(k) {
        let next = chars.get(k + 1).map(|(_, n)| *n);
        let fix = |n: usize, replacement: Option<String>, message: &'static str| Fix { start: i, end: end_of(k + n), replacement, message };
        let found = match (c, next) {
            ('\u{0E40}', Some('\u{0E40}')) => Some((2, fix(2, Some("\u{0E41}".into()), "พิมพ์ เ สองตัวแทนสระ แ"))),
            ('\u{0E4D}', Some('\u{0E32}')) => Some((2, fix(2, Some("\u{0E33}".into()), "พิมพ์ ํ กับ า แทนสระ ำ"))),
            (t, Some(v)) if (is_tone(t) || t == '\u{0E4C}') && is_above_below_vowel(v) => {
                Some((2, fix(2, Some([v, t].iter().collect()), "พิมพ์วรรณยุกต์หรือทัณฑฆาตก่อนสระ")))
            }
            ('\u{0E33}', Some(t)) if is_tone(t) => Some((2, fix(2, Some([t, '\u{0E33}'].iter().collect()), "พิมพ์วรรณยุกต์หลังสระ ำ"))),
            (a, Some(b)) if is_mark(a) && a == b => Some((2, fix(2, Some(a.to_string()), "พิมพ์เครื่องหมายซ้ำ"))),
            (a, Some(b)) if is_tone(a) && is_tone(b) => Some((2, fix(2, None, "มีวรรณยุกต์สองตัว"))),
            (a, Some(b)) if is_above_below_vowel(a) && is_above_below_vowel(b) && a != '\u{0E3A}' && b != '\u{0E3A}' => {
                Some((2, fix(2, None, "มีสระบนหรือสระล่างสองตัว")))
            }
            _ => None,
        };
        match found {
            Some((n, f)) => {
                out.push(f);
                k += n;
            }
            None => k += 1,
        }
    }
    out
}

/// Word boundaries (byte offsets) of the Thai text around `at`: the run of Thai characters and
/// spaces holding it, at most a few hundred characters each way, cut at spaces.
fn boundaries_around(text: &str, at: usize) -> (usize, Vec<usize>) {
    const REACH: usize = 600;
    let thai_or_space = |c: char| is_thai(c) || c == ' ';
    let mut start = at;
    for (i, c) in text.get(..at).unwrap_or("").char_indices().rev() {
        if !thai_or_space(c) || at - i > REACH {
            break;
        }
        start = i;
    }
    let mut end = at;
    for (i, c) in text.get(at..).unwrap_or("").char_indices() {
        if !thai_or_space(c) || i > REACH {
            break;
        }
        end = at + i + c.len_utf8();
    }
    let piece = text.get(start..end).unwrap_or("");
    let segmenter = icu_segmenter::WordSegmenter::new_dictionary(icu_segmenter::options::WordBreakInvariantOptions::default());
    (start, segmenter.segment_str(piece).map(|b| start + b).collect())
}

/// Common misspellings in `text`: (byte range, the Royal Society spelling), in order, each where
/// the word segmenter sees a word that begins and ends where it does.
pub fn misspelled(text: &str) -> Vec<(usize, usize, &'static str)> {
    let list = misspellings();
    let mut found: Vec<(usize, usize, &'static str)> = Vec::new();
    if !text.chars().any(is_thai) {
        return found;
    }
    for (wrong, right) in list {
        for (at, _) in text.match_indices(wrong) {
            found.push((at, at + wrong.len(), right));
        }
    }
    if found.is_empty() {
        return found;
    }
    found.sort_unstable();
    // Longest match first where they start together; no overlaps.
    let mut kept: Vec<(usize, usize, &'static str)> = Vec::new();
    for f in found {
        match kept.last_mut() {
            Some(last) if f.0 < last.1 => {
                if f.0 == last.0 && f.1 > last.1 {
                    *last = f;
                }
            }
            _ => kept.push(f),
        }
    }
    kept.retain(|(a, b, _)| {
        // The segmenter can break inside a word it doesn't know (คลินิค|ั|ล, "clinical"), so a
        // match followed by a vowel or mark, or preceded by a leading vowel, is part of a longer
        // word and stays unmarked.
        let after = text.get(*b..).and_then(|s| s.chars().next());
        let before = text.get(..*a).and_then(|s| s.chars().next_back());
        if after.is_some_and(continues_syllable) || before.is_some_and(is_leading_vowel) {
            return false;
        }
        let (_, bounds) = boundaries_around(text, *a);
        bounds.contains(a) && bounds.contains(b)
    });
    kept
}

/// Characters that never begin a syllable: vowels written after, above or below a consonant
/// (ะ ั า ำ ิ ี ึ ื ุ ู ฺ ๅ) and the marks ็ ่ ้ ๊ ๋ ์ ํ ๎.
fn continues_syllable(c: char) -> bool {
    ('\u{0E30}'..='\u{0E3A}').contains(&c) || c == '\u{0E45}' || ('\u{0E47}'..='\u{0E4E}').contains(&c)
}

/// The leading vowels เ แ โ ใ ไ, written before their consonant: no word starts right after one.
fn is_leading_vowel(c: char) -> bool {
    ('\u{0E40}'..='\u{0E44}').contains(&c)
}

/// The Thai issues in `text` (a paragraph), in order: misspellings and characters typed in an
/// order no Thai word has, each with its correction as the suggestion. Words in the user
/// dictionary are left alone.
pub fn check(text: &str) -> Vec<Issue> {
    if !text.chars().any(is_thai) {
        return Vec::new();
    }
    let mut v: Vec<Issue> = Vec::new();
    for (a, b, right) in misspelled(text) {
        if text.get(a..b).is_some_and(|w| crate::in_user_dictionary(&w.to_lowercase())) {
            continue;
        }
        v.push(Issue {
            start: a,
            end: b,
            kind: IssueKind::Spelling,
            message: format!("สะกดไม่ถูกต้อง ที่ถูกต้องคือ “{right}”"),
            suggestions: vec![right.to_string()],
            lang: ProofLang::Th,
        });
    }
    for f in orthography(text) {
        if v.iter().any(|i| i.start < f.end && f.start < i.end) {
            continue;
        }
        // Mark and replace the whole word holding the mistake, so the suggestion reads as a word.
        // The segmenter finds the word in the corrected text (it can't in the mistyped one).
        let (start, end) = word_around(text, &f);
        if text.get(start..end).is_some_and(|w| crate::in_user_dictionary(&w.to_lowercase())) {
            continue;
        }
        let suggestions = match (&f.replacement, text.get(start..f.start), text.get(f.end..end)) {
            (Some(r), Some(before), Some(after)) => vec![format!("{before}{r}{after}")],
            _ => Vec::new(),
        };
        v.push(Issue { start, end, kind: IssueKind::Spelling, message: f.message.to_string(), suggestions, lang: ProofLang::Th });
    }
    v.sort_by_key(|i| (i.start, i.end));
    v.dedup_by(|a, b| a.start < b.end && b.start < a.end);
    v
}

/// The bytes of the word holding `f` in `text`: word boundaries around the fix, found in the text
/// with the fix applied, mapped back.
fn word_around(text: &str, f: &Fix) -> (usize, usize) {
    let (Some(r), Some(before), Some(after)) = (&f.replacement, text.get(..f.start), text.get(f.end..)) else {
        let (_, bounds) = boundaries_around(text, f.start);
        let start = bounds.iter().copied().filter(|b| *b <= f.start).max().unwrap_or(f.start);
        return (start, bounds.iter().copied().filter(|b| *b >= f.end).min().unwrap_or(f.end));
    };
    let fixed = format!("{before}{r}{after}");
    let fixed_end = f.start + r.len();
    let (_, bounds) = boundaries_around(&fixed, f.start);
    let start = bounds.iter().copied().filter(|b| *b <= f.start).max().unwrap_or(f.start);
    let end = bounds.iter().copied().filter(|b| *b >= fixed_end).min().unwrap_or(fixed_end);
    (start, end - fixed_end + f.end)
}

/// The correction of a word Thai proofing marked: the Royal Society spelling of a known
/// misspelling, or the word with its characters typed in order. Empty when there is none.
pub fn suggest(word: &str) -> Vec<String> {
    if let Some((_, right)) = misspellings().iter().find(|(w, _)| *w == word) {
        return vec![(*right).to_string()];
    }
    let fixes = orthography(word);
    if fixes.is_empty() {
        return Vec::new();
    }
    // Every fix applied, last first so earlier byte offsets stay valid.
    let mut fixed = word.to_string();
    for f in fixes.iter().rev() {
        let Some(r) = &f.replacement else { return Vec::new() };
        if fixed.get(f.start..f.end).is_none() {
            return Vec::new();
        }
        fixed.replace_range(f.start..f.end, r);
    }
    vec![fixed]
}

#[cfg(test)]
mod tests;
