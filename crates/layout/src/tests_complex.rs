//! Complex scripts that reorder and stack (#604): Myanmar, Khmer, Devanagari. Cluster and line
//! breaking tests don't depend on installed fonts (a missing font draws .notdef boxes, but the
//! clusters and breaks are the same); shaping tests use a system font when one is installed.

use super::*;
use doc4me_doc::{Pos, StoryRef};

fn lay(doc: &Document) -> DocLayout {
    let mut c = LayoutCache::new();
    layout(doc, &mut c, &LayoutOptions::default())
}

/// Body paragraph `i`'s layout.
fn para_layout(l: &DocLayout, i: u32) -> &ParaLayout {
    for p in &l.pages {
        for it in &p.items {
            if let Placed::Lines { story: StoryRef::Body, path, para, .. } = it
                && path.0 == vec![i]
            {
                return para;
            }
        }
    }
    panic!("paragraph {i} not laid out");
}

/// "မြန်မာစာ": the syllables မြ | န် | မာ | စာ.
const MYANMAR: &str = "\u{1019}\u{103C}\u{1014}\u{103A}\u{1019}\u{102C}\u{1005}\u{102C}";

#[test]
fn myanmar_syllables_are_single_clusters() {
    let l = lay(&Document::from_text(MYANMAR));
    let pl = para_layout(&l, 0);
    let starts: Vec<usize> = pl.clusters.iter().map(|c| c.start).collect();
    // Grapheme clusters would also start at the aa signs (bytes 15 and 21), splitting မာ and စာ.
    assert_eq!(starts, vec![0, 6, 12, 18]);
    // The caret can't land inside a syllable: every caret offset is a cluster start or the end.
    let offs = pl.line_offsets(0);
    assert!(!offs.contains(&15) && !offs.contains(&21), "{offs:?}");
    // Stacked consonants (virama) and Khmer subscripts (coeng) too.
    let l = lay(&Document::from_text("\u{1019}\u{1039}\u{1018}\u{102C} \u{1781}\u{17D2}\u{1798}\u{17C2}\u{179A}"));
    let pl = para_layout(&l, 0);
    let starts: Vec<usize> = pl.clusters.iter().map(|c| c.start).collect();
    assert_eq!(starts, vec![0, 12, 13, 25]);
}

#[test]
fn myanmar_without_spaces_wraps_between_syllables() {
    let text = MYANMAR.repeat(80);
    let l = lay(&Document::from_text(&text));
    let pl = para_layout(&l, 0);
    assert!(pl.lines.len() > 1, "a long Myanmar paragraph wraps");
    let breaks = doc4me_doc::cluster::myanmar_breaks(&text);
    // A line may end before each syllable: before မာ and စာ, never before န် (a final).
    for c in &pl.clusters {
        assert_eq!(c.break_after, breaks.contains(&c.end), "break after {}", c.end);
    }
    assert!(pl.clusters.iter().any(|c| c.break_after && c.end == 12) && !pl.clusters.iter().any(|c| c.break_after && c.end == 6));
    for line in pl.lines.iter().skip(1) {
        let start = pl.clusters[line.c0].start;
        assert!(breaks.contains(&start), "line starts at syllable {start}");
    }
    // Lines are full: a syllable break, not an emergency break after one cluster.
    assert!(pl.lines[0].c1 - pl.lines[0].c0 > 10);
}

/// A face with Myanmar letters (an installed system font), or `None` to skip.
fn myanmar_face() -> Option<doc4me_fonts::FaceRef> {
    doc4me_fonts::FontDb::global().fallback_for('\u{1000}', 0).map(|f| doc4me_fonts::FaceRef::of(&f)).filter(|f| f.has_layout_tables())
}

