//! View › Window: more views of the open document, as in Word.
//!
//! Split divides the document area into two panes, each scrolled on its own (one selection, as
//! in Word's split window). New Window opens another window on the same document with its own
//! scroll position; the document, and so its selection, is shared. Arrange All tiles the
//! windows, View Side by Side puts two next to each other (with Synchronous Scrolling, which
//! scrolls them together), and Switch Windows brings one to the front.
//!
//! Each view keeps its own [`CanvasState`]; drawing one swaps it into `WordApp::canvas` for the
//! time it takes, so the canvas code works on whichever view it is drawing.

use egui::{Rect, Sense, Ui, UiBuilder, ViewportBuilder, ViewportCommand, ViewportId, pos2, vec2};
use serde_json::{Value, json};

use crate::WordApp;
use crate::canvas::CanvasState;

/// Another window on the document (View › New Window).
pub struct DocWindow {
    /// 2, 3…: the number after the colon in its title; the first window is 1.
    pub number: u32,
    pub(crate) canvas: CanvasState,
    /// Its close button was pressed: dropped once the frame is drawn.
    closing: bool,
}

/// The document's views besides the main window's canvas.
pub struct Windows {
    /// The lower pane of View › Split (`None`: not split).
    pub(crate) split: Option<Box<CanvasState>>,
    /// The upper pane's share of the split document area.
    pub split_ratio: f32,
    /// More windows (View › New Window), in the order they opened.
    pub others: Vec<DocWindow>,
    /// View Side by Side is on.
    pub side_by_side: bool,
    /// Synchronous Scrolling is on (it only acts while side by side).
    pub sync_scroll: bool,
    /// Scroll offset of each window (by number) last frame, for Synchronous Scrolling.
    last_offsets: Vec<(u32, egui::Vec2)>,
    /// Windows just scrolled along with another: their next move is that, not the user's.
    following: Vec<u32>,
    /// Bring this window to the front next frame.
    focus: Option<u32>,
    /// Arrange the windows next frame.
    arrange: Option<Arrange>,
}

