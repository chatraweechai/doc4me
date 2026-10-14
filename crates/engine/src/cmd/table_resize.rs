//! Resizing a table with the mouse, like Word: drag a border between columns (or the table's
//! right edge), a row's bottom border, or the small handle just outside the table's bottom-right
//! corner to scale the whole table. The geometry comes from the layout (what is on the page, in
//! page points), so the result is what the drag showed even when autofit or a percentage width
//! made the drawn columns differ from the stored grid. The app sends each finished drag here,
//! so every drag is one undo step.

use serde_json::Value;
use wordcraft_doc::props::HeightRule;
use wordcraft_doc::{Document, Path, StoryRef, Table};
use wordcraft_geom::Rect;
use wordcraft_layout::{DocLayout, Placed};

use super::sel_result;
use crate::{CmdError, CmdResult, CommandSpec, Session, p};

/// How near (points) a border has to be to the pointer, by default.
pub const REACH: f32 = 3.0;
/// The corner handle's side (points); it sits just outside the table's bottom-right corner.
pub const HANDLE: f32 = 6.0;
/// The narrowest column a drag leaves (points).
pub const MIN_COL: f32 = 6.0;
/// The lowest row a drag leaves (points).
pub const MIN_ROW: f32 = 4.0;
/// The largest column width or row height a drag sets (points; Word's 22 inches).
const MAX_LEN: f32 = 1584.0;

pub fn specs() -> Vec<CommandSpec> {
    vec![CommandSpec::new("table.resizeDrag", "Resize Table by Dragging", "Table Layout › Cell Size", resize_drag).params(
        r#"{"page": n, "x": pt, "y": pt, "toX": pt, "toY": pt, "shift"?: bool, "reach"?: pt} (a mouse drag in page points, 0-based page, starting on a table border or the resize handle: a column border moves (the column to its right gives or takes the difference, so the table keeps its width; with shift, or on the table's right edge, only the column left of it changes); a row's bottom border sets that row's height; the handle just outside the bottom-right corner scales every column and the rows on that page. `reach` (default 3) is how near the start has to be.)"#,
    )]
}

/// What a press on the page would grab.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    /// The vertical border at grid line `line` (1 = between the first two columns; the last is
    /// the table's right edge).
    Column(usize),
    /// The bottom border of row `row`.
    Row(usize),
    /// The resize handle at the bottom-right corner.
    Corner,
}

/// A row of the table as drawn on one page.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawnRow {
    pub row: usize,
    pub top: f32,
    pub bottom: f32,
    /// Grid lines that are cell borders in this row (a merged cell hides the ones it covers).
    edges: Vec<usize>,
}

/// A table border (or the handle) under the pointer, with the table's drawn geometry.
#[derive(Clone, Debug, PartialEq)]
pub struct ResizeTarget {
    pub edge: Edge,
    pub story: StoryRef,
    pub table: Path,
    /// Drawn grid lines, page x, left to right (columns + 1 of them).
    pub cols: Vec<f32>,
    /// Drawn rows on this page, top to bottom.
    pub rows: Vec<DrawnRow>,
    /// Whether the table's last row is on this page (the handle is shown only then).
    pub ends_here: bool,
}

/// What a drag to a point would make of the table.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plan {
    /// New column widths (points), when the columns change.
    pub widths: Option<Vec<f32>>,
    /// New heights (points) of these rows.
    pub heights: Vec<(usize, f32)>,
}

impl ResizeTarget {
    pub fn left(&self) -> f32 {
        self.cols.first().copied().unwrap_or(0.0)
    }
    pub fn right(&self) -> f32 {
        self.cols.last().copied().unwrap_or(0.0)
    }
    pub fn top(&self) -> f32 {
        self.rows.first().map_or(0.0, |r| r.top)
    }
    pub fn bottom(&self) -> f32 {
        self.rows.last().map_or(0.0, |r| r.bottom)
    }
    /// The drawn column widths.
    pub fn widths(&self) -> Vec<f32> {
        self.cols.windows(2).map(|w| (w[1] - w[0]).max(0.0)).collect()
    }
    /// The resize handle's square (page points).
    pub fn handle(&self) -> Rect {
        Rect::new(self.right() + 1.0, self.bottom() + 1.0, HANDLE, HANDLE)
    }

