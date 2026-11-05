//! Asian typography line-breaking rules (the paragraph's `kinsoku` and `word_wrap` flags), applied
//! on top of the Unicode line-break opportunities (UAX #14).
//!
//! - Kinsoku on (the default): no line starts with closing East Asian punctuation (、。，）」…)
//!   and none ends with opening punctuation (（「【…). UAX #14 already forbids most of these
//!   breaks; the lists make sure of it for the standard Japanese/Chinese sets.
//! - Kinsoku off: a line may break between any two East Asian characters, punctuation included.
//! - Word wrap off: Latin words may break at any character.
//!
//! Only East Asian characters are in the lists, and the extra breaks need letters on both sides,
//! so text with no East Asian characters breaks exactly as before unless word wrap is off.

use unicode_segmentation::UnicodeSegmentation;

/// East Asian characters a line may not start with: closing brackets and quotes, the
/// ideographic comma and full stop, fullwidth and halfwidth punctuation, the middle dot,
/// iteration and sound marks.
const NO_START: &str = "、。，．・：；？！゛゜ヽヾゝゞ々〻〉》」』】〕〗〙〛〞〟）］｝｠｡｣､･ﾞﾟ％￠";

/// East Asian characters a line may not end with: opening brackets and quotes, currency signs.
const NO_END: &str = "〈《「『【〔〖〘〚〝（［｛｟｢＄￡￥";

/// Strict first and last characters (`w:strictFirstAndLastChars`): small kana and the prolonged
/// sound mark may not start a line either.
const STRICT_NO_START: &str = "ぁぃぅぇぉっゃゅょゎゕゖァィゥェォッャュョヮヵヶㇰㇱㇲㇳㇴㇵㇶㇷㇸㇹㇺㇻㇼㇽㇾㇿーｧｨｩｪｫｬｭｮｯｰ";

/// The standard lists Word shows under First and last characters: (no line starts with, none
/// ends with), the strict additions included when `strict`.
pub fn standard_lists(strict: bool) -> (String, String) {
    let start = if strict { format!("{NO_START}{STRICT_NO_START}") } else { NO_START.to_string() };
    (start, NO_END.to_string())
}

/// Full-width punctuation set closing: its glyph sits in the left half of its square.
const CLOSING_PUNCT: &str = "、。，．）」』】〕〉》］｝〙〗〛〟｠";

/// Full-width punctuation set opening: its glyph sits in the right half of its square.
const OPENING_PUNCT: &str = "（「『【〔〈《［｛〘〖〚〝｟";

/// How a character takes part in punctuation compression: `Some(true)` closing, `Some(false)`
/// opening, `None` neither.
pub fn punct_side(c: char) -> Option<bool> {
    if CLOSING_PUNCT.contains(c) {
        Some(true)
    } else if OPENING_PUNCT.contains(c) {
        Some(false)
    } else {
        None
    }
}

/// Whether `c` is East Asian text: CJK ideographs, kana, bopomofo, CJK punctuation, and
/// fullwidth/halfwidth forms (not spaces, not combining sound marks).
pub fn is_east_asian(c: char) -> bool {
    matches!(c as u32,
        0x2E80..=0x2FFF          // radicals, ideographic description
        | 0x3001..=0x303F        // CJK symbols and punctuation (not the ideographic space)
        | 0x3040..=0x3098        // hiragana
        | 0x309B..=0x30FF        // sound marks, katakana
        | 0x3100..=0x312F        // bopomofo
        | 0x3190..=0x31FF        // kanbun, CJK strokes, katakana extensions
        | 0x3400..=0x4DBF        // CJK extension A
        | 0x4E00..=0x9FFF        // CJK unified ideographs
        | 0xF900..=0xFAFF        // compatibility ideographs
        | 0xFE30..=0xFE4F        // CJK compatibility forms
        | 0xFF01..=0xFFEF        // fullwidth and halfwidth forms
        | 0x20000..=0x3FFFF) // supplementary ideographs
}

