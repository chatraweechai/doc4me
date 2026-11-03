//! Word's Find and Replace patterns (#543): special characters (`^p`, `^t`, `^#` …) and the
//! "Use wildcards" language (`?`, `*`, `[a-z]`, `{n,m}`, `@`, `<`, `>`, `( )`), translated to a
//! regular expression for the linear-time `regex` engine, so no pattern can backtrack
//! catastrophically. Replace text understands the same codes plus `^&` (the found text), `^c`
//! (the clipboard) and, with wildcards, `\1`–`\9` (the groups).
//!
//! The search reads a story as text with a paragraph mark (`\r`, [`PARA_MARK`]) between
//! paragraphs when the pattern can match one, so `^p` finds paragraph ends; a paragraph that ends
//! a section ends with [`SECT_MARK`] instead (`^b`).

use regex::escape;

/// A paragraph mark in the searched text.
pub const PARA_MARK: char = '\r';
/// The mark of a paragraph that ends a section (`^b`), in the searched text only.
pub const SECT_MARK: char = '\u{1D}';
/// Longest Find what text, in characters.
pub const MAX_PATTERN: usize = 1024;
/// Largest count in `{n,m}`.
const MAX_COUNT: u32 = 1000;
/// Any character except a paragraph mark (`?`, `^?`, `*`).
const ANY: &str = r"[^\r\x{1D}]";

/// How the Find what text is read.
#[derive(Clone, Copy, Debug, Default)]
pub struct Opts {
    pub match_case: bool,
    pub whole_word: bool,
    pub prefix: bool,
    pub suffix: bool,
    pub ignore_punct: bool,
    pub ignore_space: bool,
    /// Word's "Use wildcards".
    pub wildcards: bool,
    /// A regular expression (Doc4Me's own option; wins over wildcards).
    pub regex: bool,
    /// Take the text literally: no `^` codes.
    pub literal: bool,
    /// "Sounds like (Japanese)": Japanese fuzzy matching.
    pub fuzzy: Fuzzy,
}

/// Japanese fuzzy find (Word's "Sounds like (Japanese)" and its Find Options (Japanese)):
/// characters that read alike match one another.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Fuzzy {
    /// Fuzzy matching is on.
    pub on: bool,
    /// Full-width and half-width forms match (Ａ/A, カ/ｶ, ガ/ｶﾞ).
    pub width: bool,
    /// Hiragana and katakana match (か/カ).
    pub kana: bool,
    /// Small kana match their full-size letters (ゃ/や, ッ/ツ).
    pub small_kana: bool,
    /// The prolonged sound mark and dashes match (ー/‐/－).
    pub long_vowel: bool,
}

impl Default for Fuzzy {
    fn default() -> Self {
        Fuzzy { on: false, width: true, kana: true, small_kana: true, long_vowel: true }
    }
}

/// Half-width katakana (U+FF61–U+FF9F) and the full-width characters they stand for.
const HALF_KANA: &str = "｡｢｣､･ｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝﾞﾟ";
const FULL_KANA: &str =
    "。「」、・ヲァィゥェォャュョッーアイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワン゛゜";
/// Small kana and their full-size letters (hiragana, then katakana).
const SMALL_KANA: &str = "ぁぃぅぇぉっゃゅょゎゕゖァィゥェォッャュョヮヵヶ";
const LARGE_KANA: &str = "あいうえおつやゆよわかけアイウエオツヤユヨワカケ";
/// Voiced and semi-voiced katakana and the letter each adds its mark to.
const VOICED_KANA: &str = "ガギグゲゴザジズゼゾダヂヅデドバビブベボヴパピプペポ";
const VOICED_BASE: &str = "カキクケコサシスセソタチツテトハヒフヘホウハヒフヘホ";
/// The prolonged sound mark and the dashes it is confused with.
const LONG_VOWELS: &str = "ーｰ‐‑−－";

