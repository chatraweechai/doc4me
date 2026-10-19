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

/// A face with Myanmar letters and Myanmar shaping rules (an installed system font), or `None` to
/// skip. A face that only maps the letters (Unifont) has GSUB for other scripts but no Myanmar
/// lookups, so the vowel sign is never moved.
fn myanmar_face() -> Option<doc4me_fonts::FaceRef> {
    use skrifa::raw::TableProvider;
    let shapes_myanmar = |f: &doc4me_fonts::FaceRef| {
        let Some(font) = f.skrifa() else { return false };
        let Ok(gsub) = font.gsub() else { return false };
        let Ok(scripts) = gsub.script_list() else { return false };
        scripts.script_records().iter().any(|r| [*b"mym2", *b"mymr"].contains(&r.script_tag().into_bytes()))
    };
    doc4me_fonts::FontDb::global().fallback_for('\u{1000}', 0).map(|f| doc4me_fonts::FaceRef::of(&f)).filter(shapes_myanmar)
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

/// A one-paragraph document in `font` (Latin and complex script) at `size`, single spaced with no
/// space before or after, with `chr` applied on top of the defaults.
fn doc_in(font: &str, size: f32, text: &str, chr: impl FnOnce(&mut doc4me_doc::props::CharProps)) -> Document {
    let mut d = Document::from_text(text);
    let c = &mut d.styles.default_chr;
    (c.font, c.font_cs, c.size, c.size_cs) = (Some(font.into()), Some(font.into()), Some(size), Some(size));
    chr(c);
    if let Some(doc4me_doc::Block::Para(p)) = d.body.get_mut(0).map(std::sync::Arc::make_mut) {
        p.props.line_spacing = Some(doc4me_doc::props::LineSpacing::Multiple(1.0));
        (p.props.space_before, p.props.space_after) = (Some(0.0), Some(0.0));
    }
    d
}

/// Sarabun's single-spaced lines are 1.853 em apart, as in Word for Windows (its usWin metrics),
/// not the 1.30 em of its hhea metrics.
#[test]
fn sarabun_lines_are_spaced_like_word_for_windows() {
    let l = lay(&doc_in("Sarabun", 16.0, "ภาษาไทย Thai", |_| {}));
    let pl = para_layout(&l, 0);
    assert!((pl.lines[0].height - 16.0 * 1.853).abs() < 0.01, "{}", pl.lines[0].height);
}

/// TH Sarabun New, TH SarabunPSK and TH SarabunIT๙ (not installed here) are drawn with the
/// bundled Sarabun at their own letter height, on their own widths and line spacing, so a page
/// holds the same text as with the real font.
#[test]
fn th_sarabun_new_keeps_its_size_widths_and_line_spacing_when_drawn_with_sarabun() {
    for font in ["TH Sarabun New", "TH SarabunPSK", "TH SarabunIT๙"] {
        let l = lay(&doc_in(font, 16.0, "กขค ABC", |_| {}));
        let pl = para_layout(&l, 0);
        // hhea 844/-457 with a 30 gap, usWin 844/457: 1.331 em.
        assert!((pl.lines[0].height - 16.0 * 1.331).abs() < 0.01, "{font}: {}", pl.lines[0].height);
        // TH Sarabun New's advance widths (ems): ก ข ค, space, A B C.
        let want = [0.386, 0.378, 0.393, 0.216, 0.400, 0.378, 0.406];
        let got: Vec<f32> = pl.clusters.iter().map(|c| c.adv).collect();
        assert_eq!(got.len(), want.len(), "{font}: {got:?}");
        for (g, w) in got.iter().zip(want) {
            assert!((g - w * 16.0).abs() < 0.001, "{font}: {got:?}");
        }
        // Drawn with Sarabun, scaled to TH Sarabun New's letter heights: ก 400/586, x 340/500.
        let style = |i: usize| &pl.styles[pl.clusters[i].style as usize];
        assert_eq!(style(0).face.family, "Sarabun");
        assert!((style(0).glyph_size() - 16.0 * 400.0 / 586.0).abs() < 0.01, "{}", style(0).glyph_size());
        assert!((style(4).glyph_size() - 16.0 * 0.68).abs() < 0.01, "{}", style(4).glyph_size());
        // Each glyph is centred on its cluster's width. Sarabun's letters, scaled, are a little
        // wider or narrower than TH Sarabun New's (ข: 0.405 em against 0.378), so a glyph may
        // start a fraction of a point before its cluster, into the side bearings.
        for c in &pl.clusters {
            for g in &pl.glyphs[c.g0 as usize..c.g1 as usize] {
                assert!(g.dx > -0.05 * 16.0 && g.dx < c.adv, "{font}: {g:?} in {c:?}");
            }
        }
    }
    // TH SarabunIT๙ shows ASCII digits as Thai digits, on the Thai digits' width; the text keeps
    // its digits.
    let l = lay(&doc_in("TH SarabunIT๙", 16.0, "12", |_| {}));
    let pl = para_layout(&l, 0);
    let face = &pl.styles[pl.clusters[0].style as usize].face;
    assert_eq!(pl.glyphs[0].gid, face.glyph_for('๑'));
    assert!((pl.clusters[0].adv - 0.449 * 16.0).abs() < 0.001);
    let l = lay(&doc_in("TH Sarabun New", 16.0, "12", |_| {}));
    let pl = para_layout(&l, 0);
    assert!((pl.clusters[0].adv - 0.362 * 16.0).abs() < 0.001);
    // Fonts that resolve to themselves are left alone.
    let l = lay(&doc_in("Sarabun", 16.0, "ก", |_| {}));
    let st = &para_layout(&l, 0).styles[0];
    assert!(st.fit.is_none() && st.glyph_scale == 1.0);
}

/// Word kerns letter pairs only with "Kerning for fonts" (`w:kern`) set, at that size and above;
/// TH Sarabun New drawn with Sarabun takes TH Sarabun New's pairs.
#[test]
fn letters_are_kerned_only_when_the_run_asks() {
    let width = |font: &str, kern: Option<f32>| {
        let l = lay(&doc_in(font, 16.0, "AV", |c| c.kern = kern));
        para_layout(&l, 0).clusters.iter().map(|c| c.adv).sum::<f32>()
    };
    for font in ["Sarabun", "TH Sarabun New"] {
        let plain = width(font, None);
        assert_eq!(plain, width(font, Some(0.0)), "{font}: 0 is off");
        assert_eq!(plain, width(font, Some(20.0)), "{font}: 16 pt is below the 20 pt threshold");
        assert!(width(font, Some(1.0)) < plain - 0.1, "{font}: kerned from 1 pt");
    }
    // TH Sarabun New's A V pair: -50 units.
    assert!((width("TH Sarabun New", None) - width("TH Sarabun New", Some(1.0)) - 0.05 * 16.0).abs() < 0.001);
}

/// Word sets no standard ligatures unless `w14:ligatures` asks for them.
#[test]
fn ligatures_only_when_the_run_asks() {
    let glyphs = |lig: Option<&str>| {
        let l = lay(&doc_in("Sarabun", 16.0, "fi", |c| c.ligatures = lig.map(str::to_string)));
        para_layout(&l, 0).glyphs.len()
    };
    assert_eq!(glyphs(None), 2);
    assert_eq!(glyphs(Some("none")), 2);
    assert_eq!(glyphs(Some("standardContextual")), 1, "Sarabun has an fi ligature");
    assert_eq!(glyphs(Some("all")), 1);
    // A value a .docx can't hold (and a save would drop) turns nothing on.
    assert_eq!(glyphs(Some("Standard")), 2);
    // Complex scripts keep the font's own ligatures (Word always forms Arabic لأ): Sarabun's ฤๅ.
    let thai = |lig: Option<&str>| {
        let l = lay(&doc_in("Sarabun", 16.0, "ฤๅ", |c| c.ligatures = lig.map(str::to_string)));
        para_layout(&l, 0).glyphs.len()
    };
    assert_eq!(thai(None), 1, "Sarabun joins ฤๅ");
    assert_eq!(thai(Some("none")), 1);
}
