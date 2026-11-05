//! Handwritten math: Insert › Equation › Ink Equation and Draw › Ink to Math.
//!
//! Word sends the ink to Microsoft's handwriting service. Doc4Me recognises it on the machine
//! with its own code: strokes are grouped into symbols, each symbol is matched against built-in
//! templates with the $P point-cloud recogniser (Vatavu, Anthony and Wobbrock, "Gestures as
//! Point Clouds", ICMI 2012: stroke order and direction don't matter), and the symbols' places
//! give the structure — superscripts, subscripts, fraction bars and square roots. The answer is
//! UnicodeMath linear text (`x^2+y^2=z^2`), which the equation editor builds, and which the
//! Ink Equation dialog lets the user correct before inserting.
//!
//! The symbols known: digits, `a b c n x y z`, `+ − = < > / ( )`, `π`, `∫`, `√` and the
//! decimal point. Strokes come from the user or a document, so their number and size are capped.

use std::sync::OnceLock;

/// One stroke: points in any unit, y down.
pub type Stroke = Vec<[f32; 2]>;

/// Most strokes and points per stroke read.
pub const MAX_STROKES: usize = 300;
const MAX_POINTS: usize = 2000;
/// Points each symbol is resampled to.
const N: usize = 32;

#[derive(Clone, Copy, Debug)]
struct Pt {
    x: f32,
    y: f32,
    id: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Bounds {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

impl Bounds {
    fn of<'a>(pts: impl IntoIterator<Item = &'a [f32; 2]>) -> Bounds {
        let mut b = Bounds { x0: f32::MAX, y0: f32::MAX, x1: f32::MIN, y1: f32::MIN };
        for [x, y] in pts {
            b.x0 = b.x0.min(*x);
            b.y0 = b.y0.min(*y);
            b.x1 = b.x1.max(*x);
            b.y1 = b.y1.max(*y);
        }
        b
    }
    fn union(self, o: Bounds) -> Bounds {
        Bounds { x0: self.x0.min(o.x0), y0: self.y0.min(o.y0), x1: self.x1.max(o.x1), y1: self.y1.max(o.y1) }
    }
    fn w(&self) -> f32 {
        self.x1 - self.x0
    }
    fn h(&self) -> f32 {
        self.y1 - self.y0
    }
    fn cx(&self) -> f32 {
        (self.x0 + self.x1) / 2.0
    }
    fn cy(&self) -> f32 {
        (self.y0 + self.y1) / 2.0
    }
}

// ---------------------------------------------------------------------------------------------
// $P

/// The points of `strokes` resampled to `N` evenly spaced points (no jumps across strokes),
/// scaled to a unit box (keeping proportions) and centred on the origin.
fn normalize(strokes: &[Stroke]) -> Vec<Pt> {
    let mut pts: Vec<Pt> = strokes.iter().enumerate().flat_map(|(id, s)| s.iter().map(move |p| Pt { x: p[0], y: p[1], id })).collect();
    if pts.is_empty() {
        return Vec::new();
    }
    // Resample.
    let len: f32 = pts.windows(2).filter(|w| w[0].id == w[1].id).map(|w| (w[1].x - w[0].x).hypot(w[1].y - w[0].y)).sum();
    let step = len / (N - 1) as f32;
    let mut out = vec![pts[0]];
    if step > 0.0 {
        let mut d = 0.0;
        let mut i = 1;
        while i < pts.len() && out.len() < N {
            let (a, b) = (pts[i - 1], pts[i]);
            if a.id == b.id {
                let seg = (b.x - a.x).hypot(b.y - a.y);
                if d + seg >= step && seg > 0.0 {
                    let t = (step - d) / seg;
                    let q = Pt { x: a.x + t * (b.x - a.x), y: a.y + t * (b.y - a.y), id: a.id };
                    out.push(q);
                    pts.insert(i, q);
                    d = 0.0;
                } else {
                    d += seg;
                }
            }
            i += 1;
        }
    }
    while out.len() < N {
        let last = *out.last().unwrap_or(&pts[0]);
        out.push(last);
    }
    // Scale and centre.
    let b = Bounds::of(out.iter().map(|p| [p.x, p.y]).collect::<Vec<_>>().iter());
    let size = b.w().max(b.h()).max(1e-6);
    let (cx, cy) = (out.iter().map(|p| p.x).sum::<f32>() / N as f32, out.iter().map(|p| p.y).sum::<f32>() / N as f32);
    out.iter().map(|p| Pt { x: (p.x - cx) / size, y: (p.y - cy) / size, id: p.id }).collect()
}

fn cloud_distance(a: &[Pt], b: &[Pt], start: usize) -> f32 {
    let n = a.len();
    let mut matched = vec![false; n];
    let mut sum = 0.0;
    let mut i = start;
    for k in 0..n {
        let mut best = f32::MAX;
        let mut index = 0;
        for (j, q) in b.iter().enumerate() {
            if !matched[j] {
                let d = (a[i].x - q.x).powi(2) + (a[i].y - q.y).powi(2);
                if d < best {
                    best = d;
                    index = j;
                }
            }
        }
        matched[index] = true;
        let weight = 1.0 - k as f32 / n as f32;
        sum += weight * best.sqrt();
        i = (i + 1) % n;
    }
    sum
}

fn greedy_match(a: &[Pt], b: &[Pt]) -> f32 {
    let n = a.len().min(b.len());
    if n == 0 {
        return f32::MAX;
    }
    let step = ((n as f32).powf(0.5)).floor().max(1.0) as usize;
    (0..n).step_by(step).map(|i| cloud_distance(a, b, i).min(cloud_distance(b, a, i))).fold(f32::MAX, f32::min)
}

// ---------------------------------------------------------------------------------------------
// Templates (a 100 × 100 box, y down)

fn arc(cx: f32, cy: f32, rx: f32, ry: f32, from: f32, to: f32) -> Stroke {
    let n = 16;
    (0..=n)
        .map(|i| {
            let t = (from + (to - from) * i as f32 / n as f32).to_radians();
            [cx + rx * t.cos(), cy + ry * t.sin()]
        })
        .collect()
}

fn line(pts: &[(f32, f32)]) -> Stroke {
    pts.iter().map(|&(x, y)| [x, y]).collect()
}

fn join(parts: &[Stroke]) -> Stroke {
    parts.iter().flatten().copied().collect()
}

struct Template {
    sym: &'static str,
    cloud: Vec<Pt>,
}

fn templates() -> &'static [Template] {
    static T: OnceLock<Vec<Template>> = OnceLock::new();
    T.get_or_init(|| {
        let defs: Vec<(&'static str, Vec<Stroke>)> = vec![
            ("0", vec![arc(50.0, 50.0, 30.0, 45.0, -90.0, 270.0)]),
            ("1", vec![line(&[(50.0, 5.0), (50.0, 95.0)])]),
            ("1", vec![line(&[(32.0, 24.0), (50.0, 5.0), (50.0, 95.0)])]),
            ("2", vec![join(&[arc(50.0, 30.0, 25.0, 25.0, 200.0, 380.0), line(&[(25.0, 95.0), (78.0, 95.0)])])]),
            ("3", vec![join(&[arc(48.0, 28.0, 22.0, 22.0, 200.0, 450.0), arc(48.0, 72.0, 23.0, 23.0, 270.0, 500.0)])]),
            ("4", vec![line(&[(62.0, 5.0), (18.0, 65.0), (85.0, 65.0)]), line(&[(62.0, 30.0), (62.0, 95.0)])]),
            ("4", vec![line(&[(62.0, 95.0), (62.0, 5.0), (18.0, 65.0), (85.0, 65.0)])]),
            ("5", vec![join(&[line(&[(72.0, 6.0), (32.0, 6.0), (30.0, 45.0)]), arc(50.0, 68.0, 25.0, 25.0, 230.0, 500.0)])]),
            ("5", vec![join(&[line(&[(32.0, 6.0), (30.0, 45.0)]), arc(50.0, 68.0, 25.0, 25.0, 230.0, 500.0)]), line(&[(32.0, 6.0), (72.0, 6.0)])]),
            (
                "6",
                vec![line(&[
                    (68.0, 8.0),
                    (50.0, 10.0),
                    (36.0, 25.0),
                    (30.0, 50.0),
                    (32.0, 75.0),
                    (45.0, 93.0),
                    (60.0, 92.0),
                    (70.0, 78.0),
                    (66.0, 62.0),
                    (52.0, 56.0),
                    (38.0, 62.0),
                    (31.0, 72.0),
                ])],
            ),
            ("7", vec![line(&[(20.0, 8.0), (80.0, 8.0), (42.0, 95.0)])]),
            (
                "8",
                vec![line(&[
                    (50.0, 50.0),
                    (30.0, 35.0),
                    (32.0, 15.0),
                    (50.0, 6.0),
                    (68.0, 15.0),
                    (70.0, 35.0),
                    (50.0, 50.0),
                    (28.0, 68.0),
                    (30.0, 88.0),
                    (50.0, 96.0),
                    (70.0, 88.0),
                    (72.0, 68.0),
                    (50.0, 50.0),
                ])],
            ),
            ("9", vec![join(&[arc(50.0, 30.0, 22.0, 22.0, 0.0, 360.0), line(&[(72.0, 30.0), (68.0, 95.0)])])]),
            ("a", vec![join(&[arc(45.0, 65.0, 22.0, 25.0, -30.0, 330.0), line(&[(67.0, 45.0), (67.0, 90.0), (76.0, 95.0)])])]),
            ("b", vec![join(&[line(&[(30.0, 5.0), (30.0, 95.0), (30.0, 62.0)]), arc(48.0, 75.0, 20.0, 20.0, 200.0, 520.0)])]),
            ("c", vec![arc(52.0, 65.0, 25.0, 28.0, 320.0, 40.0)]),
            ("n", vec![line(&[(25.0, 40.0), (25.0, 95.0)]), line(&[(25.0, 58.0), (38.0, 42.0), (58.0, 40.0), (72.0, 52.0), (72.0, 95.0)])]),
            ("n", vec![line(&[(25.0, 40.0), (25.0, 95.0), (25.0, 58.0), (38.0, 42.0), (58.0, 40.0), (72.0, 52.0), (72.0, 95.0)])]),
            ("x", vec![line(&[(22.0, 35.0), (78.0, 95.0)]), line(&[(78.0, 35.0), (22.0, 95.0)])]),
            ("y", vec![line(&[(22.0, 30.0), (50.0, 62.0)]), line(&[(78.0, 30.0), (30.0, 100.0)])]),
            ("z", vec![line(&[(22.0, 35.0), (78.0, 35.0), (22.0, 95.0), (78.0, 95.0)])]),
            ("+", vec![line(&[(50.0, 15.0), (50.0, 85.0)]), line(&[(15.0, 50.0), (85.0, 50.0)])]),
            ("(", vec![arc(75.0, 50.0, 35.0, 50.0, 240.0, 120.0)]),
            (")", vec![arc(25.0, 50.0, 35.0, 50.0, -60.0, 60.0)]),
            ("<", vec![line(&[(80.0, 15.0), (20.0, 50.0), (80.0, 85.0)])]),
            (">", vec![line(&[(20.0, 15.0), (80.0, 50.0), (20.0, 85.0)])]),
            ("/", vec![line(&[(75.0, 5.0), (25.0, 95.0)])]),
            ("∫", vec![line(&[(70.0, 8.0), (60.0, 3.0), (52.0, 10.0), (48.0, 50.0), (44.0, 90.0), (36.0, 97.0), (28.0, 92.0)])]),
            ("π", vec![line(&[(15.0, 30.0), (85.0, 25.0)]), line(&[(35.0, 28.0), (32.0, 90.0)]), line(&[(65.0, 27.0), (68.0, 90.0)])]),
        ];
        defs.into_iter().map(|(sym, strokes)| Template { sym, cloud: normalize(&strokes) }).collect()
    })
}

