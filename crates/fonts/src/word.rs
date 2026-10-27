//! Word-processor font resolution: map a requested family + bold/italic to an available face,
//! with metric-compatible substitutes for common proprietary document fonts, and Word-style line
//! metrics (Windows ascent/descent, which is what Word uses for single line spacing).

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use skrifa::raw::TableProvider;

use crate::{FaceRef, FontDb};

/// A requested font resolved to a face.
#[derive(Clone, Copy, Debug)]
pub struct Resolved {
    pub face: FaceRef,
    /// The face isn't bold but bold was asked for: embolden when drawing.
    pub synth_bold: bool,
    /// The face isn't italic but italic was asked for: slant when drawing.
    pub synth_italic: bool,
    /// The requested family wasn't available.
    pub substituted: bool,
}

/// Substitutes to try, in order, for a family that isn't installed. Open fonts that are
/// metric-compatible come first (Intos ↔ Aptos, bundled; Carlito ↔ Calibri, Caladea ↔ Cambria,
/// Liberation ↔ Arial / Times New Roman / Courier New), then common system fonts, then our bundled
/// families.
fn substitutes(family: &str) -> &'static [&'static str] {
    const SANS: &[&str] = &["Carlito", "Helvetica Neue", "Arial", "Liberation Sans", "Noto Sans", "DejaVu Sans", "Source Sans 3"];
    const SERIF: &[&str] = &["Caladea", "Times New Roman", "Liberation Serif", "Georgia", "Noto Serif", "DejaVu Serif", "Source Serif 4"];
    const MONO: &[&str] = &["Courier New", "Liberation Mono", "Menlo", "Consolas", "DejaVu Sans Mono", "JetBrains Mono"];
    const ARIAL: &[&str] = &["Liberation Sans", "Helvetica", "Helvetica Neue", "Arimo", "Source Sans 3"];
    const TIMES: &[&str] = &["Liberation Serif", "Times", "Tinos", "Source Serif 4"];
    const SYMBOL: &[&str] = &["Apple Symbols", "Segoe UI Symbol", "DejaVu Sans", "Noto Sans Symbols", "Source Sans 3"];
    // Verdana: DejaVu Sans (Bitstream Vera's design) sets English text 1.2% narrower in regular
    // and 0.2% wider in bold, and breaks 60% / 74% of lines at the same word (measured against
    // Microsoft's core-fonts Verdana); Carlito, the old fallback, was 21% narrower and broke none.
    // Inter (bundled, for the web) is the next nearest, 6.8% narrower.
    const VERDANA: &[&str] = &["DejaVu Sans", "Bitstream Vera Sans", "Inter", "Source Sans 3"];
    // Georgia: Gelasio is made metric-compatible with it; the bundled Source Serif 4 sets English
    // text 0.1% wider and breaks 54% of lines at the same word (Caladea: 8.5% narrower, 1%).
    const GEORGIA: &[&str] = &["Gelasio", "Source Serif 4", "Caladea", "Liberation Serif", "Noto Serif"];
    // Persian / Arabic document fonts. Naskh (book) faces such as B Nazanin, B Lotus, B Zar and
    // Traditional Arabic, then the sans faces (B Yekan, B Titr, IRANSans) people use for
    // headings and screens. Open fonts with full Persian coverage first.
    const NASKH: &[&str] = &["Noto Naskh Arabic", "XB Zar", "XB Niloofar", "Amiri", "Vazirmatn", "Noto Sans Arabic", "DejaVu Sans", "Source Serif 4"];
    const ARABIC_SANS: &[&str] = &["Vazirmatn", "Sahel", "Shabnam", "Samim", "Noto Sans Arabic", "Noto Naskh Arabic", "DejaVu Sans", "Source Sans 3"];
    // Palatino Linotype / Book Antiqua: macOS ships the same design as "Palatino"; open
    // look-alikes with matching metrics (TeX Gyre Pagella, URW P052 / Palladio) on Linux.
    const PALATINO: &[&str] = &[
        "Palatino Linotype",
        "Palatino",
        "Book Antiqua",
        "TeX Gyre Pagella",
        "P052",
        "URW Palladio L",
        "Palladio",
        "Caladea",
        "Times New Roman",
        "Liberation Serif",
        "Georgia",
        "Noto Serif",
        "DejaVu Serif",
        "Source Serif 4",
    ];
    // Thai document fonts (TH Sarabun New / PSK and the other national fonts, the Windows UPC
    // families, Google's Thai families): the bundled Sarabun draws them.
    const THAI: &[&str] = &["Sarabun"];
    // Aptos, Word's default font since 2023: Intos (bundled) has its advances, kerning and
    // vertical metrics, as Intos Display, Narrow and Serif have Aptos Display's, Narrow's and
    // Serif's (Display is bundled; Narrow and Serif are used when installed).
    const APTOS: &[&str] = &["Intos", "Carlito", "Helvetica Neue", "Arial", "Liberation Sans", "Noto Sans", "DejaVu Sans", "Source Sans 3"];
    const APTOS_DISPLAY: &[&str] = &["Intos Display", "Intos", "Carlito", "Arial", "Liberation Sans", "Source Sans 3"];
    const APTOS_NARROW: &[&str] = &["Intos Narrow", "Intos", "Carlito", "Arial", "Liberation Sans", "Source Sans 3"];
    const APTOS_SERIF: &[&str] =
        &["Intos Serif", "Caladea", "Times New Roman", "Liberation Serif", "Georgia", "Noto Serif", "DejaVu Serif", "Source Serif 4"];
    let f = family.to_ascii_lowercase();
    if is_thai_document_font(&f) {
        return THAI;
    }
    if is_persian_naskh(&f) {
        return NASKH;
    }
    if is_persian_sans(&f) {
        return ARABIC_SANS;
    }
    match f.as_str() {
        "aptos" | "aptos light" | "aptos semibold" | "aptos extrabold" | "aptos black" => APTOS,
        "aptos display" => APTOS_DISPLAY,
        "aptos narrow" => APTOS_NARROW,
        "aptos serif" => APTOS_SERIF,
        "palatino linotype" | "book antiqua" | "palatino" => PALATINO,
        "arial" | "helvetica" | "arial nova" => ARIAL,
        "times new roman" | "times" => TIMES,
        "courier new" | "courier" | "consolas" | "lucida console" | "cascadia code" | "cascadia mono" => MONO,
        "symbol" | "wingdings" | "segoe ui symbol" | "webdings" => SYMBOL,
        "verdana" => VERDANA,
        "georgia" => GEORGIA,
        "cambria"
        | "cambria math"
        | "garamond"
        | "constantia"
        | "baskerville old face"
        | "century"
        | "century schoolbook"
        | "bookman old style"
        | "sitka text"
        | "sitka" => SERIF,
        _ if f.contains("mono") || f.contains("code") => MONO,
        _ if f.contains("serif") && !f.contains("sans") => SERIF,
        _ => SANS,
    }
}

