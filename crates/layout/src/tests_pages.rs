//! Page geometry: mirrored margins, headers and footers taller than their margin (also when
//! linked to the previous section), endnotes continuing over pages, and the caret in a footnote
//! continued on the next page.
use super::*;
use doc4me_doc::para::{InlineObject, NoteKind};
use doc4me_doc::{PartKind, Pos, Table, para_block};

fn lay(doc: &Document) -> DocLayout {
    let mut c = LayoutCache::new();
    layout(doc, &mut c, &LayoutOptions::default())
}

fn paras(text: &str, n: usize) -> Vec<Arc<Block>> {
    (1..=n).map(|i| para_block(Paragraph::with_text(&format!("{text} {i}"), Default::default()))).collect()
}

/// Top and bottom of the lines of `story` among `items`.
fn extent(items: &[Placed], story: StoryRef) -> Option<(f32, f32)> {
    items
        .iter()
        .filter_map(|it| match it {
            Placed::Lines { story: s, y, para, l0, l1, .. } if *s == story => Some((*y, item_bottom(*y, para, *l0, *l1)?)),
            _ => None,
        })
        .reduce(|a, b| (a.0.min(b.0), a.1.max(b.1)))
}

/// The left edge of the body text on each page.
fn body_left(l: &DocLayout) -> Vec<f32> {
    l.pages
        .iter()
        .map(|p| p.items.iter().find_map(|it| if let Placed::Lines { story: StoryRef::Body, x, .. } = it { Some(*x) } else { None }).unwrap())
        .collect()
}

#[test]
fn mirrored_margins_put_the_inside_margin_toward_the_binding() {
    // #452: inside 108 (plus a 10pt gutter), outside 36. Odd pages: inside on the left; even
    // pages: on the right.
    let mut d = Document::from_text(&"Body line\n".repeat(90));
    d.last_section.margin_left = 108.0;
    d.last_section.margin_right = 36.0;
    d.last_section.gutter = 10.0;
    let plain = body_left(&lay(&d));
    assert!(plain.len() >= 2 && plain.iter().all(|x| (x - 118.0).abs() < 0.01), "not mirrored: {plain:?}");
    d.settings.mirror_margins = true;
    let l = lay(&d);
    let lefts = body_left(&l);
    for (i, (x, p)) in lefts.iter().zip(&l.pages).enumerate() {
        let want = if p.number % 2 == 1 { 118.0 } else { 36.0 };
        assert!((x - want).abs() < 0.01, "page {i} (number {}): text at {x}, want {want}", p.number);
        assert!((p.body.x - want).abs() < 0.01 && (p.body.w - d.last_section.text_width()).abs() < 0.01, "page {i}: {:?}", p.body);
    }
}

#[test]
fn a_tall_footer_pushes_the_body_up() {
    // #453: an eight-line footer reaches far above the 72pt bottom margin.
    let mut d = Document::from_text(&"Body line\n".repeat(60));
    let id = d.add_part(PartKind::Footer, paras("Footer line", 8));
    d.last_section.footers.default = Some(id);
    let l = lay(&d);
    assert!(l.pages.len() >= 2);
    let foot_top = extent(&l.pages[0].footer, StoryRef::Part(id)).unwrap().0;
    assert!(foot_top < 792.0 - 72.0, "the footer is taller than the margin: {foot_top}");
    for (i, p) in l.pages.iter().enumerate() {
        let (_, body_bottom) = extent(&p.items, StoryRef::Body).unwrap();
        let (top, _) = extent(&p.footer, StoryRef::Part(id)).unwrap();
        assert!(body_bottom <= top + 0.01, "page {i}: body ends at {body_bottom}, footer starts at {top}");
        assert!(p.body.bottom() <= top + 0.01, "page {i}: {:?}", p.body);
    }
    // Every body paragraph is still there, once.
    let placed: usize = l.pages.iter().flat_map(|p| &p.items).filter(|it| matches!(it, Placed::Lines { story: StoryRef::Body, l0: 0, .. })).count();
    assert_eq!(placed, d.body.len());
    // A one-line footer fits in the margin: the body keeps its full height.
    d.parts.insert(id, doc4me_doc::Part { kind: PartKind::Footer, blocks: paras("Footer", 1), body: Default::default() });
    let l = lay(&d);
    assert!((l.pages[0].body.bottom() - (792.0 - 72.0)).abs() < 0.01, "{:?}", l.pages[0].body);
}