/// The template symbol nearest `strokes`.
fn classify(strokes: &[Stroke]) -> &'static str {
    let cloud = normalize(strokes);
    templates().iter().map(|t| (greedy_match(&cloud, &t.cloud), t.sym)).min_by(|a, b| a.0.total_cmp(&b.0)).map_or("?", |(_, s)| s)
}

// ---------------------------------------------------------------------------------------------
// Structure

struct Ink<'a> {
    strokes: &'a [Stroke],
    bounds: Vec<Bounds>,
    /// The usual height of a symbol, for telling dots, bars and small (raised) symbols apart.
    em: f32,
}

impl Ink<'_> {
    fn flat(&self, i: usize) -> bool {
        let b = self.bounds[i];
        b.h() < 0.25 * b.w().max(1e-6) && b.w() > 0.35 * self.em
    }

    fn group_bounds(&self, g: &[usize]) -> Bounds {
        g.iter().map(|&i| self.bounds[i]).reduce(Bounds::union).unwrap_or_default()
    }

    /// Whether stroke `i` is a radical sign: down into a V on the left, up steeply, then right
    /// along the top.
    fn radical(&self, i: usize) -> bool {
        let s = &self.strokes[i];
        let b = self.bounds[i];
        if s.len() < 3 || b.w() < 0.6 * self.em || b.h() < 0.6 * self.em {
            return false;
        }
        let (Some(first), Some(last)) = (s.first(), s.last()) else { return false };
        let Some((low_i, low)) = s.iter().enumerate().max_by(|a, b| a.1[1].total_cmp(&b.1[1])) else { return false };
        let ends_top_right = last[0] > b.x0 + 0.7 * b.w() && last[1] < b.y0 + 0.25 * b.h();
        let v_on_left = low[0] < b.x0 + 0.4 * b.w() && low[1] > b.y0 + 0.75 * b.h();
        let starts_higher = first[1] < low[1] - 0.25 * b.h() && first[0] <= low[0] + 0.05 * b.w();
        // The top run: from where it reaches the top to the end, mostly horizontal.
        let top_from = s.iter().skip(low_i).position(|p| p[1] < b.y0 + 0.2 * b.h()).map(|k| low_i + k);
        let runs_right = top_from.and_then(|k| s.get(k)).is_some_and(|p| last[0] - p[0] > 0.3 * b.w());
        ends_top_right && v_on_left && starts_higher && runs_right
    }

    /// The linear text for strokes `ids`.
    fn expr(&self, ids: &[usize], depth: usize) -> String {
        if ids.is_empty() || depth > 12 {
            return String::new();
        }
        // A fraction bar: the widest flat stroke with ink both above and below it.
        let bar = ids
            .iter()
            .copied()
            .filter(|&i| self.flat(i) && self.bounds[i].w() > 0.8 * self.em)
            .filter(|&i| {
                let b = self.bounds[i];
                let within = |j: usize| j != i && self.bounds[j].cx() > b.x0 && self.bounds[j].cx() < b.x1;
                ids.iter().any(|&j| within(j) && self.bounds[j].y1 < b.cy() + 0.1 * self.em)
                    && ids.iter().any(|&j| within(j) && self.bounds[j].y0 > b.cy() - 0.1 * self.em)
                    && ids.iter().filter(|&&j| within(j)).any(|&j| !self.flat(j))
            })
            .max_by(|a, b| self.bounds[*a].w().total_cmp(&self.bounds[*b].w()));
        if let Some(bar) = bar {
            let b = self.bounds[bar];
            let (mut left, mut right, mut num, mut den) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            for &j in ids.iter().filter(|&&j| j != bar) {
                let c = self.bounds[j];
                if c.cx() < b.x0 {
                    left.push(j);
                } else if c.cx() > b.x1 {
                    right.push(j);
                } else if c.cy() < b.cy() {
                    num.push(j);
                } else {
                    den.push(j);
                }
            }
            let part = |g: &[usize]| {
                let t = self.expr(g, depth + 1);
                if t.chars().count() <= 1 || t.chars().all(|c| c.is_alphanumeric()) { t } else { format!("({t})") }
            };
            return format!("{}{}/{}{}", self.expr(&left, depth + 1), part(&num), part(&den), self.expr(&right, depth + 1));
        }
        // A square root: what is under it is its radicand.
        if let Some(r) = ids.iter().copied().filter(|&i| self.radical(i)).max_by(|a, b| self.bounds[*a].w().total_cmp(&self.bounds[*b].w())) {
            let b = self.bounds[r];
            let low_x = self.strokes[r].iter().max_by(|a, c| a[1].total_cmp(&c[1])).map_or(b.x0, |p| p[0]);
            let (mut left, mut inside, mut right) = (Vec::new(), Vec::new(), Vec::new());
            for &j in ids.iter().filter(|&&j| j != r) {
                let c = self.bounds[j];
                if c.cx() > low_x && c.cx() < b.x1 && c.cy() > b.y0 && c.cy() < b.y1 + 0.2 * self.em {
                    inside.push(j);
                } else if c.cx() <= b.cx() {
                    left.push(j);
                } else {
                    right.push(j);
                }
            }
            let under = self.expr(&inside, depth + 1);
            let under = if under.chars().count() <= 1 { under } else { format!("({under})") };
            return format!("{}√{under}{}", self.expr(&left, depth + 1), self.expr(&right, depth + 1));
        }
        self.row(ids)
    }

    /// Symbols left to right, with superscripts and subscripts.
    fn row(&self, ids: &[usize]) -> String {
        // Group strokes whose spans overlap into one symbol (the two strokes of `x`, `+`, `=`).
        let mut order: Vec<usize> = ids.to_vec();
        order.sort_by(|a, b| self.bounds[*a].x0.total_cmp(&self.bounds[*b].x0));
        let mut groups: Vec<Vec<usize>> = Vec::new();
        for i in order {
            let b = self.bounds[i];
            let merge = groups.last().is_some_and(|g| {
                let gb = self.group_bounds(g);
                let overlap = gb.x1.min(b.x1) - gb.x0.max(b.x0);
                let narrow = gb.w().min(b.w());
                // Spans that overlap by much of the narrower one (a vertical stroke inside a
                // horizontal one's span counts in full).
                overlap >= 0.0 && overlap >= 0.4 * narrow && (b.y0 < gb.y1 + 0.6 * self.em)
            });
            if merge {
                if let Some(g) = groups.last_mut() {
                    g.push(i);
                }
            } else {
                groups.push(vec![i]);
            }
        }
        let mut out = String::new();
        let mut base: Option<Bounds> = None;
        let mut script: Option<(char, String)> = None;
        let flush = |out: &mut String, script: &mut Option<(char, String)>| {
            if let Some((kind, text)) = script.take() {
                out.push(kind);
                if text.chars().count() > 1 {
                    out.push('(');
                    out.push_str(&text);
                    out.push(')');
                } else {
                    out.push_str(&text);
                }
            }
        };
        for g in &groups {
            let b = self.group_bounds(g);
            let sym = self.symbol(g, b);
            // Smaller than the symbol before it and raised (or lowered) past its middle; signs
            // that relate two sides are never scripts.
            let kind = match base {
                Some(bb) if matches!(sym, "=" | "<" | ">") || b.h() >= 0.8 * bb.h() || b.x0 <= bb.cx() => None,
                Some(bb) if b.cy() < bb.y0 + 0.4 * bb.h() && b.y1 < bb.y0 + 0.65 * bb.h() => Some('^'),
                Some(bb) if sym != "." && b.cy() > bb.y0 + 0.65 * bb.h() && b.y1 > bb.y1 + 0.1 * bb.h() => Some('_'),
                _ => None,
            };
            match (kind, &mut script) {
                (Some(k), Some((sk, text))) if *sk == k => text.push_str(sym),
                (Some(k), _) => {
                    flush(&mut out, &mut script);
                    script = Some((k, sym.to_string()));
                }
                (None, _) => {
                    flush(&mut out, &mut script);
                    out.push_str(sym);
                    if !matches!(sym, "." | "-" | "=" | "+" | "<" | ">") {
                        base = Some(b);
                    }
                }
            }
        }
        flush(&mut out, &mut script);
        out
    }

    fn symbol(&self, g: &[usize], b: Bounds) -> &'static str {
        if b.w().max(b.h()) < 0.22 * self.em {
            return ".";
        }
        if g.iter().all(|&i| self.flat(i)) {
            return if g.len() >= 2 { "=" } else { "-" };
        }
        let strokes: Vec<Stroke> = g.iter().map(|&i| self.strokes[i].clone()).collect();
        classify(&strokes)
    }
}