/// Thai document fonts that the bundled Sarabun stands in for when they aren't installed. `f` is
/// lowercase. The Thai national fonts are named "TH …" (TH Sarabun New, TH SarabunPSK,
/// TH Niramit AS …); the Windows Thai families end in "UPC" or "New".
fn is_thai_document_font(f: &str) -> bool {
    const NAMES: &[&str] = &[
        "sarabun",
        "sarabun new",
        "thsarabunnew",
        "thsarabunpsk",
        "angsana new",
        "cordia new",
        "browallia new",
        "leelawadee",
        "leelawadee ui",
        "thonburi",
        "ayuthaya",
        "silom",
        "sathu",
        "krungthep",
        "kanit",
        "prompt",
        "mitr",
        "taviraj",
        "trirong",
        "pridi",
        "chonburi",
        "athiti",
        "maitree",
        "niramit",
        "krub",
        "k2d",
        "kodchasan",
        "bai jamjuree",
        "chakra petch",
        "charmonman",
        "fahkwang",
        "srisakdi",
        "thasadith",
    ];
    // A family's static weights often have names of their own ("Sarabun Light", "Kanit SemiBold
    // Italic"): match the family they belong to.
    const WEIGHTS: &[&str] = &[
        " thin",
        " extralight",
        " extra light",
        " light",
        " regular",
        " medium",
        " semibold",
        " semi bold",
        " bold",
        " extrabold",
        " extra bold",
        " black",
    ];
    let mut base = f.strip_suffix(" italic").unwrap_or(f);
    if let Some(b) = WEIGHTS.iter().find_map(|w| base.strip_suffix(w)) {
        base = b;
    }
    NAMES.contains(&f) || NAMES.contains(&base) || f.starts_with("th ") || f.ends_with("upc") || f.contains("thai")
}

/// Naskh-style Persian/Arabic document fonts (the "B …" family names come from the common
/// Persian font packs; matched by name only).
fn is_persian_naskh(f: &str) -> bool {
    const NAMES: &[&str] = &[
        "b nazanin",
        "b lotus",
        "b zar",
        "b mitra",
        "b badr",
        "b yagut",
        "b roya",
        "b compset",
        "b nazanin bold",
        "b lotus bold",
        "nazanin",
        "lotus",
        "zar",
        "mitra",
        "badr",
        "yagut",
        "roya",
        "ir nazanin",
        "ir lotus",
        "ir zar",
        "ir mitra",
        "ir badr",
        "irnazanin",
        "irlotus",
        "irzar",
        "irmitra",
        "irbadr",
        "traditional arabic",
        "simplified arabic",
        "arabic typesetting",
        "sakkal majalla",
        "adobe arabic",
        "andalus",
        "times new roman (arabic)",
    ];
    NAMES.contains(&f)
}

