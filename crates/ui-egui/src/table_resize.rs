//! Resizing tables with the mouse, like Word: over a column border the pointer turns into a
//! column-resize arrow, over a row's bottom border a row-resize arrow; over a table a small
//! handle shows past its bottom-right corner. Dragging shows a dashed preview of the new border
//! (or outline); releasing runs `table.resizeDrag` with page coordinates, one undo step. Shift
//! while releasing a column border changes only the column on its left. The geometry decisions
//! are the engine's (`cmd::table_resize`).

use egui::{Pos2, Rect, Stroke, Ui, pos2};
use serde_json::json;
use wordcraft_engine::cmd::table_resize::{self, Edge, ResizeTarget};
use wordcraft_layout::DocLayout;

use crate::WordApp;
use crate::theme::Tokens;

/// How near (screen pixels) the pointer has to be to a border to grab it.
const REACH_PX: f32 = 4.0;

/// A table border (or the handle) being dragged.
#[derive(Clone, Debug)]
pub(crate) struct ResizeDrag {
    page: usize,
    from: (f32, f32),
    to: (f32, f32),
    target: ResizeTarget,
}

/// Screen pixels per page point on `page`.
fn page_scale(rects: &[Rect], layout: &DocLayout, scale: f32, page: usize) -> Option<f32> {
    let r = rects.get(page)?;
    let sc = layout.pages.get(page).filter(|pg| pg.w > 0.0).map_or(scale, |pg| r.width() / pg.w);
    (sc > 0.0 && sc.is_finite()).then_some(sc)
}

fn to_screen(rects: &[Rect], layout: &DocLayout, scale: f32, page: usize, x: f32, y: f32) -> Option<Pos2> {
    let r = rects.get(page)?;
    let sc = page_scale(rects, layout, scale, page)?;
    Some(pos2(r.min.x + x * sc, r.min.y + y * sc))
}

/// Whether table resizing is in reach: not while a pen, the eraser, an object drag or a text
/// drag owns the pointer, nor while editing a header or footer from the body (or back).
fn available(app: &WordApp) -> bool {
    app.canvas.table_tool.is_none()
        && app.canvas.ink.is_none()
        && app.canvas.obj_drag.is_none()
        && !app.canvas.drag_selecting()
        && crate::ink::cursor(app).is_none()
}

/// The border or handle under screen point `p`, if any.
fn target_at(app: &WordApp, layout: &DocLayout, rects: &[Rect], scale: f32, p: Pos2) -> Option<(usize, f32, f32, ResizeTarget)> {
    let (page, x, y) = crate::canvas::page_at(rects, layout, scale, p)?;
    let sc = page_scale(rects, layout, scale, page)?;
    let t = table_resize::resize_target(layout, &app.session.doc, page, x, y, REACH_PX / sc)?;
    Some((page, x, y, t))
}

/// Mouse input: a press on a border starts a drag, moving updates it, releasing applies it.
/// `false` (nothing done) when the pointer isn't on a border, so the caret logic runs.
pub fn pointer(app: &mut WordApp, ui: &Ui, resp: &egui::Response, rects: &[Rect], layout: &DocLayout, scale: f32) -> bool {
    let latest = ui.input(|i| i.pointer.latest_pos());
    if let Some(mut d) = app.canvas.table_resize.take() {
        if let Some(p) = latest
            && let Some(r) = rects.get(d.page)
            && let Some(sc) = page_scale(rects, layout, scale, d.page)
        {
            d.to = ((p.x - r.min.x) / sc, (p.y - r.min.y) / sc);
        }
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            return true;
        }
        if ui.input(|i| i.pointer.primary_down()) {
            app.canvas.table_resize = Some(d);
            return true;
        }
        // A click without a move changes nothing.
        if (d.to.0 - d.from.0).abs() < 0.5 && (d.to.1 - d.from.1).abs() < 0.5 {
            return true;
        }
        let shift = ui.input(|i| i.modifiers.shift);
        let sc = page_scale(rects, layout, scale, d.page).unwrap_or(1.0);
        let params = json!({
            "page": d.page, "x": d.from.0, "y": d.from.1, "toX": d.to.0, "toY": d.to.1,
            "shift": shift, "reach": REACH_PX / sc,
        });
        if let Err(e) = app.run("table.resizeDrag", params) {
            app.status(e);
        }
        return true;
    }
    if !available(app) || !(ui.input(|i| i.pointer.primary_pressed()) && resp.contains_pointer()) {
        return false;
    }
    let Some(p) = latest else { return false };
    let Some((page, x, y, target)) = target_at(app, layout, rects, scale, p) else { return false };
    app.canvas.table_resize = Some(ResizeDrag { page, from: (x, y), to: (x, y), target });
    true
}