#[test]
fn a_linked_tall_header_pushes_the_next_sections_body_down() {
    // #454: section 2 has no header of its own and shows section 1's (link to previous).
    let mut d = Document::from_text("First section\nSecond section");
    let id = d.add_part(PartKind::Header, paras("Header line", 8));
    let first = SectionProps { headers: doc4me_doc::section::HeaderSet { default: Some(id), ..Default::default() }, ..d.last_section.clone() };
    d.para_mut(StoryRef::Body, &Path::top(0)).unwrap().section = Some(Box::new(first));
    let l = lay(&d);
    assert_eq!(l.pages.len(), 2);
    for (i, p) in l.pages.iter().enumerate() {
        assert_eq!(p.header_story, Some(id), "page {i}");
        let (_, header_bottom) = extent(&p.header, StoryRef::Part(id)).unwrap();
        assert!(header_bottom > 72.0, "the header reaches past the top margin");
        let (body_top, _) = extent(&p.items, StoryRef::Body).unwrap();
        assert!(body_top >= header_bottom - 0.01, "page {i}: body at {body_top}, header ends at {header_bottom}");
    }
    assert_eq!(l.pages[0].body.y, l.pages[1].body.y, "same header, same body top");
}

#[test]
fn long_endnotes_continue_on_new_pages() {
    // #456: an 80-paragraph endnote after a one-line body.
    let mut d = Document::from_text("Body");
    let id = d.add_part(PartKind::Endnote, paras("Endnote paragraph", 80));
    d.insert_object(&Pos::body(0, 4), InlineObject::NoteRef { kind: NoteKind::Endnote, id, custom: String::new() }, &Default::default()).unwrap();
    let l = lay(&d);
    assert!(l.pages.len() >= 2, "{} pages", l.pages.len());
    let mut seen = Vec::new();
    for (i, p) in l.pages.iter().enumerate() {
        if let Some((top, bottom)) = extent(&p.items, StoryRef::Part(id)) {
            assert!(top >= p.body.y - 0.01 && bottom <= p.body.bottom() + 0.01, "page {i}: note at {top}..{bottom}, body {:?}", p.body);
        }
        for it in &p.items {
            if let Placed::Lines { story: StoryRef::Part(s), path, l0: 0, .. } = it
                && *s == id
            {
                seen.push(path.0.clone());
            }
        }
    }
    assert_eq!(seen, (0..80u32).map(|i| vec![i]).collect::<Vec<_>>(), "every paragraph once, in order");
    // The caret reaches the last paragraph, on a page.
    let c = l.caret(&Pos { story: StoryRef::Part(id), path: Path::top(79), off: 0 }).unwrap();
    assert!(c.top < 792.0 - 72.0, "{c:?}");
}

#[test]
fn caret_in_a_continued_footnote_is_found_on_its_page() {
    // #457: one long footnote paragraph split over pages 1 and 2.
    let mut d = Document::from_text(&"Body text line.\n".repeat(16));
    let id = d.add_part(
        PartKind::Footnote,
        vec![para_block(Paragraph::with_text(&"A long footnote that runs on and on. ".repeat(100), Default::default()))],
    );
    d.insert_object(&Pos::body(4, 4), InlineObject::NoteRef { kind: NoteKind::Footnote, id, custom: String::new() }, &Default::default()).unwrap();
    let l = lay(&d);
    // The first line of the piece on page 2.
    let (para, k) = l.pages[1]
        .items
        .iter()
        .find_map(|it| match it {
            Placed::Lines { story: StoryRef::Part(s), para, l0, .. } if *s == id && *l0 > 0 => Some((para.clone(), *l0)),
            _ => None,
        })
        .expect("the note continues on page 2");
    let at = |off: usize| Pos { story: StoryRef::Part(id), path: Path::top(0), off };
    let cont = at(para.lines[k + 1].start);
    for hint in [0, 1] {
        assert_eq!(l.caret_on(&at(0), hint).map(|c| c.page), Some(0), "start, hint {hint}");
        let c = l.caret_on(&cont, hint).unwrap_or_else(|| panic!("no caret with hint {hint}"));
        assert_eq!(c.page, 1, "hint {hint}");
        // Up goes to the line above, not to the note's start.
        let (up, _) = l.vertical(&cont, None, -1, hint).unwrap();
        assert_eq!(para.line_of(up.off), k, "hint {hint}: {up:?}");
    }
}