/// Sans Persian/Arabic fonts for headings and screens.
fn is_persian_sans(f: &str) -> bool {
    const NAMES: &[&str] = &[
        "b yekan",
        "b titr",
        "b homa",
        "b koodak",
        "b traffic",
        "b jadid",
        "b elham",
        "b kamran",
        "b davat",
        "b ferdosi",
        "yekan",
        "titr",
        "homa",
        "koodak",
        "traffic",
        "ir titr",
        "ir homa",
        "irtitr",
        "irhoma",
        "iransans",
        "iransansweb",
        "iran sans",
        "iranyekan",
        "iran yekan",
        "iranyekanweb",
        "dana",
        "estedad",
        "dubai",
        "aldhabi",
        "urdu typesetting",
        "arabic transparent",
    ];
    NAMES.contains(&f) || f.starts_with("iransans") || f.starts_with("iranyekan")
}

fn style_name(bold: bool, italic: bool) -> &'static str {
    match (bold, italic) {
        (false, false) => "Regular",
        (true, false) => "Bold",
        (false, true) => "Italic",
        (true, true) => "Bold Italic",
    }
}

/// A face name used as a font name, as documents do ("Arial Bold", "Times New Roman Bold
/// Italic"): the family and the style it names, when `family` isn't itself an installed family.
/// Windows finds such faces by their full names, so Word draws "Arial Bold" with Arial's bold;
/// families whose names end in other words ("Calibri Light", "Arial Black") stay as they are.
pub fn face_name(family: &str) -> Option<(&str, bool, bool)> {
    const STYLES: [(&str, bool, bool); 7] = [
        (" bold italic", true, true),
        (" bold oblique", true, true),
        (" bolditalic", true, true),
        (" italic", false, true),
        (" oblique", false, true),
        (" bold", true, false),
        (" regular", false, false),
    ];
    let lower = family.to_ascii_lowercase();
    let (suffix, bold, italic) = STYLES.iter().find(|(s, ..)| lower.ends_with(s))?;
    let base = family.get(..family.len() - suffix.len())?.trim_end();
    if base.is_empty() || FontDb::global().has_family(family) {
        return None;
    }
    Some((base, *bold, *italic))
}

/// The family `family` is drawn with when it isn't installed (Word's Font Substitution), or
/// `None` when it is.
pub fn substitute_for(family: &str) -> Option<String> {
    if let Some((base, ..)) = face_name(family) {
        return substitute_for(base);
    }
    let db = FontDb::global();
    if db.has_family(family) {
        return None;
    }
    Some(substitutes(family).iter().find(|s| db.has_family(s)).copied().unwrap_or(crate::FALLBACK_FAMILY).to_string())
}

/// Resolve `family` with bold/italic to a face (memoised until user fonts are added or removed,
/// [`FontDb::generation`]).
pub fn resolve(family: &str, bold: bool, italic: bool) -> Resolved {
    type Memo = (u64, HashMap<(String, bool, bool), Resolved>);
    static CACHE: OnceLock<Mutex<Memo>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new((0, HashMap::new())));
    let db = FontDb::global();
    let generation = db.generation();
    let key = (family.to_string(), bold, italic);
    {
        let mut c = cache.lock().unwrap_or_else(|e| e.into_inner());
        if c.0 != generation {
            *c = (generation, HashMap::new());
        }
        if let Some(r) = c.1.get(&key) {
            return *r;
        }
    }
    if let Some((base, b, i)) = face_name(family) {
        let r = resolve(base, bold || b, italic || i);
        let mut c = cache.lock().unwrap_or_else(|e| e.into_inner());
        if c.0 == generation {
            c.1.insert(key, r);
        }
        return r;
    }
    let (fam, substituted) = if db.has_family(family) {
        (family.to_string(), false)
    } else {
        let s = substitutes(family).iter().find(|s| db.has_family(s)).copied().unwrap_or(crate::FALLBACK_FAMILY);
        (s.to_string(), true)
    };
    let face = db.face(&fam, style_name(bold, italic));
    let r = Resolved { synth_bold: bold && face.weight < 600.0, synth_italic: italic && !face.italic, substituted, face: FaceRef::of(&face) };
    let mut c = cache.lock().unwrap_or_else(|e| e.into_inner());
    if c.0 == generation {
        c.1.insert(key, r);
    }
    r
}

/// The standard Unicode character for a Symbol or Wingdings character stored the way Word stores
/// it, as the private-use code U+F000 + the font's own code (U+F0B7 is the Symbol bullet). Used
/// when that font isn't installed: a substitute draws something else, or nothing, at the
/// private-use code. `None` for other fonts and for codes not in the table.
pub fn symbol_font_char(family: &str, c: char) -> Option<char> {
    let code = u32::from(c).checked_sub(0xF000)?;
    let u = match (family.to_ascii_lowercase().as_str(), code) {
        // Adobe Symbol encoding.
        ("symbol", 0xA7) => '\u{2663}',
        ("symbol", 0xA8) => '\u{2666}',
        ("symbol", 0xA9) => '\u{2665}',
        ("symbol", 0xAA) => '\u{2660}',
        ("symbol", 0xAE) => '\u{2192}',
        ("symbol", 0xB7) => '\u{2022}',
        ("symbol", 0xD7) => '\u{22C5}',
        ("symbol", 0xE0) => '\u{25CA}',
        // Wingdings characters common as list bullets.
        ("wingdings", 0x6C) => '\u{25CF}',
        ("wingdings", 0x6E) => '\u{25A0}',
        ("wingdings", 0x76) => '\u{2756}',
        ("wingdings", 0xA7) => '\u{25AA}',
        ("wingdings", 0xD8) => '\u{27A2}',
        ("wingdings", 0xFC) => '\u{2714}',
        _ => return None,
    };
    Some(u)
}

