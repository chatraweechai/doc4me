//! AutoFormat (Ctrl+Alt+K): Word's AutoFormat run over the whole document, or over the selected
//! paragraphs, with the options on AutoCorrect Options › AutoFormat — heading styles for short
//! lines followed by body text, lists from typed labels ("1. ", "* "), the Body Text style for
//! the other paragraphs, and the replacements AutoFormat As You Type makes as you type (smart
//! quotes, ordinals, fractions, dashes, *bold* and _italic_, links). "AutoFormat and review each
//! change" keeps the document as it was so each changed paragraph can be rejected, or all of
//! them (Word's Review AutoFormat Changes).

use doc4me_doc::numbering::ListKind;
use doc4me_doc::para::OBJ;
use doc4me_doc::props::NumRef;
use doc4me_doc::props::VertAlign;
use doc4me_doc::{Block, Document, ParaProps, Path, Pos, StoryRef, Style, StyleKind};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;

use crate::{CmdError, CmdResult, CommandSpec, Selection, Session, p};

/// AutoCorrect Options › AutoFormat: what AutoFormat changes (Word's defaults: all on).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AutoFormatPrefs {
    /// Apply: Built-in Heading styles.
    pub headings: bool,
    /// Apply: List styles (numbered lists from "1. ", "a) "…).
    pub lists: bool,
    /// Apply: Automatic bulleted lists ("* ", "- "…).
    pub bullets: bool,
    /// Apply: Other paragraph styles (Body Text).
    pub other_styles: bool,
    /// Replace: "Straight quotes" with “smart quotes”.
    pub quotes: bool,
    /// Replace: Ordinals (1st) with superscript.
    pub ordinals: bool,
    /// Replace: Fractions (1/2) with fraction character (½).
    pub fractions: bool,
    /// Replace: Hyphens (--) with dash (—).
    pub dashes: bool,
    /// Replace: *Bold* and _italic_ with real formatting.
    pub emphasis: bool,
    /// Replace: Internet and network paths with hyperlinks.
    pub links: bool,
    /// Preserve: Styles (paragraphs that already have a style keep it).
    pub preserve_styles: bool,
}

impl Default for AutoFormatPrefs {
    fn default() -> Self {
        AutoFormatPrefs {
            headings: true,
            lists: true,
            bullets: true,
            other_styles: true,
            quotes: true,
            ordinals: true,
            fractions: true,
            dashes: true,
            emphasis: true,
            links: true,
            preserve_styles: true,
        }
    }
}