impl Default for Windows {
    fn default() -> Self {
        Windows {
            split: None,
            split_ratio: 0.5,
            others: Vec::new(),
            side_by_side: false,
            sync_scroll: true,
            last_offsets: Vec::new(),
            following: Vec::new(),
            focus: None,
            arrange: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Arrange {
    /// Arrange All: every window, stacked.
    All,
    /// View Side by Side: the first two windows, left and right.
    SideBySide,
}

/// The viewport of window `number` (1 is the main window).
pub fn viewport_id(number: u32) -> ViewportId {
    if number <= 1 { ViewportId::ROOT } else { ViewportId::from_hash_of(("doc4me-window", number)) }
}

/// Where Arrange All puts `n` windows on `screen`: stacked one above the other, each the full
/// width, as Word tiles them.
pub fn tiles(screen: Rect, n: usize) -> Vec<Rect> {
    let n = n.max(1);
    let h = screen.height() / n as f32;
    (0..n).map(|i| Rect::from_min_size(pos2(screen.min.x, screen.min.y + h * i as f32), vec2(screen.width(), h))).collect()
}

/// Where View Side by Side puts two windows on `screen`: the left and right halves.
pub fn halves(screen: Rect) -> [Rect; 2] {
    let w = screen.width() / 2.0;
    [Rect::from_min_size(screen.min, vec2(w, screen.height())), Rect::from_min_size(pos2(screen.min.x + w, screen.min.y), vec2(w, screen.height()))]
}

impl Windows {
    /// The open windows' numbers, the main window (1) first.
    pub fn numbers(&self) -> Vec<u32> {
        std::iter::once(1).chain(self.others.iter().map(|w| w.number)).collect()
    }

    fn open(&mut self) -> u32 {
        let number = self.others.iter().map(|w| w.number).max().unwrap_or(1) + 1;
        self.others.push(DocWindow { number, canvas: CanvasState::default(), closing: false });
        self.focus = Some(number);
        number
    }
}

/// A View › Window request from a command (`{"window": "new" | "split" | "arrange" | "sideBySide"
/// | "syncScroll" | "switch", …}`).
pub(crate) fn request(app: &mut WordApp, req: &Value) {
    let flag = |v: Option<&Value>, now: bool| v.and_then(Value::as_bool).unwrap_or(!now);
    let w = &mut app.windows;
    match req.get("window").and_then(Value::as_str) {
        Some("new") => {
            w.open();
        }
        Some("split") => {
            if !flag(req.get("value"), w.split.is_some()) {
                w.split = None;
            } else if w.split.is_none() {
                w.split = Some(Box::default());
            }
        }
        Some("arrange") => w.arrange = Some(Arrange::All),
        Some("sideBySide") => {
            w.side_by_side = flag(req.get("value"), w.side_by_side);
            if w.side_by_side {
                // Word needs a second window to compare with: open one on this document.
                if w.others.is_empty() {
                    w.open();
                }
                w.arrange = Some(Arrange::SideBySide);
            }
        }
        Some("syncScroll") => w.sync_scroll = flag(req.get("value"), w.sync_scroll),
        Some("switch") => {
            if let Some(n) = req.get("to").and_then(Value::as_u64).and_then(|n| u32::try_from(n).ok())
                && w.numbers().contains(&n)
            {
                w.focus = Some(n);
            }
        }
        _ => {}
    }
}

/// The state View › Window shows (`ui.windows`, and the ribbon's checked buttons).
pub fn state(app: &WordApp) -> Value {
    let w = &app.windows;
    json!({
        "windows": w.numbers(),
        "split": w.split.is_some(),
        "sideBySide": w.side_by_side,
        "syncScroll": w.sync_scroll,
    })
}

/// The document area: one canvas, or the two panes of a split window with a bar between them
/// (drag it to move the split, double-click it to remove the split).
pub(crate) fn document_area(app: &mut WordApp, ui: &mut Ui) {
    let Some(mut lower) = app.windows.split.take() else {
        crate::canvas::show(app, ui);
        return;
    };
    const BAR: f32 = 6.0;
    let full = ui.available_rect_before_wrap();
    let ratio = app.windows.split_ratio.clamp(0.15, 0.85);
    let upper_h = ((full.height() - BAR) * ratio).max(0.0);
    let upper = Rect::from_min_size(full.min, vec2(full.width(), upper_h));
    let bar = Rect::from_min_size(pos2(full.min.x, upper.max.y), vec2(full.width(), BAR));
    let lower_rect = Rect::from_min_max(pos2(full.min.x, bar.max.y), full.max);
    ui.scope_builder(UiBuilder::new().max_rect(upper).id_salt("pane_upper"), |ui| crate::canvas::show(app, ui));
    let resp = ui.interact(bar, ui.id().with("split_bar"), Sense::click_and_drag()).on_hover_cursor(egui::CursorIcon::ResizeVertical);
    let t = crate::theme::Tokens::get(ui.ctx());
    ui.painter().rect_filled(bar, 0.0, if resp.hovered() || resp.dragged() { t.accent } else { t.border });
    if resp.dragged()
        && let Some(p) = resp.interact_pointer_pos()
        && full.height() > 0.0
    {
        app.windows.split_ratio = ((p.y - full.min.y) / full.height()).clamp(0.15, 0.85);
    }
    std::mem::swap(&mut app.canvas, &mut lower);
    ui.scope_builder(UiBuilder::new().max_rect(lower_rect).id_salt("pane_lower"), |ui| crate::canvas::show(app, ui));
    std::mem::swap(&mut app.canvas, &mut lower);
    // A link clicked in the lower pane opens like one in the upper.
    if let Some(url) = lower.open_url.take() {
        app.canvas.open_url = Some(url);
    }
    if !resp.double_clicked() {
        app.windows.split = Some(lower);
    }
    ui.advance_cursor_after_rect(full);
}

/// Whether any view of the document has the keyboard (so the window's own shortcuts don't run
/// a second time).
pub(crate) fn any_canvas_focused(app: &WordApp) -> bool {
    app.canvas.focused || app.windows.split.as_ref().is_some_and(|c| c.focused) || app.windows.others.iter().any(|w| w.canvas.focused)
}

/// Draw the other windows (each a viewport of its own: a real window on the desktop, a window
/// inside the main one where the host can't open more), then arrange, focus and scroll them
/// together as asked.
pub(crate) fn show_others(app: &mut WordApp, ui: &mut Ui) {
    let ctx = ui.ctx().clone();
    let title = app.title_stem();
    for i in 0..app.windows.others.len() {
        let Some(number) = app.windows.others.get(i).map(|w| w.number) else { continue };
        let name = format!("{title}:{number}");
        let builder = ViewportBuilder::default().with_title(name).with_inner_size([960.0, 720.0]);
        ctx.show_viewport_immediate(viewport_id(number), builder, |ui, _class| {
            let Some(w) = app.windows.others.get_mut(i) else { return };
            let mut canvas = std::mem::take(&mut w.canvas);
            std::mem::swap(&mut app.canvas, &mut canvas);
            let fill = crate::theme::Tokens::get(ui.ctx()).canvas;
            egui::CentralPanel::default().frame(egui::Frame::NONE.fill(fill)).show(ui, |ui| crate::canvas::show(app, ui));
            std::mem::swap(&mut app.canvas, &mut canvas);
            if let Some(url) = canvas.open_url.take() {
                app.canvas.open_url = Some(url);
            }
            let closing = ui.ctx().input(|inp| inp.viewport().close_requested());
            if let Some(w) = app.windows.others.get_mut(i) {
                w.canvas = canvas;
                w.closing |= closing;
            }
        });
    }
    let before = app.windows.others.len();
    app.windows.others.retain(|w| !w.closing);
    if app.windows.others.is_empty() && before > 0 {
        app.windows.side_by_side = false;
    }
    if let Some(n) = app.windows.focus.take() {
        ctx.send_viewport_cmd_to(viewport_id(n), ViewportCommand::Focus);
    }
    if let Some(how) = app.windows.arrange.take() {
        arrange(app, &ctx, how);
    }
    sync_scroll(app);
}

/// Move and size the windows for Arrange All or View Side by Side.
fn arrange(app: &WordApp, ctx: &egui::Context, how: Arrange) {
    let Some(size) = ctx.input(|i| i.viewport().monitor_size) else { return };
    let screen = Rect::from_min_size(egui::Pos2::ZERO, size);
    let numbers = app.windows.numbers();
    let places: Vec<Rect> = match how {
        Arrange::All => tiles(screen, numbers.len()),
        Arrange::SideBySide => halves(screen).to_vec(),
    };
    for (n, r) in numbers.iter().zip(places) {
        let id = viewport_id(*n);
        ctx.send_viewport_cmd_to(id, ViewportCommand::Maximized(false));
        ctx.send_viewport_cmd_to(id, ViewportCommand::OuterPosition(r.min));
        ctx.send_viewport_cmd_to(id, ViewportCommand::InnerSize(r.size()));
    }
}

/// Synchronous Scrolling: while side by side, a window the user scrolls takes the others along by
/// the same amount.
fn sync_scroll(app: &mut WordApp) {
    let views: Vec<(u32, egui::Vec2)> =
        std::iter::once((1, app.canvas.scroll_offset)).chain(app.windows.others.iter().map(|w| (w.number, w.canvas.scroll_offset))).collect();
    let on = app.windows.sync_scroll && app.windows.side_by_side && views.len() > 1;
    let last = std::mem::take(&mut app.windows.last_offsets);
    let following = std::mem::take(&mut app.windows.following);
    app.windows.last_offsets = views.clone();
    if !on {
        return;
    }
    let moved = views.iter().find_map(|(n, now)| {
        let was = last.iter().find(|(m, _)| m == n).map(|(_, o)| *o)?;
        (!following.contains(n) && (*now - was).length() > 0.5).then_some((*n, *now - was))
    });
    let Some((source, delta)) = moved else { return };
    for (n, now) in views {
        if n == source {
            continue;
        }
        let to = (now + delta).max(egui::Vec2::ZERO);
        if n == 1 {
            app.canvas.request_scroll(to);
        } else if let Some(w) = app.windows.others.iter_mut().find(|w| w.number == n) {
            w.canvas.request_scroll(to);
        }
        app.windows.following.push(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> WordApp {
        WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text("Text")), crate::Services::default())
    }

    #[test]
    fn window_commands_open_split_and_switch() {
        let mut a = app();
        assert_eq!(state(&a)["windows"], json!([1]));
        a.run("view.newWindow", json!({})).unwrap();
        a.run("view.newWindow", json!({})).unwrap();
        assert_eq!(state(&a)["windows"], json!([1, 2, 3]));
        a.run("view.split", json!({})).unwrap();
        assert_eq!(state(&a)["split"], true);
        a.run("view.split", json!({})).unwrap();
        assert_eq!(state(&a)["split"], false, "Split again removes it");
        a.run("view.switchWindows", json!({"window": 3})).unwrap();
        assert_eq!(a.windows.focus, Some(3));
        a.run("view.switchWindows", json!({"window": 9})).unwrap();
        assert_eq!(a.windows.focus, Some(3), "no window 9: nothing changes");
        a.run("view.arrangeAll", json!({})).unwrap();
        assert_eq!(a.windows.arrange, Some(Arrange::All));
    }

    #[test]
    fn side_by_side_opens_a_second_window_and_scrolls_together() {
        let mut a = app();
        a.run("view.sideBySide", json!({})).unwrap();
        assert_eq!(state(&a)["windows"], json!([1, 2]), "a window to compare with");
        assert_eq!((state(&a)["sideBySide"].clone(), state(&a)["syncScroll"].clone()), (json!(true), json!(true)));
        // The user scrolls window 2 by 120: window 1 follows by as much, once.
        sync_scroll(&mut a);
        a.windows.others[0].canvas.scroll_offset = vec2(0.0, 120.0);
        sync_scroll(&mut a);
        assert_eq!(a.canvas.scroll_request(), Some(vec2(0.0, 120.0)));
        a.canvas.scroll_offset = vec2(0.0, 120.0);
        sync_scroll(&mut a);
        assert_eq!(a.windows.others[0].canvas.scroll_request(), None, "following isn't scrolling");
        a.run("view.syncScroll", json!({})).unwrap();
        assert_eq!(state(&a)["syncScroll"], false);
        a.run("view.sideBySide", json!({})).unwrap();
        assert_eq!(state(&a)["sideBySide"], false);
    }

    #[test]
    fn arrange_all_stacks_and_side_by_side_halves() {
        let screen = Rect::from_min_size(pos2(0.0, 0.0), vec2(1200.0, 900.0));
        assert_eq!(
            tiles(screen, 3),
            [
                Rect::from_min_size(pos2(0.0, 0.0), vec2(1200.0, 300.0)),
                Rect::from_min_size(pos2(0.0, 300.0), vec2(1200.0, 300.0)),
                Rect::from_min_size(pos2(0.0, 600.0), vec2(1200.0, 300.0)),
            ]
        );
        assert_eq!(
            halves(screen),
            [Rect::from_min_size(pos2(0.0, 0.0), vec2(600.0, 900.0)), Rect::from_min_size(pos2(600.0, 0.0), vec2(600.0, 900.0))]
        );
    }

    #[test]
    fn split_and_other_windows_draw_their_own_views() {
        let mut a = app();
        a.run("view.split", json!({})).unwrap();
        a.run("view.newWindow", json!({})).unwrap();
        let mut h = egui_kittest::Harness::builder().with_size(vec2(1200.0, 800.0)).build_ui_state(
            |ui, a: &mut WordApp| {
                let ctx = ui.ctx().clone();
                a.logic(&ctx);
                a.ui(ui);
            },
            a,
        );
        for _ in 0..4 {
            h.step();
        }
        let a = h.state();
        assert!(a.windows.split.as_ref().is_some_and(|c| c.canvas_rect.is_some()), "the lower pane drew");
        assert!(a.windows.others[0].canvas.canvas_rect.is_some(), "window 2 drew");
        assert!(a.canvas.canvas_rect.is_some(), "the upper pane drew");
    }
}