    /// The new widths and heights a drag that moved the pointer by (`dx`, `dy`) points gives.
    pub fn plan(&self, dx: f32, dy: f32, shift: bool) -> Plan {
        let (dx, dy) =
            (if dx.is_finite() { dx.clamp(-MAX_LEN, MAX_LEN) } else { 0.0 }, if dy.is_finite() { dy.clamp(-MAX_LEN, MAX_LEN) } else { 0.0 });
        let mut w = self.widths();
        match self.edge {
            Edge::Column(line) => {
                let Some(left) = line.checked_sub(1).and_then(|i| w.get(i).copied()) else { return Plan::default() };
                let right = w.get(line).copied();
                let (new_left, new_right) = match right {
                    // An inner border: the neighbour on the right makes up the difference.
                    Some(right) if !shift => {
                        let d = dx.clamp(MIN_COL - left, (right - MIN_COL).max(MIN_COL - left));
                        (left + d, Some(right - d))
                    }
                    // The right edge (or Shift): only the column on the left changes.
                    _ => ((left + dx).clamp(MIN_COL, MAX_LEN), None),
                };
                if let Some(x) = w.get_mut(line - 1) {
                    *x = new_left;
                }
                if let (Some(r), Some(x)) = (new_right, w.get_mut(line)) {
                    *x = r;
                }
                Plan { widths: Some(w), heights: Vec::new() }
            }
            Edge::Row(row) => {
                let Some(r) = self.rows.iter().find(|r| r.row == row) else { return Plan::default() };
                Plan { widths: None, heights: vec![(row, (r.bottom - r.top + dy).clamp(MIN_ROW, MAX_LEN))] }
            }
            Edge::Corner => {
                let (width, height) = (self.right() - self.left(), self.bottom() - self.top());
                let kx = if width > 1.0 { ((width + dx) / width).clamp(0.01, 100.0) } else { 1.0 };
                let ky = if height > 1.0 { ((height + dy) / height).clamp(0.01, 100.0) } else { 1.0 };
                let widths = ((kx - 1.0).abs() > 0.001).then(|| w.iter().map(|x| (x * kx).clamp(MIN_COL, MAX_LEN)).collect());
                let heights = if (ky - 1.0).abs() > 0.001 {
                    self.rows.iter().map(|r| (r.row, ((r.bottom - r.top) * ky).clamp(MIN_ROW, MAX_LEN))).collect()
                } else {
                    Vec::new()
                };
                Plan { widths, heights }
            }
        }
    }
}

/// A table cell's area on a page.
struct CellBox {
    rect: Rect,
    table: Path,
    story: StoryRef,
    row: usize,
    cell: usize,
}

fn cell_boxes(l: &DocLayout, page: usize) -> Vec<CellBox> {
    let Some(pg) = l.pages.get(page) else { return Vec::new() };
    pg.items
        .iter()
        .filter_map(|it| match it {
            Placed::Cell { rect, table, row, cell, story, .. } => {
                Some(CellBox { rect: *rect, table: table.clone(), story: *story, row: *row, cell: *cell })
            }
            _ => None,
        })
        .collect()
}

/// The drawn geometry of one table on a page, from its cells' boxes.
fn geometry(t: &Table, boxes: &[&CellBox]) -> Option<(Vec<f32>, Vec<DrawnRow>)> {
    let n = t.cols();
    if n == 0 || n > wordcraft_doc::table::MAX_COLS || boxes.is_empty() {
        return None;
    }
    let mut lines: Vec<Option<f32>> = vec![None; n + 1];
    let mut rows: Vec<DrawnRow> = Vec::new();
    for b in boxes {
        let g = t.grid_col(b.row, b.cell);
        let span = t.rows.get(b.row).and_then(|r| r.cells.get(b.cell)).map_or(1, wordcraft_doc::Cell::span);
        let end = g.saturating_add(span).min(n);
        if let Some(x) = lines.get_mut(g) {
            x.get_or_insert(b.rect.x);
        }
        if let Some(x) = lines.get_mut(end) {
            x.get_or_insert(b.rect.right());
        }
        match rows.iter_mut().find(|r| r.row == b.row) {
            Some(r) => {
                r.top = r.top.min(b.rect.y);
                r.bottom = r.bottom.max(b.rect.bottom());
                r.edges.extend([g, end]);
            }
            None => rows.push(DrawnRow { row: b.row, top: b.rect.y, bottom: b.rect.bottom(), edges: vec![g, end] }),
        }
    }
    // A line no cell on this page ends at: between its drawn neighbours, as the grid shares it.
    let (first, last) = (lines.first().copied().flatten()?, lines.last().copied().flatten()?);
    let weight = |i: usize| t.grid.get(i).copied().filter(|w| t.grid.len() == n && w.is_finite() && *w > 0.0).unwrap_or(1.0);
    let mut cols = Vec::with_capacity(n + 1);
    let mut prev = (0usize, first);
    for i in 0..=n {
        let x = match lines.get(i).copied().flatten() {
            Some(x) => x,
            None => {
                let next = (i + 1..=n).find_map(|j| lines.get(j).copied().flatten().map(|x| (j, x))).unwrap_or((n, last));
                let span: f32 = (prev.0..next.0).map(weight).sum();
                let part: f32 = (prev.0..i).map(weight).sum();
                prev.1 + (next.1 - prev.1) * if span > 0.0 { part / span } else { 0.0 }
            }
        };
        let x = x.max(cols.last().copied().unwrap_or(x));
        if lines.get(i).copied().flatten().is_some() {
            prev = (i, x);
        }
        cols.push(x);
    }
    rows.sort_by_key(|r| r.row);
    Some((cols, rows))
}

