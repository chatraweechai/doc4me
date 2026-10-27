//! Tools › AutoSummarize (Word 97–2003): the document's key sentences, found the way Word found
//! them — by how often their words come up in the document — on this computer, with no AI and
//! no online service. The summary is highlighted, put at the top of the document, or made into a
//! new document; the document's keywords and comments can be set from it. Words are found by the
//! Unicode word segmenter with its dictionaries, so Thai (written without spaces between words)
//! and CJK text are counted word by word, and a space between two Thai letters ends a sentence.

use std::collections::{BTreeMap, BTreeSet};

use doc4me_doc::para::OBJ;
use doc4me_doc::props::{CharProps, Highlight};
use doc4me_doc::{Block, Document, Paragraph, Path, Pos, StoryRef};
use serde_json::{Value, json};

use crate::{CmdError, CmdResult, CommandSpec, Selection, Session, p};

pub fn specs() -> Vec<CommandSpec> {
    vec![CommandSpec::new("tools.autoSummarize", "AutoSummarize", "Tools", auto_summarize).params(
        r#"{"type"?: "highlight|top|newDocument|none" (none: only report), "percent"?: 1-100 (of the sentences; default 25), "sentences"?: n, "words"?: n (at most that many words), "updateProperties"?: bool (keywords and comments from the summary)} → {"sentences": [string], "keywords": [string], "words": n, "of": n}"#,
    )]
}

/// Most sentences looked at (a very long document is summarised from its first ones).
const MAX_SENTENCES: usize = 20_000;

/// Words that say little about what a text is about (English, the language of most documents
/// Word's AutoSummarize was used on; other languages' frequent short words are left out by length).
const STOP_WORDS: &[&str] = &[
    "a",
    "about",
    "above",
    "after",
    "again",
    "against",
    "all",
    "also",
    "am",
    "an",
    "and",
    "any",
    "are",
    "as",
    "at",
    "be",
    "because",
    "been",
    "before",
    "being",
    "below",
    "between",
    "both",
    "but",
    "by",
    "can",
    "could",
    "did",
    "do",
    "does",
    "doing",
    "down",
    "during",
    "each",
    "few",
    "for",
    "from",
    "further",
    "had",
    "has",
    "have",
    "having",
    "he",
    "her",
    "here",
    "hers",
    "herself",
    "him",
    "himself",
    "his",
    "how",
    "i",
    "if",
    "in",
    "into",
    "is",
    "it",
    "its",
    "itself",
    "just",
    "may",
    "me",
    "might",
    "more",
    "most",
    "must",
    "my",
    "myself",
    "no",
    "nor",
    "not",
    "now",
    "of",
    "off",
    "on",
    "once",
    "only",
    "or",
    "other",
    "our",
    "ours",
    "ourselves",
    "out",
    "over",
    "own",
    "same",
    "shall",
    "she",
    "should",
    "so",
    "some",
    "such",
    "than",
    "that",
    "the",
    "their",
    "theirs",
    "them",
    "themselves",
    "then",
    "there",
    "these",
    "they",
    "this",
    "those",
    "through",
    "to",
    "too",
    "under",
    "until",
    "up",
    "upon",
    "us",
    "very",
    "was",
    "we",
    "were",
    "what",
    "when",
    "where",
    "which",
    "while",
    "who",
    "whom",
    "why",
    "will",
    "with",
    "within",
    "without",
    "would",
    "you",
    "your",
    "yours",
    "yourself",
    "yourselves",
];

/// Thai function words (the Thai text's "the", "and", "of"…), which say little on their own.
const THAI_STOP_WORDS: &[&str] = &[
    "กับ",
    "การ",
    "ก็",
    "ของ",
    "คือ",
    "ความ",
    "จะ",
    "จาก",
    "ซึ่ง",
    "ด้วย",
    "ได้",
    "ต้อง",
    "ถึง",
    "ทั้ง",
    "ที่",
    "นั้น",
    "นี้",
    "ใน",
    "แต่",
    "โดย",
    "และ",
    "เป็น",
    "เพื่อ",
    "มา",
    "มี",
    "เมื่อ",
    "ยัง",
    "แล้ว",
    "ว่า",
    "หรือ",
    "อยู่",
    "อย่าง",
    "อีก",
    "ไป",
    "ไม่",
    "ให้",
];