impl AutoFormatPrefs {
    /// The `tools.autocorrect` `autoFormat` names of the switches.
    pub fn switches(&mut self) -> [(&'static str, &mut bool); 11] {
        [
            ("headings", &mut self.headings),
            ("lists", &mut self.lists),
            ("bullets", &mut self.bullets),
            ("otherStyles", &mut self.other_styles),
            ("quotes", &mut self.quotes),
            ("ordinals", &mut self.ordinals),
            ("fractions", &mut self.fractions),
            ("dashes", &mut self.dashes),
            ("emphasis", &mut self.emphasis),
            ("links", &mut self.links),
            ("preserveStyles", &mut self.preserve_styles),
        ]
    }
}

/// "AutoFormat and review each change": the document before AutoFormat and the paragraphs it
/// changed (top-level body blocks: AutoFormat adds and removes none, so their places hold).
#[derive(Clone, Debug)]
pub struct AutoFormatReview {
    pub before: Document,
    /// Indexes into the body of the changed blocks, in document order.
    pub changed: Vec<usize>,
    /// The changed blocks as AutoFormat left them, and the body's length: an edit to one of
    /// them, or one that adds or removes blocks, ends the review.
    pub after: Vec<Arc<Block>>,
    pub len: usize,
}

impl AutoFormatReview {
    /// Whether the document is still as AutoFormat (and the rejections since) left it.
    fn current(&self, doc: &Document) -> bool {
        doc.body.len() == self.len && self.changed.iter().zip(&self.after).all(|(&i, b)| doc.body.get(i) == Some(b))
    }
}

/// Longest line taken for a heading, in characters, and most words.
const HEADING_CHARS: usize = 80;
const HEADING_WORDS: usize = 12;
/// A paragraph at least this long is body text even without a full stop (Thai and other
/// languages don't end sentences with one).
const BODY_CHARS: usize = 80;

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("format.autoFormat", "AutoFormat", "AutoCorrect Options › AutoFormat", auto_format)
            .params(r#"{"review"?: bool (keep the changes for format.autoFormatReview)} — the selected paragraphs, or the whole document"#)
            .key("Mod+Alt+K"),
        CommandSpec::new("format.autoFormatReview", "Review AutoFormat Changes", "AutoCorrect Options › AutoFormat", review)
            .params(r#"{"action"?: "acceptAll|rejectAll|reject|next|previous" (reject: the change at the caret, or `index`), "index"?: int} → {changes: [{index, block, before, after}]} (no action: the list)"#),
    ]
}

fn auto_format(s: &mut Session, v: &Value) -> CmdResult {
    let o = s.prefs.autocorrect.autoformat.clone();
    let before = s.doc.clone();
    // The selected top-level paragraphs, or all of them.
    let (a, b) = s.sel.ordered();
    let range = if s.sel.is_collapsed() || a.story != StoryRef::Body {
        0..s.doc.body.len()
    } else {
        top(&a.path)..top(&b.path).saturating_add(1).min(s.doc.body.len())
    };
    // Replacements in every paragraph of the range, tables included.
    for path in s.doc.para_paths(StoryRef::Body) {
        if range.contains(&top(&path)) {
            replace_in(s, &path, &o)?;
        }
    }
    styles(s, range, &o)?;
    let changed: Vec<usize> = (0..s.doc.body.len()).filter(|&i| before.body.get(i) != s.doc.body.get(i)).collect();
    if !changed.is_empty() {
        s.touch();
        s.clamp_selection();
    }
    let n = changed.len();
    if n == 0 {
        // Nothing to format: no undo step, the document isn't marked changed.
        s.only_read();
    }
    let review = p::bool(v, "review").unwrap_or(false) && n > 0;
    let after = changed.iter().filter_map(|&i| s.doc.body.get(i).cloned()).collect();
    s.autoformat_review = review.then_some(AutoFormatReview { before, changed, after, len: s.doc.body.len() });
    Ok(json!({"changed": n, "review": review}))
}

/// The top-level block a path is in.
fn top(p: &Path) -> usize {
    p.0.first().copied().unwrap_or(0) as usize
}

// ---------------------------------------------------------------------------------------------
// Replacements

/// One replacement in a paragraph: bytes `a..b` become `text` (with the formatting of the first
/// character), then `fmt` applies to the new text.
struct Edit {
    a: usize,
    b: usize,
    text: String,
    fmt: Option<Fmt>,
}

#[derive(Clone)]
enum Fmt {
    Bold,
    Italic,
    Superscript,
    Link(String),
}

fn replace_in(s: &mut Session, path: &Path, o: &AutoFormatPrefs) -> Result<(), CmdError> {
    let rules: [(bool, fn(&str) -> Vec<Edit>); 6] =
        [(o.quotes, quotes), (o.dashes, dashes), (o.fractions, fractions), (o.ordinals, ordinals), (o.emphasis, emphasis), (o.links, links)];
    for (on, rule) in rules {
        if !on {
            continue;
        }
        let Some(para) = s.doc.para(StoryRef::Body, path) else { return Ok(()) };
        // Text in a link stays as it is (an address with "--" or "_" in it), and so does text
        // deleted under Track Changes.
        let kept = |a: usize, b: usize| {
            (a..b.max(a + 1)).any(|i| para.text.is_char_boundary(i) && { let c = para.props_of_char(i); c.link.is_some() || c.del.is_some() })
        };
        let mut edits: Vec<Edit> = rule(&para.text).into_iter().filter(|e| !para.text[e.a..e.b].contains(OBJ) && !kept(e.a, e.b)).collect();
        edits.sort_by_key(|e| std::cmp::Reverse(e.a));
        for e in edits {
            let pos = |off: usize| Pos { story: StoryRef::Body, path: path.clone(), off };
            let para = s.doc.para_mut(StoryRef::Body, path)?;
            let end = if e.text != para.text[e.a..e.b] {
                let props = para.props_of_char(e.a).clone();
                para.delete(e.a, e.b)?;
                para.insert_text(e.a, &e.text, &props)?;
                e.a + e.text.len()
            } else {
                e.b
            };
            if let Some(f) = e.fmt {
                s.doc.format_range(&pos(e.a), &pos(end), &|c| match &f {
                    Fmt::Bold => c.bold = Some(true),
                    Fmt::Italic => c.italic = Some(true),
                    Fmt::Superscript => c.vert_align = Some(VertAlign::Superscript),
                    Fmt::Link(href) => {
                        c.link = Some(href.clone());
                        c.style = Some("Hyperlink".into());
                    }
                })?;
            }
        }
    }
    Ok(())
}

/// Characters a quote after which opens (and the start of the paragraph).
fn opens_after(prev: Option<char>) -> bool {
    prev.is_none_or(|p| p.is_whitespace() || "([{<\u{2014}\u{2013}".contains(p))
}

fn quotes(t: &str) -> Vec<Edit> {
    let mut out = Vec::new();
    let mut prev = None;
    for (i, c) in t.char_indices() {
        let rep = match c {
            '"' => Some(if opens_after(prev) { "\u{201C}" } else { "\u{201D}" }),
            '\'' => Some(if opens_after(prev) { "\u{2018}" } else { "\u{2019}" }),
            _ => None,
        };
        if let Some(r) = rep {
            out.push(Edit { a: i, b: i + 1, text: r.into(), fmt: None });
        }
        prev = Some(c);
    }
    out
}

/// "word -- word" and "word - word" → en dash, "word--word" → em dash.
fn dashes(t: &str) -> Vec<Edit> {
    let mut out = Vec::new();
    let word = |c: Option<char>| c.is_some_and(char::is_alphanumeric);
    let mut i = 0;
    while let Some(k) = t[i..].find('-').map(|k| k + i) {
        let before = t[..k].chars().next_back();
        let run = t[k..].chars().take_while(|c| *c == '-').count();
        let after = t[k + run..].chars().next();
        let spaced = before == Some(' ') && after == Some(' ') && word(t[..k - 1].chars().next_back()) && word(t[k + run + 1..].chars().next());
        if run == 2 && word(before) && word(after) {
            out.push(Edit { a: k, b: k + 2, text: "\u{2014}".into(), fmt: None });
        } else if (run == 1 || run == 2) && spaced {
            out.push(Edit { a: k, b: k + run, text: "\u{2013}".into(), fmt: None });
        }
        i = k + run;
    }
    out
}

fn fractions(t: &str) -> Vec<Edit> {
    let mut out = Vec::new();
    for (f, c) in [("1/2", "½"), ("1/4", "¼"), ("3/4", "¾")] {
        for (k, _) in t.match_indices(f) {
            let before = t[..k].chars().next_back();
            let after = t[k + 3..].chars().next();
            let edge = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric() && c != '/' && c != '.' && c != ',');
            if edge(before) && edge(after) {
                out.push(Edit { a: k, b: k + 3, text: c.into(), fmt: None });
            }
        }
    }
    out
}