/// Every table drawn on a page, with its geometry there (`edge` is a placeholder).
fn tables_on(l: &DocLayout, doc: &Document, page: usize) -> Vec<ResizeTarget> {
    let boxes = cell_boxes(l, page);
    let mut tables: Vec<(StoryRef, &Path)> = Vec::new();
    for b in &boxes {
        if !tables.iter().any(|(s, p)| *s == b.story && **p == b.table) {
            tables.push((b.story, &b.table));
        }
    }
    tables
        .into_iter()
        .filter_map(|(story, path)| {
            let t = doc.table(story, path)?;
            let mine: Vec<&CellBox> = boxes.iter().filter(|b| b.story == story && b.table == *path).collect();
            let (cols, rows) = geometry(t, &mine)?;
            let ends_here = rows.last().is_some_and(|r| r.row + 1 == t.rows.len());
            Some(ResizeTarget { edge: Edge::Corner, story, table: path.clone(), cols, rows, ends_here })
        })
        .collect()
}

fn on_handle(t: &ResizeTarget, x: f32, y: f32, reach: f32) -> bool {
    let h = t.handle();
    t.ends_here && x >= h.x - reach / 2.0 && x <= h.right() + reach / 2.0 && y >= h.y - reach / 2.0 && y <= h.bottom() + reach / 2.0
}

/// The innermost table under page point (`x`, `y`) whose resize handle is on this page, or whose
/// handle is under the point: the one whose handle the app shows.
pub fn table_under(l: &DocLayout, doc: &Document, page: usize, x: f32, y: f32, reach: f32) -> Option<ResizeTarget> {
    if !x.is_finite() || !y.is_finite() {
        return None;
    }
    let reach = if reach.is_finite() { reach.clamp(0.5, 24.0) } else { REACH };
    tables_on(l, doc, page)
        .into_iter()
        .filter(|t| t.ends_here && ((x >= t.left() && x <= t.right() && y >= t.top() && y <= t.bottom()) || on_handle(t, x, y, reach)))
        .max_by_key(|t| t.table.0.len())
}

/// The table border or resize handle within `reach` points of page point (`x`, `y`): the
/// nearest, and of nested tables the innermost.
pub fn resize_target(l: &DocLayout, doc: &Document, page: usize, x: f32, y: f32, reach: f32) -> Option<ResizeTarget> {
    if !x.is_finite() || !y.is_finite() {
        return None;
    }
    let reach = if reach.is_finite() { reach.clamp(0.5, 24.0) } else { REACH };
    let mut best: Option<(f32, usize, ResizeTarget)> = None;
    for mut target in tables_on(l, doc, page) {
        let depth = target.table.0.len();
        let mut offer = |d: f32, edge: Edge, target: &mut ResizeTarget| {
            if d <= reach && best.as_ref().is_none_or(|(bd, bdepth, _)| d < *bd - 0.01 || (d <= *bd + 0.01 && depth > *bdepth)) {
                target.edge = edge;
                best = Some((d, depth, target.clone()));
            }
        };
        if on_handle(&target, x, y, reach) {
            offer(0.0, Edge::Corner, &mut target);
        }
        let (left, right, top, bottom) = (target.left(), target.right(), target.top(), target.bottom());
        if x < left - reach || x > right + reach || y < top - reach || y > bottom + reach {
            continue;
        }
        // Column borders: in the row under the pointer, only lines that are cell borders there.
        let row_here = target.rows.iter().find(|r| y >= r.top && y <= r.bottom).cloned();
        if let Some(r) = &row_here {
            for line in 1..target.cols.len() {
                if r.edges.contains(&line)
                    && let Some(cx) = target.cols.get(line).copied()
                {
                    offer((x - cx).abs(), Edge::Column(line), &mut target);
                }
            }
        }
        // Row borders: the bottom of each drawn row.
        if x >= left && x <= right {
            for r in target.rows.clone() {
                offer((y - r.bottom).abs(), Edge::Row(r.row), &mut target);
            }
        }
    }
    best.map(|b| b.2)
}