/// `c` with the foldings `f` turns on: what fuzzy matching compares.
fn fold(c: char, f: &Fuzzy) -> char {
    let mut c = c;
    if f.width {
        let u = c as u32;
        if (0xFF01..=0xFF5E).contains(&u) {
            c = char::from_u32(u - 0xFEE0).unwrap_or(c);
        } else if c == '\u{3000}' {
            c = ' ';
        } else if let Some(i) = HALF_KANA.chars().position(|h| h == c) {
            c = FULL_KANA.chars().nth(i).unwrap_or(c);
        }
    }
    if f.small_kana
        && let Some(i) = SMALL_KANA.chars().position(|k| k == c)
    {
        c = LARGE_KANA.chars().nth(i).unwrap_or(c);
    }
    if f.kana && ('\u{30A1}'..='\u{30F6}').contains(&c) {
        c = char::from_u32(c as u32 - 0x60).unwrap_or(c);
    }
    if f.long_vowel && LONG_VOWELS.contains(c) {
        c = 'ー';
    }
    c
}

/// The characters fuzzy matching may meet: ASCII, the kana, full-width and half-width forms and
/// the dashes.
fn fuzzy_domain() -> impl Iterator<Item = char> {
    [
        (0x20u32, 0x7E),
        (0x3000, 0x3000),
        (0x3041, 0x3096),
        (0x309B, 0x309C),
        (0x30A1, 0x30FA),
        (0x30FC, 0x30FC),
        (0xFF01, 0xFF5E),
        (0xFF61, 0xFF9F),
        (0x2010, 0x2011),
        (0x2212, 0x2212),
    ]
    .into_iter()
    .flat_map(|(a, b)| (a..=b).filter_map(char::from_u32))
}

/// A regex atom matching `c` and every character (or half-width pair, ｶﾞ for ガ) that reads
/// alike under `f`.
fn fuzzy_atom(c: char, f: &Fuzzy) -> String {
    let key = fold(c, f);
    let mut alts: Vec<String> = vec![lit(c)];
    for d in fuzzy_domain() {
        if d != c && fold(d, f) == key {
            alts.push(lit(d));
        }
    }
    // A voiced kana is two characters in half-width katakana: the letter and its mark.
    if f.width {
        let voiced: Vec<char> = VOICED_KANA.chars().collect();
        for (i, base) in VOICED_BASE.chars().enumerate() {
            let Some(&v) = voiced.get(i) else { continue };
            let hira = char::from_u32(v as u32 - 0x60).filter(|_| v != 'ヴ');
            if fold(v, f) != key && hira.is_none_or(|h| fold(h, f) != key) {
                continue;
            }
            if let Some(hb) = FULL_KANA.chars().position(|k| k == base).and_then(|j| HALF_KANA.chars().nth(j)) {
                let mark = if "パピプペポ".contains(v) { 'ﾟ' } else { 'ﾞ' };
                alts.push(format!("{}{}", lit(hb), lit(mark)));
            }
        }
    }
    alts.dedup();
    if alts.len() == 1 { alts.remove(0) } else { format!("(?:{})", alts.join("|")) }
}

/// A pattern ready for the regex engine.
#[derive(Debug)]
pub struct Compiled {
    pub pattern: String,
    pub case_insensitive: bool,
    /// The pattern can match a paragraph mark, so the search must read across paragraphs.
    pub spans_paras: bool,
}

/// What a `^` code stands for in Find what.
fn find_code(c: char) -> Option<&'static str> {
    Some(match c {
        'p' => r"\r",
        'b' => r"\x{1D}",
        't' => r"\t",
        '?' => ANY,
        '#' => "[0-9]",
        '$' => r"\p{L}",
        '^' => r"\^",
        'l' => r"\n",
        'm' => r"\x{0C}",
        'n' => r"\x{0E}",
        '+' => r"\x{2014}",
        '=' => r"\x{2013}",
        's' => r"\x{A0}",
        '~' => r"\x{2011}",
        '-' => r"\x{AD}",
        'w' => r"[ \t\x{A0}]+",
        'g' => r"\x{FFFC}",
        '%' => "§",
        'v' => "¶",
        _ => return None,
    })
}

/// What a `^` code stands for in Replace with.
fn replace_code(c: char) -> Option<char> {
    Some(match c {
        'p' => PARA_MARK,
        't' => '\t',
        '^' => '^',
        'l' => '\n',
        'm' => '\u{0C}',
        'n' => '\u{0E}',
        '+' => '\u{2014}',
        '=' => '\u{2013}',
        's' => '\u{A0}',
        '~' => '\u{2011}',
        '-' => '\u{AD}',
        '%' => '§',
        'v' => '¶',
        _ => return None,
    })
}