/// Words: (start, end) of each run of characters between whitespace.
fn words(t: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut i = 0;
    std::iter::from_fn(move || {
        let start = i + t[i..].find(|c: char| !c.is_whitespace())?;
        let end = t[start..].find(char::is_whitespace).map_or(t.len(), |k| k + start);
        i = end;
        Some((start, end))
    })
}

fn ordinals(t: &str) -> Vec<Edit> {
    let mut out = Vec::new();
    for (a, b) in words(t) {
        let w = t[a..b].trim_end_matches(|c: char| ".,;:!?)]\"'”’".contains(c));
        if let Some(n) = super::tools::ordinal_suffix(w) {
            let end = a + w.len();
            out.push(Edit { a: end - n, b: end, text: t[end - n..end].into(), fmt: Some(Fmt::Superscript) });
        }
    }
    out
}

/// *bold* and _italic_: the markers go, the text between takes the formatting. A marker opens
/// after a space or the paragraph's start and closes before a space, punctuation or the end,
/// with no space just inside either.
fn emphasis(t: &str) -> Vec<Edit> {
    // Marked stretches: (opening marker, closing marker, formatting).
    let mut pairs: Vec<(usize, usize, Fmt)> = Vec::new();
    for (m, fmt) in [('*', Fmt::Bold), ('_', Fmt::Italic)] {
        let mut i = 0;
        while let Some(k) = t[i..].find(m).map(|k| k + i) {
            i = k + 1;
            let opens = t[..k].chars().next_back().is_none_or(|c| c.is_whitespace() || "([{\"“'‘".contains(c));
            let first = t[k + 1..].chars().next();
            if !opens || first.is_none_or(|c| c.is_whitespace() || c == m) {
                continue;
            }
            let Some(close) = t[k + 1..].find(m).map(|c| c + k + 1) else { break };
            let after = t[close + 1..].chars().next();
            if t[k + 1..close].ends_with(char::is_whitespace) || after.is_some_and(|c| c.is_alphanumeric() || c == m) {
                continue;
            }
            pairs.push((k, close, fmt.clone()));
            i = close + 1;
        }
    }
    // Overlapping stretches (a * inside an _..._) are left as they are.
    pairs.sort_by_key(|p| p.0);
    let mut out = Vec::new();
    let mut end = 0;
    for (k, close, fmt) in pairs {
        if k < end {
            continue;
        }
        end = close + 1;
        out.push(Edit { a: k, b: k + 1, text: String::new(), fmt: None });
        out.push(Edit { a: k + 1, b: close, text: t[k + 1..close].into(), fmt: Some(fmt) });
        out.push(Edit { a: close, b: close + 1, text: String::new(), fmt: None });
    }
    out
}