fn resize_drag(s: &mut Session, v: &Value) -> CmdResult {
    let page =
        p::u64(v, "page").map(|n| usize::try_from(n).unwrap_or(usize::MAX)).ok_or_else(|| CmdError::Params("`page` (number) is required".into()))?;
    let (x, y) = (p::req_f32(v, "x")?, p::req_f32(v, "y")?);
    let (to_x, to_y) = (p::req_f32(v, "toX")?, p::req_f32(v, "toY")?);
    let shift = p::bool(v, "shift").unwrap_or(false);
    let reach = p::f32(v, "reach").unwrap_or(REACH);
    let l = s.layout();
    if l.pages.get(page).is_none() {
        return Err(CmdError::Params(format!("no page {page}")));
    }
    let target =
        resize_target(&l, &s.doc, page, x, y, reach).ok_or_else(|| CmdError::Failed("there's no table border or resize handle there".into()))?;
    let plan = target.plan(to_x - x, to_y - y, shift);
    if plan.widths.is_none() && plan.heights.is_empty() {
        return sel_result(s);
    }
    let t = s.doc.table_mut(target.story, &target.table)?;
    if let Some(widths) = plan.widths
        && widths.len() == t.cols()
    {
        t.grid = widths;
        // The drag's widths are the table's now: no preferred table width scales them again.
        t.props.width = None;
        t.props.width_pct = None;
        super::table::sync_cell_widths(t);
    }
    for (r, h) in &plan.heights {
        if let Some(row) = t.rows.get_mut(*r) {
            row.props.height = Some(*h);
            if row.props.height_rule != HeightRule::Exact {
                row.props.height_rule = HeightRule::AtLeast;
            }
        }
    }
    touch(s, &target)?;
    if plan.heights.is_empty() {
        return sel_result(s);
    }
    // An "at least" height leaves out the cells' margins and border bands, which the layout adds
    // (and which depend on the table style): measure what the rows came to and take the extra
    // off once. A row its text holds taller keeps the dragged height.
    let drawn = drawn_heights(&s.layout(), page, &target);
    let mut fixed = Vec::new();
    let t = s.doc.table_mut(target.story, &target.table)?;
    for (r, want) in &plan.heights {
        let Some(got) = drawn.iter().find(|(i, _)| i == r).map(|(_, h)| *h) else { continue };
        if let Some(row) = t.rows.get_mut(*r)
            && row.props.height_rule == HeightRule::AtLeast
            && got > want + 0.05
        {
            row.props.height = Some((want - (got - want)).max(1.0));
            fixed.push((*r, *want, got));
        }
    }
    if fixed.is_empty() {
        return sel_result(s);
    }
    touch(s, &target)?;
    let again = drawn_heights(&s.layout(), page, &target);
    let t = s.doc.table_mut(target.story, &target.table)?;
    for (r, want, got) in fixed {
        let now = again.iter().find(|(i, _)| *i == r).map_or(got, |(_, h)| *h);
        if now >= got - 0.05
            && let Some(row) = t.rows.get_mut(r)
        {
            row.props.height = Some(want);
        }
    }
    touch(s, &target)?;
    sel_result(s)
}

/// The drawn height of each of the target table's rows on `page`.
fn drawn_heights(l: &DocLayout, page: usize, target: &ResizeTarget) -> Vec<(usize, f32)> {
    let mut out: Vec<(usize, f32)> = Vec::new();
    for b in cell_boxes(l, page).iter().filter(|b| b.story == target.story && b.table == target.table) {
        match out.iter_mut().find(|(r, _)| *r == b.row) {
            Some((_, h)) => *h = h.max(b.rect.h),
            None => out.push((b.row, b.rect.h)),
        }
    }
    out
}