/// The pages a body paragraph's lines are on, in order, with how many lines each holds.
fn line_pages(l: &DocLayout, path: &Path) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    for (pi, page) in l.pages.iter().enumerate() {
        for it in &page.items {
            if let Placed::Lines { story: StoryRef::Body, path: p, l0, l1, .. } = it
                && p == path
            {
                match out.last_mut() {
                    Some((last, n)) if *last == pi => *n += l1 - l0,
                    _ => out.push((pi, l1 - l0)),
                }
            }
        }
    }
    out
}

fn long_para(words: usize) -> Arc<Block> {
    para_block(Paragraph::with_text(&"word ".repeat(words), Default::default()))
}

/// Widow/orphan control: wherever the page ends, a three-line paragraph is never split (a 2 + 1
/// split leaves a widow, 1 + 2 an orphan), and a four-line one only splits 2 + 2.
#[test]
fn widow_control_never_leaves_one_line_of_a_short_paragraph() {
    // Word counts that make three and four lines.
    let lines = |words: usize| {
        let mut d = Document::from_text("");
        d.body.insert(0, long_para(words));
        line_pages(&lay(&d), &Path::top(0)).iter().map(|(_, k)| k).sum::<usize>()
    };
    let three_words = (5..200).find(|w| lines(*w) == 3).expect("three lines");
    let four_words = (5..200).find(|w| lines(*w) == 4).expect("four lines");
    for n in 30..60 {
        let mut d = Document::from_text(&"Body line\n".repeat(n));
        let three = n;
        d.body.insert(three, long_para(three_words));
        d.body.insert(three + 1, long_para(four_words));
        let l = lay(&d);
        let first = line_pages(&l, &Path::top(three));
        assert_eq!(first.len(), 1, "{n} lines before: a three-line paragraph split {first:?}");
        for (path, what) in [(three, "first"), (three + 1, "second")] {
            let pages = line_pages(&l, &Path::top(path));
            if pages.len() > 1 {
                assert!(pages.first().is_some_and(|(_, k)| *k >= 2) && pages.last().is_some_and(|(_, k)| *k >= 2), "{n}: {what} {pages:?}");
            }
        }
    }
}

/// Keep with next runs on: two headings that keep with the next paragraph stay on the page of
/// that paragraph's first lines, wherever the page ends.
#[test]
fn a_chain_of_keep_with_next_headings_stays_with_the_text() {
    for n in 30..60 {
        let mut d = Document::from_text(&"Body line\n".repeat(n));
        let mut h1 = Paragraph::with_text("Heading one", Default::default());
        h1.props.keep_next = Some(true);
        let mut h2 = Paragraph::with_text("Heading two", Default::default());
        h2.props.keep_next = Some(true);
        d.body.insert(n, para_block(h1));
        d.body.insert(n + 1, para_block(h2));
        d.body.insert(n + 2, long_para(60));
        let l = lay(&d);
        let page = |i: usize| line_pages(&l, &Path::top(i)).first().map(|(p, _)| *p);
        assert_eq!((page(n), page(n + 1)), (page(n + 2), page(n + 2)), "{n} lines before");
    }
}