fn links(t: &str) -> Vec<Edit> {
    let mut out = Vec::new();
    for (a, b) in words(t) {
        let w = &t[a..b];
        let lower = w.to_ascii_lowercase();
        if !(lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("www.")) || w.len() <= 8 {
            continue;
        }
        let url = w.trim_end_matches(|c: char| ".,;:!?)]\"'”’".contains(c));
        let href = if lower.starts_with("www.") { format!("http://{url}") } else { url.to_string() };
        out.push(Edit { a, b: a + url.len(), text: url.into(), fmt: Some(Fmt::Link(href)) });
    }
    out
}

// ---------------------------------------------------------------------------------------------
// Paragraph styles

/// A typed list label: its kind of list and its number (1 for the first item).
fn label(text: &str, continuing: Option<ListKind>) -> Option<(ListKind, u32, usize)> {
    let (lab, rest) = text.split_once([' ', '\t'])?;
    if rest.trim().is_empty() {
        return None;
    }
    let skip = lab.len() + 1 + (rest.len() - rest.trim_start().len());
    let bullet = match lab {
        "*" | "•" => Some(ListKind::Bullet),
        "-" => Some(ListKind::BulletChar('-')),
        ">" => Some(ListKind::BulletChar('\u{27A2}')),
        "->" => Some(ListKind::BulletChar('\u{2794}')),
        "=>" => Some(ListKind::BulletChar('\u{21E8}')),
        _ => None,
    };
    if let Some(k) = bullet {
        return Some((k, 1, skip));
    }
    let (body, paren) = match (lab.strip_suffix('.'), lab.strip_suffix(')')) {
        (Some(b), _) => (b, false),
        (_, Some(b)) => (b, true),
        _ => return None,
    };
    if body.is_empty() || body.chars().count() > 6 {
        return None;
    }
    if body.bytes().all(|c| c.is_ascii_digit()) {
        let n = body.parse().ok()?;
        return Some((if paren { ListKind::NumberedParen } else { ListKind::Numbered }, n, skip));
    }
    let roman = roman_value(body);
    let letter = (body.len() == 1).then(|| body.as_bytes()[0]).filter(u8::is_ascii_alphabetic);
    // "i." is a letter in a list of letters, else a Roman one.
    let as_letter = |c: u8| -> Option<(ListKind, u32, usize)> {
        let kind = match (c.is_ascii_lowercase(), paren) {
            (true, false) => ListKind::LowerLetterDot,
            (true, true) => ListKind::LowerLetter,
            (false, false) => ListKind::UpperLetter,
            (false, true) => return None,
        };
        Some((kind, u32::from(c.to_ascii_lowercase() - b'a') + 1, skip))
    };
    match (roman, letter) {
        (Some(n), Some(c)) if !matches!(continuing, Some(ListKind::LowerLetterDot | ListKind::UpperLetter | ListKind::LowerLetter)) => {
            roman_kind(body, paren).map(|k| (k, n, skip)).or_else(|| as_letter(c))
        }
        (_, Some(c)) => as_letter(c),
        (Some(n), None) => roman_kind(body, paren).map(|k| (k, n, skip)),
        _ => None,
    }
}

fn roman_kind(body: &str, paren: bool) -> Option<ListKind> {
    match (body.bytes().all(|c| c.is_ascii_lowercase()), paren) {
        (true, false) => Some(ListKind::LowerRoman),
        (false, false) => Some(ListKind::Outline),
        _ => None,
    }
}

/// i … xxxix (or I … XXXIX).
fn roman_value(s: &str) -> Option<u32> {
    let lower = s.to_ascii_lowercase();
    if s != lower && s != s.to_ascii_uppercase() {
        return None;
    }
    (1..40).find(|&n| roman(n) == lower)
}

fn roman(mut n: u32) -> String {
    let mut out = String::new();
    for (v, r) in [(10, "x"), (9, "ix"), (5, "v"), (4, "iv"), (1, "i")] {
        while n >= v {
            out.push_str(r);
            n -= v;
        }
    }
    out
}

fn terminal(t: &str) -> bool {
    t.trim_end().trim_end_matches(|c: char| "\"'”’)]".contains(c)).ends_with(['.', ',', ';', ':', '!', '?', '…'])
}

/// A body paragraph: ends a sentence, or is long.
fn body_like(t: &str) -> bool {
    terminal(t) || t.trim().chars().count() > BODY_CHARS
}