/// Mark the table's paragraphs changed, so the layout redoes them.
fn touch(s: &mut Session, target: &ResizeTarget) -> Result<(), CmdError> {
    let story = target.story;
    let paths: Vec<Path> =
        s.doc.para_paths(story).into_iter().filter(|q| q.0.len() > target.table.0.len() && q.0.starts_with(&target.table.0)).collect();
    for q in paths {
        s.doc.para_mut(story, &q)?.touch();
    }
    // A new document revision: the next `layout()` lays the change out instead of reusing the
    // cached pages.
    s.touch();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn first_table(s: &Session) -> Table {
        s.doc.body.iter().find_map(|b| b.as_table()).cloned().unwrap()
    }

    fn cell_rect(s: &mut Session, row: usize, cell: usize) -> Rect {
        s.layout().pages[0]
            .items
            .iter()
            .find_map(|it| match it {
                Placed::Cell { rect, row: r, cell: c, .. } if *r == row && *c == cell => Some(*rect),
                _ => None,
            })
            .unwrap()
    }

    fn table_session(rows: usize, cols: usize) -> Session {
        let mut s = Session::new(wordcraft_doc::Document::new());
        s.run("insert.table", &json!({"rows": rows, "cols": cols})).unwrap();
        s
    }

    /// Dragging the border between two columns moves it; the right neighbour gives up the room,
    /// so the table keeps its width; one undo puts it back.
    #[test]
    fn a_column_border_drag_trades_width_with_the_neighbour() {
        let mut s = table_session(2, 3);
        let start = first_table(&s);
        let a = cell_rect(&mut s, 0, 0);
        let (b0, total0) = (cell_rect(&mut s, 0, 1), cell_rect(&mut s, 0, 2).right() - a.x);
        let y = a.y + a.h / 2.0;
        s.run("table.resizeDrag", &json!({"page": 0, "x": a.right() + 1.0, "y": y, "toX": a.right() + 31.0, "toY": y})).unwrap();
        let (a1, b1) = (cell_rect(&mut s, 0, 0), cell_rect(&mut s, 0, 1));
        assert!((a1.w - (a.w + 30.0)).abs() < 1.0, "{a:?} → {a1:?}");
        assert!((b1.w - (b0.w - 30.0)).abs() < 1.0, "{b0:?} → {b1:?}");
        let total1 = cell_rect(&mut s, 0, 2).right() - a1.x;
        assert!((total1 - total0).abs() < 1.0, "the table keeps its width");
        // The second row follows: one border for the whole column.
        assert!((cell_rect(&mut s, 1, 0).w - a1.w).abs() < 0.5);
        s.run("edit.undo", &json!({})).unwrap();
        assert_eq!(first_table(&s), start, "one undo step");
    }

    /// With Shift only the left column changes and the table grows; the right edge does the same.
    #[test]
    fn shift_and_the_right_edge_change_the_table_width() {
        let mut s = table_session(1, 2);
        let (a, b) = (cell_rect(&mut s, 0, 0), cell_rect(&mut s, 0, 1));
        let y = a.y + a.h / 2.0;
        s.run("table.resizeDrag", &json!({"page": 0, "x": a.right(), "y": y, "toX": a.right() - 40.0, "toY": y, "shift": true})).unwrap();
        let (a1, b1) = (cell_rect(&mut s, 0, 0), cell_rect(&mut s, 0, 1));
        assert!((a1.w - (a.w - 40.0)).abs() < 1.0 && (b1.w - b.w).abs() < 1.0, "{a1:?} {b1:?}");
        s.run("table.resizeDrag", &json!({"page": 0, "x": b1.right() - 1.0, "y": y, "toX": b1.right() - 51.0, "toY": y})).unwrap();
        let b2 = cell_rect(&mut s, 0, 1);
        assert!((b2.w - (b1.w - 50.0)).abs() < 1.0, "{b1:?} → {b2:?}");
    }

    /// No column gets narrower than the minimum, however far the drag goes.
    #[test]
    fn column_drags_stop_at_the_minimum_width() {
        let mut s = table_session(1, 2);
        let (a, b) = (cell_rect(&mut s, 0, 0), cell_rect(&mut s, 0, 1));
        let y = a.y + a.h / 2.0;
        s.run("table.resizeDrag", &json!({"page": 0, "x": a.right(), "y": y, "toX": a.right() + 5000.0, "toY": y})).unwrap();
        let t = first_table(&s);
        assert!((t.grid[1] - MIN_COL).abs() < 0.01, "{:?}", t.grid);
        assert!((t.grid[0] + t.grid[1] - (a.w + b.w)).abs() < 1.0);
    }

    /// Dragging a row's bottom border sets its height ("at least"), and the row grows.
    #[test]
    fn a_row_border_drag_sets_the_row_height() {
        let mut s = table_session(2, 2);
        let a = cell_rect(&mut s, 0, 0);
        let x = a.x + a.w / 2.0;
        s.run("table.resizeDrag", &json!({"page": 0, "x": x, "y": a.bottom() + 1.0, "toX": x, "toY": a.bottom() + 41.0})).unwrap();
        let t = first_table(&s);
        assert_eq!(t.rows[0].props.height_rule, HeightRule::AtLeast);
        // The stored height leaves out the margins and borders; the drawn row is what was dragged.
        assert!(t.rows[0].props.height.unwrap() < a.h + 40.0, "{:?}", t.rows[0].props);
        let h = cell_rect(&mut s, 0, 0).h;
        assert!((h - (a.h + 40.0)).abs() < 0.1, "{} → {h}", a.h);
        assert_eq!(t.rows[1].props.height, None, "the other row is untouched");
    }

    /// The handle past the bottom-right corner scales every column and row.
    #[test]
    fn the_corner_handle_scales_the_table() {
        let mut s = table_session(2, 2);
        let (a, d) = (cell_rect(&mut s, 0, 0), cell_rect(&mut s, 1, 1));
        let (w, h) = (d.right() - a.x, d.bottom() - a.y);
        let (hx, hy) = (d.right() + 1.0 + HANDLE / 2.0, d.bottom() + 1.0 + HANDLE / 2.0);
        let l = s.layout();
        let tgt = resize_target(&l, &s.doc, 0, hx, hy, REACH).unwrap();
        assert_eq!(tgt.edge, Edge::Corner);
        // Half as wide, twice as tall: the handle moves by that much.
        s.run("table.resizeDrag", &json!({"page": 0, "x": hx, "y": hy, "toX": hx - w * 0.5, "toY": hy + h})).unwrap();
        let (a1, d1) = (cell_rect(&mut s, 0, 0), cell_rect(&mut s, 1, 1));
        assert!(((d1.right() - a1.x) - w * 0.5).abs() < 1.5, "{w} → {}", d1.right() - a1.x);
        assert!(((d1.bottom() - a1.y) - h * 2.0).abs() < 0.5, "{h} → {}", d1.bottom() - a1.y);
    }

    /// A merged cell hides the border it covers: no column border to grab in its middle.
    #[test]
    fn merged_cells_have_no_border_inside() {
        let mut s = table_session(2, 2);
        let a = cell_rect(&mut s, 0, 0);
        s.run("table.eraser", &json!({"page": 0, "x": a.right(), "y": a.y + a.h / 2.0})).unwrap();
        let l = s.layout();
        let m = cell_rect(&mut s, 0, 0);
        let mid = resize_target(&l, &s.doc, 0, a.right(), m.y + m.h / 2.0, REACH);
        assert!(mid.as_ref().is_none_or(|t| t.edge != Edge::Column(1)), "{mid:?}");
        // The second row still has it.
        let b = cell_rect(&mut s, 1, 0);
        let t = resize_target(&l, &s.doc, 0, b.right(), b.y + b.h / 2.0, REACH).unwrap();
        assert_eq!(t.edge, Edge::Column(1));
    }

    /// Away from any border, or with hostile numbers: an error and no change, never a panic.
    #[test]
    fn misses_and_hostile_params_change_nothing() {
        let mut s = table_session(2, 2);
        let start = first_table(&s);
        let c = cell_rect(&mut s, 0, 0);
        let mid = json!({"page": 0, "x": c.x + c.w / 2.0, "y": c.y + c.h / 2.0, "toX": 10, "toY": 10});
        assert!(s.run("table.resizeDrag", &mid).is_err());
        assert!(s.run("table.resizeDrag", &json!({"page": 0, "x": 1e30, "y": -1e30, "toX": 1e30, "toY": 1e30})).is_err());
        assert!(s.run("table.resizeDrag", &json!({"page": 99, "x": 1, "y": 1, "toX": 1, "toY": 1})).is_err());
        assert!(s.run("table.resizeDrag", &json!({})).is_err());
        let y = c.y + c.h / 2.0;
        s.run("table.resizeDrag", &json!({"page": 0, "x": c.right(), "y": y, "toX": f32::MAX, "toY": y, "reach": 1e9})).unwrap();
        assert!(first_table(&s).grid.iter().all(|w| w.is_finite() && *w >= MIN_COL));
        s.run("edit.undo", &json!({})).unwrap();
        assert_eq!(first_table(&s), start);
    }
}