/// A character code (`^13`, `^0151`, `^u8212`) as the character it stands for here.
fn code_char(n: u32) -> Option<char> {
    Some(match n {
        13 => PARA_MARK,
        11 => '\n',
        12 => '\u{0C}',
        14 => '\u{0E}',
        30 => '\u{2011}',
        31 => '\u{AD}',
        n => char::from_u32(n).filter(|c| !c.is_control() || *c == '\t')?,
    })
}

/// Read a numeric code after `^` (`13`, `0151`, `u8212`) starting at `chars[i]`; returns the
/// character and how many characters it took.
fn numeric_code(chars: &[char], i: usize) -> Option<(char, usize)> {
    let (start, max) = if chars.get(i) == Some(&'u') { (i + 1, 5) } else { (i, 4) };
    let digits: String = chars.get(start..).unwrap_or(&[]).iter().take(max).take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    let n: u32 = digits.parse().ok()?;
    Some((code_char(n)?, start - i + digits.chars().count()))
}

/// A literal character as a regex atom.
fn lit(c: char) -> String {
    if c == PARA_MARK {
        return r"\r".into();
    }
    escape(c.encode_utf8(&mut [0; 4]))
}

/// Translate Word's Find what text.
pub fn compile(query: &str, o: &Opts) -> Result<Compiled, String> {
    if query.chars().count() > MAX_PATTERN {
        return Err(format!("the Find what text is longer than {MAX_PATTERN} characters"));
    }
    let (pat, spans) = if o.regex {
        (query.to_string(), false)
    } else if o.literal {
        (escape(query), false)
    } else if o.wildcards {
        wildcards(query)?
    } else {
        special(query, o)
    };
    let wrap = |p: String| format!("(?:{p})");
    let pat = if o.wildcards && !o.regex {
        pat
    } else if o.whole_word {
        format!(r"\b{}\b", wrap(pat))
    } else {
        match (o.prefix, o.suffix) {
            (true, true) => format!(r"\b{}\b", wrap(pat)),
            (true, false) => format!(r"\b{}", wrap(pat)),
            (false, true) => format!(r"{}\b", wrap(pat)),
            (false, false) => pat,
        }
    };
    // Word's wildcard search always matches case.
    let case_insensitive = !o.match_case && !(o.wildcards && !o.regex);
    Ok(Compiled { pattern: pat, case_insensitive, spans_paras: spans })
}

/// Normal search: literal text with `^` codes, optionally skipping punctuation and white space.
fn special(query: &str, o: &Opts) -> (String, bool) {
    let chars: Vec<char> = query.chars().collect();
    let mut atoms: Vec<String> = Vec::new();
    let mut spans = false;
    let mut i = 0;
    while let Some(&c) = chars.get(i) {
        i += 1;
        if c == '^'
            && let Some(&n) = chars.get(i)
        {
            if let Some(code) = find_code(n) {
                spans |= matches!(n, 'p' | 'b');
                atoms.push(code.to_string());
                i += 1;
                continue;
            }
            if let Some((ch, used)) = numeric_code(&chars, i) {
                spans |= ch == PARA_MARK;
                atoms.push(lit(ch));
                i += used;
                continue;
            }
            // Not a code: the caret itself.
        }
        if (o.ignore_punct && is_punct(c)) || (o.ignore_space && is_space(c)) {
            continue;
        }
        atoms.push(if o.fuzzy.on { fuzzy_atom(c, &o.fuzzy) } else { lit(c) });
    }
    let filler = match (o.ignore_punct, o.ignore_space) {
        (true, true) => r"[\p{P} \t\x{A0}]*",
        (true, false) => r"\p{P}*",
        (false, true) => r"[ \t\x{A0}]*",
        (false, false) => "",
    };
    (atoms.join(filler), spans)
}

fn is_punct(c: char) -> bool {
    c.is_ascii_punctuation() || matches!(c, '\u{2010}'..='\u{2027}' | '\u{00A1}' | '\u{00BF}' | '\u{00AB}' | '\u{00BB}')
}

fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\u{A0}')
}