/// A line's (ascent, descent) from hhea values, both positive: the line gap goes above the ascent.
fn hhea_line(ascender: i16, descender: i16, line_gap: i16) -> (f64, f64) {
    (f64::from(ascender) + f64::from(line_gap.max(0)), f64::from(descender).abs())
}

/// A line's (ascent, descent) the way Word for Windows measures it (GDI text metrics): OS/2
/// usWinAscent and usWinDescent, plus the external leading, the part of the hhea line gap that
/// the win metrics don't already cover, above the ascent. Times New Roman (win 1825/443, hhea
/// 1825/-443, gap 87): 1912 + 443, like 13.8 pt at 12 pt. Sarabun (win 1286/567, hhea 1068/-232,
/// gap 0): 1286 + 567, 1.853 em, where Word for Mac's hhea gives 1.30 em.
fn win_line(win_ascent: u16, win_descent: u16, ascender: i16, descender: i16, line_gap: i16) -> (f64, f64) {
    let win = f64::from(win_ascent) + f64::from(win_descent);
    let hhea = f64::from(ascender) - f64::from(descender);
    let external = (f64::from(line_gap) - (win - hhea)).max(0.0);
    (f64::from(win_ascent) + external, f64::from(win_descent))
}

/// A line's (ascent, descent) in font units from a face's OS/2 win metrics and hhea values:
/// [`win_line`] when the win metrics are set and sane (not zero, at most four ems), else
/// [`hhea_line`] (at most two ems), else `None`. With `prefer_hhea` the hhea line comes first.
fn pick_line(win: Option<(u16, u16)>, hhea: Option<(i16, i16, i16)>, upem: f64, prefer_hhea: bool) -> Option<(f64, f64)> {
    let ok = |m: &(f64, f64), max_ems: f64| m.0 > 0.0 && m.0 + m.1 <= upem * max_ems;
    let (asc, desc, gap) = hhea.unwrap_or((0, 0, 0));
    let win = win.filter(|(a, d)| *a > 0 && u32::from(*a) + u32::from(*d) > 0).map(|(a, d)| win_line(a, d, asc, desc, gap)).filter(|m| ok(m, 4.0));
    let hhea = hhea.map(|(a, d, g)| hhea_line(a, d, g)).filter(|m| ok(m, 2.0));
    if prefer_hhea { hhea.or(win) } else { win.or(hhea) }
}

/// Line metrics for a face in font units: (ascent, descent), both positive, the way Word for
/// Windows measures single line spacing ([`win_line`]), so a page holds the same lines here, in
/// Word on Windows and in print on every platform. hhea values (the line gap above the ascent)
/// when the face has no OS/2 table or its win metrics are zero or absurd (over four ems).
pub fn line_metrics(face: &crate::FontFace) -> (f64, f64) {
    face_line(face, false)
}

/// The cached line of `face`: [`pick_line`] over its tables, or its ascent and descent.
fn face_line(face: &crate::FontFace, prefer_hhea: bool) -> (f64, f64) {
    type Cache = Mutex<HashMap<(u32, bool), (f64, f64)>>;
    static CACHE: OnceLock<Cache> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(v) = cache.lock().unwrap_or_else(|e| e.into_inner()).get(&(face.id(), prefer_hhea)) {
        return *v;
    }
    let mut v = (face.ascent.max(1.0), face.descent.abs());
    if let Some(f) = face.skrifa() {
        let hhea = f.hhea().ok().map(|h| (h.ascender().to_i16(), h.descender().to_i16(), h.line_gap().to_i16()));
        let win = f.os2().ok().map(|o| (o.us_win_ascent(), o.us_win_descent()));
        if let Some(m) = pick_line(win, hhea, face.upem, prefer_hhea) {
            v = m;
        }
    }
    cache.lock().unwrap_or_else(|e| e.into_inner()).insert((face.id(), prefer_hhea), v);
    v
}

