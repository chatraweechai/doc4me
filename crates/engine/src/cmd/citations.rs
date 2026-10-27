//! References: citations and bibliography (APA, MLA, Chicago, IEEE, Harvard, ISO 690, Turabian,
//! GB/T 7714, GOST, SIST02), source manager, index, table of figures, cross-references, table
//! of authorities, footnote options.

use doc4me_doc::para::InlineObject;
use doc4me_doc::props::{CharProps, TabAlign, TabLeader, TabStop};
use doc4me_doc::{Block, Document, Paragraph, Path, Pos, Source, StoryRef};
use serde_json::{Value, json};

use super::{delete_selection, sel_result};
use crate::{CmdError, CmdResult, CommandSpec, Selection, Session, p};

/// How a style orders its bibliography (and, for numbered styles, numbers its citations).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    /// Alphabetical by the first author's surname (the title when there is no author).
    Author,
    /// Alphabetical by title.
    Title,
    /// In order of first citation in the document.
    Cited,
}

/// How one author's name is written in a bibliography entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Name {
    /// Rivera, Alex
    LastFirst,
    /// Alex Rivera
    FirstLast,
    /// Rivera, A.
    LastInitials,
    /// A. Rivera
    InitialsLast,
    /// Rivera A.
    LastSpaceInitials,
    /// RIVERA, Alex
    UpperLastFirst,
    /// RIVERA A
    UpperLastBareInitials,
}

/// The author list of a bibliography entry.
#[derive(Clone, Copy, Debug)]
pub struct Names {
    /// The first author's form, and every later author's.
    pub first: Name,
    pub rest: Name,
    /// Between names; between exactly two; before the last of three or more.
    pub sep: &'static str,
    pub two: &'static str,
    pub last: &'static str,
    /// List at most this many authors, then `et_al` (0: list them all).
    pub max: usize,
    pub et_al: &'static str,
}

/// The in-text citation.
#[derive(Clone, Copy, Debug)]
pub enum Cite {
    /// `(Who<year_sep>Year<pages_sep>Pages)`.
    AuthorDate { year_sep: &'static str, pages_sep: &'static str },
    /// `(Who Pages)`.
    AuthorPage,
    /// `<open>n<close>`; pages follow `pages_sep` inside the brackets (left out when `None`).
    Number { open: &'static str, close: &'static str, pages_sep: Option<&'static str> },
}

/// A citation and bibliography style, as data. Entry templates fill `{n}`, `{names}`, `{year}`,
/// `{title}`, `{city}`, `{publisher}`, `{pub}` (city: publisher), `{journal}`, `{volume}`,
/// `{pages}`, `{url}` and `{type}` (GB/T 7714 document type code); text between `⟦` and `⟧` is
/// left out when a field in it is empty, and a full stop never doubles up.
#[derive(Debug)]
pub struct CiteStyle {
    /// The name shown in the style picker and stored in the session.
    pub name: &'static str,
    /// Lowercase letter/digit prefixes of Word's style names and style-sheet file names that mean
    /// this style (`gostname` matches "GOST - Name Sort"); the longest match wins.
    pub aliases: &'static [&'static str],
    pub cite: Cite,
    /// Joins two authors in a citation ("&" or "and").
    pub and: &'static str,
    /// Cite this many authors or more as "First et al.".
    pub et_al_from: usize,
    pub order: Order,
    pub names: Names,
    /// Entry template for a book (or anything without a journal), and for a journal article.
    pub book: &'static str,
    pub article: &'static str,
    /// Default title of the bibliography.
    pub bib_title: &'static str,
}

impl CiteStyle {
    /// Numbered styles cite `[1]`/`(1)` and list entries without a hanging indent.
    pub fn numbered(&self) -> bool {
        matches!(self.cite, Cite::Number { .. })
    }
}

const fn names(first: Name, rest: Name, sep: &'static str, two: &'static str, last: &'static str) -> Names {
    Names { first, rest, sep, two, last, max: 0, et_al: "" }
}

const AUTHOR_DATE: Cite = Cite::AuthorDate { year_sep: " ", pages_sep: ", p. " };

/// Every style, in picker order (alphabetical). Formats follow the published style guides'
/// rules for the source types Doc4Me records, written in our own words.
pub static STYLES: [CiteStyle; 12] = [
    CiteStyle {
        name: "APA",
        aliases: &["apa"],
        cite: Cite::AuthorDate { year_sep: ", ", pages_sep: ", p. " },
        and: "&",
        et_al_from: 3,
        order: Order::Author,
        names: names(Name::LastInitials, Name::LastInitials, ", ", ", & ", ", & "),
        book: "⟦{names} ⟧({year}). {title}.⟦ {publisher}.⟧⟦ {url}⟧",
        article: "⟦{names} ⟧({year}). {title}. {journal}⟦, {volume}⟧⟦, {pages}⟧.⟦ {url}⟧",
        bib_title: "References",
    },
    CiteStyle {
        name: "Chicago",
        aliases: &["chicago"],
        cite: Cite::AuthorDate { year_sep: " ", pages_sep: ", " },
        and: "and",
        et_al_from: 3,
        order: Order::Author,
        names: names(Name::LastFirst, Name::FirstLast, ", ", ", and ", ", and "),
        book: "⟦{names}. ⟧{year}. {title}.⟦ {pub}.⟧⟦ {url}⟧",
        article: "⟦{names}. ⟧{year}. {title}. {journal}⟦, {volume}⟧⟦, {pages}⟧.⟦ {url}⟧",
        bib_title: "References",
    },
    // GB/T 7714 sequence-coding: numbered by first citation, surnames in capitals with bare
    // initials, at most three authors, a document type code after the title.
    CiteStyle {
        name: "GB/T 7714",
        aliases: &["gb"],
        cite: Cite::Number { open: "[", close: "]", pages_sep: Some(", ") },
        and: "and",
        et_al_from: 3,
        order: Order::Cited,
        names: Names { max: 3, et_al: ", et al.", ..names(Name::UpperLastBareInitials, Name::UpperLastBareInitials, ", ", ", ", ", ") },
        book: "[{n}] ⟦{names}. ⟧{title}[{type}].⟦ {pub},⟧ {year}.⟦ {url}.⟧",
        article: "[{n}] ⟦{names}. ⟧{title}[{type}]. {journal}, {year}⟦, {volume}⟧⟦: {pages}⟧.⟦ {url}.⟧",
        bib_title: "References",
    },
    // GOST 7.1: a numbered list in alphabetical order (by author or by title); areas separated
    // by " – ", the container after "//".
    CiteStyle {
        name: "GOST (name sort)",
        aliases: &["gost", "gostname"],
        cite: Cite::Number { open: "[", close: "]", pages_sep: Some(", p. ") },
        and: "and",
        et_al_from: 3,
        order: Order::Author,
        names: names(Name::LastSpaceInitials, Name::LastSpaceInitials, ", ", ", ", ", "),
        book: GOST_BOOK,
        article: GOST_ARTICLE,
        bib_title: "References",
    },
    CiteStyle {
        name: "GOST (title sort)",
        aliases: &["gosttitle"],
        cite: Cite::Number { open: "[", close: "]", pages_sep: Some(", p. ") },
        and: "and",
        et_al_from: 3,
        order: Order::Title,
        names: names(Name::LastSpaceInitials, Name::LastSpaceInitials, ", ", ", ", ", "),
        book: GOST_BOOK,
        article: GOST_ARTICLE,
        bib_title: "References",
    },
    // Harvard as taught by Anglia Ruskin: "Surname, I., Year. Title. Place: Publisher."
    CiteStyle {
        name: "Harvard (Anglia)",
        aliases: &["harvard"],
        cite: Cite::AuthorDate { year_sep: ", ", pages_sep: ", p. " },
        and: "and",
        et_al_from: 4,
        order: Order::Author,
        names: names(Name::LastInitials, Name::LastInitials, ", ", " and ", " and "),
        book: "⟦{names}, ⟧{year}. {title}.⟦ {pub}.⟧⟦ Available at: <{url}>.⟧",
        article: "⟦{names}, ⟧{year}. {title}. {journal}⟦, {volume}⟧⟦, pp. {pages}⟧.⟦ Available at: <{url}>.⟧",
        bib_title: "References",
    },
    CiteStyle {
        name: "IEEE",
        aliases: &["ieee"],
        cite: Cite::Number { open: "[", close: "]", pages_sep: None },
        and: "and",
        et_al_from: 3,
        order: Order::Cited,
        names: names(Name::InitialsLast, Name::InitialsLast, ", ", ", ", ", "),
        book: "[{n}] ⟦{names}, ⟧“{title},”⟦ {pub},⟧ {year}.⟦ {url}⟧",
        article: "[{n}] ⟦{names}, ⟧“{title},” {journal}⟦, {volume}⟧⟦, {pages}⟧, {year}.⟦ {url}⟧",
        bib_title: "References",
    },
    // ISO 690 first element and date: surnames in capitals, the year after the names.
    CiteStyle {
        name: "ISO 690 (author-date)",
        aliases: &["iso690"],
        cite: AUTHOR_DATE,
        and: "and",
        et_al_from: 4,
        order: Order::Author,
        names: names(Name::UpperLastFirst, Name::UpperLastFirst, ", ", " and ", " and "),
        book: "⟦{names}, ⟧{year}. {title}.⟦ {pub}.⟧⟦ Available from: {url}⟧",
        article: "⟦{names}, ⟧{year}. {title}. {journal}⟦, {volume}⟧⟦, {pages}⟧.⟦ Available from: {url}⟧",
        bib_title: "References",
    },
    // ISO 690 numeric reference: numbered by first citation, the year with the imprint.
    CiteStyle {
        name: "ISO 690 (numerical)",
        aliases: &["iso690num", "iso690nmerical"],
        cite: Cite::Number { open: "(", close: ")", pages_sep: Some(", p. ") },
        and: "and",
        et_al_from: 4,
        order: Order::Cited,
        names: names(Name::UpperLastFirst, Name::UpperLastFirst, ", ", " and ", " and "),
        book: "{n}. ⟦{names}. ⟧{title}.⟦ {pub},⟧ {year}.⟦ Available from: {url}⟧",
        article: "{n}. ⟦{names}. ⟧{title}. {journal}. {year}⟦, {volume}⟧⟦, {pages}⟧.⟦ Available from: {url}⟧",
        bib_title: "References",
    },
    CiteStyle {
        name: "MLA",
        aliases: &["mla"],
        cite: Cite::AuthorPage,
        and: "and",
        et_al_from: 3,
        order: Order::Author,
        names: Names { max: 1, et_al: ", et al.", ..names(Name::LastFirst, Name::LastFirst, ", ", ", ", ", ") },
        book: "⟦{names}. ⟧“{title}.”⟦ {publisher},⟧ {year}.⟦ {url}⟧",
        article: "⟦{names}. ⟧“{title}.” {journal}⟦, {volume}⟧⟦, {pages}⟧, {year}.⟦ {url}⟧",
        bib_title: "Works Cited",
    },
    // SIST 02: numbered by first citation, full names separated by semicolons.
    CiteStyle {
        name: "SIST02",
        aliases: &["sist"],
        cite: Cite::Number { open: "(", close: ")", pages_sep: Some(", p. ") },
        and: "and",
        et_al_from: 3,
        order: Order::Cited,
        names: names(Name::LastFirst, Name::LastFirst, "; ", "; ", "; "),
        book: "{n}) ⟦{names}. ⟧{title}.⟦ {city},⟧⟦ {publisher},⟧ {year}.⟦ {url}⟧",
        article: "{n}) ⟦{names}. ⟧{title}. {journal}. {year}⟦, vol. {volume}⟧⟦, p. {pages}⟧.⟦ {url}⟧",
        bib_title: "References",
    },
    // Turabian: author-date citations, a bibliography with the year in the imprint.
    CiteStyle {
        name: "Turabian",
        aliases: &["turabian"],
        cite: Cite::AuthorDate { year_sep: " ", pages_sep: ", " },
        and: "and",
        et_al_from: 4,
        order: Order::Author,
        names: names(Name::LastFirst, Name::FirstLast, ", ", ", and ", ", and "),
        book: "⟦{names}. ⟧{title}.⟦ {pub},⟧ {year}.⟦ {url}.⟧",
        article: "⟦{names}. ⟧“{title}.” {journal}⟦ {volume}⟧ ({year})⟦: {pages}⟧.⟦ {url}.⟧",
        bib_title: "Bibliography",
    },
];