/// "Use wildcards" search.
fn wildcards(query: &str) -> Result<(String, bool), String> {
    let chars: Vec<char> = query.chars().collect();
    let mut out = String::new();
    let mut spans = false;
    let mut i = 0;
    while let Some(&c) = chars.get(i) {
        i += 1;
        match c {
            '?' => out.push_str(ANY),
            '*' => {
                out.push_str(ANY);
                out.push_str("*?");
            }
            '@' => out.push('+'),
            '<' => out.push_str(r"\b{start}"),
            '>' => out.push_str(r"\b{end}"),
            '(' | ')' => out.push(c),
            '{' => {
                let end = chars.get(i..).and_then(|r| r.iter().position(|c| *c == '}')).ok_or("a { needs a closing }")?;
                let body: String = chars.get(i..i + end).unwrap_or(&[]).iter().collect();
                out.push_str(&count(&body)?);
                i += end + 1;
            }
            '[' => {
                let end = chars.get(i..).and_then(|r| r.iter().position(|c| *c == ']')).ok_or("a [ needs a closing ]")?;
                out.push_str(&class(chars.get(i..i + end).unwrap_or(&[]))?);
                i += end + 1;
            }
            '\\' => match chars.get(i) {
                Some(d) if d.is_ascii_digit() => return Err("back-references (\\1) work in Replace with, not in Find what".into()),
                Some(&n) => {
                    out.push_str(&lit(n));
                    i += 1;
                }
                None => out.push_str(r"\\"),
            },
            '^' => {
                let Some(&n) = chars.get(i) else {
                    out.push_str(r"\^");
                    continue;
                };
                if let Some(code) = find_code(n) {
                    spans |= matches!(n, 'p' | 'b');
                    out.push_str(code);
                    i += 1;
                } else if let Some((ch, used)) = numeric_code(&chars, i) {
                    spans |= ch == PARA_MARK;
                    out.push_str(&lit(ch));
                    i += used;
                } else {
                    out.push_str(r"\^");
                }
            }
            c => out.push_str(&lit(c)),
        }
    }
    Ok((out, spans))
}

/// `{n}`, `{n,}`, `{n,m}` (and `{,m}`).
fn count(body: &str) -> Result<String, String> {
    let bad = || format!("{{{body}}} is not a valid count");
    let num = |s: &str| -> Result<Option<u32>, String> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(None);
        }
        let n: u32 = s.parse().map_err(|_| bad())?;
        if n > MAX_COUNT {
            return Err(format!("counts go up to {MAX_COUNT}"));
        }
        Ok(Some(n))
    };
    match body.split_once([',', ';']) {
        None => Ok(format!("{{{}}}", num(body)?.ok_or_else(bad)?)),
        Some((a, b)) => {
            let a = num(a)?.unwrap_or(0);
            match num(b)? {
                Some(b) if b < a => Err(bad()),
                Some(b) => Ok(format!("{{{a},{b}}}")),
                None => Ok(format!("{{{a},}}")),
            }
        }
    }
}

/// `[abc]`, `[a-z]`, `[!x]`: the characters inside are literal.
fn class(inner: &[char]) -> Result<String, String> {
    let (neg, inner) = match inner.split_first() {
        Some(('!', rest)) => (true, rest),
        _ => (false, inner),
    };
    if inner.is_empty() {
        return Err("[ ] needs at least one character".into());
    }
    let mut out = String::from(if neg { r"[^\r\x{1D}" } else { "[" });
    let mut i = 0;
    while let Some(&c) = inner.get(i) {
        let c = if c == '\\' && i + 1 < inner.len() {
            i += 1;
            inner.get(i).copied().unwrap_or(c)
        } else {
            c
        };
        i += 1;
        if inner.get(i) == Some(&'-')
            && let Some(&hi) = inner.get(i + 1)
        {
            if hi < c {
                return Err(format!("[{c}-{hi}] is not in ascending order"));
            }
            out.push_str(&format!("{}-{}", lit(c), lit(hi)));
            i += 2;
        } else {
            out.push_str(&lit(c));
        }
    }
    out.push(']');
    Ok(out)
}

/// A piece of Replace with text.
#[derive(Clone, Debug, PartialEq)]
pub enum Piece {
    Lit(String),
    /// `^&`: the text found.
    Found,
    /// `\n` with wildcards: group `n`.
    Group(usize),
    /// `^c`: the clipboard.
    Clip,
}