#[test]
fn myanmar_prevowel_is_drawn_before_its_consonant() {
    let Some(face) = myanmar_face() else {
        eprintln!("skipped: no Myanmar font with OpenType tables installed");
        return;
    };
    // ကေ: in memory consonant first, on screen the vowel sign ေ first.
    let ka = face.glyph_for('\u{1000}');
    let g = doc4me_fonts::shape(&face, "\u{1000}\u{1031}", &[], |c| c);
    assert_eq!(g.len(), 2, "{g:?}");
    assert_eq!(g[1].gid, ka, "the consonant is drawn second: {g:?}");
    assert!(g.iter().all(|x| x.cluster == 0), "one cluster: {g:?}");
    // Itemised by script: the same syllable after Latin text in the same face shapes the same
    // (shaped as one Latin run, the vowel sign would stay after the consonant).
    if face.covers('H') {
        let mixed = doc4me_fonts::shape(&face, "Hi \u{1000}\u{1031}", &[], |c| c);
        let tail: Vec<u32> = mixed.iter().filter(|x| x.cluster >= 3).map(|x| x.gid).collect();
        assert_eq!(tail, g.iter().map(|x| x.gid).collect::<Vec<_>>());
    }
}

#[test]
fn myanmar_in_a_latin_font_falls_back_to_a_regular_myanmar_face() {
    if myanmar_face().is_none() {
        eprintln!("skipped: no Myanmar font with OpenType tables installed");
        return;
    }
    let l = lay(&Document::from_text(MYANMAR));
    let pl = para_layout(&l, 0);
    let faces: Vec<_> = pl.clusters.iter().map(|c| pl.styles[c.style as usize].face).collect();
    assert!(faces.windows(2).all(|w| w[0].id() == w[1].id()), "one face for the word");
    let f = faces[0];
    assert!(f.has_layout_tables(), "{} can reorder Myanmar", f.family);
    assert!(!f.italic && (f.weight - 400.0).abs() <= 100.0, "a regular weight, not {} {}", f.family, f.style);
    assert!(pl.glyphs.iter().all(|g| g.gid != 0), "no missing glyphs");
    // The caret after the first syllable sits after its glyphs.
    let x6 = l.caret(&Pos::body(0, 6)).map(|c| c.x).unwrap_or(0.0);
    let x0 = l.caret(&Pos::body(0, 0)).map(|c| c.x).unwrap_or(0.0);
    assert!(x6 > x0);
}

/// "ภาษาไทยง่ายนิดเดียว" (Thai is quite easy): five words, no spaces between them.
const THAI: &str = "ภาษาไทยง่ายนิดเดียว";

#[test]
fn thai_without_spaces_wraps_between_dictionary_words() {
    let text = THAI.repeat(40);
    let l = lay(&Document::from_text(&text));
    let pl = para_layout(&l, 0);
    assert!(pl.lines.len() > 1, "a long Thai paragraph wraps");
    let words = doc4me_doc::cluster::dictionary_breaks(&text);
    assert!(words.contains(&"ภาษา".len()) && words.contains(&"ภาษาไทย".len()), "{:?}", &words[..words.len().min(8)]);
    // Every line after the first starts at a word boundary, never inside a word (between a
    // consonant and its vowel, or in the middle of ภาษา).
    for line in pl.lines.iter().skip(1) {
        let start = pl.clusters[line.c0].start;
        assert!(words.contains(&start), "line starts inside a word at byte {start}");
    }
    // Lines are full: word breaks, not an emergency break after a cluster or two.
    assert!(pl.lines[0].c1 - pl.lines[0].c0 > 20);
    // A tone mark stays in its consonant's cluster: ง่ is one caret stop.
    let ngo = "ภาษาไทย".len();
    let starts: Vec<usize> = pl.clusters.iter().map(|c| c.start).take_while(|&s| s < THAI.len()).collect();
    assert!(starts.contains(&ngo) && !starts.contains(&(ngo + 3)), "{starts:?}");
}