const GOST_BOOK: &str = "{n}. ⟦{names} ⟧{title}. –⟦ {city} :⟧⟦ {publisher},⟧ {year}.⟦ – URL: {url}.⟧";
const GOST_ARTICLE: &str = "{n}. ⟦{names} ⟧{title} // {journal}. – {year}.⟦ – Vol. {volume}.⟧⟦ – P. {pages}.⟧⟦ – URL: {url}.⟧";

/// The style called `name`: one of ours (any case), or Word's style name or style-sheet file
/// name (`Harvard - Anglia`, `\GostTitle.XSL`).
pub fn find_style(name: &str) -> Option<&'static CiteStyle> {
    if let Some(st) = STYLES.iter().find(|x| x.name.eq_ignore_ascii_case(name.trim())) {
        return Some(st);
    }
    let key: String = name.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect();
    let key = key.strip_suffix("xsl").unwrap_or(&key);
    STYLES
        .iter()
        .flat_map(|st| st.aliases.iter().map(move |a| (st, *a)))
        .filter(|(_, a)| key.starts_with(a))
        .max_by_key(|(_, a)| a.len())
        .map(|(st, _)| st)
}

/// The session's style; APA when the name is unknown.
pub fn style(name: &str) -> &'static CiteStyle {
    find_style(name).unwrap_or(&STYLES[0])
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        CommandSpec::new("references.sources", "Manage Sources", "References › Citations & Bibliography", sources)
            .params(r#"{"add"?: Source, "remove"?: tag} → list"#),
        CommandSpec::new("references.citation", "Insert Citation", "References › Citations & Bibliography", citation).params(r#"{"tag"?: string, "source"?: Source (added if new), "pages"?: string}"#),
        // Insert Citation › Add New Placeholder…: cite a source known only by its name for now.
        CommandSpec::new("references.placeholder", "Add New Placeholder", "References › Citations & Bibliography", |s, v| {
            let name = p::req_str(v, "name")?.trim().to_string();
            if name.is_empty() || name.chars().count() > 255 || !name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                return Err(CmdError::Params("a placeholder's name is letters, digits, _ and -, with no spaces".into()));
            }
            if !s.doc.sources.iter().any(|x| x.tag == name) {
                s.doc.sources.push(Source { tag: name.clone(), kind: "other".into(), ..Default::default() });
            }
            citation(s, &json!({"tag": name}))
        })
        .params(r#"{"name": string (letters, digits, _ and -)} — cites the placeholder, adding it as a source if new"#),
        CommandSpec::new("references.citationStyle", "Style", "References › Citations & Bibliography", |s, v| {
            let st = p::str(v, "style").unwrap_or("APA");
            let st = find_style(st).ok_or_else(|| {
                CmdError::Params(format!("style must be one of {:?}", STYLES.iter().map(|x| x.name).collect::<Vec<_>>()))
            })?;
            s.bib_style = st.name.to_string();
            // Recorded in the document too, so a save keeps it.
            s.doc.settings.bib_style = Some(st.name.to_string());
            update_citations(s)?;
            Ok(json!({"style": s.bib_style}))
        })
        .params(
            r#"{"style": "APA|Chicago|GB/T 7714|GOST (name sort)|GOST (title sort)|Harvard (Anglia)|IEEE|ISO 690 (author-date)|ISO 690 (numerical)|MLA|SIST02|Turabian" (Word's style names work too)}"#,
        ),
        CommandSpec::new("references.bibliography", "Bibliography", "References › Citations & Bibliography", |s, v| {
            let title = p::str(v, "title").unwrap_or(style(&s.bib_style).bib_title).to_string();
            generated_list(s, "BIBLIOGRAPHY", &title)?;
            sel_result(s)
        })
        .params(r#"{"title"?: string}"#),
        CommandSpec::new("references.tableOfFigures", "Insert Table of Figures", "References › Captions", table_of_figures).params(
            r#"{"label"?: string (captions with this label; default Figure), "style"?: string (Options › Style: paragraphs of this style instead), "identifier"?: "A"-"Z" (Options › Table entry fields: TC fields with this \f letter), "includeLabel"?: bool (default true: "Figure 1: …"; false drops the label and number), "pages"?: bool (default true), "rightAlign"?: bool (default true), "leader"?: "dot|hyphen|underscore|middleDot|none"}"#,
        ),
        CommandSpec::new("references.updateFigures", "Update Table", "References › Captions", |s, _| {
            update_generated(s)?;
            sel_result(s)
        }),
        CommandSpec::new("references.markCitation", "Mark Citation", "References › Table of Authorities", mark_citation)
            .params(r#"{"entry"?: string (the long citation; default the selection), "short"?: string (the short citation; default the long one), "category"?: 1-16 (default 1, Cases)}"#)
            .key(super::non_mac("Alt+Shift+I")),
        CommandSpec::new("references.toaCategory", "Edit Category", "References › Table of Authorities", toa_category)
            .params(r#"{"category"?: 1-16, "name"?: string (empty: Word's name again)} → {"categories": [16 names]}"#)
            .pure(),
        // Mark Table of Contents Entry (Alt+Shift+O): a hidden TC field that a table of contents
        // with "Table entry fields" (`tcFields`, TOC's \f) lists.
        CommandSpec::new("references.markTocEntry", "Mark Table of Contents Entry", "References › Table of Contents", mark_toc_entry)
            .params(r#"{"entry"?: string (the selection), "identifier"?: "A"-"Z" (which table; default C, a table of contents), "level"?: 1-9 (default 1)}"#)
            .key(super::non_mac("Alt+Shift+O")),
        CommandSpec::new("references.tableOfAuthorities", "Insert Table of Authorities", "References › Table of Authorities", table_of_authorities)
            .params(r#"{"category"?: 0-16 (0 = All: one table per category in use, each under its name), "passim"?: bool (default true: five or more pages read "passim"), "leader"?: "dot|hyphen|underscore|middleDot|none", "names"?: [16 strings] (headings for the categories the user hasn't renamed, e.g. Word's names in the interface language)}"#),
        CommandSpec::new("references.updateAuthorities", "Update Table", "References › Table of Authorities", |s, _| {
            update_generated(s)?;
            sel_result(s)
        }),
        CommandSpec::new("insert.crossReference", "Cross-reference", "Insert › Links", cross_ref)
            .params(r#"{"to": "heading|bookmark|figure|table", "target": string (text / name / number), "show"?: "text|page|number|aboveBelow"} or {} to list targets"#),
        CommandSpec::new("references.noteOptions", "Footnote and Endnote", "References › Footnotes", super::references::note_options).params(
            r#"{"kind"?: "footnote|endnote", "pos"?: "pageBottom|beneathText" (footnotes) | "sectEnd|docEnd" (endnotes), "numFmt"?: "decimal|lowerRoman|upperRoman|lowerLetter|upperLetter|…", "numStart"?: 1-32767, "numRestart"?: "continuous|eachSect|eachPage" (each page: footnotes only), "scope"?: "document|section", "footnoteFormat"?: …, "endnoteFormat"?: …} — no settings: the options in effect at the caret"#,
        ),
        CommandSpec::new("references.researcher", "Researcher", "References › Research", |s, _| {
            s.status = "Researcher needs an online service; Doc4Me keeps your documents offline.".into();
            Ok(json!({"available": false}))
        })
        .pure(),
    ]
}

fn authors(a: &str) -> Vec<(String, String)> {
    a.split(';')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|x| match x.split_once(',') {
            Some((last, first)) => (last.trim().to_string(), first.trim().to_string()),
            None => match x.rsplit_once(' ') {
                Some((first, last)) => (last.trim().to_string(), first.trim().to_string()),
                None => (x.to_string(), String::new()),
            },
        })
        .collect()
}

/// "Alex Mei" → "A. M." (`dots`), or "A M" without them.
fn initials(first: &str, dots: bool) -> String {
    let sep = if dots { ". " } else { " " };
    let mut out = first.split_whitespace().filter_map(|w| w.chars().next()).map(String::from).collect::<Vec<_>>().join(sep);
    if dots && !out.is_empty() {
        out.push('.');
    }
    out
}

fn name(form: Name, last: &str, first: &str) -> String {
    let join = |a: String, sep: &str, b: String| if a.is_empty() || b.is_empty() { format!("{a}{b}") } else { format!("{a}{sep}{b}") };
    match form {
        Name::LastFirst => join(last.to_string(), ", ", first.to_string()),
        Name::FirstLast => join(first.to_string(), " ", last.to_string()),
        Name::LastInitials => join(last.to_string(), ", ", initials(first, true)),
        Name::InitialsLast => join(initials(first, true), " ", last.to_string()),
        Name::LastSpaceInitials => join(last.to_string(), " ", initials(first, true)),
        Name::UpperLastFirst => join(last.to_uppercase(), ", ", first.to_string()),
        Name::UpperLastBareInitials => join(last.to_uppercase(), " ", initials(first, false)),
    }
}

fn name_list(au: &[(String, String)], form: &Names) -> String {
    let shown = if form.max > 0 { au.get(..form.max).unwrap_or(au) } else { au };
    let parts: Vec<String> = shown.iter().enumerate().map(|(i, (l, f))| name(if i == 0 { form.first } else { form.rest }, l, f)).collect();
    let mut out = match parts.as_slice() {
        [] => String::new(),
        [one] => one.clone(),
        [a, b] => format!("{a}{}{b}", form.two),
        [init @ .., z] => format!("{}{}{z}", init.join(form.sep), form.last),
    };
    if shown.len() < au.len() {
        out.push_str(form.et_al);
    }
    out
}

/// Append `s`, dropping its leading full stop when `out` already ends with one ("M." + ". ").
fn push_text(out: &mut String, s: &str) {
    match s.strip_prefix('.') {
        Some(rest) if out.ends_with('.') => out.push_str(rest),
        _ => out.push_str(s),
    }
}

/// Fill one template segment; `None` when `optional` and a field in it is empty.
fn fill_segment(seg: &str, field: &dyn Fn(&str) -> String, optional: bool) -> Option<String> {
    let mut out = String::new();
    let mut rest = seg;
    while let Some((before, after)) = rest.split_once('{') {
        push_text(&mut out, before);
        let Some((key, after)) = after.split_once('}') else {
            push_text(&mut out, after);
            return Some(out);
        };
        let value = field(key);
        if value.is_empty() && optional {
            return None;
        }
        push_text(&mut out, &value);
        rest = after;
    }
    push_text(&mut out, rest);
    Some(out)
}

/// Fill an entry template (see [`CiteStyle`]).
fn render(template: &str, field: &dyn Fn(&str) -> String) -> String {
    let mut out = String::new();
    let mut rest = template;
    while !rest.is_empty() {
        let (seg, optional, after) = match rest.strip_prefix('⟦') {
            Some(group) => match group.split_once('⟧') {
                Some((g, after)) => (g, true, after),
                None => (group, true, ""),
            },
            None => match rest.split_once('⟦') {
                Some((plain, _)) => (plain, false, rest.get(plain.len()..).unwrap_or("")),
                None => (rest, false, ""),
            },
        };
        if let Some(text) = fill_segment(seg, field, optional) {
            push_text(&mut out, &text);
        }
        rest = after;
    }
    out
}

/// In-text citation for a style; `n` is the source's number in numbered styles.
/// A placeholder: a source known only by its tag (Insert Citation › Add New Placeholder), filled
/// in later. Its citations show the tag and the bibliography leaves it out, as in Word.
pub fn is_placeholder(src: &Source) -> bool {
    src.author.trim().is_empty() && src.title.trim().is_empty() && src.year.trim().is_empty()
}

pub fn cite(src: &Source, style_name: &str, n: usize, pages: &str) -> String {
    let st = style(style_name);
    if is_placeholder(src) && !matches!(st.cite, Cite::Number { .. }) {
        return format!("({})", src.tag);
    }
    let au = authors(&src.author);
    let lasts: Vec<&str> = au.iter().map(|a| a.0.as_str()).collect();
    let who = match lasts.as_slice() {
        [] => src.title.clone(),
        [one] => one.to_string(),
        [first, ..] if lasts.len() >= st.et_al_from.max(2) => format!("{first} et al."),
        [a, b] => format!("{a} {} {b}", st.and),
        [init @ .., z] => format!("{} {} {z}", init.join(", "), st.and),
    };
    match st.cite {
        Cite::AuthorPage if pages.is_empty() => format!("({who})"),
        Cite::AuthorPage => format!("({who} {pages})"),
        Cite::AuthorDate { year_sep, .. } if pages.is_empty() => format!("({who}{year_sep}{})", src.year),
        Cite::AuthorDate { year_sep, pages_sep } => format!("({who}{year_sep}{}{pages_sep}{pages})", src.year),
        Cite::Number { open, close, pages_sep: Some(sep) } if !pages.is_empty() => format!("{open}{n}{sep}{pages}{close}"),
        Cite::Number { open, close, .. } => format!("{open}{n}{close}"),
    }
}

/// GB/T 7714 document type code: M book, J journal article, R report, EB/OL web page, Z other;
/// "/OL" marks a source read online.
fn gb_type(src: &Source) -> String {
    let code = match src.kind.to_ascii_lowercase().as_str() {
        "website" | "web" => return "EB/OL".into(),
        _ if !src.journal.is_empty() => "J",
        "book" | "" => "M",
        "article" => "J",
        "report" => "R",
        _ => "Z",
    };
    if src.url.is_empty() { code.into() } else { format!("{code}/OL") }
}

/// Bibliography entry for a style; `n` is the source's number in numbered styles.
pub fn entry(src: &Source, style_name: &str, n: usize) -> String {
    let st = style(style_name);
    let au = authors(&src.author);
    let field = |key: &str| -> String {
        match key {
            "n" => n.to_string(),
            "names" => name_list(&au, &st.names),
            "year" if src.year.is_empty() => "n.d.".into(),
            "year" => src.year.clone(),
            "title" => src.title.clone(),
            "city" => src.city.clone(),
            "publisher" => src.publisher.clone(),
            "pub" => [src.city.as_str(), src.publisher.as_str()].into_iter().filter(|x| !x.is_empty()).collect::<Vec<_>>().join(": "),
            "journal" => src.journal.clone(),
            "volume" => src.volume.clone(),
            "pages" => src.pages.clone(),
            "url" => src.url.clone(),
            "type" => gb_type(src),
            _ => String::new(),
        }
    };
    render(if src.journal.is_empty() { st.book } else { st.article }, &field)
}

fn sources(s: &mut Session, v: &Value) -> CmdResult {
    if let Some(a) = v.get("add") {
        let mut src: Source = serde_json::from_value(a.clone()).map_err(|e| CmdError::Params(e.to_string()))?;
        if src.tag.is_empty() {
            let base: String = authors(&src.author)
                .first()
                .map(|x| x.0.clone())
                .unwrap_or_else(|| "Source".into())
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect();
            src.tag = format!("{base}{}", src.year);
        }
        // Edit Source keeps the source's place in the list and refreshes its citations.
        match s.doc.sources.iter().position(|x| x.tag == src.tag) {
            Some(i) => {
                if let Some(x) = s.doc.sources.get_mut(i) {
                    *x = src;
                }
                update_citations(s)?;
            }
            None => s.doc.sources.push(src),
        }
        s.touch();
    }
    if let Some(t) = p::str(v, "remove") {
        s.doc.sources.retain(|x| x.tag != t);
    }
    Ok(serde_json::to_value(&s.doc.sources).unwrap_or(Value::Null))
}

fn citation(s: &mut Session, v: &Value) -> CmdResult {
    if v.get("source").is_some() {
        sources(s, &json!({"add": v.get("source")}))?;
    }
    let tag = match p::str(v, "tag") {
        Some(t) => t.to_string(),
        None => match v.get("source") {
            Some(_) => s.doc.sources.last().map(|x| x.tag.clone()).unwrap_or_default(),
            None => return Err(CmdError::Params("give `tag` or `source`".into())),
        },
    };
    let src = s.doc.sources.iter().find(|x| x.tag == tag).cloned().ok_or_else(|| CmdError::Params(format!("no source `{tag}`")))?;
    let pages = p::str(v, "pages").unwrap_or("").to_string();
    let st = style(&s.bib_style);
    let n = number_of(&s.doc, &bib_order(&s.doc, st), &tag);
    let props = s.typing_props();
    let at = delete_selection(s)?;
    let instr = if pages.is_empty() { format!("CITATION {tag}") } else { format!("CITATION {tag} \\p {pages}") };
    let end = s.doc.insert_object(&at, InlineObject::Field { instr, result: cite(&src, st.name, n, &pages), locked: false, code: false }, &props)?;
    s.sel = Selection::caret(end);
    // A new citation can renumber the others (numbered styles count in order of first citation).
    if st.numbered() {
        refresh_citations(s)?;
    }
    Ok(json!({"tag": tag}))
}

/// The tag and `\p` pages of a CITATION field instruction.
fn citation_field(instr: &str) -> Option<(&str, &str)> {
    let mut words = instr.split_whitespace();
    if words.next()? != "CITATION" {
        return None;
    }
    let tag = words.next()?;
    let pages = instr.split("\\p").nth(1).map(str::trim).unwrap_or("");
    Some((tag, pages))
}

/// Tags cited in the body, in order of first citation.
fn cited_tags(doc: &Document) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for path in doc.para_paths(StoryRef::Body) {
        let Some(p) = doc.para(StoryRef::Body, &path) else { continue };
        for o in &p.objects {
            if let InlineObject::Field { instr, .. } = o
                && let Some((tag, _)) = citation_field(instr)
                && seen.insert(tag.to_string())
            {
                out.push(tag.to_string());
            }
        }
    }
    out
}

fn author_key(src: &Source) -> String {
    authors(&src.author).first().map(|a| a.0.to_lowercase()).unwrap_or_else(|| src.title.to_lowercase())
}

/// Indices into `doc.sources` in bibliography order: the cited sources (every source while
/// nothing is cited), sorted or in order of first citation as the style asks.
fn bib_order(doc: &Document, st: &CiteStyle) -> Vec<usize> {
    let cited = cited_tags(doc);
    let mut idx: Vec<usize> = if cited.is_empty() {
        (0..doc.sources.len()).filter(|i| !doc.sources.get(*i).is_some_and(is_placeholder)).collect()
    } else if st.order == Order::Cited {
        cited.iter().filter_map(|t| doc.sources.iter().position(|x| &x.tag == t && !is_placeholder(x))).collect()
    } else {
        let cited: std::collections::HashSet<&str> = cited.iter().map(String::as_str).collect();
        doc.sources.iter().enumerate().filter(|(_, x)| cited.contains(x.tag.as_str()) && !is_placeholder(x)).map(|(i, _)| i).collect()
    };
    let key =
        |i: &usize, by_title: bool| doc.sources.get(*i).map(|x| if by_title { x.title.to_lowercase() } else { author_key(x) }).unwrap_or_default();
    match st.order {
        Order::Author => idx.sort_by_cached_key(|i| key(i, false)),
        Order::Title => idx.sort_by_cached_key(|i| key(i, true)),
        Order::Cited => {}
    }
    idx
}

/// A source's number in a numbered style: its place in the bibliography order.
fn number_of(doc: &Document, order: &[usize], tag: &str) -> usize {
    order.iter().position(|&i| doc.sources.get(i).is_some_and(|x| x.tag == tag)).unwrap_or(order.len()).saturating_add(1)
}

/// Refresh CITATION results and generated lists.
pub fn update_citations(s: &mut Session) -> Result<(), CmdError> {
    refresh_citations(s)?;
    update_generated(s)
}

/// Refresh the results of the CITATION fields in the body.
fn refresh_citations(s: &mut Session) -> Result<(), CmdError> {
    let st = style(&s.bib_style);
    let order = bib_order(&s.doc, st);
    for path in s.doc.para_paths(StoryRef::Body) {
        let Some(p) = s.doc.para(StoryRef::Body, &path) else { continue };
        let updates: Vec<(usize, String)> = p
            .objects
            .iter()
            .enumerate()
            .filter_map(|(k, o)| match o {
                InlineObject::Field { instr, .. } => {
                    let (tag, pages) = citation_field(instr)?;
                    let src = s.doc.sources.iter().find(|x| x.tag == tag)?;
                    Some((k, cite(src, st.name, number_of(&s.doc, &order, tag), pages)))
                }
                _ => None,
            })
            .collect();
        if updates.is_empty() {
            continue;
        }
        let para = s.doc.para_mut(StoryRef::Body, &path)?;
        for (k, r) in updates {
            if let Some(InlineObject::Field { result, .. }) = para.objects.get_mut(k) {
                *result = r;
            }
        }
        para.touch();
    }
    Ok(())
}

/// `references.markTocEntry`: `TC "entry" \f C \l "1"` (as Word writes it), hidden, after the
/// selection, which stays as it is.
fn mark_toc_entry(s: &mut Session, v: &Value) -> CmdResult {
    let raw = p::str(v, "entry").map(str::to_string).unwrap_or_else(|| s.selected_text());
    let entry: String = raw.chars().map(|c| if c.is_control() { ' ' } else { c }).filter(|c| *c != '"').take(255).collect();
    let entry = entry.split_whitespace().collect::<Vec<_>>().join(" ");
    if entry.is_empty() {
        return Err(CmdError::Params("select the entry's text or give `entry`".into()));
    }
    let id = p::str(v, "identifier").unwrap_or("C").trim().to_ascii_uppercase();
    if id.len() != 1 || !id.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(CmdError::Params("`identifier` is one letter, A to Z".into()));
    }
    let level = match v.get("level") {
        None | Some(Value::Null) => 1,
        Some(l) => l.as_u64().filter(|l| (1..=9).contains(l)).ok_or_else(|| CmdError::Params("`level` is 1 to 9".into()))?,
    };
    let (_, b) = s.sel.ordered();
    // A backslash is escaped, so one ending the entry can't swallow the closing quote.
    let instr = format!("TC \"{}\" \\f {id} \\l \"{level}\"", entry.replace('\\', "\\\\"));
    s.doc.insert_object(
        &b,
        InlineObject::Field { instr, result: String::new(), locked: false, code: false },
        &CharProps { hidden: Some(true), ..Default::default() },
    )?;
    Ok(json!({"entry": entry, "identifier": id, "level": level}))
}

/// Generated lists (BIBLIOGRAPHY, INDEX, table of figures, TOA): a heading paragraph holding
/// the field, then entry paragraphs marked with the "GeneratedEntry" style family.
pub(crate) fn generated_list(s: &mut Session, instr: &str, title: &str) -> Result<(), CmdError> {
    generated_lists(s, &[(instr.to_string(), title.to_string())], None)
}

/// Several lists one after another at the caret (Table of Authorities › All: one per category),
/// their entries' tab leader `leader` (dots when `None`).
fn generated_lists(s: &mut Session, lists: &[(String, String)], leader: Option<TabLeader>) -> Result<(), CmdError> {
    let at = delete_selection(s)?;
    if at.story != StoryRef::Body || at.path.depth() > 0 {
        return Err(CmdError::Failed("lists go in the main text".into()));
    }
    let i = if s.doc.para_at(&at).is_some_and(|p| p.is_empty()) { at.path.last() } else { s.doc.split_paragraph(&at)?.path.last() };
    for (k, (instr, title)) in lists.iter().enumerate() {
        let mut head = if title.is_empty() { Paragraph::new() } else { Paragraph::with_text(title, CharProps::default()).styled("Heading1") };
        let end = head.len();
        head.insert_object(
            end,
            InlineObject::Field { instr: instr.clone(), result: String::new(), locked: false, code: false },
            &CharProps::default(),
        )?;
        s.doc.insert_block(StoryRef::Body, &Path::top(i + k), Block::Para(head))?;
    }
    rebuild_generated(s, leader)
}

/// The generated list a paragraph's field makes: BIBLIOGRAPHY, INDEX, TOA, or a TOC with `\c`
/// (a table of figures; a TOC without it is the table of contents).
fn list_kind(p: &Paragraph) -> Option<String> {
    p.objects.iter().find_map(|o| match o {
        InlineObject::Field { instr, .. } => {
            let t = instr.trim_start();
            let f = super::references::FieldCode::parse(t);
            (matches!(f.name.as_str(), "BIBLIOGRAPHY" | "INDEX" | "TOA") || (f.name == "TOC" && f.has('c'))).then(|| t.to_string())
        }
        _ => None,
    })
}

const ENTRY_STYLE: &str = "ListEntry";

/// Rebuild every generated list.
pub fn update_generated(s: &mut Session) -> Result<(), CmdError> {
    rebuild_generated(s, None)
}

/// Rebuild every generated list; each keeps its entries' tab leader, and a list without entries
/// yet takes `leader` (dots when `None`).
fn rebuild_generated(s: &mut Session, leader: Option<TabLeader>) -> Result<(), CmdError> {
    if s.doc.styles.get(ENTRY_STYLE).is_none() {
        s.doc.styles.upsert(doc4me_doc::Style {
            id: ENTRY_STYLE.into(),
            name: "List Entry".into(),
            kind: doc4me_doc::StyleKind::Paragraph,
            based_on: Some("Normal".into()),
            para: doc4me_doc::ParaProps { space_after: Some(4.0), ..Default::default() },
            hidden: true,
            ..Default::default()
        });
    }
    let mut i = 0;
    while i < s.doc.body.len() {
        let kind = s.doc.body.get(i).and_then(|b| b.as_para()).and_then(list_kind);
        let Some(kind) = kind else {
            i += 1;
            continue;
        };
        if kind.starts_with("INDEX") {
            super::index::rebuild(s, i, None)?;
            i += 1;
            continue;
        }
        // Remove old entries, keeping the leader they had.
        let mut kept = None;
        while let Some(p) = s.doc.body.get(i + 1).and_then(|b| b.as_para()).filter(|p| p.props.style.as_deref() == Some(ENTRY_STYLE)) {
            kept = kept.or_else(|| p.props.tabs.as_ref().and_then(|t| t.last()).map(|t| t.leader));
            s.doc.remove_block(StoryRef::Body, &Path::top(i + 1))?;
        }
        let leader = kept.or(leader).unwrap_or(TabLeader::Dot);
        let lines = entries_for(s, &kind);
        let width = super::page::sect(s).text_width();
        for (k, (text, hanging)) in lines.into_iter().enumerate() {
            let mut para = Paragraph::with_text(&text, CharProps::default()).styled(ENTRY_STYLE);
            if hanging {
                para.props.indent_left = Some(36.0);
                para.props.indent_first = Some(-36.0);
            }
            if text.contains('\t') {
                para.props.tabs = Some(vec![TabStop { pos: width - 0.5, align: TabAlign::Right, leader }]);
            }
            s.doc.insert_block(StoryRef::Body, &Path::top(i + 1 + k), Block::Para(para))?;
        }
        i += 1;
    }
    s.touch();
    Ok(())
}

fn page_of(s: &mut Session, pos: &Pos) -> u32 {
    let l = s.layout();
    l.caret(pos).and_then(|c| l.pages.get(c.page)).map(|p| p.number).unwrap_or(1)
}

/// Entry lines (text, hanging indent) for a generated list.
fn entries_for(s: &mut Session, kind: &str) -> Vec<(String, bool)> {
    if kind.starts_with("BIBLIOGRAPHY") {
        let st = style(&s.bib_style);
        return bib_order(&s.doc, st)
            .into_iter()
            .enumerate()
            .filter_map(|(k, i)| s.doc.sources.get(i).map(|x| (entry(x, st.name, k.saturating_add(1)), !st.numbered())))
            .collect();
    }
    let f = super::references::FieldCode::parse(kind);
    if f.name == "TOC" {
        return figure_entries(s, &f);
    }
    if f.name == "TOA" {
        return authority_entries(s, &f);
    }
    // Index: group by entry, sort, list pages.
    let mut marks: Vec<(String, Pos)> = Vec::new();
    for path in s.doc.para_paths(StoryRef::Body) {
        let Some(p) = s.doc.para(StoryRef::Body, &path) else { continue };
        for off in p.object_offsets() {
            if let Some(InlineObject::Field { instr, .. }) = p.object_at(off)
                && instr.trim_start().starts_with("XE")
            {
                marks.push((instr.split('"').nth(1).unwrap_or("").to_string(), Pos { story: StoryRef::Body, path: path.clone(), off }));
            }
        }
    }
    let mut grouped: std::collections::BTreeMap<String, Vec<u32>> = Default::default();
    for (e, pos) in marks {
        let pg = page_of(s, &pos);
        let pages = grouped.entry(e).or_default();
        if !pages.contains(&pg) {
            pages.push(pg);
        }
    }
    let mut out = Vec::new();
    let mut letter = None;
    for (e, pages) in grouped {
        let first = e.chars().next().map(|c| c.to_ascii_uppercase());
        if first != letter {
            letter = first;
            if let Some(l) = first {
                out.push((l.to_string(), false));
            }
        }
        let list = pages.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
        out.push((format!("{e}, {list}"), true));
    }
    out
}

/// The line for an entry on `page`: after a tab (right-aligned with the leader), after the `\p`
/// separator, or without a page number (`\n`).
fn with_page(f: &super::references::FieldCode, text: &str, page: &str) -> String {
    if f.has('n') {
        text.to_string()
    } else if let Some(sep) = f.arg('p') {
        format!("{text}{sep}{page}")
    } else {
        format!("{text}\t{page}")
    }
}

/// "Figure 1: A kiln" without its label and number (`\a`): "A kiln".
fn without_label(text: &str, label: &str) -> String {
    let rest = text.strip_prefix(label).unwrap_or(text).trim_start();
    let rest = rest.trim_start_matches(|c: char| c.is_ascii_digit() || matches!(c, '.' | '-' | '\u{2013}' | '\u{2014}'));
    let rest = rest.trim_start().trim_start_matches([':', '.', '\u{2013}', '\u{2014}', '-']).trim_start();
    if rest.is_empty() { text.to_string() } else { rest.to_string() }
}

/// A table of figures (TOC with `\c`): captions with the `\c` label, paragraphs of the `\t`
/// style, and TC fields with the `\f` identifier, in document order.
fn figure_entries(s: &mut Session, f: &super::references::FieldCode) -> Vec<(String, bool)> {
    let label = f.arg('c').map(str::to_string).filter(|l| !l.is_empty());
    let style_name = f.arg('t').and_then(|t| t.split([',', ';']).next()).map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
    let tc = f.has('f').then(|| f.arg('f').unwrap_or("").to_ascii_uppercase());
    let mut found: Vec<(String, Pos)> = Vec::new();
    for path in s.doc.para_paths(StoryRef::Body) {
        let Some(p) = s.doc.para(StoryRef::Body, &path) else { continue };
        let st = p.props.style.as_deref().unwrap_or("Normal");
        let text = p.plain_text().replace(['\u{000C}', '\n', '\t'], " ").trim().to_string();
        let by_label = label.as_deref().filter(|l| st == "Caption" && text.starts_with(l));
        let by_style = style_name.as_deref().is_some_and(|n| st == n || s.doc.styles.get(st).is_some_and(|x| x.name.eq_ignore_ascii_case(n)));
        if !text.is_empty() && (by_label.is_some() || by_style) {
            let shown = match by_label {
                Some(l) if f.has('a') => without_label(&text, l),
                _ => text,
            };
            found.push((shown, Pos { story: StoryRef::Body, path: path.clone(), off: 0 }));
        }
        if let Some(id) = &tc {
            for off in p.object_offsets() {
                let Some(InlineObject::Field { instr, .. }) = p.object_at(off) else { continue };
                let c = super::references::FieldCode::parse(instr);
                let kind = c.arg('f').unwrap_or("C").to_ascii_uppercase();
                let text = c.args.first().map(|t| t.trim().to_string()).unwrap_or_default();
                if c.name == "TC" && *id == kind && !text.is_empty() {
                    found.push((text, Pos { story: StoryRef::Body, path: path.clone(), off }));
                }
            }
        }
    }
    found.into_iter().map(|(t, pos)| (with_page(f, &t, &page_of(s, &pos).to_string()), false)).collect()
}

/// Word's Table of Authorities categories.
pub const TOA_CATEGORIES: [&str; 16] = [
    "Cases",
    "Statutes",
    "Other Authorities",
    "Rules",
    "Treatises",
    "Regulations",
    "Constitutional Provisions",
    "8",
    "9",
    "10",
    "11",
    "12",
    "13",
    "14",
    "15",
    "16",
];

/// Category `n`'s name (1–16): the user's, or Word's.
pub fn toa_category_name(s: &Session, n: u32) -> String {
    let i = n.clamp(1, 16) as usize - 1;
    s.prefs.toa_categories.get(i).map(|x| x.trim()).filter(|x| !x.is_empty()).unwrap_or(TOA_CATEGORIES[i]).to_string()
}

/// The citations a TA field marks: long citation, category (1–16; Cases when missing).
fn ta_of(instr: &str) -> Option<(String, u32)> {
    let f = super::references::FieldCode::parse(instr);
    if f.name != "TA" {
        return None;
    }
    let long = f.arg('l').or(f.arg('s')).map(str::trim).filter(|t| !t.is_empty())?.to_string();
    Some((long, f.arg('c').and_then(|c| c.trim().parse::<u32>().ok()).unwrap_or(1).clamp(1, 16)))
}

/// A table of authorities: the TA citations of the `\c` category (all without one), each with
/// its pages, alphabetically; `\p`: five or more pages read "passim".
fn authority_entries(s: &mut Session, f: &super::references::FieldCode) -> Vec<(String, bool)> {
    let want = f.arg('c').and_then(|c| c.trim().parse::<u32>().ok());
    let mut marks: Vec<(String, Pos)> = Vec::new();
    for path in s.doc.para_paths(StoryRef::Body) {
        let Some(p) = s.doc.para(StoryRef::Body, &path) else { continue };
        for off in p.object_offsets() {
            if let Some(InlineObject::Field { instr, .. }) = p.object_at(off)
                && let Some((long, cat)) = ta_of(instr)
                && want.is_none_or(|w| w == cat)
            {
                marks.push((long, Pos { story: StoryRef::Body, path: path.clone(), off }));
            }
        }
    }
    let mut grouped: std::collections::BTreeMap<String, (String, Vec<u32>)> = Default::default();
    for (long, pos) in marks {
        let pg = page_of(s, &pos);
        let (_, pages) = grouped.entry(long.to_lowercase()).or_insert_with(|| (long.clone(), Vec::new()));
        if !pages.contains(&pg) {
            pages.push(pg);
        }
    }
    grouped
        .into_values()
        .map(|(long, mut pages)| {
            pages.sort_unstable();
            let list =
                if f.has('p') && pages.len() >= 5 { "passim".to_string() } else { pages.iter().map(u32::to_string).collect::<Vec<_>>().join(", ") };
            // TOA's `\p` is passim, not a separator: the pages always follow a tab.
            (format!("{long}\t{list}"), false)
        })
        .collect()
}

/// Insert Table of Figures with its options.
fn table_of_figures(s: &mut Session, v: &Value) -> CmdResult {
    let text = |k: &str| p::str(v, k).map(str::trim).filter(|t| !t.is_empty());
    let clean = |t: &str, what: &str| -> Result<String, CmdError> {
        if t.chars().count() > 255 || t.contains(['"', '\\', '\n', '\r']) {
            return Err(CmdError::Params(format!("{what} can't have quotes, backslashes or line breaks")));
        }
        Ok(t.to_string())
    };
    let style = text("style").map(|t| clean(t, "the style")).transpose()?;
    if let Some(st) = &style
        && s.doc.styles.find(st).is_none()
        && s.doc.styles.get(st).is_none()
    {
        return Err(CmdError::Params(format!("no style `{st}`")));
    }
    let id = text("identifier").map(str::to_ascii_uppercase);
    if id.as_deref().is_some_and(|i| i.len() != 1 || !i.chars().all(|c| c.is_ascii_uppercase())) {
        return Err(CmdError::Params("the identifier is one letter, A–Z".into()));
    }
    let mut instr = String::from("TOC");
    if style.is_none() && id.is_none() {
        instr += &format!(" \\c \"{}\"", clean(text("label").unwrap_or("Figure"), "the label")?);
        if p::bool(v, "includeLabel") == Some(false) {
            instr += " \\a";
        }
    } else {
        if let Some(st) = &style {
            let name = s.doc.styles.find(st).or(s.doc.styles.get(st)).map(|x| x.name.clone()).unwrap_or_else(|| st.clone());
            instr += &format!(" \\t \"{name},1\"");
        }
        if let Some(i) = &id {
            instr += &format!(" \\f {i}");
        }
        instr += " \\c";
    }
    if p::bool(v, "pages") == Some(false) {
        instr += " \\n";
    } else if p::bool(v, "rightAlign") == Some(false) {
        instr += " \\p \" \"";
    }
    let leader = p::str(v, "leader").map(super::references::leader);
    generated_lists(s, &[(instr, String::new())], leader)?;
    sel_result(s)
}

/// Mark Citation: a hidden TA field after the selection, with its long and short citation and
/// category.
fn mark_citation(s: &mut Session, v: &Value) -> CmdResult {
    let entry = p::str(v, "entry").map(str::to_string).unwrap_or_else(|| s.selected_text().trim().to_string());
    let entry = entry.replace(['"', '\\', '\n', '\r'], " ").trim().to_string();
    if entry.is_empty() {
        return Err(CmdError::Params("select a citation or give `entry`".into()));
    }
    let short = p::str(v, "short")
        .map(|t| t.replace(['"', '\\', '\n', '\r'], " ").trim().to_string())
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| entry.clone());
    let category = p::u64(v, "category").unwrap_or(1);
    if !(1..=16).contains(&category) {
        return Err(CmdError::Params("the category is 1–16".into()));
    }
    let (_, b) = s.sel.ordered();
    let instr = format!("TA \\l \"{entry}\" \\s \"{short}\" \\c {category}");
    s.doc.insert_object(
        &b,
        InlineObject::Field { instr, result: String::new(), locked: false, code: false },
        &CharProps { hidden: Some(true), ..Default::default() },
    )?;
    Ok(json!({"entry": entry, "short": short, "category": category}))
}

/// Edit Category: rename category 1–16 (an empty name is Word's again); the names either way.
fn toa_category(s: &mut Session, v: &Value) -> CmdResult {
    if let Some(n) = p::u64(v, "category") {
        if !(1..=16).contains(&n) {
            return Err(CmdError::Params("the category is 1–16".into()));
        }
        let name = p::str(v, "name").unwrap_or("").trim().to_string();
        if name.chars().count() > 40 || name.contains(['\n', '\r', '"', '\\']) {
            return Err(CmdError::Params("a category name has up to 40 characters, on one line, without quotes".into()));
        }
        let c = &mut s.prefs.toa_categories;
        c.resize(16, String::new());
        c[n as usize - 1] = if name == TOA_CATEGORIES[n as usize - 1] { String::new() } else { name };
        if c.iter().all(String::is_empty) {
            c.clear();
        }
    }
    Ok(json!({"categories": (1..=16).map(|n| toa_category_name(s, n)).collect::<Vec<_>>()}))
}

/// Insert Table of Authorities: one category, or All (a table for each category in use, under
/// its name).
fn table_of_authorities(s: &mut Session, v: &Value) -> CmdResult {
    let category = p::u64(v, "category").unwrap_or(0);
    if category > 16 {
        return Err(CmdError::Params("the category is 0 (All) or 1–16".into()));
    }
    let passim = if p::bool(v, "passim").unwrap_or(true) { " \\p" } else { "" };
    let cats: Vec<u32> = if category == 0 {
        let mut used: Vec<u32> = s
            .doc
            .para_paths(StoryRef::Body)
            .iter()
            .filter_map(|path| s.doc.para(StoryRef::Body, path))
            .flat_map(|p| p.objects.iter())
            .filter_map(|o| match o {
                InlineObject::Field { instr, .. } => ta_of(instr).map(|(_, c)| c),
                _ => None,
            })
            .collect();
        used.sort_unstable();
        used.dedup();
        if used.is_empty() { vec![1] } else { used }
    } else {
        vec![category as u32]
    };
    // A heading per category: the user's name, else the caller's (Word's names in the interface
    // language), else Word's English one.
    let given: Vec<&str> = v.get("names").and_then(Value::as_array).map(|a| a.iter().map(|n| n.as_str().unwrap_or("")).collect()).unwrap_or_default();
    let heading = |c: u32| {
        let own = s.prefs.toa_categories.get(c as usize - 1).map(|x| x.trim()).filter(|x| !x.is_empty());
        let local = given.get(c as usize - 1).map(|x| x.trim()).filter(|x| !x.is_empty() && x.chars().count() <= 40 && !x.contains(['\n', '\r']));
        own.or(local).map(str::to_string).unwrap_or_else(|| toa_category_name(s, c))
    };
    let lists: Vec<(String, String)> = cats.iter().map(|&c| (format!("TOA \\h \\c \"{c}\"{passim}"), heading(c))).collect();
    let leader = p::str(v, "leader").map(super::references::leader);
    generated_lists(s, &lists, leader)?;
    sel_result(s)
}

fn cross_ref(s: &mut Session, v: &Value) -> CmdResult {
    let to = p::str(v, "to").unwrap_or("heading");
    let targets = ref_targets(s, to);
    let Some(target) = p::str(v, "target") else {
        return Ok(Value::Array(targets.iter().map(|(t, _)| json!(t)).collect()));
    };
    let find = |ts: Vec<(String, Pos)>| ts.into_iter().find(|(t, _)| t == target || t.starts_with(target));
    let (text, pos) = find(targets).ok_or_else(|| CmdError::Params(format!("no {to} `{target}`")))?;
    let show = p::str(v, "show").unwrap_or("text");
    let page = page_of(s, &pos);
    let result = match show {
        "page" => page.to_string(),
        "number" => text.split(':').next().unwrap_or(&text).trim().to_string(),
        "aboveBelow" => {
            if pos < s.sel.focus {
                "above".into()
            } else {
                "below".into()
            }
        }
        _ if to == "bookmark" => super::references::bookmark_text(&s.doc, &text).unwrap_or_default(),
        _ => text.clone(),
    };
    let props = s.typing_props();
    let at = delete_selection(s)?;
    // The field points at a bookmark: the named one, or a hidden `_Ref` bookmark on the target
    // paragraph (as Word does), so Update Field and Word can resolve it.
    let (name, at) = if to == "bookmark" {
        (text, at)
    } else {
        // Deleting the selection can move paragraphs: look the target up again.
        let (_, pos) = find(ref_targets(s, to)).ok_or_else(|| CmdError::Params(format!("no {to} `{target}`")))?;
        ref_bookmark(s, &pos, show == "number", at)?
    };
    let instr = format!("{} {name} \\h", if show == "page" { "PAGEREF" } else { "REF" });
    let end = s.doc.insert_object(&at, InlineObject::Field { instr, result: result.clone(), locked: false, code: false }, &props)?;
    s.sel = Selection::caret(end);
    Ok(json!({"inserted": result}))
}

/// Cross-reference targets of a kind: (shown text or bookmark name, position).
fn ref_targets(s: &Session, to: &str) -> Vec<(String, Pos)> {
    let mut targets: Vec<(String, Pos)> = Vec::new();
    match to {
        "bookmark" => targets = s.doc.bookmarks(),
        "figure" | "table" | "equation" => {
            let label = match to {
                "table" => "Table",
                "equation" => "Equation",
                _ => "Figure",
            };
            for path in s.doc.para_paths(StoryRef::Body) {
                if let Some(p) = s.doc.para(StoryRef::Body, &path)
                    && p.props.style.as_deref() == Some("Caption")
                    && p.plain_text().starts_with(label)
                {
                    targets.push((p.plain_text(), Pos { story: StoryRef::Body, path: path.clone(), off: 0 }));
                }
            }
        }
        _ => {
            for path in s.doc.para_paths(StoryRef::Body) {
                if let Some(p) = s.doc.para(StoryRef::Body, &path)
                    && s.doc.styles.resolve_para(&p.props).outline_level.is_some()
                    && !p.plain_text().trim().is_empty()
                {
                    targets.push((p.plain_text().trim().to_string(), Pos { story: StoryRef::Body, path: path.clone(), off: 0 }));
                }
            }
        }
    }
    targets
}

/// The hidden `_Ref` bookmark around the target paragraph at `pos` (its label and number only
/// when `number_only`), reusing one already there. Returns its name and `at` moved past the
/// bookmark marks inserted before it.
fn ref_bookmark(s: &mut Session, pos: &Pos, number_only: bool, mut at: Pos) -> Result<(String, Pos), CmdError> {
    const MARK: usize = doc4me_doc::para::OBJ.len_utf8();
    let para = s.doc.para(pos.story, &pos.path).ok_or_else(|| CmdError::Params("cross-reference target is gone".into()))?;
    let end = if number_only { para.text.find(':').unwrap_or(para.len()) } else { para.len() };
    if let Some(InlineObject::BookmarkStart { name }) = para.object_at(0)
        && name.starts_with("_Ref")
    {
        // Reuse it when it covers the same extent: up to the colon, or the whole paragraph.
        let same = para.object_offsets().into_iter().any(|o| {
            matches!(para.object_at(o), Some(InlineObject::BookmarkEnd { name: n }) if n == name)
                && para.text.get(o.saturating_add(MARK)..).is_some_and(|rest| if number_only { rest.starts_with(':') } else { rest.is_empty() })
        });
        if same {
            return Ok((name.clone(), at));
        }
    }
    let next =
        s.doc.bookmarks().iter().filter_map(|(n, _)| n.strip_prefix("_Ref").and_then(|d| d.parse::<u64>().ok())).max().unwrap_or(0).saturating_add(1);
    let name = format!("_Ref{next:09}");
    let props = CharProps::default();
    s.doc.insert_object(&Pos { off: end, ..pos.clone() }, InlineObject::BookmarkEnd { name: name.clone() }, &props)?;
    s.doc.insert_object(&Pos { off: 0, ..pos.clone() }, InlineObject::BookmarkStart { name: name.clone() }, &props)?;
    if at.story == pos.story && at.path == pos.path {
        if at.off >= end {
            at.off = at.off.saturating_add(2 * MARK);
        } else if at.off > 0 {
            at.off = at.off.saturating_add(MARK);
        }
    }
    Ok((name, at))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    #[test]
    fn citations_and_bibliography() {
        let mut s = Session::new(doc4me_doc::Document::new());
        s.run("text.insert", &json!({"text": "Studios matter "})).unwrap();
        s.run("references.citation", &json!({"source": {"author": "Rivera, Alex; Chen, Mei", "title": "Shared Spaces", "year": "2021", "publisher": "Harbor Books", "city": "Portland"}})).unwrap();
        assert!(s.doc.plain_text(StoryRef::Body).contains("(Rivera & Chen, 2021)"), "{}", s.doc.plain_text(StoryRef::Body));
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.bibliography", &json!({})).unwrap();
        let t = s.doc.plain_text(StoryRef::Body);
        assert!(t.contains("Rivera, A., & Chen, M. (2021). Shared Spaces. Harbor Books."), "{t}");
        s.run("references.citationStyle", &json!({"style": "MLA"})).unwrap();
        let t = s.doc.plain_text(StoryRef::Body);
        assert!(t.contains("(Rivera and Chen)"), "{t}");
        s.run("references.citationStyle", &json!({"style": "IEEE"})).unwrap();
        assert!(s.doc.plain_text(StoryRef::Body).contains("[1]"));
    }

    #[test]
    fn bibliography_round_trip_docx() {
        let mut s = Session::new(doc4me_doc::Document::new());
        s.run("text.insert", &json!({"text": "Studios matter "})).unwrap();
        s.run(
            "references.citation",
            &json!({
                "source": {
                    "author": "Rivera, Alex; Chen, Mei",
                    "title": "Shared Spaces",
                    "year": "2021",
                    "publisher": "Harbor Books",
                    "city": "Portland",
                    "editor": "Johnson, Robert",
                    "edition": "2nd",
                    "issue": "4",
                    "month": "10",
                    "day": "15",
                    "accessed": "2024-05-15",
                    "doi": "10.1000/182"
                }
            }),
        )
        .unwrap();
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.bibliography", &json!({})).unwrap();

        // Serialize to DOCX and back — sources must survive.
        let bytes = doc4me_docx::write(&s.doc).expect("docx write");
        let doc = doc4me_docx::read(&bytes).expect("docx read");
        assert_eq!(doc.sources.len(), 1, "round-tripped sources: {:?}", doc.sources);
        assert_eq!(doc.sources[0].author, "Rivera, Alex; Chen, Mei");
        assert_eq!(doc.sources[0].title, "Shared Spaces");
        assert_eq!(doc.sources[0].editor, "Johnson, Robert");
        assert_eq!(doc.sources[0].edition, "2nd");
        assert_eq!(doc.sources[0].issue, "4");
        assert_eq!(doc.sources[0].month, "10");
        assert_eq!(doc.sources[0].day, "15");
        assert_eq!(doc.sources[0].accessed, "2024-05-15");
        assert_eq!(doc.sources[0].doi, "10.1000/182");

        // Round-tripped document in a new session works with citation styles.
        let mut s2 = Session::new(doc);
        s2.run("references.citationStyle", &json!({"style": "Harvard (Anglia)"})).unwrap();
        let t = s2.doc.plain_text(StoryRef::Body);
        assert!(t.contains("(Rivera and Chen, 2021)"), "{t}");
        assert!(t.contains("Rivera, A. and Chen, M., 2021. Shared Spaces. Portland: Harbor Books."), "{t}");
    }

    #[test]
    fn citation_style_survives_docx_save_and_reopen() {
        let mut s = Session::new(doc4me_doc::Document::new());
        s.run("references.citation", &json!({"source": {"author": "Rivera, Alex", "title": "Shared Spaces", "year": "2021"}})).unwrap();
        s.run("references.citationStyle", &json!({"style": "Harvard (Anglia)"})).unwrap();
        let bytes = doc4me_docx::write(&s.doc).expect("docx write");
        let doc = doc4me_docx::read(&bytes).expect("docx read");
        assert_eq!(doc.settings.bib_style.as_deref(), Some("Harvard (Anglia)"));
        assert_eq!(Session::new(doc.clone()).bib_style, "Harvard (Anglia)");
        let mut other = Session::new(doc4me_doc::Document::new());
        other.set_document(doc);
        assert_eq!(other.bib_style, "Harvard (Anglia)");
        // A document without a recorded style is APA.
        assert_eq!(Session::new(doc4me_doc::Document::new()).bib_style, "APA");
    }

    #[test]
    fn index_and_figures() {
        let mut s = Session::new(doc4me_doc::Document::from_text("Kilns are hot.\nPresses are heavy.\nMore kilns."));
        s.run("select.text", &json!({"text": "Kilns"})).unwrap();
        s.run("references.markEntry", &json!({})).unwrap();
        s.run("select.text", &json!({"text": "Presses"})).unwrap();
        s.run("references.markEntry", &json!({})).unwrap();
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.index", &json!({})).unwrap();
        let t = s.doc.plain_text(StoryRef::Body);
        assert!(t.contains("Kilns, 1") && t.contains("Presses, 1"), "{t}");
        s.run("caret.docStart", &json!({})).unwrap();
        s.run("references.caption", &json!({"label": "Figure", "text": "The kiln"})).unwrap();
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("references.tableOfFigures", &json!({})).unwrap();
        assert!(s.doc.plain_text(StoryRef::Body).contains("Figure 1: The kiln\t1"));
        let list = s.run("insert.crossReference", &json!({"to": "figure"})).unwrap();
        assert_eq!(list.as_array().unwrap().len(), 1);
        s.run("insert.crossReference", &json!({"to": "figure", "target": "Figure 1", "show": "number"})).unwrap();
        assert!(s.doc.plain_text(StoryRef::Body).contains("Figure 1"));
        // The number-only reference marks just "Figure 1" of the caption.
        let (name, _) = s.doc.bookmarks().into_iter().find(|(n, _)| n.starts_with("_Ref")).unwrap();
        assert_eq!(crate::cmd::references::bookmark_text(&s.doc, &name).as_deref(), Some("Figure 1"));
        assert!(fields(&s).contains(&(format!("REF {name} \\h"), "Figure 1".to_string())), "{:?}", fields(&s));
    }

    /// #661: References › Table of Authorities › Update Table picks up citations marked after the table.
    #[test]
    fn table_of_authorities_updates() {
        let mut s = Session::new(doc4me_doc::Document::from_text("Smith v. Jones held it.\nDoe v. Roe agreed."));
        s.run("select.text", &json!({"text": "Smith v. Jones"})).unwrap();
        s.run("references.markCitation", &json!({})).unwrap();
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.tableOfAuthorities", &json!({})).unwrap();
        let t = s.doc.plain_text(StoryRef::Body);
        assert!(t.contains("Smith v. Jones") && !t.contains("Doe v. Roe, "), "{t}");
        s.run("select.text", &json!({"text": "Doe v. Roe"})).unwrap();
        s.run("references.markCitation", &json!({})).unwrap();
        s.run("references.updateAuthorities", &json!({})).unwrap();
        let after = s.doc.plain_text(StoryRef::Body);
        assert!(after.matches("Doe v. Roe").count() > t.matches("Doe v. Roe").count(), "{after}");
    }

    /// dlg-165: Table of Figures › Options: from a style or from TC fields with an identifier,
    /// without the label and number, without page numbers or not right-aligned, with a leader.
    #[test]
    fn table_of_figures_options() {
        let mut s = Session::new(doc4me_doc::Document::from_text("A kiln\nQuote here\nBody"));
        s.run("caret.docStart", &json!({})).unwrap();
        s.run("references.caption", &json!({"label": "Figure", "text": "The kiln"})).unwrap();
        s.sel = Selection::caret(Pos::body(2, 0));
        s.run("styles.apply", &json!({"style": "Quote"})).unwrap();
        s.run("select.text", &json!({"text": "Body"})).unwrap();
        s.run("references.markTocEntry", &json!({"entry": "Marked entry", "identifier": "F"})).unwrap();
        let end = |s: &mut Session| {
            s.run("caret.docEnd", &json!({})).unwrap();
            s.run("text.newParagraph", &json!({})).unwrap();
        };
        let lines = |s: &Session| {
            s.doc
                .body
                .iter()
                .filter_map(|b| b.as_para())
                .filter(|p| p.props.style.as_deref() == Some(ENTRY_STYLE))
                .map(|p| p.plain_text())
                .collect::<Vec<_>>()
        };
        end(&mut s);
        s.run("references.tableOfFigures", &json!({"includeLabel": false, "leader": "hyphen"})).unwrap();
        assert_eq!(lines(&s), ["The kiln\t1"]);
        let tab =
            s.doc.body.iter().filter_map(|b| b.as_para()).find(|p| p.props.style.as_deref() == Some(ENTRY_STYLE)).and_then(|p| p.props.tabs.clone());
        assert_eq!(tab.unwrap()[0].leader, TabLeader::Hyphen);
        assert!(fields(&s).iter().any(|(i, _)| i == "TOC \\c \"Figure\" \\a"), "{:?}", fields(&s));
        s.run("edit.undo", &json!({})).unwrap();
        // Options › Style and Table entry fields together, page numbers not right-aligned.
        end(&mut s);
        s.run("references.tableOfFigures", &json!({"style": "Quote", "identifier": "f", "rightAlign": false})).unwrap();
        assert_eq!(lines(&s), ["Quote here 1", "Marked entry 1"]);
        // The leader and the entries stay through Update Table; a table of figures isn't the
        // table of contents.
        s.run("references.updateFigures", &json!({})).unwrap();
        assert_eq!(lines(&s), ["Quote here 1", "Marked entry 1"]);
        s.run("edit.undo", &json!({})).unwrap();
        s.run("edit.undo", &json!({})).unwrap();
        end(&mut s);
        s.run("references.tableOfFigures", &json!({"pages": false})).unwrap();
        assert_eq!(lines(&s), ["Figure 1: The kiln"]);
        for bad in [json!({"style": "No such"}), json!({"identifier": "AB"}), json!({"label": "Fig\"ure"})] {
            assert!(s.run("references.tableOfFigures", &bad).is_err(), "{bad}");
        }
    }

    /// dlg-029: Edit Category renames Table of Authorities categories; Mark Citation files a
    /// citation under one, and All makes a table per category in use, under its name.
    #[test]
    fn table_of_authorities_categories() {
        let mut s = Session::new(doc4me_doc::Document::from_text("Smith v. Jones held it.\nThe Clean Air Act applies.\nSmith v. Jones again."));
        let cats = s.run("references.toaCategory", &json!({})).unwrap();
        assert_eq!(cats["categories"][0], "Cases");
        assert_eq!(cats["categories"][15], "16");
        let cats = s.run("references.toaCategory", &json!({"category": 8, "name": "Treaties"})).unwrap();
        assert_eq!(cats["categories"][7], "Treaties");
        assert!(s.run("references.toaCategory", &json!({"category": 17, "name": "x"})).is_err());
        s.run("select.text", &json!({"text": "Smith v. Jones"})).unwrap();
        s.run("references.markCitation", &json!({"short": "Smith"})).unwrap();
        s.run("select.text", &json!({"text": "Clean Air Act"})).unwrap();
        s.run("references.markCitation", &json!({"category": 2})).unwrap();
        s.sel = Selection::caret(Pos::body(2, 0));
        s.run("select.text", &json!({"text": "Smith v. Jones"})).ok();
        s.run("references.markCitation", &json!({"entry": "Treaty of Rome", "category": 8})).unwrap();
        assert!(s.run("references.markCitation", &json!({"entry": "x", "category": 0})).is_err());
        assert!(fields(&s).iter().any(|(i, _)| i == "TA \\l \"Smith v. Jones\" \\s \"Smith\" \\c 1"), "{:?}", fields(&s));
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.tableOfAuthorities", &json!({})).unwrap();
        let t: Vec<String> = s.doc.body.iter().filter_map(|b| b.as_para()).map(|p| p.plain_text()).filter(|t| !t.is_empty()).collect();
        let at = |w: &str| t.iter().position(|x| x == w || x.starts_with(w)).unwrap_or_else(|| panic!("{w}: {t:?}"));
        assert!(
            at("Cases") < at("Smith v. Jones\t1") && at("Statutes") < at("Clean Air Act\t1") && at("Treaties") < at("Treaty of Rome\t1"),
            "{t:?}"
        );
        // One category only.
        s.run("edit.undo", &json!({})).unwrap();
        s.run("references.tableOfAuthorities", &json!({"category": 2})).unwrap();
        let t = s.doc.plain_text(StoryRef::Body);
        assert!(t.contains("Clean Air Act\t1") && !t.contains("Smith v. Jones\t1"), "{t}");
        assert!(s.run("references.tableOfAuthorities", &json!({"category": 17})).is_err());
    }

    /// Field instructions and results in the body, in order.
    fn fields(s: &Session) -> Vec<(String, String)> {
        let mut out = Vec::new();
        for path in s.doc.para_paths(StoryRef::Body) {
            for o in &s.doc.para(StoryRef::Body, &path).unwrap().objects {
                if let InlineObject::Field { instr, result, .. } = o {
                    out.push((instr.clone(), result.clone()));
                }
            }
        }
        out
    }

    #[test]
    fn cross_references_point_at_real_bookmarks() {
        let mut s = Session::new(doc4me_doc::Document::from_text("Kilns\nSee here."));
        s.run("caret.docStart", &json!({})).unwrap();
        s.run("styles.apply", &json!({"style": "Heading 1"})).unwrap();
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("insert.crossReference", &json!({"to": "heading", "target": "Kilns"})).unwrap();
        // The heading gets a hidden bookmark, and the REF field names it (not a literal `_Ref`).
        let marks = s.doc.bookmarks();
        assert_eq!(marks.len(), 1, "{marks:?}");
        let name = marks[0].0.clone();
        assert!(name.starts_with("_Ref") && name.len() > 4, "{name}");
        assert_eq!(marks[0].1.path, s.doc.para_paths(StoryRef::Body)[0]);
        assert_eq!(fields(&s), vec![(format!("REF {name} \\h"), "Kilns".to_string())]);
        // A second reference to the same heading reuses the bookmark.
        s.run("insert.crossReference", &json!({"to": "heading", "target": "Kilns", "show": "page"})).unwrap();
        assert_eq!(s.doc.bookmarks().len(), 1);
        assert_eq!(fields(&s)[1], (format!("PAGEREF {name} \\h"), "1".to_string()));
        // Update Field follows the heading's new text.
        s.run("select.text", &json!({"text": "ilns"})).unwrap();
        s.run("text.insert", &json!({"text": "ILNS"})).unwrap();
        s.run("references.updateFields", &json!({})).unwrap();
        assert_eq!(fields(&s)[0].1, "KILNS");
        assert!(s.doc.plain_text(StoryRef::Body).ends_with("See here.KILNS1"), "{}", s.doc.plain_text(StoryRef::Body));
    }

    #[test]
    fn cross_reference_to_a_bookmark_shows_its_text() {
        let mut s = Session::new(doc4me_doc::Document::from_text("Kilns are hot.\nSee "));
        s.run("select.text", &json!({"text": "Kilns"})).unwrap();
        s.run("insert.bookmark", &json!({"name": "Spot"})).unwrap();
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("insert.crossReference", &json!({"to": "bookmark", "target": "Spot"})).unwrap();
        assert_eq!(s.doc.bookmarks().len(), 1, "no extra bookmark for a named one");
        assert_eq!(fields(&s), vec![("REF Spot \\h".to_string(), "Kilns".to_string())]);
    }

    #[test]
    fn cross_reference_inside_its_own_target_paragraph() {
        let mut s = Session::new(doc4me_doc::Document::from_text("Kilns"));
        s.run("styles.apply", &json!({"style": "Heading 1"})).unwrap();
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("insert.crossReference", &json!({"to": "heading", "target": "Kilns"})).unwrap();
        assert_eq!(s.doc.plain_text(StoryRef::Body), "KilnsKilns");
        // The field sits after the bookmark, so it doesn't mark itself.
        let name = s.doc.bookmarks()[0].0.clone();
        assert_eq!(crate::cmd::references::bookmark_text(&s.doc, &name).as_deref(), Some("Kilns"));
    }

    fn two_sources(s: &mut Session, a_title: &str) {
        let a = json!({"tag": "A", "author": "Rivera, Alex; Chen, Mei", "title": a_title, "year": "2021", "publisher": "Harbor Books", "city": "Portland"});
        let b = json!({"tag": "B", "author": "Okafor, Ngozi", "title": "Kilns and Clay", "year": "2019", "journal": "Studio Quarterly", "volume": "4", "pages": "10-20"});
        s.run("references.sources", &json!({"add": a})).unwrap();
        s.run("references.sources", &json!({"add": b})).unwrap();
    }

    #[test]
    fn author_date_styles_cite_and_list() {
        let mut s = Session::new(doc4me_doc::Document::new());
        two_sources(&mut s, "Shared Spaces");
        s.run("references.citation", &json!({"tag": "A", "pages": "12"})).unwrap();
        s.run("references.citation", &json!({"tag": "B"})).unwrap();
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.bibliography", &json!({})).unwrap();
        let cases = [
            (
                "Harvard (Anglia)",
                "(Rivera and Chen, 2021, p. 12)(Okafor, 2019)",
                "Okafor, N., 2019. Kilns and Clay. Studio Quarterly, 4, pp. 10-20.\nRivera, A. and Chen, M., 2021. Shared Spaces. Portland: Harbor Books.",
            ),
            (
                "ISO 690 - First Element and Date",
                "(Rivera and Chen 2021, p. 12)(Okafor 2019)",
                "OKAFOR, Ngozi, 2019. Kilns and Clay. Studio Quarterly, 4, 10-20.\nRIVERA, Alex and CHEN, Mei, 2021. Shared Spaces. Portland: Harbor Books.",
            ),
            (
                "turabian",
                "(Rivera and Chen 2021, 12)(Okafor 2019)",
                "Okafor, Ngozi. “Kilns and Clay.” Studio Quarterly 4 (2019): 10-20.\nRivera, Alex, and Mei Chen. Shared Spaces. Portland: Harbor Books, 2021.",
            ),
        ];
        for (style, cites, list) in cases {
            s.run("references.citationStyle", &json!({"style": style})).unwrap();
            let t = s.doc.plain_text(StoryRef::Body);
            assert!(t.starts_with(cites) && t.contains(list), "{style}: {t}");
        }
        assert!(s.run("references.citationStyle", &json!({"style": "Vancouver"})).unwrap_err().to_string().contains("Turabian"));
        assert_eq!(s.bib_style, "Turabian", "an unknown style keeps the current one");
    }

    #[test]
    fn numbered_styles_count_in_order_of_first_citation() {
        let mut s = Session::new(doc4me_doc::Document::new());
        two_sources(&mut s, "Shared Spaces");
        s.run("references.citationStyle", &json!({"style": "ISO 690 (numerical)"})).unwrap();
        s.run("text.insert", &json!({"text": "Later "})).unwrap();
        s.run("references.citation", &json!({"tag": "A", "pages": "12"})).unwrap();
        assert_eq!(fields(&s)[0].1, "(1, p. 12)");
        // Citing B earlier in the text makes it number 1 and renumbers A.
        s.run("caret.docStart", &json!({})).unwrap();
        s.run("references.citation", &json!({"tag": "B"})).unwrap();
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.bibliography", &json!({})).unwrap();
        let cases = [
            (
                "ISO 690 (numerical)",
                "(1)Later (2, p. 12)",
                "1. OKAFOR, Ngozi. Kilns and Clay. Studio Quarterly. 2019, 4, 10-20.\n2. RIVERA, Alex and CHEN, Mei. Shared Spaces. Portland: Harbor Books, 2021.",
            ),
            (
                "GB/T 7714",
                "[1]Later [2, 12]",
                "[1] OKAFOR N. Kilns and Clay[J]. Studio Quarterly, 2019, 4: 10-20.\n[2] RIVERA A, CHEN M. Shared Spaces[M]. Portland: Harbor Books, 2021.",
            ),
            (
                "SIST02",
                "(1)Later (2, p. 12)",
                "1) Okafor, Ngozi. Kilns and Clay. Studio Quarterly. 2019, vol. 4, p. 10-20.\n2) Rivera, Alex; Chen, Mei. Shared Spaces. Portland, Harbor Books, 2021.",
            ),
            (
                "IEEE",
                "[1]Later [2]",
                "[1] N. Okafor, “Kilns and Clay,” Studio Quarterly, 4, 10-20, 2019.\n[2] A. Rivera, M. Chen, “Shared Spaces,” Portland: Harbor Books, 2021.",
            ),
        ];
        for (style, cites, list) in cases {
            s.run("references.citationStyle", &json!({"style": style})).unwrap();
            let t = s.doc.plain_text(StoryRef::Body);
            assert!(t.starts_with(cites) && t.contains(list), "{style}: {t}");
        }
    }

    #[test]
    fn gost_numbers_follow_its_name_or_title_sort() {
        let mut s = Session::new(doc4me_doc::Document::new());
        two_sources(&mut s, "Atlas of Studios");
        s.run("references.citation", &json!({"tag": "A"})).unwrap();
        s.run("references.citation", &json!({"tag": "B", "pages": "15"})).unwrap();
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.bibliography", &json!({})).unwrap();
        let okafor = "Okafor N. Kilns and Clay // Studio Quarterly. – 2019. – Vol. 4. – P. 10-20.";
        let rivera = "Rivera A., Chen M. Atlas of Studios. – Portland : Harbor Books, 2021.";
        // Word's style and style-sheet names pick the same styles.
        s.run("references.citationStyle", &json!({"style": "\\GostName.XSL"})).unwrap();
        assert_eq!(s.bib_style, "GOST (name sort)");
        let t = s.doc.plain_text(StoryRef::Body);
        assert!(t.starts_with("[2][1, p. 15]") && t.contains(&format!("1. {okafor}\n2. {rivera}")), "{t}");
        s.run("references.citationStyle", &json!({"style": "GOST - Title Sort"})).unwrap();
        let t = s.doc.plain_text(StoryRef::Body);
        assert!(t.starts_with("[1][2, p. 15]") && t.contains(&format!("1. {rivera}\n2. {okafor}")), "{t}");
        assert_eq!(find_style("ISO 690 - Numerical Reference").map(|x| x.name), Some("ISO 690 (numerical)"));
        assert_eq!(find_style("Harvard - Anglia 2008").map(|x| x.name), Some("Harvard (Anglia)"));
        assert!(find_style("").is_none() && find_style("Vancouver").is_none());
    }

    /// Mark Table of Contents Entry (Alt+Shift+O): a hidden TC field, as Word writes it, that a
    /// table of contents built from table entry fields lists at its level; bad input is refused.
    #[test]
    fn mark_toc_entry_feeds_the_table_of_contents() {
        let mut s = Session::new(Document::from_text("Kilns\nPresses"));
        s.run("select.text", &json!({"text": "Kilns"})).unwrap();
        assert_eq!(s.run("references.markTocEntry", &json!({"level": 2})).unwrap()["level"], 2);
        assert!(fields(&s).iter().any(|(i, _)| i == "TC \"Kilns\" \\f C \\l \"2\""), "{:?}", fields(&s));
        assert!(s.run("references.markTocEntry", &json!({"entry": "x", "identifier": "CC"})).is_err());
        assert!(s.run("references.markTocEntry", &json!({"entry": "x", "level": 10})).is_err());
        s.run("caret.docStart", &json!({})).unwrap();
        assert!(s.run("references.markTocEntry", &json!({})).is_err(), "no selection, no entry");
        // Quotes and line breaks can't end the field's text early.
        s.run("references.markTocEntry", &json!({"entry": "Glaze \"notes\"\nand tests"})).unwrap();
        assert!(fields(&s).iter().any(|(i, _)| i == "TC \"Glaze notes and tests\" \\f C \\l \"1\""), "{:?}", fields(&s));
        s.run("references.markTocEntry", &json!({"entry": "Kiln C:\\", "level": 2})).unwrap();
        let tc = fields(&s)
            .into_iter()
            .map(|(i, _)| crate::cmd::references::FieldCode::parse(&i))
            .find(|c| c.args.first().is_some_and(|a| a.starts_with("Kiln C")))
            .unwrap();
        assert_eq!((tc.args[0].as_str(), tc.arg('l')), ("Kiln C:\\", Some("2")), "a trailing backslash keeps the switches");
        s.run("caret.docEnd", &json!({})).unwrap();
        s.run("text.newParagraph", &json!({})).unwrap();
        s.run("references.toc", &json!({"headingStyles": false, "outlineLevels": false, "tcFields": true})).unwrap();
        let t = s.doc.plain_text(StoryRef::Body);
        assert!(t.contains("Kilns\t1") && t.contains("Glaze notes and tests\t1"), "{t}");
    }
}