/// Read Replace with text: `^` codes (unless `literal`), and `\1`–`\9` with wildcards.
pub fn parse_replace(with: &str, o: &Opts) -> Vec<Piece> {
    if o.literal || o.regex {
        return vec![Piece::Lit(with.to_string())];
    }
    let chars: Vec<char> = with.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    let flush = |cur: &mut String, out: &mut Vec<Piece>| {
        if !cur.is_empty() {
            out.push(Piece::Lit(std::mem::take(cur)));
        }
    };
    let mut i = 0;
    while let Some(&c) = chars.get(i) {
        i += 1;
        if c == '\\' && o.wildcards {
            match chars.get(i) {
                Some(d) if d.is_ascii_digit() && *d != '0' => {
                    flush(&mut cur, &mut out);
                    out.push(Piece::Group(d.to_digit(10).unwrap_or(0) as usize));
                    i += 1;
                    continue;
                }
                Some(&n) => {
                    cur.push(n);
                    i += 1;
                    continue;
                }
                None => {}
            }
        }
        if c == '^'
            && let Some(&n) = chars.get(i)
        {
            if n == '&' || n == 'c' {
                flush(&mut cur, &mut out);
                out.push(if n == '&' { Piece::Found } else { Piece::Clip });
                i += 1;
                continue;
            }
            if let Some(ch) = replace_code(n) {
                cur.push(ch);
                i += 1;
                continue;
            }
            if let Some((ch, used)) = numeric_code(&chars, i) {
                cur.push(ch);
                i += used;
                continue;
            }
        }
        cur.push(c);
    }
    flush(&mut cur, &mut out);
    out
}

/// Searched text as it is inserted again: marks become paragraph breaks.
fn as_inserted(s: &str) -> String {
    s.replace(SECT_MARK, "\r")
}

/// Fill in Replace with text for one match: `found` is the matched text, `group(n)` a group's.
pub fn expand(pieces: &[Piece], found: &str, group: &dyn Fn(usize) -> Option<String>, clip: &str) -> String {
    let mut out = String::new();
    for p in pieces {
        match p {
            Piece::Lit(t) => out.push_str(t),
            Piece::Found => out.push_str(&as_inserted(found)),
            Piece::Group(n) => out.push_str(&as_inserted(&group(*n).unwrap_or_default())),
            Piece::Clip => out.push_str(&clip.replace("\r\n", "\r").replace('\n', "\r")),
        }
    }
    out
}

#[cfg(test)]
mod fuzzy_tests {
    use super::{Fuzzy, Opts, compile};

    fn finds(query: &str, text: &str, fuzzy: Fuzzy) -> bool {
        let c = compile(query, &Opts { fuzzy, ..Default::default() }).unwrap();
        regex::RegexBuilder::new(&c.pattern).case_insensitive(c.case_insensitive).build().unwrap().is_match(text)
    }

    /// Sounds like (Japanese): width, hiragana/katakana, small kana and the long vowel fold,
    /// each only with its option; without fuzzy matching the text must be the same.
    #[test]
    fn japanese_fuzzy_find_matches_what_reads_alike() {
        let on = Fuzzy { on: true, ..Default::default() };
        assert!(finds("コーヒー", "ｺｰﾋｰ", on), "half-width katakana");
        assert!(finds("ガイド", "ｶﾞｲﾄﾞ", on), "voiced half-width pairs");
        assert!(finds("がいど", "ｶﾞｲﾄﾞ", on), "hiragana against half-width katakana");
        assert!(finds("きゃっと", "キヤツト", on), "small kana and kana type");
        assert!(finds("ＡＢＣ１２", "abc12", on), "full-width Latin, case ignored");
        assert!(finds("データ", "デ‐タ", on), "a dash for the long vowel");
        assert!(!finds("コーヒー", "ｺｰﾋｰ", Fuzzy::default()), "off: exact");
        assert!(!finds("かな", "カナ", Fuzzy { kana: false, ..on }), "kana option off");
        assert!(!finds("キャ", "キヤ", Fuzzy { small_kana: false, ..on }), "small kana option off");
        assert!(!finds("ア", "あ", Fuzzy { kana: false, width: false, ..on }));
        // Other text is unchanged.
        assert!(finds("漢字", "漢字", on) && !finds("漢字", "漢子", on));
    }
}