/// Thai, kana, CJK ideographs and Hangul: scripts whose words are short and unspaced.
fn short_word_script(c: char) -> bool {
    matches!(c, '\u{0E00}'..='\u{0E7F}' | '\u{3040}'..='\u{30FF}' | '\u{3400}'..='\u{9FFF}' | '\u{AC00}'..='\u{D7AF}' | '\u{F900}'..='\u{FAFF}')
}

/// A sentence of the body: where it is and its text.
#[derive(Clone, Debug)]
pub struct Sentence {
    pub start: Pos,
    pub end: Pos,
    pub text: String,
}

/// The body's sentences, in order: each paragraph split after `.`, `!`, `?`, `…` and the
/// ideographic stops, before a space or the paragraph's end, and at a space between two Thai
/// letters, where Thai ends a sentence (inline objects are left out).
pub fn sentences(doc: &Document) -> Vec<Sentence> {
    let mut out = Vec::new();
    for path in doc.para_paths(StoryRef::Body) {
        let Some(p) = doc.para(StoryRef::Body, &path) else { continue };
        split_para(p, &mut |a, b, text| {
            out.push(Sentence { start: Pos::new(StoryRef::Body, path.clone(), a), end: Pos::new(StoryRef::Body, path.clone(), b), text })
        });
        if out.len() >= MAX_SENTENCES {
            out.truncate(MAX_SENTENCES);
            break;
        }
    }
    out
}

fn split_para(p: &Paragraph, emit: &mut dyn FnMut(usize, usize, String)) {
    let t = &p.text;
    let thai = |c: char| ('\u{0E01}'..='\u{0E5B}').contains(&c);
    let mut start = 0;
    let mut prev: Option<char> = None;
    let mut chars = t.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        let stop = matches!(c, '.' | '!' | '?' | '…' | '。' | '！' | '？');
        let next = chars.peek().map(|(_, n)| *n);
        let next_space = next.is_none_or(char::is_whitespace);
        let thai_space = c == ' ' && prev.is_some_and(thai) && next.is_some_and(thai);
        if (stop && next_space) || thai_space || next.is_none() {
            let end = i + c.len_utf8();
            push_sentence(t, start, end, emit);
            start = end;
        }
        prev = Some(c);
    }
}

fn push_sentence(t: &str, a: usize, b: usize, emit: &mut dyn FnMut(usize, usize, String)) {
    let Some(raw) = t.get(a..b) else { return };
    let lead = raw.len() - raw.trim_start().len();
    let text: String = raw.trim().chars().filter(|c| *c != OBJ).collect();
    if text.chars().any(char::is_alphanumeric) {
        let a = a + lead;
        let b = a + raw.trim().len();
        emit(a, b, text);
    }
}

/// A sentence's significant words, lower-cased: the segmenter's letter words (numbers and
/// punctuation left out), at least three letters long (two in Thai, CJK and Hangul), not a
/// function word.
fn words(text: &str) -> Vec<String> {
    let segmenter = icu_segmenter::WordSegmenter::new_dictionary(icu_segmenter::options::WordBreakInvariantOptions::default());
    let mut out = Vec::new();
    let mut last = 0;
    for (b, kind) in segmenter.segment_str(text).iter_with_word_type() {
        let piece = text.get(last..b).unwrap_or("");
        last = b;
        if kind != icu_segmenter::options::WordType::Letter {
            continue;
        }
        let w = piece.trim_matches(|c| c == '\'' || c == '’').to_lowercase();
        let min = if w.chars().any(short_word_script) { 2 } else { 3 };
        if w.chars().count() >= min && !STOP_WORDS.contains(&w.as_str()) && !THAI_STOP_WORDS.contains(&w.as_str()) {
            out.push(w);
        }
    }
    out
}

/// How much of the document the summary keeps.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    Percent(u32),
    Sentences(usize),
    Words(usize),
}