/// Word's single line of fonts whose open substitutes, or whose own win metrics, are spaced
/// differently, per em (ascent, descent): Cambria (drawn with Caladea, 1.30 em), Courier New (drawn
/// with JetBrains Mono where only the bundled fonts exist, 1.565 em), Verdana and Georgia (drawn
/// with DejaVu Sans and Source Serif 4) and Aptos, whose lines in Word's PDFs step 1.2207 em (its
/// hhea and typo metrics, 1923 + 577), not its win metrics' 1.2847.
fn requested_line(family: &str) -> Option<(f64, f64)> {
    match family.trim().to_ascii_lowercase().as_str() {
        "cambria" | "cambria math" => Some((1946.0 / 2048.0, 455.0 / 2048.0)),
        "courier new" | "courier" => Some((1705.0 / 2048.0, 615.0 / 2048.0)),
        // Verdana (win 2059 / 430: 1.215 em; DejaVu Sans is 1.164) and Georgia (win 1878 / 449:
        // 1.136 em; Source Serif 4 is 1.371), every style alike.
        "verdana" => Some((2059.0 / 2048.0, 430.0 / 2048.0)),
        "georgia" => Some((1878.0 / 2048.0, 449.0 / 2048.0)),
        // Every Aptos family (Display, Narrow, Serif, Mono, each weight): hhea 1923 / 577.
        f if is_aptos(f) => Some((1923.0 / 2048.0, 577.0 / 2048.0)),
        _ => None,
    }
}

/// Aptos, Aptos Display, Narrow, Serif, Mono and each weight.
fn is_aptos(family: &str) -> bool {
    let f = family.trim().to_ascii_lowercase();
    f == "aptos" || f.starts_with("aptos ")
}

/// Single line spacing for text a document sets in `family`, drawn with `face` ([`resolve`]),
/// at `size` points: (ascent, descent) in points. The face's own Word for Windows line
/// ([`line_metrics`]) when it is the font asked for, except Aptos ([`requested_line`]). A
/// substitute keeps the line of the font it stands in for where that is known
/// ([`requested_line`]), and otherwise its hhea line, which
/// metric-compatible substitutes share with their originals and which keeps a Thai font drawn
/// with Sarabun (Angsana New, Cordia New…) near its own small-letter spacing (1.30 em, not
/// Sarabun's 1.853 em).
pub fn single_line(family: &str, face: &crate::FontFace, substituted: bool, size: f64) -> (f64, f64) {
    // "Arial Bold" spaces lines as Arial does.
    let family = face_name(family).map_or(family, |(base, ..)| base);
    // Aptos's own win metrics aren't what Word spaces it by, so its line holds when it is installed.
    if let Some((a, d)) = requested_line(family).filter(|_| substituted || is_aptos(family)) {
        return (a * size, d * size);
    }
    if !substituted {
        let (a, d) = line_metrics(face);
        let k = size / face.upem.max(1.0);
        return (a * k, d * k);
    }
    let (a, d) = face_line(face, true);
    let k = size / face.upem.max(1.0);
    (a * k, d * k)
}

/// How a substitute stands in for a font a document asks for, so its text keeps that font's size,
/// widths and line spacing and pages hold what they hold in Word with the real font: TH Sarabun
/// New and TH SarabunPSK, drawn with the bundled Sarabun when they are not installed. Sarabun's
/// letters are about 1.47 times as tall as theirs at the same point size, and its widths differ
/// letter by letter, so its glyphs are scaled to their height
/// ([`MetricFit::scale`]) and set on their advance widths ([`MetricFit::advance`], with their
/// kerning, [`MetricFit::kern`]); lines take their line spacing ([`MetricFit::line`]).
#[derive(Clone, Copy)]
pub struct MetricFit {
    style: &'static crate::thsarabun::ThSarabunStyle,
}

impl std::fmt::Debug for MetricFit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetricFit").field("thai_scale", &self.style.thai_scale).finish()
    }
}

impl PartialEq for MetricFit {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.style, other.style)
    }
}

impl MetricFit {
    /// The substitute's glyph scale: for Thai letters (the height of ก) or Latin ones (of x).
    pub fn scale(&self, thai: bool) -> f32 {
        if thai { self.style.thai_scale } else { self.style.latin_scale }
    }

    /// The advance width of `c` in the original font, in ems; `None` when it has no such character
    /// (the substitute's own width applies).
    pub fn advance(&self, c: char) -> Option<f32> {
        let a = self.style.advances;
        a.binary_search_by_key(&c, |e| e.0).ok().and_then(|i| a.get(i)).map(|e| f32::from(e.1) / 1000.0)
    }

    /// The original font's kerning between `left` and `right`, in ems (when the run is kerned).
    pub fn kern(&self, left: char, right: char) -> f32 {
        let k = crate::thsarabun::KERN;
        k.binary_search_by(|e| (e.0, e.1).cmp(&(left, right))).ok().and_then(|i| k.get(i)).map_or(0.0, |e| f32::from(e.2) / 1000.0)
    }

    /// The original font's line (ascent with the external leading, descent), in ems, measured
    /// as [`line_metrics`] measures fonts.
    pub fn line(&self) -> (f64, f64) {
        let (a, d) = crate::thsarabun::LINE;
        (a / 1000.0, d / 1000.0)
    }
}