/// The UnicodeMath linear text for handwritten `strokes`, or why there is none.
pub fn recognize(strokes: &[Stroke]) -> Result<String, String> {
    let strokes: Vec<Stroke> = strokes
        .iter()
        .take(MAX_STROKES)
        .map(|s| {
            let step = s.len().div_ceil(MAX_POINTS).max(1);
            s.iter().step_by(step).copied().filter(|p| p[0].is_finite() && p[1].is_finite()).collect::<Stroke>()
        })
        .filter(|s| !s.is_empty())
        .collect();
    if strokes.is_empty() {
        return Err("there is no ink to recognise".into());
    }
    let bounds: Vec<Bounds> = strokes.iter().map(|s| Bounds::of(s.iter())).collect();
    // The usual symbol height: the median of the strokes' heights, flat ones left out.
    let mut heights: Vec<f32> = bounds.iter().filter(|b| b.h() >= 0.25 * b.w()).map(|b| b.h()).collect();
    heights.sort_by(f32::total_cmp);
    let em = heights.get(heights.len() / 2).copied().filter(|h| *h > 0.0).unwrap_or_else(|| bounds.iter().map(Bounds::w).fold(1.0, f32::max));
    let ink = Ink { strokes: &strokes, bounds, em };
    let ids: Vec<usize> = (0..strokes.len()).collect();
    let text = ink.expr(&ids, 0);
    if text.is_empty() { Err("no math was recognised".into()) } else { Ok(text) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `sym` as someone might write it, `size` high at (`x`, `y`), wobbled a little.
    fn written(sym: &str, x: f32, y: f32, size: f32) -> Vec<Stroke> {
        let strokes = shape(sym);
        let mut seed = (x * 7.0 + y * 13.0 + size) as u32 | 1;
        let mut wobble = || {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            (seed % 1000) as f32 / 1000.0 - 0.5
        };
        strokes
            .iter()
            .map(|s| s.iter().map(|p| [x + p[0] / 100.0 * size + wobble() * size * 0.04, y + p[1] / 100.0 * size + wobble() * size * 0.04]).collect())
            .collect()
    }

    /// The shapes people write, a little unlike the templates.
    fn shape(sym: &str) -> Vec<Stroke> {
        match sym {
            "x" => vec![line(&[(25.0, 30.0), (75.0, 95.0)]), line(&[(72.0, 32.0), (28.0, 92.0)])],
            "y" => vec![line(&[(25.0, 28.0), (52.0, 60.0)]), line(&[(76.0, 28.0), (32.0, 98.0)])],
            "2" => vec![join(&[arc(50.0, 30.0, 24.0, 24.0, 190.0, 385.0), line(&[(28.0, 94.0), (76.0, 96.0)])])],
            "3" => vec![join(&[arc(50.0, 28.0, 21.0, 22.0, 195.0, 445.0), arc(50.0, 72.0, 24.0, 22.0, 275.0, 495.0)])],
            "+" => vec![line(&[(52.0, 18.0), (50.0, 84.0)]), line(&[(18.0, 52.0), (84.0, 50.0)])],
            "1" => vec![line(&[(48.0, 6.0), (52.0, 94.0)])],
            "7" => vec![line(&[(22.0, 10.0), (78.0, 10.0), (45.0, 94.0)])],
            "a" => vec![join(&[arc(46.0, 66.0, 21.0, 24.0, -20.0, 330.0), line(&[(66.0, 46.0), (67.0, 92.0)])])],
            "b" => vec![join(&[line(&[(32.0, 6.0), (31.0, 94.0), (31.0, 60.0)]), arc(49.0, 74.0, 19.0, 20.0, 205.0, 515.0)])],
            "0" => vec![arc(50.0, 50.0, 28.0, 44.0, -80.0, 280.0)],
            "(" => vec![arc(72.0, 50.0, 32.0, 48.0, 235.0, 125.0)],
            ")" => vec![arc(28.0, 50.0, 32.0, 48.0, -55.0, 55.0)],
            "n" => vec![line(&[(26.0, 42.0), (26.0, 94.0), (26.0, 60.0), (40.0, 44.0), (58.0, 42.0), (70.0, 54.0), (71.0, 94.0)])],
            "π" => vec![line(&[(16.0, 32.0), (84.0, 27.0)]), line(&[(36.0, 30.0), (33.0, 88.0)]), line(&[(64.0, 29.0), (67.0, 88.0)])],
            _ => templates_strokes(sym),
        }
    }

    fn templates_strokes(sym: &str) -> Vec<Stroke> {
        match sym {
            "4" => vec![line(&[(62.0, 5.0), (18.0, 65.0), (85.0, 65.0)]), line(&[(62.0, 30.0), (62.0, 95.0)])],
            "5" => vec![join(&[line(&[(72.0, 6.0), (32.0, 6.0), (30.0, 45.0)]), arc(50.0, 68.0, 25.0, 25.0, 230.0, 500.0)])],
            "6" => vec![line(&[
                (68.0, 8.0),
                (50.0, 10.0),
                (36.0, 25.0),
                (30.0, 50.0),
                (32.0, 75.0),
                (45.0, 93.0),
                (60.0, 92.0),
                (70.0, 78.0),
                (66.0, 62.0),
                (52.0, 56.0),
                (38.0, 62.0),
                (31.0, 72.0),
            ])],
            "8" => vec![line(&[
                (50.0, 50.0),
                (30.0, 35.0),
                (32.0, 15.0),
                (50.0, 6.0),
                (68.0, 15.0),
                (70.0, 35.0),
                (50.0, 50.0),
                (28.0, 68.0),
                (30.0, 88.0),
                (50.0, 96.0),
                (70.0, 88.0),
                (72.0, 68.0),
                (50.0, 50.0),
            ])],
            "9" => vec![join(&[arc(50.0, 30.0, 22.0, 22.0, 0.0, 360.0), line(&[(72.0, 30.0), (68.0, 95.0)])])],
            "c" => vec![arc(52.0, 65.0, 25.0, 28.0, 320.0, 40.0)],
            "z" => vec![line(&[(22.0, 35.0), (78.0, 35.0), (22.0, 95.0), (78.0, 95.0)])],
            "<" => vec![line(&[(80.0, 15.0), (20.0, 50.0), (80.0, 85.0)])],
            ">" => vec![line(&[(20.0, 15.0), (80.0, 50.0), (20.0, 85.0)])],
            "/" => vec![line(&[(75.0, 5.0), (25.0, 95.0)])],
            "∫" => vec![line(&[(70.0, 8.0), (60.0, 3.0), (52.0, 10.0), (48.0, 50.0), (44.0, 90.0), (36.0, 97.0), (28.0, 92.0)])],
            other => panic!("no test shape for {other}"),
        }
    }

    #[test]
    fn every_symbol_is_recognised_when_written_a_little_differently() {
        let symbols = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "n", "x", "y", "z", "+", "(", ")", "<", ">", "/", "∫", "π"];
        let mut wrong = Vec::new();
        for (k, sym) in symbols.iter().enumerate() {
            for size in [30.0, 80.0] {
                let strokes = written(sym, 10.0 * k as f32, 5.0, size);
                let got = classify(&strokes);
                if got != *sym {
                    wrong.push(format!("{sym} at {size} → {got}"));
                }
            }
        }
        assert!(wrong.is_empty(), "{wrong:?}");
    }

    fn place(parts: &[(&str, f32, f32, f32)]) -> Vec<Stroke> {
        parts.iter().flat_map(|&(sym, x, y, size)| written(sym, x, y, size)).collect()
    }

    #[test]
    fn rows_scripts_fractions_and_roots_are_laid_out() {
        // x² + y² = z²: the twos are small and raised.
        let mut ink =
            place(&[("x", 0.0, 20.0, 40.0), ("2", 42.0, 5.0, 20.0), ("+", 75.0, 20.0, 40.0), ("y", 125.0, 20.0, 40.0), ("2", 167.0, 5.0, 20.0)]);
        ink.push(line(&[(200.0, 35.0), (240.0, 35.0)]));
        ink.push(line(&[(200.0, 48.0), (240.0, 48.0)]));
        ink.extend(place(&[("z", 255.0, 20.0, 40.0), ("2", 297.0, 5.0, 20.0)]));
        assert_eq!(recognize(&ink).unwrap(), "x^2+y^2=z^2");
        // a over b+1, then minus 3.
        let mut ink = place(&[("a", 30.0, 0.0, 40.0)]);
        ink.push(line(&[(0.0, 50.0), (110.0, 50.0)]));
        ink.extend(place(&[("b", 0.0, 60.0, 40.0), ("+", 38.0, 60.0, 35.0), ("1", 75.0, 60.0, 40.0)]));
        ink.push(line(&[(125.0, 50.0), (150.0, 50.0)]));
        ink.extend(place(&[("3", 160.0, 30.0, 40.0)]));
        assert_eq!(recognize(&ink).unwrap(), "a/(b+1)-3");
        // A subscript.
        let ink = place(&[("x", 0.0, 0.0, 40.0), ("1", 42.0, 30.0, 20.0)]);
        assert_eq!(recognize(&ink).unwrap(), "x_1");
        // The square root of x+1.
        let mut ink = vec![line(&[(0.0, 40.0), (10.0, 35.0), (22.0, 70.0), (35.0, 0.0), (140.0, 0.0)])];
        ink.extend(place(&[("x", 40.0, 15.0, 40.0), ("+", 82.0, 15.0, 35.0), ("1", 120.0, 15.0, 40.0)]));
        assert_eq!(recognize(&ink).unwrap(), "√(x+1)");
        // A decimal point.
        let mut ink = place(&[("3", 0.0, 0.0, 40.0)]);
        ink.push(line(&[(45.0, 38.0), (46.0, 39.0)]));
        ink.extend(place(&[("1", 55.0, 0.0, 40.0)]));
        assert_eq!(recognize(&ink).unwrap(), "3.1");
    }

    #[test]
    fn hostile_ink_is_handled() {
        assert!(recognize(&[]).is_err());
        assert!(recognize(&[vec![]]).is_err());
        assert!(recognize(&[vec![[f32::NAN, 1.0]]]).is_err());
        // One point, a huge stroke, many strokes: an answer or a reason, never a panic.
        let _ = recognize(&[vec![[5.0, 5.0]]]);
        let big: Stroke = (0..100_000).map(|i| [i as f32, (i % 7) as f32]).collect();
        let _ = recognize(&[big]);
        let many: Vec<Stroke> = (0..1000).map(|i| vec![[i as f32 * 3.0, 0.0], [i as f32 * 3.0 + 2.0, 10.0]]).collect();
        let _ = recognize(&many);
    }
}