/// The heading level for a short line, if it reads as one: "2.1 Scope" is level 2; a line in
/// capitals is level 1, and a mixed-case one is level 2 once the document has had one in
/// capitals (level 1 otherwise).
fn heading(t: &str, caps_seen: bool) -> Option<u8> {
    let t = t.trim();
    let first = t.chars().next()?;
    if t.chars().count() > HEADING_CHARS || t.split_whitespace().count() > HEADING_WORDS || terminal(t) || !t.chars().any(char::is_alphabetic) {
        return None;
    }
    // Caseless scripts (Thai…) start a heading with any letter.
    if !(first.is_uppercase() || first.is_ascii_digit() || (first.is_alphabetic() && !first.is_lowercase())) {
        return None;
    }
    if first.is_ascii_digit() {
        let (num, rest) = t.split_once(char::is_whitespace)?;
        let parts: Vec<&str> = num.trim_end_matches('.').split('.').collect();
        if rest.trim().is_empty() || !parts.iter().all(|p| !p.is_empty() && p.bytes().all(|c| c.is_ascii_digit())) {
            return None;
        }
        return Some(parts.len().min(9) as u8);
    }
    let cased: Vec<char> = t.chars().filter(|c| c.is_uppercase() || c.is_lowercase()).collect();
    if cased.len() >= 2 && cased.iter().all(|c| c.is_uppercase()) {
        return Some(1);
    }
    Some(if caps_seen { 2 } else { 1 })
}

fn plain_style(style: Option<&str>) -> bool {
    style.is_none_or(|s| s == "Normal")
}

fn styles(s: &mut Session, range: std::ops::Range<usize>, o: &AutoFormatPrefs) -> Result<(), CmdError> {
    let texts: Vec<Option<String>> = s.doc.body.iter().map(|b| b.as_para().map(|p| p.plain_text())).collect();
    let mut caps_seen = false;
    let mut list: Option<(ListKind, u32, u32)> = None;
    for i in range {
        let Some(Some(text)) = texts.get(i) else {
            list = None;
            continue;
        };
        let Some(para) = s.doc.body.get(i).and_then(|b| b.as_para()) else { continue };
        let free = plain_style(para.props.style.as_deref()) || !o.preserve_styles;
        let listed = para.props.numbering.is_some_and(|n| n.num != 0);
        if text.trim().is_empty() || !free || listed {
            list = None;
            continue;
        }
        // Lists: a run of labels counting up from the first ("1.", "2." …), or of bullets.
        // The label is read from the paragraph's own text (offsets count its objects; a
        // paragraph that starts with a bookmark or field isn't taken as a list).
        let continuing = list.map(|(k, _, _)| k);
        if let Some((kind, n, skip)) = label(&para.text, continuing) {
            let on = if kind.is_bullet() { o.bullets } else { o.lists };
            let num = match list {
                Some((k, last, num)) if k == kind && (kind.is_bullet() || n == last + 1) => Some(num),
                _ if kind.is_bullet() => Some(s.doc.numbering.find_kind(kind).unwrap_or_else(|| s.doc.numbering.add_list(kind))),
                _ if n == 1 => Some(s.doc.numbering.add_list(kind)),
                _ => None,
            };
            if let (true, Some(num)) = (on, num) {
                let para = s.doc.para_mut(StoryRef::Body, &Path::top(i))?;
                para.delete(0, skip)?;
                para.props.numbering = Some(NumRef { num, level: 0 });
                para.props.style = Some("ListParagraph".into());
                para.touch();
                list = Some((kind, n, num));
                continue;
            }
        }
        list = None;
        // Headings: a short line between body text (or the start) and body text.
        let next = texts[i + 1..].iter().flatten().find(|t| !t.trim().is_empty());
        // The paragraph before (past empty ones) is body text or a heading, not another short
        // line (an address, a list typed without labels).
        let prev = (0..i).rev().find(|&j| texts[j].as_ref().is_none_or(|t| !t.trim().is_empty()));
        let prev_ok = prev.is_none_or(|j| {
            texts[j].as_deref().is_some_and(body_like)
                || s.doc.body.get(j).and_then(|b| b.as_para()).and_then(|p| p.props.style.as_deref()).is_some_and(|st| st.starts_with("Heading"))
        });
        if o.headings
            && let Some(level) = heading(text, caps_seen)
            && next.is_some_and(|t| body_like(t) && label(t, None).is_none())
            && prev_ok
        {
            if level == 1 && !text.chars().any(char::is_lowercase) && text.chars().any(char::is_uppercase) {
                caps_seen = true;
            }
            super::para::set_heading(s, StoryRef::Body, &Path::top(i), Some(level))?;
            continue;
        }
        if o.other_styles && plain_style(s.doc.body.get(i).and_then(|b| b.as_para()).and_then(|p| p.props.style.as_deref())) {
            ensure_body_text(s);
            let para = s.doc.para_mut(StoryRef::Body, &Path::top(i))?;
            para.props.style = Some("BodyText".into());
            para.touch();
        }
    }
    Ok(())
}