/// The [`MetricFit`] for a requested family drawn with a substitute: `Some` for TH Sarabun New and
/// TH SarabunPSK (any spacing or case) when they are not installed and resolve to the bundled
/// Sarabun; `None` otherwise, including when the real font is installed. Other Thai document fonts
/// are drawn with Sarabun at its own size and widths.
pub fn metric_fit(family: &str, bold: bool, italic: bool) -> Option<MetricFit> {
    // Layout asks for every shaped piece: rule most families out without allocating.
    if !family.trim_start().get(..2).is_some_and(|p| p.eq_ignore_ascii_case("th")) {
        return None;
    }
    let key: String = family.chars().filter(|c| !c.is_whitespace()).flat_map(char::to_lowercase).collect();
    if !matches!(key.as_str(), "thsarabunnew" | "thsarabunpsk") {
        return None;
    }
    let r = resolve(family, bold, italic);
    if !r.substituted || r.face.family != "Sarabun" {
        return None;
    }
    let style = match (bold, italic) {
        (false, false) => &crate::thsarabun::REGULAR,
        (true, false) => &crate::thsarabun::BOLD,
        (false, true) => &crate::thsarabun::ITALIC,
        (true, true) => &crate::thsarabun::BOLD_ITALIC,
    };
    Some(MetricFit { style })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_line_gap_goes_above_the_ascent() {
        // Times New Roman (hhea 1825 / -443, gap 87), and a font with a large gap: Word puts the
        // first baseline the ascent plus the whole gap below the line top.
        assert_eq!(hhea_line(1825, -443, 87), (1912.0, 443.0));
        assert_eq!(hhea_line(1536, -512, 452), (1988.0, 512.0));
        // A negative gap adds nothing; the most negative descender doesn't overflow.
        assert_eq!(hhea_line(1000, -300, -50), (1000.0, 300.0));
        assert_eq!(hhea_line(1000, i16::MIN, 0), (1000.0, 32768.0));
    }

    #[test]
    fn single_spacing_uses_word_for_windows_metrics() {
        // Times New Roman and Arial: the win metrics equal hhea, so the whole gap is external
        // leading (13.8 pt at 12 pt). Calibri: the win metrics already cover hhea + gap (2500).
        assert_eq!(win_line(1825, 443, 1825, -443, 87), (1912.0, 443.0));
        assert_eq!(win_line(1854, 434, 1854, -434, 67), (1921.0, 434.0));
        assert_eq!(win_line(1950, 550, 1536, -512, 452), (1950.0, 550.0));
        // Sarabun: 1.853 em on Windows (hhea would give 1.30 em).
        assert_eq!(win_line(1286, 567, 1068, -232, 0), (1286.0, 567.0));
        // Extremes don't overflow; a negative gap adds nothing.
        assert_eq!(win_line(u16::MAX, u16::MAX, i16::MIN, i16::MAX, i16::MIN), (65535.0, 65535.0));
        assert_eq!(win_line(0, 0, 0, 0, 0), (0.0, 0.0));
        // The bundled Sarabun measures that way.
        let r = resolve("Sarabun", false, false);
        assert_eq!(r.face.family, "Sarabun");
        assert_eq!(line_metrics(&r.face), (1286.0, 567.0));
        // Zero win metrics fall back to hhea, as do absurd ones; hhea first when asked.
        assert_eq!(pick_line(Some((0, 0)), Some((800, -200, 0)), 1000.0, false), Some((800.0, 200.0)));
        assert_eq!(pick_line(Some((9000, 0)), Some((800, -200, 0)), 1000.0, false), Some((800.0, 200.0)));
        assert_eq!(pick_line(Some((1286, 567)), Some((1068, -232, 0)), 1000.0, true), Some((1068.0, 232.0)));
        assert_eq!(pick_line(None, None, 1000.0, false), None);
    }

    /// A substitute keeps the spacing of the font it stands in for: Cambria 1.172 em and Courier
    /// New 1.133 em whatever draws them, a Thai font drawn with Sarabun near 1.30 em, and the
    /// font asked for its own Word for Windows line (Sarabun 1.853 em).
    #[test]
    fn face_names_resolve_to_their_family_and_style() {
        // govuk-029's headings ask for "Arial Bold": Word draws Arial's bold, not a fallback.
        let a = resolve("Arial Bold", false, false);
        let b = resolve("Arial", true, false);
        assert_eq!(a.face.id(), b.face.id());
        assert_eq!((a.synth_bold, a.substituted), (b.synth_bold, b.substituted));
        assert_eq!(resolve("Times New Roman Bold Italic", false, false).face.id(), resolve("Times New Roman", true, true).face.id());
        assert_eq!(resolve("arial italic", false, false).face.id(), resolve("Arial", false, true).face.id());
        let face = crate::FontDb::global().face(crate::FALLBACK_FAMILY, "Regular");
        assert_eq!(single_line("Arial Bold", &face, true, 12.0), single_line("Arial", &face, true, 12.0));
        assert_eq!(substitute_for("Arial Bold"), substitute_for("Arial"));
        // Other words are part of the family name.
        assert_eq!(face_name("Calibri Light"), None);
        assert_eq!(face_name("Arial Black"), None);
        assert_eq!(face_name("Bold"), None);
        assert_eq!(face_name("Bembo Regular"), Some(("Bembo", false, false)));
    }

    #[test]
    fn substitutes_keep_the_requested_fonts_line() {
        let r = resolve("Sarabun", false, false);
        let em = |(a, d): (f64, f64)| ((a + d) * 1000.0).round() / 1000.0;
        assert_eq!(em(single_line("Sarabun", &r.face, false, 1.0)), 1.853);
        assert_eq!(em(single_line("Cordia New", &r.face, true, 1.0)), 1.3);
        assert_eq!(em(single_line("Cambria", &r.face, true, 1.0)), 1.172);
        assert_eq!(em(single_line("Courier New", &r.face, true, 1.0)), 1.133);
        // Points scale with the size.
        assert_eq!(em(single_line("Cambria", &r.face, true, 12.0)), 14.068);
    }

    #[test]
    fn th_sarabun_fonts_fit_sarabun_to_their_metrics() {
        for name in ["TH Sarabun New", "TH SarabunPSK", "th sarabun new", "THSarabunNew", "TH Sarabun PSK"] {
            let f = metric_fit(name, false, false).unwrap_or_else(|| panic!("{name}"));
            assert_eq!(f.advance('ก'), Some(0.386), "{name}");
            assert_eq!(f.advance('\u{0E48}'), Some(0.0), "{name}: tone marks take no width");
            assert_eq!(f.advance('😀'), None);
            assert!((f.scale(true) - 400.0 / 586.0).abs() < 1e-5 && (f.scale(false) - 0.68).abs() < 1e-5);
            assert_eq!(f.kern('A', 'V'), -0.05);
            assert_eq!(f.kern('ก', 'ข'), 0.0);
            assert_eq!(f.line(), (0.874, 0.457));
        }
        // Bold has its own widths and Thai scale (ก 407 high against Sarabun Bold's 586).
        let b = metric_fit("TH Sarabun New", true, false).unwrap();
        assert!((b.scale(true) - 407.0 / 586.0).abs() < 1e-5);
        assert_ne!(b.advance('ก'), Some(0.386));
        // Other families, and fonts that are themselves available, are drawn as they are; TH
        // SarabunIT๙ has no metrics here and is drawn with Sarabun at its own size.
        for name in ["Sarabun", "Arial", "TH Niramit AS", "TH SarabunIT๙", "TH SarabunIT9", "", "T", "THX", "ทดสอบ"] {
            assert!(metric_fit(name, false, false).is_none(), "{name}");
        }
    }

    /// Aptos isn't installed here: the bundled Intos (metric-compatible) draws it, Intos Display
    /// draws Aptos Display, and lines keep the spacing Word gives Aptos (1923 + 577 of 2048).
    #[test]
    fn aptos_is_drawn_with_the_bundled_intos() {
        for (asked, family) in [("Aptos", "Intos"), ("aptos", "Intos"), ("Aptos Display", "Intos Display"), ("Aptos SemiBold", "Intos")] {
            let r = resolve(asked, false, false);
            assert!(r.substituted, "{asked}");
            assert_eq!(r.face.family, family, "{asked}");
            let em = |(a, d): (f64, f64)| ((a + d) * 10000.0).round() / 10000.0;
            assert_eq!(em(single_line(asked, &r.face, true, 1.0)), 1.2207, "{asked}");
            // The same when Aptos itself is installed.
            assert_eq!(em(single_line(asked, &r.face, false, 1.0)), 1.2207, "{asked}");
        }
        for (bold, italic, style) in [(true, false, "Bold"), (false, true, "Italic"), (true, true, "Bold Italic")] {
            let r = resolve("Aptos", bold, italic);
            assert_eq!((r.face.family.as_str(), r.face.style.as_str()), ("Intos", style));
            assert!(!r.synth_bold && !r.synth_italic);
        }
        // Narrow and Serif are not bundled: their own look-alikes when installed, else the
        // next substitutes; Aptos Mono stays with the monospaced ones.
        assert_eq!(substitutes("Aptos Narrow").first(), Some(&"Intos Narrow"));
        assert_eq!(substitutes("Aptos Serif").first(), Some(&"Intos Serif"));
        assert_eq!(substitutes("Aptos Mono").first(), Some(&"Courier New"));
        assert_eq!(requested_line("Aptos Mono"), Some((1923.0 / 2048.0, 577.0 / 2048.0)));
        assert_eq!(requested_line("Aptosx"), None);
    }

    #[test]
    fn thai_families_named_by_weight_are_thai_document_fonts() {
        for f in ["Sarabun Light", "Sarabun ExtraBold", "Kanit Light", "Prompt SemiBold", "Mitr Medium Italic", "Sarabun Thin Italic"] {
            assert!(is_thai_document_font(&f.to_ascii_lowercase()), "{f}");
        }
        for f in ["Inter Light", "Arial Black", "Segoe UI Semibold", "Light"] {
            assert!(!is_thai_document_font(&f.to_ascii_lowercase()), "{f}");
        }
    }

    #[test]
    fn unknown_family_substitutes() {
        let r = resolve("Definitely Not A Font 123", false, false);
        assert!(r.substituted);
        let b = resolve("Source Serif 4", true, false);
        assert!(!b.substituted);
        assert!(!b.synth_bold);
        let (a, d) = line_metrics(&b.face);
        assert!(a > 0.0 && d >= 0.0);
    }

    #[test]
    fn palatino_family_prefers_palatino_look_alikes_over_generic_serif() {
        for name in ["Palatino Linotype", "Book Antiqua", "palatino"] {
            let list = substitutes(name);
            let pos = |n: &str| list.iter().position(|x| *x == n);
            assert!(pos("Palatino").is_some() && pos("TeX Gyre Pagella").is_some() && pos("P052").is_some(), "{name}");
            // All of them come before the generic serif fallbacks (Caladea, Times New Roman).
            assert!(pos("Palatino") < pos("Caladea") && pos("TeX Gyre Pagella") < pos("Times New Roman"), "{name}");
        }
        // Other serif families are unchanged.
        assert_eq!(substitutes("Garamond").first(), Some(&"Caladea"));
    }

    /// Verdana and Georgia go to the open fonts nearest their widths (DejaVu Sans, Source Serif
    /// 4) and keep Word's own line spacing for them.
    #[test]
    fn verdana_and_georgia_keep_their_widths_and_lines() {
        assert_eq!(substitutes("Verdana").first(), Some(&"DejaVu Sans"));
        assert!(substitutes("Georgia").iter().position(|x| *x == "Source Serif 4") < substitutes("Georgia").iter().position(|x| *x == "Caladea"));
        let r = resolve("Georgia", false, false);
        let em = |(a, d): (f64, f64)| ((a + d) * 1000.0).round() / 1000.0;
        assert_eq!(em(single_line("Georgia", &r.face, true, 1.0)), 1.136);
        assert_eq!(em(single_line("Verdana", &r.face, true, 1.0)), 1.215);
    }

    #[test]
    fn thai_document_fonts_are_drawn_with_the_bundled_sarabun() {
        for name in ["TH Sarabun New", "TH SarabunPSK", "TH SarabunIT๙", "TH Niramit AS", "Angsana New", "AngsanaUPC", "Cordia New", "Noto Sans Thai"]
        {
            assert_eq!(substitutes(name), &["Sarabun"], "{name}");
        }
        // Other families keep their substitutes ("Thai" only in a name like "Noto Sans Thai").
        assert_eq!(substitutes("Garamond").first(), Some(&"Caladea"));
        assert_ne!(substitutes("Tahoma"), &["Sarabun"]);
        assert_ne!(substitutes("Theinhardt"), &["Sarabun"]);
        // Not installed here, so the request resolves to Sarabun itself (bundled).
        let r = resolve("TH Sarabun New Doc4Me Test", false, false);
        assert!(r.substituted);
        assert_eq!(r.face.family, "Sarabun");
        let b = resolve("Sarabun", true, false);
        assert!(!b.substituted && !b.synth_bold);
        assert_eq!(b.face.style, "Bold");
    }

    #[test]
    fn symbol_font_chars_map_to_unicode() {
        assert_eq!(symbol_font_char("Symbol", '\u{F0B7}'), Some('\u{2022}'));
        assert_eq!(symbol_font_char("symbol", '\u{F0B7}'), Some('\u{2022}'));
        assert_eq!(symbol_font_char("Wingdings", '\u{F0A7}'), Some('\u{25AA}'));
        assert_eq!(symbol_font_char("Wingdings", '\u{F0D8}'), Some('\u{27A2}'));
        // Same code, other font; unknown codes; ordinary characters.
        assert_eq!(symbol_font_char("Arial", '\u{F0B7}'), None);
        assert_eq!(symbol_font_char("Symbol", '\u{F041}'), None);
        assert_eq!(symbol_font_char("Symbol", 'a'), None);
        assert_eq!(symbol_font_char("Symbol", '\u{10FFFF}'), None);
    }

    #[test]
    fn resolves_weights_named_as_their_own_family() {
        // Word stores a Medium face as the family "Inter Medium" (#219).
        let r = resolve("Inter Medium", false, false);
        assert!(!r.substituted && !r.synth_bold && !r.synth_italic);
        assert_eq!((r.face.family.as_str(), r.face.style.as_str()), ("Inter", "Medium"));
        // Bold and italic within it are drawn from the Medium face.
        let b = resolve("Inter Medium", true, true);
        assert_eq!(b.face.style, "Medium");
        assert!(!b.substituted && b.synth_bold && b.synth_italic);
    }

    #[test]
    fn synthesizes_missing_styles() {
        // JetBrains Mono ships only Regular here.
        let r = resolve("JetBrains Mono", true, true);
        assert!(r.synth_bold && r.synth_italic);
    }
}