/// A table row whose paragraphs keep with the next goes to the next page with that row (a heading
/// row with the rows it heads), and a page break typed in a cell starts a new page there.
#[test]
fn table_rows_keep_with_next_and_page_breaks_in_cells() {
    let build = |n: usize, keep: bool| {
        let mut d = Document::from_text(&"Body line\n".repeat(n));
        let mut t = Table::new(2, 1, 468.0);
        let mut head = Paragraph::with_text("Section heading", Default::default());
        head.props.keep_next = Some(keep);
        t.rows[0].cells[0].blocks = vec![para_block(head)];
        t.rows[1].cells[0].blocks = paras("Row text", 6);
        t.rows[1].props.cant_split = true;
        d.insert_block(StoryRef::Body, &Path::top(n), Block::Table(t)).unwrap();
        d
    };
    let page_of = |l: &DocLayout, path: Vec<u32>| l.caret(&Pos { story: StoryRef::Body, path: Path(path), off: 0 }).map(|c| c.page);
    // Where, without Keep with next, the heading row ends page 1 and the next row starts page 2.
    let n = (20..60)
        .find(|n| {
            let l = lay(&build(*n, false));
            page_of(&l, vec![*n as u32, 0, 0, 0]).map(|p| p + 1) == page_of(&l, vec![*n as u32, 1, 0, 0])
        })
        .expect("a page end between the rows");
    let l = lay(&build(n, true));
    assert_eq!(page_of(&l, vec![n as u32, 0, 0, 0]), page_of(&l, vec![n as u32, 1, 0, 0]), "the heading row goes with the next one");

    let mut d = Document::from_text("Before");
    let mut t = Table::new(1, 1, 468.0);
    t.rows[0].cells[0].blocks = vec![
        para_block(Paragraph::with_text("Top part", Default::default())),
        para_block(Paragraph::with_text(&format!("{}After the break", doc4me_doc::para::PAGE_BREAK), Default::default())),
    ];
    d.insert_block(StoryRef::Body, &Path::top(1), Block::Table(t.clone())).unwrap();
    let l = lay(&d);
    let text_page =
        |l: &DocLayout, para: u32, off: usize| l.caret(&Pos { story: StoryRef::Body, path: Path(vec![1, 0, 0, para]), off }).map(|c| c.page);
    let top = text_page(&l, 0, 0).expect("laid out");
    assert_eq!(text_page(&l, 1, 1), Some(top + 1), "the text after the break starts the next page");
    // Not in a row of exact height, which never breaks.
    t.rows[0].props.height = Some(200.0);
    t.rows[0].props.height_rule = doc4me_doc::props::HeightRule::Exact;
    let mut d = Document::from_text("Before");
    d.insert_block(StoryRef::Body, &Path::top(1), Block::Table(t)).unwrap();
    let l = lay(&d);
    assert_eq!(text_page(&l, 1, 1), text_page(&l, 0, 0));
}

/// Keep with next on a long paragraph ties only its last lines to the next paragraph: wherever
/// the page ends, its last line shares a page with the next one's first, and it may break
/// across pages (Word doesn't move it whole).
#[test]
fn a_long_keep_with_next_paragraph_breaks_and_keeps_its_end_with_the_next() {
    let mut split_seen = false;
    for n in 25..60 {
        let mut d = Document::from_text(&"Body line\n".repeat(n));
        let mut long = Paragraph::with_text(&"word ".repeat(140), Default::default());
        long.props.keep_next = Some(true);
        d.body.insert(n, para_block(long));
        d.body.insert(n + 1, long_para(30));
        let l = lay(&d);
        let first = line_pages(&l, &Path::top(n));
        let next = line_pages(&l, &Path::top(n + 1));
        assert_eq!(first.last().map(|(p, _)| *p), next.first().map(|(p, _)| *p), "{n}: {first:?} / {next:?}");
        split_seen |= first.len() > 1;
    }
    assert!(split_seen, "the long paragraph never broke across pages");
}

/// Word's automatic (HTML) spacing: 14 pt between paragraphs, none between the items of a list
/// nor above the document's first paragraph.
#[test]
fn automatic_spacing_skips_list_items_and_the_first_paragraph() {
    let mut d = Document::from_text("");
    let num = d.numbering.add_list(doc4me_doc::ListKind::Bullet);
    let auto = |text: &str, list: bool| {
        let mut p = Paragraph::with_text(text, Default::default());
        p.props.space_before_auto = Some(true);
        p.props.space_after_auto = Some(true);
        p.props.space_before = Some(5.0);
        p.props.space_after = Some(5.0);
        if list {
            p.props.numbering = Some(doc4me_doc::props::NumRef { num, level: 0 });
        }
        para_block(p)
    };
    d.body = vec![auto("Title", false), auto("Intro", false), auto("One", true), auto("Two", true), auto("After", false)];
    let l = lay(&d);
    let top = |i: u32| l.caret(&Pos { story: StoryRef::Body, path: Path::top(i as usize), off: 0 }).unwrap().top;
    let line = top(3) - top(2);
    assert!((top(0) - d.last_section.margin_top).abs() < 6.0, "no space above the first paragraph: {}", top(0));
    assert!((top(2) - top(1) - line - 14.0).abs() < 0.5, "14 pt between paragraphs: {} vs line {line}", top(2) - top(1));
    assert!((top(4) - top(3) - line - 14.0).abs() < 0.5, "14 pt after the list");
    let height = l.caret(&Pos { story: StoryRef::Body, path: Path::top(2), off: 0 }).unwrap().height;
    assert!((line - height).abs() < 1.0, "no space between list items: {line} for a {height} pt line");
}