/// Word's built-in Body Text style (Normal with 6 pt after), for documents without it.
fn ensure_body_text(s: &mut Session) {
    if s.doc.styles.get("BodyText").is_none() {
        s.doc.styles.upsert(Style {
            id: "BodyText".into(),
            name: "Body Text".into(),
            kind: StyleKind::Paragraph,
            based_on: Some("Normal".into()),
            para: ParaProps { space_after: Some(6.0), ..Default::default() },
            priority: Some(99),
            ..Default::default()
        });
    }
}

// ---------------------------------------------------------------------------------------------
// Review AutoFormat Changes

fn review(s: &mut Session, v: &Value) -> CmdResult {
    let Some(r) = s.autoformat_review.as_ref() else {
        return Err(CmdError::Failed("there are no AutoFormat changes to review (run AutoFormat with review)".into()));
    };
    if !r.current(&s.doc) {
        s.autoformat_review = None;
        return Err(CmdError::Failed("the document changed after AutoFormat, so its changes can't be reviewed any more".into()));
    }
    let changes = |s: &Session| -> Value {
        let Some(r) = &s.autoformat_review else { return json!([]) };
        let text = |d: &Document, i: usize| d.body.get(i).and_then(|b| b.as_para()).map(|p| p.plain_text()).unwrap_or_default();
        r.changed.iter().enumerate().map(|(k, &i)| json!({"index": k, "block": i, "before": text(&r.before, i), "after": text(&s.doc, i)})).collect()
    };
    let caret = top(&s.sel.focus.path);
    let action = p::str(v, "action");
    if matches!(action, None | Some("acceptAll" | "next" | "previous")) {
        // The changes stay as they are: no undo step.
        s.only_read();
    }
    match action {
        None => {}
        Some("acceptAll") => {
            s.autoformat_review = None;
            return Ok(json!({"changes": []}));
        }
        Some("rejectAll") => {
            let before = s.autoformat_review.take().map(|r| r.before).unwrap_or_default();
            s.doc = before;
            s.touch();
            s.clamp_selection();
            return Ok(json!({"changes": []}));
        }
        Some("reject") => {
            let r = s.autoformat_review.as_ref().ok_or_else(|| CmdError::Failed("no AutoFormat review".into()))?;
            let k = match p::u64(v, "index") {
                Some(k) => k as usize,
                None => r
                    .changed
                    .iter()
                    .position(|&i| i == caret)
                    .ok_or_else(|| CmdError::Failed("the paragraph at the insertion point wasn't changed by AutoFormat".into()))?,
            };
            let i = *r.changed.get(k).ok_or_else(|| CmdError::Params(format!("no change {k}")))?;
            // The paragraph as it was (a list or style AutoFormat added stays defined, unused).
            let old = r.before.body.get(i).cloned().ok_or_else(|| CmdError::Failed("that change is gone".into()))?;
            if let Some(b) = s.doc.body.get_mut(i) {
                *b = old;
            }
            s.touch();
            s.clamp_selection();
            if let Some(r) = s.autoformat_review.as_mut() {
                r.changed.remove(k);
                r.after.remove(k);
                if r.changed.is_empty() {
                    s.autoformat_review = None;
                }
            }
        }
        Some(dir @ ("next" | "previous")) => {
            let r = s.autoformat_review.as_ref().ok_or_else(|| CmdError::Failed("no AutoFormat review".into()))?;
            let found = if dir == "next" {
                r.changed.iter().copied().find(|&i| i > caret || (i == caret && s.sel.is_collapsed())).or(r.changed.first().copied())
            } else {
                r.changed.iter().rev().copied().find(|&i| i < caret).or(r.changed.last().copied())
            };
            if let Some(i) = found {
                let len = s.doc.body.get(i).and_then(|b| b.as_para()).map_or(0, |p| p.len());
                s.sel = Selection {
                    anchor: Pos { story: StoryRef::Body, path: Path::top(i), off: 0 },
                    focus: Pos { story: StoryRef::Body, path: Path::top(i), off: len },
                };
            }
        }
        Some(other) => return Err(CmdError::Params(format!("unknown action {other:?}"))),
    }
    Ok(json!({"changes": changes(s)}))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(paras: &[&str]) -> Session {
        Session::new(Document::from_text(&paras.join("\n")))
    }

    fn texts(s: &Session) -> Vec<String> {
        s.doc.body.iter().filter_map(|b| b.as_para()).map(|p| p.plain_text()).collect()
    }

    fn style(s: &Session, i: usize) -> Option<String> {
        s.doc.body.get(i).and_then(|b| b.as_para()).and_then(|p| p.props.style.clone())
    }

    /// ks-132: Ctrl+Alt+K formats the document: headings, lists, Body Text and the replacements.
    #[test]
    fn autoformat_styles_and_replaces() {
        let mut s = doc(&[
            "INTRODUCTION",
            "This report -- the first one -- says \"hello\" to the 1st reader and *really* means _it_ at www.example.org.",
            "Scope of work",
            "It covers 1/2 of the year--and more.",
            "1. Plan",
            "2. Build",
            "3. Ship",
            "* apples",
            "* pears",
            "Sincerely",
        ]);
        let reg = s.registry.clone();
        assert_eq!(s.keymap.resolve(&reg, "Mod+Alt+K").map(|c| c.id), Some("format.autoFormat"));
        let r = s.run("format.autoFormat", &json!({})).unwrap();
        assert_eq!(r["review"], false);
        let t = texts(&s);
        assert_eq!(
            t[1],
            "This report \u{2013} the first one \u{2013} says \u{201C}hello\u{201D} to the 1st reader and really means it at www.example.org."
        );
        assert_eq!(t[3], "It covers ½ of the year\u{2014}and more.");
        assert_eq!(style(&s, 0).as_deref(), Some("Heading1"), "a line in capitals");
        assert_eq!(style(&s, 2).as_deref(), Some("Heading2"), "mixed case after a heading in capitals");
        assert_eq!(style(&s, 1).as_deref(), Some("BodyText"));
        assert!(s.doc.styles.get("BodyText").is_some_and(|st| st.para.space_after == Some(6.0)));
        assert_eq!(&t[4..9], ["Plan", "Build", "Ship", "apples", "pears"]);
        let num = |i: usize| s.doc.body[i].as_para().and_then(|p| p.props.numbering).map(|n| n.num);
        assert!(num(4).is_some() && num(4) == num(5) && num(5) == num(6), "one numbered list");
        assert!(num(7).is_some() && num(7) == num(8) && num(7) != num(4), "one bulleted list");
        // "Sincerely" has no body text after it: it stays body text.
        assert_eq!(style(&s, 9).as_deref(), Some("BodyText"));
        let p1 = s.doc.body[1].as_para().unwrap();
        let at = |w: &str| p1.text.find(w).unwrap();
        assert_eq!(p1.props_of_char(at("st reader")).vert_align, Some(VertAlign::Superscript));
        assert_eq!(p1.props_of_char(at("really")).bold, Some(true));
        assert_eq!(p1.props_of_char(at("it at")).italic, Some(true));
        assert_eq!(p1.props_of_char(at("www.")).link.as_deref(), Some("http://www.example.org"));
        assert_ne!(p1.props_of_char(at(".org.") + 4).link.as_deref(), Some("http://www.example.org"), "the full stop stays outside");
        // One undo step takes it all back.
        s.run("edit.undo", &json!({})).unwrap();
        assert_eq!(
            texts(&s)[0..2],
            [
                "INTRODUCTION".to_string(),
                "This report -- the first one -- says \"hello\" to the 1st reader and *really* means _it_ at www.example.org.".to_string()
            ]
        );
    }

    #[test]
    fn autoformat_options_and_preserved_styles() {
        let mut s = doc(&["Title line", "Body text that ends here.", "\"quoted\""]);
        s.doc.para_mut(StoryRef::Body, &Path::top(2)).unwrap().props.style = Some("Quote".into());
        let mut o = AutoFormatPrefs::default();
        for (_, b) in o.switches() {
            *b = false;
        }
        o.quotes = true;
        s.prefs.autocorrect.autoformat = o;
        s.sel = Selection::caret(Pos::body(0, 0));
        s.run("format.autoFormat", &json!({})).unwrap();
        assert_eq!(texts(&s)[2], "\u{201C}quoted\u{201D}");
        assert_eq!(style(&s, 0), None, "headings off");
        assert_eq!(style(&s, 1), None, "other styles off");
        assert_eq!(style(&s, 2).as_deref(), Some("Quote"));
        // Preserve Styles: a paragraph with a style keeps it.
        s.prefs.autocorrect.autoformat = AutoFormatPrefs::default();
        let mut s2 = doc(&["Title line", "Body text that ends here."]);
        s2.doc.para_mut(StoryRef::Body, &Path::top(0)).unwrap().props.style = Some("Title".into());
        s2.run("format.autoFormat", &json!({})).unwrap();
        assert_eq!(style(&s2, 0).as_deref(), Some("Title"));
        assert_eq!(style(&s2, 1).as_deref(), Some("BodyText"));
        // Off, the heading is made.
        s2.run("edit.undo", &json!({})).unwrap();
        s2.prefs.autocorrect.autoformat.preserve_styles = false;
        s2.run("format.autoFormat", &json!({})).unwrap();
        assert_eq!(style(&s2, 0).as_deref(), Some("Heading1"));
    }

    /// A paragraph that starts with a bookmark keeps it: offsets count objects.
    #[test]
    fn objects_are_left_alone() {
        let mut s = doc(&["1. Plan with a \"quote\"", "2. Build"]);
        s.sel = Selection::caret(Pos::body(0, 0));
        s.run("insert.bookmark", &json!({"name": "start"})).unwrap();
        s.sel = Selection::caret(Pos::body(0, 0));
        s.run("format.autoFormat", &json!({})).unwrap();
        assert!(s.doc.bookmarks().iter().any(|(n, _)| n == "start"));
        assert!(texts(&s)[0].contains("\u{201C}quote\u{201D}"));
        assert!(texts(&s)[0].starts_with("1. Plan"), "not taken as a list: {:?}", texts(&s));
    }

    #[test]
    fn labels_and_headings() {
        assert_eq!(label("1. Plan", None).map(|(k, n, _)| (k, n)), Some((ListKind::Numbered, 1)));
        assert_eq!(label("iii. Third", None).map(|(k, n, _)| (k, n)), Some((ListKind::LowerRoman, 3)));
        assert_eq!(label("i. After h", Some(ListKind::LowerLetterDot)).map(|(k, n, _)| (k, n)), Some((ListKind::LowerLetterDot, 9)));
        assert_eq!(label("b) Two", None).map(|(k, n, _)| (k, n)), Some((ListKind::LowerLetter, 2)));
        assert_eq!(label("IV. Four", None).map(|(k, n, _)| (k, n)), Some((ListKind::Outline, 4)));
        assert!(label("1.", None).is_none() && label("e.g. this", None).is_none() && label("Mr. Smith", None).is_none());
        assert_eq!(heading("2.1 Scope", false), Some(2));
        assert_eq!(heading("บทนำ", false), Some(1), "Thai has no capitals");
        assert_eq!(heading("This ends.", false), None);
        assert_eq!(heading("lowercase start", false), None);
        assert_eq!(dashes("a - b").len(), 1);
        assert!(dashes("x-y and --flag").is_empty());
        assert!(fractions("11/2 and 1/20 and 1/2.5").is_empty());
        assert!(emphasis("snake_case_name and 2*3*4").is_empty());
    }

    /// dlg-150: AutoFormat and review each change: reject one change, then all, or accept.
    #[test]
    fn review_autoformat_changes() {
        let mut s = doc(&["SUMMARY", "It's \"fine\".", "Nothing to change here"]);
        let r = s.run("format.autoFormat", &json!({"review": true})).unwrap();
        assert_eq!(r["review"], true);
        let list = s.run("format.autoFormatReview", &json!({})).unwrap();
        let changes = list["changes"].as_array().unwrap().clone();
        assert_eq!(changes.len(), 3, "{changes:?}");
        assert_eq!(changes[1]["before"], "It's \"fine\".");
        assert_eq!(changes[1]["after"], "It\u{2019}s \u{201C}fine\u{201D}.");
        // Find the next change from the start: the heading is selected.
        s.sel = Selection::caret(Pos::body(0, 0));
        s.run("format.autoFormatReview", &json!({"action": "next"})).unwrap();
        assert_eq!(top(&s.sel.focus.path), 0);
        s.run("format.autoFormatReview", &json!({"action": "reject"})).unwrap();
        assert_eq!(style(&s, 0), None, "the heading change is rejected");
        assert_eq!(texts(&s)[1], "It\u{2019}s \u{201C}fine\u{201D}.", "the others stay");
        // Reject All: the document as it was.
        let left = s.run("format.autoFormatReview", &json!({"action": "rejectAll"})).unwrap();
        assert_eq!(left["changes"].as_array().map(Vec::len), Some(0));
        assert_eq!(texts(&s)[1], "It's \"fine\".");
        assert!(s.run("format.autoFormatReview", &json!({})).is_err(), "the review is over");
        // Accept All keeps them; another edit ends a review. (The selection, still the first
        // paragraph, would limit AutoFormat to it.)
        s.sel = Selection::caret(Pos::body(0, 0));
        s.run("format.autoFormat", &json!({"review": true})).unwrap();
        s.run("format.autoFormatReview", &json!({"action": "acceptAll"})).unwrap();
        assert_eq!(texts(&s)[1], "It\u{2019}s \u{201C}fine\u{201D}.");
        s.run("edit.undo", &json!({})).unwrap();
        s.run("format.autoFormat", &json!({"review": true})).unwrap();
        s.run("text.insert", &json!({"text": "x"})).unwrap();
        assert!(s.run("format.autoFormatReview", &json!({"action": "rejectAll"})).is_err());
    }
}