/// The resize arrow over a border or the handle, and while dragging.
pub fn cursor(app: &WordApp, ui: &Ui, resp: &egui::Response, layout: &DocLayout, rects: &[Rect], scale: f32) {
    let edge = match &app.canvas.table_resize {
        Some(d) => Some(d.target.edge),
        None if resp.hovered() && available(app) => {
            ui.input(|i| i.pointer.latest_pos()).and_then(|p| target_at(app, layout, rects, scale, p)).map(|(_, _, _, t)| t.edge)
        }
        None => None,
    };
    let icon = match edge {
        Some(Edge::Column(_)) => egui::CursorIcon::ResizeColumn,
        Some(Edge::Row(_)) => egui::CursorIcon::ResizeRow,
        Some(Edge::Corner) => egui::CursorIcon::ResizeNwSe,
        None => return,
    };
    ui.ctx().set_cursor_icon(icon);
}

/// The handle past the bottom-right corner of the table under the pointer, and while dragging,
/// a dashed preview of the new border or outline.
pub fn paint(app: &WordApp, painter: &egui::Painter, t: &Tokens, layout: &DocLayout, rects: &[Rect], scale: f32) {
    let s = |page: usize, x: f32, y: f32| to_screen(rects, layout, scale, page, x, y);
    let dash = Stroke::new(1.0, t.text);
    if let Some(d) = &app.canvas.table_resize {
        let shift = painter.ctx().input(|i| i.modifiers.shift);
        let tg = &d.target;
        let plan = tg.plan(d.to.0 - d.from.0, d.to.1 - d.from.1, shift);
        let (left, top) = (tg.left(), tg.top());
        let right = plan.widths.as_ref().map_or(tg.right(), |w| left + w.iter().sum::<f32>());
        let bottom = top + tg.rows.iter().map(|r| plan.heights.iter().find(|(i, _)| *i == r.row).map_or(r.bottom - r.top, |(_, h)| *h)).sum::<f32>();
        let line = |a: Option<Pos2>, b: Option<Pos2>| {
            if let (Some(a), Some(b)) = (a, b) {
                painter.extend(egui::Shape::dashed_line(&[a, b], dash, 3.0, 3.0));
            }
        };
        match tg.edge {
            Edge::Column(i) => {
                let x = left + plan.widths.as_ref().map_or(0.0, |w| w.iter().take(i).sum::<f32>());
                line(s(d.page, x, top), s(d.page, x, tg.bottom()));
            }
            Edge::Row(r) => {
                let y = tg.rows.iter().find(|x| x.row == r).map_or(top, |x| x.top) + plan.heights.first().map_or(0.0, |(_, h)| *h);
                line(s(d.page, left, y), s(d.page, tg.right(), y));
            }
            Edge::Corner => {
                if let (Some(a), Some(b)) = (s(d.page, left, top), s(d.page, right, bottom)) {
                    let r = Rect::from_two_pos(a, b);
                    let pts = [r.left_top(), r.right_top(), r.right_bottom(), r.left_bottom(), r.left_top()];
                    painter.extend(egui::Shape::dashed_line(&pts, dash, 3.0, 3.0));
                }
            }
        }
        return;
    }
    if !available(app) {
        return;
    }
    // The handle shows while the pointer is over the table (or on the handle itself).
    let Some(p) = painter.ctx().input(|i| i.pointer.hover_pos()) else { return };
    let Some((page, x, y)) = crate::canvas::page_at(rects, layout, scale, p) else { return };
    let Some(sc) = page_scale(rects, layout, scale, page) else { return };
    let Some(tg) = table_resize::table_under(layout, &app.session.doc, page, x, y, REACH_PX / sc) else { return };
    let h = tg.handle();
    if let (Some(a), Some(b)) = (s(page, h.x, h.y), s(page, h.right(), h.bottom())) {
        let r = Rect::from_two_pos(a, b);
        painter.rect_filled(r, 0.0, egui::Color32::WHITE);
        painter.rect_stroke(r, 0.0, Stroke::new(1.0, t.text_dim), egui::StrokeKind::Inside);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Screen ↔ page conversion survives a page list that doesn't match the layout.
    #[test]
    fn conversions_survive_missing_pages() {
        let layout = DocLayout::default();
        assert!(page_scale(&[], &layout, 1.0, 0).is_none());
        assert!(to_screen(&[], &layout, 1.0, 3, 1.0, 1.0).is_none());
        let rects = [Rect::from_min_size(pos2(10.0, 20.0), egui::vec2(100.0, 200.0))];
        assert_eq!(page_scale(&rects, &layout, 2.0, 0), Some(2.0));
        assert_eq!(to_screen(&rects, &layout, 2.0, 0, 5.0, 5.0), Some(pos2(20.0, 30.0)));
        assert!(page_scale(&rects, &layout, 0.0, 0).is_none());
    }

    type H = egui_kittest::Harness<'static, WordApp>;

    /// The whole window with a 2×3 table whose first cell's text is selected.
    fn table_window() -> H {
        let mut a = WordApp::new(wordcraft_engine::Session::new(wordcraft_doc::Document::new()), Default::default());
        a.run("insert.table", json!({"rows": 2, "cols": 3})).unwrap();
        a.run("text.insert", json!({"text": "Product"})).unwrap();
        a.run("select.text", json!({"text": "Product"})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        for _ in 0..6 {
            h.step();
        }
        h
    }

    /// Cell (`row`, `cell`)'s rect in page points.
    fn cell(h: &mut H, row: usize, cell: usize) -> wordcraft_geom::Rect {
        h.state_mut().session.layout().pages[0]
            .items
            .iter()
            .find_map(|it| match it {
                wordcraft_layout::Placed::Cell { rect, row: r, cell: c, .. } if *r == row && *c == cell => Some(*rect),
                _ => None,
            })
            .unwrap()
    }

    fn screen(h: &mut H, x: f32, y: f32) -> Pos2 {
        crate::canvas::page_to_screen(h.state_mut(), 0, x, y).unwrap()
    }

    fn button(h: &H, at: Pos2, pressed: bool) {
        h.event(egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed, modifiers: Default::default() });
    }

    fn steps(h: &mut H, n: usize) {
        for _ in 0..n {
            h.step();
        }
    }

    /// A real mouse drag of a column border: the resize arrow on hover, a preview and no change
    /// while dragging, then one undo step on release, the selection untouched.
    #[test]
    fn dragging_a_column_border_with_the_mouse_is_one_undo_step() {
        let mut h = table_window();
        let (a, b) = (cell(&mut h, 0, 0), cell(&mut h, 0, 1));
        let sel = h.state().session.sel.clone();
        let depth = h.state().session.undo_depth();
        let start = screen(&mut h, a.right(), a.y + a.h / 2.0);
        h.event(egui::Event::PointerMoved(start));
        steps(&mut h, 2);
        assert_eq!(h.output().platform_output.cursor_icon, egui::CursorIcon::ResizeColumn, "the resize arrow over the border");
        button(&h, start, true);
        h.step();
        // 30 page points to the right, in a few moves.
        let end = screen(&mut h, a.right() + 30.0, a.y + a.h / 2.0);
        for k in 1..=5 {
            h.event(egui::Event::PointerMoved(start + (end - start) * (k as f32 / 5.0)));
            h.step();
        }
        assert!(h.state().canvas.table_resize.is_some(), "a drag is in progress");
        assert_eq!(h.state().session.undo_depth(), depth, "nothing is written while dragging");
        assert_eq!(cell(&mut h, 0, 0), a, "the table waits for the release");
        button(&h, end, false);
        steps(&mut h, 3);
        assert!(h.state().canvas.table_resize.is_none());
        assert_eq!(h.state().session.undo_depth(), depth + 1, "one undo step for the drag");
        let (a1, b1) = (cell(&mut h, 0, 0), cell(&mut h, 0, 1));
        assert!((a1.w - (a.w + 30.0)).abs() < 1.0, "{a:?} → {a1:?}");
        assert!((b1.w - (b.w - 30.0)).abs() < 1.0, "{b:?} → {b1:?}");
        assert_eq!(h.state().session.sel, sel, "the selection is kept");
        assert!(h.state_mut().session.undo());
        assert_eq!(cell(&mut h, 0, 0), a);
    }

    /// Clicks inside a cell still place the caret (the border doesn't steal them), and Esc drops a
    /// drag without changing anything.
    #[test]
    fn clicks_in_cells_still_move_the_caret_and_esc_cancels_a_drag() {
        let mut h = table_window();
        let c = cell(&mut h, 1, 1);
        let at = screen(&mut h, c.x + c.w / 2.0, c.y + c.h / 2.0);
        h.event(egui::Event::PointerMoved(at));
        h.step();
        button(&h, at, true);
        h.step();
        button(&h, at, false);
        steps(&mut h, 3);
        let focus = &h.state().session.sel.focus;
        assert_eq!(focus.path.cell().map(|(_, r, cl)| (r, cl)), Some((1, 1)), "the caret went into the clicked cell");
        let table = h.state().session.doc.body.iter().find_map(|b| b.as_table()).cloned();
        let depth = h.state().session.undo_depth();
        let a = cell(&mut h, 0, 0);
        let start = screen(&mut h, a.x + a.w / 2.0, a.bottom());
        h.event(egui::Event::PointerMoved(start));
        h.step();
        button(&h, start, true);
        h.step();
        let end = start + egui::vec2(0.0, 40.0);
        h.event(egui::Event::PointerMoved(end));
        h.step();
        assert!(h.state().canvas.table_resize.is_some(), "the row border is being dragged");
        h.event(egui::Event::Key { key: egui::Key::Escape, physical_key: None, pressed: true, repeat: false, modifiers: Default::default() });
        h.step();
        button(&h, end, false);
        steps(&mut h, 3);
        assert_eq!(h.state().session.undo_depth(), depth, "Esc cancelled it");
        assert_eq!(h.state().session.doc.body.iter().find_map(|b| b.as_table()).cloned(), table);
    }
}