/// The key sentences (indexes into `sents`, in document order) and the keywords (most frequent
/// significant words first).
pub fn summarize(sents: &[Sentence], len: Length) -> (Vec<usize>, Vec<String>) {
    let per: Vec<Vec<String>> = sents.iter().map(|s| words(&s.text)).collect();
    let mut freq: BTreeMap<&str, u32> = BTreeMap::new();
    for ws in &per {
        // A word counts once per sentence it is in.
        let uniq: BTreeSet<&str> = ws.iter().map(String::as_str).collect();
        for w in uniq {
            *freq.entry(w).or_default() += 1;
        }
    }
    // A word in only one sentence says nothing about the whole.
    let score = |ws: &[String]| -> f64 {
        let total: u32 = ws.iter().map(|w| freq.get(w.as_str()).copied().unwrap_or(0)).filter(|n| *n > 1).sum();
        f64::from(total) / (ws.len() as f64 + 1.0).sqrt()
    };
    let mut order: Vec<usize> = (0..sents.len()).collect();
    // Highest score first; earlier sentences first among equals (openings often say most).
    order.sort_by(|a, b| score(&per[*b]).total_cmp(&score(&per[*a])).then(a.cmp(b)));
    let mut chosen = Vec::new();
    let mut used_words = 0;
    for i in order {
        let n = sents[i].text.split_whitespace().count();
        let keep = match len {
            Length::Percent(pc) => chosen.len() < ((sents.len() * pc as usize).div_ceil(100)).max(1),
            Length::Sentences(k) => chosen.len() < k,
            Length::Words(k) => used_words + n <= k || (chosen.is_empty() && n <= k),
        };
        if !keep {
            if matches!(len, Length::Words(_)) {
                continue;
            }
            break;
        }
        chosen.push(i);
        used_words += n;
    }
    chosen.sort_unstable();
    let mut kw: Vec<(&str, u32)> = freq.into_iter().filter(|(_, n)| *n > 1).collect();
    kw.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    (chosen, kw.into_iter().take(5).map(|(w, _)| w.to_string()).collect())
}