/// Adjust the break opportunities `opps` (byte offsets into `text`: a line may start there) for
/// the paragraph's kinsoku and word-wrap settings. The result is sorted and has no duplicates.
/// `rules` are the document's first and last characters (File › Options › Typography): strict
/// rules add small kana to the characters no line starts with, custom lists replace the standard
/// ones.
pub fn apply(text: &str, opps: &mut Vec<usize>, kinsoku: bool, word_wrap: bool, rules: &doc4me_doc::Kinsoku) {
    let next_char = |i: usize| text.get(i..).and_then(|t| t.chars().next());
    let prev_char = |i: usize| text.get(..i).and_then(|t| t.chars().next_back());
    if kinsoku {
        let no_start = |c: char| match rules.no_start.as_deref() {
            Some(list) => list.contains(c),
            None => NO_START.contains(c) || (rules.strict && STRICT_NO_START.contains(c)),
        };
        let no_end = |c: char| rules.no_end.as_deref().unwrap_or(NO_END).contains(c);
        opps.retain(|&i| !next_char(i).is_some_and(no_start) && !prev_char(i).is_some_and(no_end));
    } else {
        // Between any two East Asian characters.
        for (i, c) in text.char_indices().skip(1) {
            if is_east_asian(c) && prev_char(i).is_some_and(is_east_asian) {
                opps.push(i);
            }
        }
    }
    if !word_wrap {
        // Between any two letters or digits that are not East Asian (whole graphemes, so accents
        // stay with their letters).
        let latin = |g: &str| g.chars().next().is_some_and(|c| c.is_alphanumeric() && !is_east_asian(c));
        let mut prev: Option<&str> = None;
        for (i, g) in text.grapheme_indices(true) {
            if prev.is_some_and(latin) && latin(g) {
                opps.push(i);
            }
            prev = Some(g);
        }
    }
    opps.sort_unstable();
    opps.dedup();
}

#[cfg(test)]
mod tests {
    use super::apply;

    /// Where a line may start, as the text from there to the next opportunity.
    fn breaks(text: &str, kinsoku: bool, word_wrap: bool) -> Vec<String> {
        let mut opps: Vec<usize> = unicode_linebreak::linebreaks(text).map(|(i, _)| i).filter(|&i| i < text.len()).collect();
        apply(text, &mut opps, kinsoku, word_wrap, &Default::default());
        opps.iter().map(|&i| text.get(i..).and_then(|t| t.chars().next()).map(String::from).unwrap_or_default()).collect()
    }

    #[test]
    fn kinsoku_and_word_wrap_break_opportunities() {
        let ja = "日本語、「漢字」。テスト";
        let on = breaks(ja, true, true);
        for c in ["、", "」", "。", "漢"] {
            assert!(!on.iter().any(|s| s == c), "kinsoku: no line starts with {c}: {on:?}");
        }
        assert!(on.iter().any(|s| s == "「"), "a line may start at an opening bracket: {on:?}");
        assert!(on.iter().any(|s| s == "テ"), "{on:?}");
        let off = breaks(ja, false, true);
        for c in ["、", "」", "。", "漢"] {
            assert!(off.iter().any(|s| s == c), "kinsoku off: a line may start with {c}: {off:?}");
        }
        // Latin text: unchanged by kinsoku, broken anywhere without word wrap.
        let en = "Hello wonderful (world), 50%.";
        assert_eq!(breaks(en, true, true), breaks(en, false, true));
        assert_eq!(breaks(en, true, true), ["w", "(", "5"]);
        let any = breaks("Hello café", true, false);
        assert_eq!(any, ["e", "l", "l", "o", "c", "a", "f", "é"]);
    }

    /// File › Options › Typography: strict rules keep small kana and ー off the start of a line;
    /// custom lists replace the standard ones.
    #[test]
    fn strict_and_custom_first_and_last_characters() {
        let starts = |text: &str, rules: &doc4me_doc::Kinsoku| -> Vec<String> {
            let mut opps: Vec<usize> = unicode_linebreak::linebreaks(text).map(|(i, _)| i).filter(|&i| i < text.len()).collect();
            apply(text, &mut opps, true, true, rules);
            opps.iter().map(|&i| text.get(i..).and_then(|t| t.chars().next()).map(String::from).unwrap_or_default()).collect()
        };
        let ja = "キャット、コーヒー「茶」";
        let standard = starts(ja, &Default::default());
        let strict = starts(ja, &doc4me_doc::Kinsoku { strict: true, ..Default::default() });
        for c in ["ャ", "ー"] {
            assert!(!strict.iter().any(|s| s == c), "strict: no line starts with {c}: {strict:?}");
        }
        assert!(standard.len() >= strict.len(), "{standard:?} {strict:?}");
        // Custom lists replace the standard ones: no line starts with キ, none ends with 漢 (so
        // none starts with the 字 after it); 漢 itself may start one.
        let custom = doc4me_doc::Kinsoku { no_start: Some("キ".into()), no_end: Some("漢".into()), ..Default::default() };
        let c = starts("日キ漢字", &custom);
        assert_eq!(c, ["漢"]);
        // The same text with the standard lists breaks between any two of its characters.
        assert_eq!(starts("日キ漢字", &Default::default()), ["キ", "漢", "字"]);
    }
}