#[test]
fn thai_is_drawn_with_the_bundled_sarabun_by_default() {
    // New documents use Sarabun (bundled) for Latin and Thai text alike, so Thai never falls back
    // to whatever the system has, or to boxes.
    let doc = Document::from_text("Doc4Me ภาษาไทย");
    let rc = doc.styles.resolve_char(Some("Normal"), &doc4me_doc::props::CharProps::default());
    assert_eq!(rc.font, "Sarabun");
    assert_eq!(rc.font_cs, "Sarabun");
    let face = doc4me_fonts::word::resolve(&rc.font_cs, false, false).face;
    assert!(!doc4me_fonts::word::resolve(&rc.font_cs, false, false).substituted);
    assert!("ภาษาไทยง่าย".chars().all(|c| face.covers(c)));
    // Sarabun carries OpenType tables, so marks stack: over an upper vowel (ที่) the tone mark
    // takes the font's raised form, not the one it has over a bare consonant (ท่).
    assert!(face.has_layout_tables());
    let stacked = doc4me_fonts::shape(&face, "ที่", &[], |c| c);
    let lone = doc4me_fonts::shape(&face, "ท่", &[], |c| c);
    assert_eq!(stacked.len(), 3, "consonant, vowel and tone mark: {stacked:?}");
    assert!(stacked.iter().chain(&lone).all(|g| g.gid != 0), "no .notdef");
    assert_ne!(stacked[2].gid, lone[1].gid, "the tone mark over a vowel is raised: {stacked:?} {lone:?}");
    // Sara am (ำ) splits so the tone mark of น้ำ sits between its nikhahit and aa.
    let nam = doc4me_fonts::shape(&face, "น้ำ", &[], |c| c);
    assert!(nam.len() >= 3 && nam.last().is_some_and(|g| g.x_advance > 0), "{nam:?}");
}

#[test]
fn thai_distributed_spreads_full_lines_but_not_the_last() {
    // Thai documents use Thai Distributed (`thaiDistribute`) for body text: full lines reach the
    // right margin with the room spread between characters; the paragraph's last line stays
    // as typed (Distributed would stretch it across the page).
    let text = THAI.repeat(12);
    let mut doc = Document::from_text(&text);
    if let Some(doc4me_doc::Block::Para(p)) = doc.body.get_mut(0).map(std::sync::Arc::make_mut) {
        p.props.align = Some(doc4me_doc::Align::ThaiDistribute);
    }
    let l = lay(&doc);
    let pl = para_layout(&l, 0);
    assert!(pl.lines.len() >= 2, "{} lines", pl.lines.len());
    // The end x of a line (its last entry in `xs`).
    let end = |line: &crate::para::Line| line.xs.last().copied().unwrap_or(0.0);
    let first = &pl.lines[0];
    assert!((end(first) - first.right).abs() < 1.0, "a full line reaches the margin: {} of {}", end(first), first.right);
    // Spread between characters: the gaps grow, not just one gap at a space.
    let gaps = |line: &crate::para::Line| -> Vec<f32> {
        (line.c0..line.c1).map(|k| line.xs[k + 1 - line.c0] - line.xs[k - line.c0] - pl.clusters[k].adv).collect()
    };
    let g = gaps(first);
    assert!(g.iter().filter(|g| **g > 0.01).count() > g.len() / 2, "{g:?}");
    // The last line keeps its natural spacing.
    let last = pl.lines.last().unwrap();
    assert!(gaps(last).iter().all(|g| g.abs() < 0.01), "the last line isn't stretched: {:?}", gaps(last));
}

#[test]
fn distributed_lines_ending_in_a_space_still_reach_the_margin() {
    // Thai puts spaces between phrases, so a line often wraps after one. That space hangs past
    // the margin; the text itself still ends at it.
    let text = format!("{THAI} ").repeat(16);
    for align in [doc4me_doc::Align::ThaiDistribute, doc4me_doc::Align::Distribute] {
        let mut doc = Document::from_text(&text);
        if let Some(doc4me_doc::Block::Para(p)) = doc.body.get_mut(0).map(std::sync::Arc::make_mut) {
            p.props.align = Some(align);
        }
        let l = lay(&doc);
        let pl = para_layout(&l, 0);
        assert!(pl.lines.len() >= 2, "{} lines", pl.lines.len());
        let mut checked = 0;
        for line in &pl.lines[..pl.lines.len() - 1] {
            let Some(last) = (line.c0..line.c1).rev().find(|&k| pl.clusters[k].kind != crate::para::ClKind::Space) else { continue };
            if last + 1 == line.c1 {
                continue;
            }
            let end = line.xs[last + 1 - line.c0];
            assert!((end - line.right).abs() < 0.5, "{align:?}: the text ends at {end}, the margin is {}", line.right);
            checked += 1;
        }
        assert!(checked > 0, "{align:?}: no line ended in a space");
    }
}