fn auto_summarize(s: &mut Session, v: &Value) -> CmdResult {
    let kind = p::str(v, "type").unwrap_or("highlight");
    if !matches!(kind, "highlight" | "top" | "newDocument" | "none") {
        return Err(CmdError::Params("type: highlight, top, newDocument or none".into()));
    }
    let len = if let Some(n) = p::u64(v, "sentences") {
        Length::Sentences(n.clamp(1, MAX_SENTENCES as u64) as usize)
    } else if let Some(n) = p::u64(v, "words") {
        Length::Words(n.clamp(1, 1_000_000) as usize)
    } else {
        let pc = p::u64(v, "percent").unwrap_or(25);
        if !(1..=100).contains(&pc) {
            return Err(CmdError::Params("percent: 1–100".into()));
        }
        Length::Percent(pc as u32)
    };
    let sents = sentences(&s.doc);
    if sents.is_empty() {
        return Err(CmdError::Disabled("the document has no sentences to summarize".into()));
    }
    let (chosen, keywords) = summarize(&sents, len);
    let texts: Vec<String> = chosen.iter().map(|i| sents[*i].text.clone()).collect();
    let words: usize = texts.iter().map(|t| t.split_whitespace().count()).sum();
    let of: usize = sents.iter().map(|t| t.text.split_whitespace().count()).sum();
    let update = p::bool(v, "updateProperties").unwrap_or(false);
    match kind {
        "highlight" => {
            // Later ones first so the earlier positions stay where they are.
            for i in chosen.iter().rev() {
                let st = &sents[*i];
                s.doc.format_range(&st.start, &st.end, &|c: &mut CharProps| c.highlight = Some(Highlight::Yellow))?;
            }
        }
        "top" => {
            // A paragraph of its own in the Normal style, before whatever the document starts
            // with (a heading or a table keeps its own formatting).
            s.doc.insert_block(StoryRef::Body, &Path::top(0), Block::Para(Paragraph::with_text(&texts.join(" "), CharProps::default())))?;
            s.sel = Selection::caret(Pos::new(StoryRef::Body, Path::top(0), 0));
        }
        "newDocument" => {
            let mut d = Document::new();
            d.styles = s.doc.styles.clone();
            d.body = texts.iter().map(|t| std::sync::Arc::new(Block::Para(Paragraph::with_text(t, CharProps::default())))).collect();
            if d.body.is_empty() {
                d.body.push(std::sync::Arc::new(Block::Para(Paragraph::new())));
            }
            d.core.title = s.doc.core.title.clone();
            // A new, unsaved document in place of this one (the caller asks to save first).
            s.set_document(d);
            s.path = None;
            s.dirty = true;
        }
        _ if !update => s.only_read(),
        _ => {}
    }
    // Keywords and comments of the document the summary is in (the new one, for a new document).
    if update {
        s.doc.core.keywords = keywords.join(", ");
        s.doc.core.description = texts.join(" ");
    }
    Ok(json!({"sentences": texts, "keywords": keywords, "words": words, "of": of}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(paras: &[&str]) -> Document {
        let mut d = Document::from_text(paras[0]);
        for p in &paras[1..] {
            d.body.push(std::sync::Arc::new(doc4me_doc::Block::Para(Paragraph::with_text(p, CharProps::default()))));
        }
        d
    }

    #[test]
    fn sentences_split_after_stops() {
        let d = doc(&["Rust is fast. Is it safe? Yes! v1.2 ships…", "No stop here", "。", "แผงโซลาร์ผลิตไฟฟ้า ราคาลดลงมาก 2 ปี"]);
        let t: Vec<String> = sentences(&d).into_iter().map(|s| s.text).collect();
        assert_eq!(t, ["Rust is fast.", "Is it safe?", "Yes!", "v1.2 ships…", "No stop here", "แผงโซลาร์ผลิตไฟฟ้า", "ราคาลดลงมาก 2 ปี"]);
    }

    #[test]
    fn key_sentences_are_the_ones_about_the_subject() {
        let d = doc(&[
            "Solar panels turn sunlight into electricity.",
            "My cat likes the garden.",
            "Solar panels on roofs make electricity for homes.",
            "The weather was nice yesterday.",
            "Cheaper solar panels made electricity from sunlight common.",
            "Lunch was late.",
        ]);
        let s = sentences(&d);
        let (chosen, kw) = summarize(&s, Length::Sentences(2));
        let picked: Vec<&str> = chosen.iter().map(|i| s[*i].text.as_str()).collect();
        assert!(picked.iter().all(|t| t.contains("Solar") || t.contains("solar")), "{picked:?}");
        assert_eq!(&kw[..3], ["electricity", "panels", "solar"]);
        // Percent and word budgets.
        assert_eq!(summarize(&s, Length::Percent(50)).0.len(), 3);
        let (w, _) = summarize(&s, Length::Words(8));
        assert!(w.iter().map(|i| s[*i].text.split_whitespace().count()).sum::<usize>() <= 8);
        assert!(!w.is_empty());
    }

    #[test]
    fn thai_is_counted_word_by_word() {
        // Unspaced Thai: the segmenter's words, without the function words.
        let w = words("การผลิตไฟฟ้าจากแสงอาทิตย์และลม");
        assert!(w.contains(&"ไฟฟ้า".to_string()) && w.contains(&"แสง".to_string()), "{w:?}");
        assert!(!w.iter().any(|x| x == "และ" || x == "การ" || x == "จาก"), "{w:?}");
        let d = doc(&[
            "แผงโซลาร์ผลิตไฟฟ้าจากแสงแดด",
            "แมวของฉันชอบสวน",
            "แผงโซลาร์บนหลังคาผลิตไฟฟ้าให้บ้าน",
            "เมื่อวานอากาศดี",
            "แผงโซลาร์ราคาถูกทำให้ไฟฟ้าจากแสงแดดแพร่หลาย",
        ]);
        let s = sentences(&d);
        let (chosen, kw) = summarize(&s, Length::Sentences(2));
        assert!(chosen.iter().all(|i| s[*i].text.contains("แผงโซลาร์")), "{chosen:?}");
        assert!(kw.contains(&"ไฟฟ้า".to_string()), "{kw:?}");
    }
}
