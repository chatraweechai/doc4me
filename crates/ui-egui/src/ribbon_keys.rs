//! Word's keyboard access to the window outside the page.
//!
//! - After Alt (or F10) shows the KeyTips, Tab or an arrow key puts the focus on the ribbon: Left
//!   and Right on the tabs, Up on the active tab, Down or Tab on its first command (Shift+Tab its
//!   last). egui's own focus moves (Tab, Shift+Tab, the arrow keys) then go through the commands,
//!   and Enter or Space presses the focused one. Tab on a tab goes to that tab's first command;
//!   Ctrl+Left/Right jump to the previous or next group; Down or Alt+Down opens a command's list,
//!   whose first item takes the focus (Down goes on through it, Right opens a submenu); Escape
//!   closes the list back onto its command, then leaves the ribbon for the document.
//!   Ctrl+Shift+F10 shows the focused command's tooltip and Shift+F10 its shortcut menu.
//! - F6 and Shift+F6 go round the document, the open task panes, the status bar and the ribbon.
//! - Shift+F10 on the page opens the shortcut menu at the caret.
//! - Alt+F6 goes from a dialog box back to the document.
//!
//! The widgets record themselves each frame ([`tab`], [`group`], [`control`], [`stop`],
//! [`sub_menu`]); [`logic`] reads the keys the next frame, before layout, against what they
//! recorded.

use egui::{Context, CornerRadius, FocusDirection, Id, Key, Popup, Rect, Response, Sense, Stroke, Ui, Vec2};

use crate::WordApp;
use crate::theme::Tokens;

/// Where an F6 stop sits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    Pane,
    Status,
}

/// A ribbon command as drawn last frame.
#[derive(Clone, Copy, Debug)]
struct Control {
    id: Id,
    group: usize,
    /// The popup of the list it opens.
    list: Option<Id>,
    /// It has a shortcut menu of its own (Add to Quick Access Toolbar…).
    menu: bool,
}

/// What the widgets recorded during one frame.
#[derive(Clone, Debug, Default)]
struct Recorded {
    /// The tab strip: each header's id and tab name.
    tabs: Vec<(Id, String)>,
    /// The active tab's commands in drawing order.
    controls: Vec<Control>,
    /// F6 stops: the status bar's and each open task pane's, with the area they stand for.
    stops: Vec<(Id, Region, Rect)>,
    /// Submenu buttons inside the open menus.
    submenus: Vec<Id>,
    group: usize,
    /// The ribbon's commands are being drawn (the same widgets also make the mini toolbar).
    recording: bool,
}

#[derive(Clone, Debug, Default)]
struct State {
    building: Recorded,
    last: Recorded,
    /// The focused widget at the end of the last frame (Escape clears egui's before `logic`).
    focused: Option<Id>,
    /// Tab or an arrow key pressed while the KeyTips showed: (key, Shift held).
    entry: Option<(Key, bool)>,
    /// Ctrl+Shift+F10: the command whose tooltip shows while it keeps the focus.
    tip: Option<Id>,
    /// A list or menu the keyboard opened, until it closes.
    list: Option<Id>,
    /// The ribbon command whose shortcut menu Shift+F10 opened.
    shortcut_menu: Option<Id>,
    /// The next list or menu to show gives its first item the focus ([`focus_first`]) within
    /// this many frames.
    focus_first: u8,
}

fn with<R>(ctx: &Context, f: impl FnOnce(&mut State) -> R) -> R {
    ctx.data_mut(|d| f(d.get_temp_mut_or_default::<State>(Id::new("wc_ribbon_keys"))))
}

/// The focus ring round a widget the keyboard reached.
fn ring(ui: &Ui, rect: Rect) {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_stroke(rect.expand(1.0), CornerRadius::same(4), Stroke::new(2.0, t.accent), egui::StrokeKind::Inside);
}

/// A ribbon tab's header.
pub fn tab(ui: &Ui, resp: &Response, name: &str) {
    with(ui.ctx(), |s| s.building.tabs.push((resp.id, name.to_string())));
    if resp.has_focus() {
        ring(ui, resp.rect);
    }
}

/// A ribbon group starts: Ctrl+Left/Right jump between groups.
pub fn group(ctx: &Context) {
    with(ctx, |s| s.building.group += 1);
}

/// The ribbon starts or stops drawing its commands.
pub fn recording(ctx: &Context, on: bool) {
    with(ctx, |s| s.building.recording = on);
}

/// A command button (on the ribbon while [`recording`]). `list` is the widget whose drop-down list
/// Down opens (the command itself, or a split button's arrow); `menu` says it has its own shortcut
/// menu.
pub fn control(ui: &Ui, resp: &Response, list: Option<&Response>, menu: bool) {
    let list = list.map(Popup::default_response_id);
    with(ui.ctx(), |s| {
        if s.building.recording {
            let group = s.building.group;
            s.building.controls.push(Control { id: resp.id, group, list, menu });
        }
    });
    if resp.has_focus() {
        ring(ui, resp.rect);
    }
}

/// F6's stop for a task pane or the status bar, at the top of its contents: it takes no room, and
/// while it has the focus a ring goes round `area`; Tab goes on into the pane's own controls.
pub fn stop(ui: &mut Ui, region: Region, area: Rect) {
    let id = ui.id().with("wc_f6_stop");
    let resp = ui.interact(Rect::from_min_size(area.min, Vec2::splat(1.0)), id, Sense::focusable_noninteractive());
    with(ui.ctx(), |s| s.building.stops.push((id, region, area)));
    if resp.has_focus() {
        ring(ui, area.shrink(1.0));
    }
}

/// A submenu inside a menu (what `ui.menu_button` makes there); Right opens it from the keyboard.
pub fn sub_menu<'a, R>(ui: &mut Ui, atoms: impl egui::IntoAtoms<'a>, add: impl FnOnce(&mut Ui) -> R) -> egui::InnerResponse<Option<R>> {
    let inside = egui::containers::menu::is_in_menu(ui);
    let r = ui.menu_button(atoms, add);
    if inside {
        with(ui.ctx(), |s| s.building.submenus.push(r.response.id));
    }
    r
}

/// At the top of a list's or menu's contents: when the keyboard opened it, its first item takes
/// the focus.
pub fn focus_first(ui: &mut Ui) {
    // A menu's first frame only measures it: nothing in it can take the focus yet.
    if ui.is_sizing_pass() || !with(ui.ctx(), |s| std::mem::take(&mut s.focus_first) > 0) {
        return;
    }
    // A stop that passes the focus straight on to the next widget, the list's first item.
    let id = ui.id().with("wc_focus_first");
    ui.memory_mut(|m| {
        m.request_focus(id);
        m.move_focus(FocusDirection::Next);
    });
    ui.interact(Rect::from_min_size(ui.cursor().min, Vec2::splat(1.0)), id, Sense::focusable_noninteractive());
}

/// Enter or Space opened a command's list: its first item takes the focus.
pub fn opened_from_keys(ctx: &Context, popup: Id) {
    with(ctx, |s| {
        s.list = Some(popup);
        s.focus_first = 3;
    });
}

/// Ctrl+Shift+F10 asked for this command's tooltip.
pub fn tip_shown(ctx: &Context, id: Id) -> bool {
    with(ctx, |s| s.tip == Some(id))
}

/// Show the tooltip Ctrl+Shift+F10 asked for, beside the command. Its popup id is its own: the
/// command's default one belongs to its list or shortcut menu, which may open the same frame.
pub fn show_tip(resp: &Response, add: impl FnOnce(&mut Ui)) {
    Popup::from_response(resp).id(resp.id.with("wc_key_tip")).kind(egui::PopupKind::Tooltip).show(add);
}

/// Shift+F10 opened this command's shortcut menu: where it shows.
pub fn shortcut_menu_at(ctx: &Context, resp: &Response) -> Option<egui::Pos2> {
    with(ctx, |s| s.shortcut_menu == Some(resp.id)).then(|| resp.rect.left_bottom())
}

/// Tab or an arrow key while the KeyTips show: the keys take the focus onto the ribbon.
pub fn enters_ribbon(key: Key) -> bool {
    matches!(key, Key::Tab | Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown)
}

/// Remember the key that took the KeyTips onto the ribbon; [`logic`] moves the focus.
pub fn enter(ctx: &Context, key: Key, shift: bool) {
    with(ctx, |s| s.entry = Some((key, shift)));
}

/// Focus `id` now, cancelling the move egui read from this frame's keys.
fn focus_now(ctx: &Context, id: Id) {
    ctx.memory_mut(|m| {
        m.request_focus(id);
        m.move_focus(FocusDirection::None);
    });
}

/// The keys, read before layout against what the widgets recorded last frame.
pub fn logic(app: &mut WordApp, ctx: &Context) {
    let (last, before, entry, list) = with(ctx, |s| {
        s.last = std::mem::take(&mut s.building);
        s.focus_first = s.focus_first.saturating_sub(1);
        (s.last.clone(), s.focused, s.entry.take(), s.list)
    });
    let focus = ctx.memory(|m| m.focused());
    // (egui's memory can't be read inside `with`, which holds the context.)
    let menu_closed = with(ctx, |s| s.shortcut_menu).is_some_and(|id| !Popup::is_id_open(ctx, popup_id(id)));
    with(ctx, |s| {
        if s.tip.is_some() && s.tip != focus {
            s.tip = None;
        }
        if menu_closed {
            s.shortcut_menu = None;
        }
    });
    let is_tab = |id: Id| last.tabs.iter().any(|t| t.0 == id);
    let control = |id: Id| last.controls.iter().position(|c| c.id == id);
    // A list the keyboard opened has closed (an item ran): typing goes on in the document. The
    // item that ran may keep egui's focus for a frame after its list has gone.
    if let Some(popup) = list
        && !Popup::is_id_open(ctx, popup)
    {
        with(ctx, |s| s.list = None);
        if focus.is_none_or(|f| !is_tab(f) && control(f).is_none() && !last.stops.iter().any(|s| s.0 == f)) {
            app.canvas.want_focus = true;
        }
    }
    let current_tab = last.tabs.iter().position(|t| t.1 == app.ui.tab);
    if let Some((key, shift)) = entry {
        match key {
            Key::ArrowLeft | Key::ArrowRight if !last.tabs.is_empty() => {
                let n = last.tabs.len();
                let i = current_tab.unwrap_or(0);
                let next = if key == Key::ArrowLeft { (i + n - 1) % n } else { (i + 1) % n };
                focus_now(ctx, last.tabs[next].0);
            }
            Key::ArrowUp => {
                if let Some(i) = current_tab {
                    focus_now(ctx, last.tabs[i].0);
                }
            }
            _ => {
                let c = if key == Key::Tab && shift { last.controls.last() } else { last.controls.first() };
                if let Some(c) = c {
                    focus_now(ctx, c.id);
                } else if let Some(i) = current_tab {
                    focus_now(ctx, last.tabs[i].0);
                }
            }
        }
    }
    let events = ctx.input(|i| i.events.clone());
    let mut consumed = Vec::new();
    let canvas = crate::windows::any_canvas_focused(app);
    for e in &events {
        let egui::Event::Key { key, pressed: true, modifiers: m, .. } = e else { continue };
        let plain = !m.alt && !m.command && !m.ctrl;
        match key {
            Key::F6 if plain => {
                cycle(app, ctx, &last, focus, m.shift);
                consumed.push(*key);
            }
            // Alt+F6: from a dialog box back to the document.
            Key::F6 if m.alt && !m.command && !m.ctrl && !m.shift && app.dialog.is_some() && !canvas => {
                app.canvas.want_focus = true;
                consumed.push(*key);
            }
            // Ctrl+Shift+F10: the focused command's tooltip.
            Key::F10 if m.shift && m.command && !m.alt => {
                if let Some(f) = focus.filter(|f| control(*f).is_some()) {
                    with(ctx, |s| s.tip = Some(f));
                    consumed.push(*key);
                }
            }
            // Shift+F10: the shortcut menu, at the caret on the page or under a ribbon command.
            Key::F10 if m.shift && plain => {
                if app.canvas.focused && app.dialog.is_none() {
                    app.canvas.context_from_keys = true;
                    consumed.push(*key);
                } else if let Some(c) = focus.and_then(control).map(|i| last.controls[i])
                    && c.menu
                {
                    let popup = popup_id(c.id);
                    Popup::open_id(ctx, popup);
                    with(ctx, |s| {
                        s.shortcut_menu = Some(c.id);
                        s.list = Some(popup);
                        s.focus_first = 3;
                    });
                    consumed.push(*key);
                }
            }
            Key::Escape if !m.any() && !canvas => {
                // A list the keyboard opened from a command closes back onto the command;
                // otherwise the focus leaves the ribbon (or a pane's or the status bar's stop) for
                // the document.
                if let Some(c) = last.controls.iter().find(|c| c.list.is_some_and(|p| Some(p) == list && Popup::is_id_open(ctx, p))) {
                    focus_now(ctx, c.id);
                    with(ctx, |s| s.list = None);
                } else if before.is_some_and(|f| is_tab(f) || control(f).is_some() || last.stops.iter().any(|s| s.0 == f)) {
                    app.canvas.want_focus = true;
                }
            }
            // Tab on a tab: its first command.
            Key::Tab if !m.any() && focus.is_some_and(is_tab) => {
                if let Some(c) = last.controls.first() {
                    focus_now(ctx, c.id);
                    consumed.push(*key);
                }
            }
            // Ctrl+Left/Right: the previous or next group's first command.
            Key::ArrowLeft | Key::ArrowRight if m.command && !m.shift && !m.alt => {
                if let Some(i) = focus.and_then(control) {
                    let g = last.controls[i].group;
                    let target = if *key == Key::ArrowRight {
                        last.controls.iter().find(|c| c.group > g)
                    } else {
                        let prev = last.controls.iter().rev().find(|c| c.group < g).map(|c| c.group);
                        prev.and_then(|p| last.controls.iter().find(|c| c.group == p))
                    };
                    if let Some(t) = target {
                        focus_now(ctx, t.id);
                    }
                    consumed.push(*key);
                }
            }
            // Down or Alt+Down: open the focused command's list.
            Key::ArrowDown if (!m.any() || (m.alt && !m.command && !m.ctrl && !m.shift)) => {
                if let Some(popup) = focus.and_then(control).and_then(|i| last.controls[i].list) {
                    Popup::open_id(ctx, popup);
                    opened_from_keys(ctx, popup);
                    ctx.memory_mut(|mm| mm.move_focus(FocusDirection::None));
                    consumed.push(*key);
                }
            }
            // Right on a submenu: open it; egui's own move then goes into it.
            Key::ArrowRight if !m.any() => {
                if let Some(f) = focus.filter(|f| last.submenus.contains(f))
                    && ctx.read_response(egui::containers::menu::SubMenu::id_from_widget_id(f)).is_none()
                {
                    ctx.input_mut(|i| {
                        i.events.push(egui::Event::Key {
                            key: Key::Enter,
                            physical_key: None,
                            pressed: true,
                            repeat: false,
                            modifiers: egui::Modifiers::NONE,
                        })
                    });
                }
            }
            _ => {}
        }
    }
    if !consumed.is_empty() {
        ctx.input_mut(|i| i.events.retain(|e| !matches!(e, egui::Event::Key { key, pressed: true, .. } if consumed.contains(key))));
    }
}

/// F6 (Shift+F6 backwards): the document, each open task pane, the status bar, the ribbon.
fn cycle(app: &mut WordApp, ctx: &Context, last: &Recorded, focus: Option<Id>, back: bool) {
    // `None` stands for the document.
    let mut stops: Vec<Option<Id>> = vec![None];
    stops.extend(last.stops.iter().filter(|s| s.1 == Region::Pane).map(|s| Some(s.0)));
    stops.extend(last.stops.iter().filter(|s| s.1 == Region::Status).map(|s| Some(s.0)));
    let tab = last.tabs.iter().find(|t| t.1 == app.ui.tab).or(last.tabs.first()).map(|t| t.0);
    stops.extend(tab.map(Some));
    let here = if crate::windows::any_canvas_focused(app) {
        0
    } else if let Some(f) = focus {
        if let Some(i) = stops.iter().position(|s| *s == Some(f)) {
            i
        } else if tab.is_some() && (last.tabs.iter().any(|t| t.0 == f) || last.controls.iter().any(|c| c.id == f)) {
            stops.len() - 1
        } else {
            // Inside a pane or the status bar: the stop whose area holds the focused widget.
            let rect = ctx.read_response(f).map(|r| r.rect);
            last.stops
                .iter()
                .find(|s| rect.is_some_and(|r| s.2.contains(r.center())))
                .and_then(|s| stops.iter().position(|x| *x == Some(s.0)))
                .unwrap_or(0)
        }
    } else {
        0
    };
    let n = stops.len();
    let next = if back { (here + n - 1) % n } else { (here + 1) % n };
    match stops[next] {
        None => app.canvas.want_focus = true,
        Some(id) => focus_now(ctx, id),
    }
}

/// After the frame: what has the focus, for the next frame's Escape.
pub fn end_frame(ctx: &Context) {
    let focused = ctx.memory(|m| m.focused());
    with(ctx, |s| s.focused = focused);
}

/// [`Popup::default_response_id`] from a widget's id alone.
fn popup_id(widget: Id) -> Id {
    widget.with("popup")
}

#[cfg(test)]
mod tests {
    use egui::{Id, Key, Modifiers};
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;
    use serde_json::json;

    use super::{Recorded, Region, popup_id, with};
    use crate::WordApp;
    use crate::keytips::Phase;

    fn harness(text: &str) -> Harness<'static, WordApp> {
        let mut a = WordApp::new(doc4me_engine::Session::new(doc4me_doc::Document::from_text(text)), crate::Services::default());
        a.canvas.want_focus = true;
        let mut h = Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, app: &mut WordApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            a,
        );
        h.run_steps(4);
        assert!(h.state().canvas.focused, "the page has the keyboard");
        h
    }

    fn press(h: &mut Harness<'_, WordApp>, key: Key, modifiers: Modifiers) {
        for pressed in [true, false] {
            h.input_mut().events.push(egui::Event::Key { key, physical_key: None, pressed, repeat: false, modifiers });
            h.step();
        }
    }

    const NONE: Modifiers = Modifiers::NONE;
    const SHIFT: Modifiers = Modifiers::SHIFT;
    const ALT: Modifiers = Modifiers::ALT;
    const CTRL: Modifiers = Modifiers { alt: false, ctrl: true, shift: false, mac_cmd: false, command: true };

    /// What the widgets recorded in the last frame.
    fn recorded(h: &Harness<'_, WordApp>) -> Recorded {
        with(&h.ctx, |s| s.building.clone())
    }

    fn focused(h: &Harness<'_, WordApp>) -> Option<Id> {
        h.ctx.memory(|m| m.focused())
    }

    /// The ribbon command a screen reader names `label`.
    fn command(h: &Harness<'_, WordApp>, label: &str) -> Id {
        let r = recorded(h);
        h.query_all_by_label(label)
            .find_map(|n| {
                let at = n.rect().center();
                r.controls.iter().find(|c| h.ctx.read_response(c.id).is_some_and(|x| x.rect.contains(at)))
            })
            .unwrap_or_else(|| panic!("no ribbon command {label}"))
            .id
    }

    fn focus(h: &mut Harness<'_, WordApp>, id: Id) {
        h.ctx.memory_mut(|m| m.request_focus(id));
        h.run_steps(2);
        assert_eq!(focused(h), Some(id));
    }

    fn group_of(h: &Harness<'_, WordApp>, id: Option<Id>) -> usize {
        recorded(h).controls.iter().find(|c| Some(c.id) == id).map(|c| c.group).expect("a ribbon command has the focus")
    }

    /// ks-040, ks-042, ks-044: after F10 (or Alt) Down takes the focus to the ribbon's first
    /// command; Tab, Shift+Tab and the arrow keys go through the commands and Space presses one.
    #[test]
    fn keytips_then_tab_and_arrows_move_through_the_ribbon() {
        let mut h = harness("Hello world");
        h.state_mut().run("select.all", json!({})).unwrap();
        press(&mut h, Key::F10, NONE);
        assert_eq!(h.state().ui.keytips, Phase::Tabs);
        press(&mut h, Key::ArrowDown, NONE);
        let first = recorded(&h).controls[0].id;
        assert_eq!(h.state().ui.keytips, Phase::Off);
        assert_eq!(focused(&h), Some(first), "the Home tab's first command (Paste)");
        assert!(!h.state().canvas.focused);
        press(&mut h, Key::Tab, NONE);
        let second = focused(&h);
        assert!(second.is_some() && second != Some(first) && recorded(&h).controls.iter().any(|c| Some(c.id) == second));
        press(&mut h, Key::Tab, SHIFT);
        assert_eq!(focused(&h), Some(first), "Shift+Tab goes back");
        let x = h.ctx.read_response(first).unwrap().rect.center().x;
        press(&mut h, Key::ArrowRight, NONE);
        let right = focused(&h).and_then(|id| h.ctx.read_response(id)).expect("a command to the right");
        assert!(right.rect.center().x > x);
        // Space presses the focused command, then the page has the keyboard again.
        let bold = command(&h, "Bold");
        focus(&mut h, bold);
        press(&mut h, Key::Space, NONE);
        h.run_steps(2);
        assert_eq!(h.state_mut().session.run("format.state", &json!({})).unwrap()["bold"], true);
        assert!(h.state().canvas.focused);
        assert_eq!(h.state().session.doc.plain_text(doc4me_doc::StoryRef::Body), "Hello world", "no key reached the text");
    }

    /// ks-041 and the tabs: Left and Right after F10 walk the tabs (each comes up as it takes the
    /// focus), Tab on a tab goes to its first command, Ctrl+Left/Right jump between groups and
    /// Escape goes back to the document.
    #[test]
    fn tabs_and_groups_by_keyboard() {
        let mut h = harness("Text");
        press(&mut h, Key::F10, NONE);
        press(&mut h, Key::ArrowRight, NONE);
        h.run_steps(2);
        let insert = recorded(&h).tabs.iter().find(|t| t.1 == "Insert").unwrap().0;
        assert_eq!(focused(&h), Some(insert));
        assert_eq!(h.state().ui.tab, "Insert");
        press(&mut h, Key::Tab, NONE);
        let first = recorded(&h).controls[0].id;
        assert_eq!(focused(&h), Some(first), "Insert's first command");
        let g = group_of(&h, focused(&h));
        press(&mut h, Key::ArrowRight, CTRL);
        let next = focused(&h);
        assert!(group_of(&h, next) > g, "the next group");
        assert_eq!(recorded(&h).controls.iter().find(|c| c.group == group_of(&h, next)).map(|c| c.id), next, "its first command");
        press(&mut h, Key::ArrowLeft, CTRL);
        assert_eq!(focused(&h), Some(first));
        press(&mut h, Key::Escape, NONE);
        h.run_steps(2);
        assert!(h.state().canvas.focused, "Escape leaves the ribbon for the document");
    }

    /// ks-045, ks-046, ks-047: Down (or Alt+Down) opens the focused command's list with its first
    /// item focused, Down goes on through it, and Escape closes it back onto the command.
    #[test]
    fn down_opens_a_commands_list_and_escape_closes_it() {
        let mut h = harness("Text");
        let spacing = command(&h, "Line and Paragraph Spacing");
        focus(&mut h, spacing);
        press(&mut h, Key::ArrowDown, NONE);
        let list = popup_id(spacing);
        assert!(egui::Popup::is_id_open(&h.ctx, list));
        let item = focused(&h);
        assert!(item.is_some() && item != Some(spacing), "the list's first item has the focus");
        press(&mut h, Key::ArrowDown, NONE);
        let below = focused(&h);
        assert!(below.is_some() && below != item, "Down: the next item");
        assert!(egui::Popup::is_id_open(&h.ctx, list));
        press(&mut h, Key::Escape, NONE);
        assert!(!egui::Popup::is_id_open(&h.ctx, list));
        assert_eq!(focused(&h), Some(spacing), "back on the command");
        press(&mut h, Key::ArrowDown, ALT);
        assert!(egui::Popup::is_id_open(&h.ctx, list), "Alt+Down opens it too");
        assert!(focused(&h).is_some_and(|f| f != spacing));
        press(&mut h, Key::Escape, NONE);
        press(&mut h, Key::Escape, NONE);
        h.run_steps(2);
        assert!(h.state().canvas.focused);
    }

    /// ks-050: Right on a submenu in an open menu opens it and moves into it.
    #[test]
    fn right_opens_a_submenu() {
        let mut h = harness("Text");
        let effects = command(&h, "Text Effects and Typography");
        focus(&mut h, effects);
        press(&mut h, Key::ArrowDown, NONE);
        let fill = h.get_by_label_contains("Text Fill").rect().center();
        // The button's row (a screen reader's box for it spans the menu's width).
        let sub = recorded(&h)
            .submenus
            .into_iter()
            .find(|id| h.ctx.read_response(*id).is_some_and(|r| r.rect.y_range().contains(fill.y)))
            .expect("Text Fill");
        focus(&mut h, sub);
        let submenu = egui::containers::menu::SubMenu::id_from_widget_id(sub);
        assert!(h.ctx.read_response(submenu).is_none(), "closed before Right");
        press(&mut h, Key::ArrowRight, NONE);
        h.run_steps(1);
        assert!(h.ctx.read_response(submenu).is_some(), "Right opened it");
        let inside = focused(&h).and_then(|id| h.ctx.read_response(id)).expect("an item has the focus");
        assert!(inside.id != sub && inside.rect.left() >= h.ctx.read_response(sub).unwrap().rect.right() - 1.0, "the focus moved into the submenu");
    }

    /// ks-295, ks-296: F6 goes round the document, the task pane, the status bar and the ribbon;
    /// Shift+F6 the other way.
    #[test]
    fn f6_goes_round_the_document_a_pane_the_status_bar_and_the_ribbon() {
        let mut h = harness("Text");
        h.state_mut().run("view.navigationPane", json!({"value": true})).unwrap();
        h.state_mut().canvas.want_focus = true;
        h.run_steps(3);
        assert!(h.state().canvas.focused);
        let r = recorded(&h);
        let stop = |region| r.stops.iter().find(|s| s.1 == region).map(|s| s.0);
        let (pane, status) = (stop(Region::Pane), stop(Region::Status));
        let home = r.tabs.iter().find(|t| t.1 == "Home").map(|t| t.0);
        assert!(pane.is_some() && status.is_some() && home.is_some());
        for expected in [pane, status, home] {
            press(&mut h, Key::F6, NONE);
            assert_eq!(focused(&h), expected);
            assert!(!h.state().canvas.focused);
        }
        press(&mut h, Key::F6, NONE);
        h.run_steps(1);
        assert!(h.state().canvas.focused, "round to the document");
        for expected in [home, status] {
            press(&mut h, Key::F6, SHIFT);
            assert_eq!(focused(&h), expected);
        }
        press(&mut h, Key::Escape, NONE);
        h.run_steps(2);
        assert!(h.state().canvas.focused);
    }

    /// ks-049, ks-043: Shift+F10 opens the shortcut menu at the caret, or a focused command's own
    /// one; Ctrl+Shift+F10 shows the focused command's tooltip.
    #[test]
    fn shift_f10_menus_and_ctrl_shift_f10_tooltips() {
        let mut h = harness("Text");
        press(&mut h, Key::F10, SHIFT);
        h.run_steps(1);
        assert!(h.state().canvas.context_menu_open);
        assert!(h.query_by_label_contains("Paragraph…").is_some(), "the page's shortcut menu");
        assert!(focused(&h).is_some() && !h.state().canvas.focused, "the menu has the keyboard");
        press(&mut h, Key::Escape, NONE);
        h.run_steps(2);
        assert!(!h.state().canvas.context_menu_open);
        assert!(h.state().canvas.focused);

        let bold = command(&h, "Bold");
        focus(&mut h, bold);
        assert!(h.query_by_label_contains("Bold (").is_none());
        press(&mut h, Key::F10, Modifiers { shift: true, ..CTRL });
        h.run_steps(1);
        assert!(h.query_by_label_contains("Bold (").is_some(), "the tooltip shows");
        press(&mut h, Key::F10, SHIFT);
        assert!(h.get_by_label("Add to Quick Access Toolbar").is_focused());
        press(&mut h, Key::Enter, NONE);
        h.run_steps(2);
        assert!(h.state().session.ribbon.qat().contains(&"format.bold"));
        assert!(h.state().canvas.focused, "the document has the keyboard again");
    }

    /// ks-299: Alt+F6 goes from a dialog box back to the document; the dialog stays open.
    #[test]
    fn alt_f6_goes_from_a_dialog_back_to_the_document() {
        let mut h = harness("Text");
        h.state_mut().run("ui.dialog", json!({"name": "find"})).unwrap();
        h.run_steps(3);
        if h.state().canvas.focused {
            // Into the dialog's first field, as a click there would.
            press(&mut h, Key::F6, NONE);
        }
        assert!(!h.state().canvas.focused);
        press(&mut h, Key::F6, ALT);
        h.run_steps(1);
        assert!(h.state().canvas.focused);
        assert!(h.state().dialog.is_some(), "the dialog stays open");
    }

    /// ks-109: typing the first letters of an AutoText entry's name shows a tip; Enter inserts the
    /// entry in place of the letters.
    #[test]
    fn autotext_tip_then_enter_inserts_the_entry() {
        let mut h = harness("Yours sincerely");
        h.state_mut().run("select.all", json!({})).unwrap();
        h.state_mut().run("insert.autoText", json!({"save": "Signoff"})).unwrap();
        h.state_mut().run("caret.docEnd", json!({})).unwrap();
        h.state_mut().run("text.newParagraph", json!({})).unwrap();
        for c in "Sign".chars() {
            h.input_mut().events.push(egui::Event::Text(c.to_string()));
            h.step();
        }
        assert_eq!(h.state().canvas.autotext_tip.as_deref(), Some("Signoff"));
        h.run_steps(1);
        assert!(h.query_by_label_contains("Press ENTER to Insert").is_some(), "the tip shows");
        press(&mut h, Key::Enter, NONE);
        assert_eq!(h.state().session.doc.plain_text(doc4me_doc::StoryRef::Body), "Yours sincerely\nYours sincerely");
        assert!(h.state().canvas.autotext_tip.is_none());
        // Another key puts the tip away; Enter then starts a new paragraph as usual.
        for c in "Sign".chars() {
            h.input_mut().events.push(egui::Event::Text(c.to_string()));
            h.step();
        }
        press(&mut h, Key::ArrowLeft, NONE);
        assert!(h.state().canvas.autotext_tip.is_none());
        press(&mut h, Key::Enter, NONE);
        assert!(h.state().session.doc.plain_text(doc4me_doc::StoryRef::Body).ends_with("Sig\nn"));
    }

    /// ks-265–ks-268: Read Mode's Home and End, a page number then Enter, and Escape.
    #[test]
    fn read_mode_home_end_page_number_and_escape() {
        let text = (0..300).map(|i| format!("Paragraph {i} of a long book, read a page at a time in Read Mode.")).collect::<Vec<_>>().join("\n");
        let mut h = harness(&text);
        h.state_mut().run("view.readMode", json!({})).unwrap();
        h.state_mut().canvas.want_focus = true;
        h.run_steps(6);
        assert!(h.state().canvas.cols >= 3, "{} pages", h.state().canvas.cols);
        let settle = |h: &mut Harness<'_, WordApp>| h.run_steps(60);
        press(&mut h, Key::End, NONE);
        settle(&mut h);
        let (x, max) = (h.state().canvas.scroll_offset.x, h.state().canvas.scroll_max.x);
        assert!(max > 0.0 && (x - max).abs() < 1.0, "End: the last page ({x} of {max})");
        press(&mut h, Key::Home, NONE);
        settle(&mut h);
        assert!(h.state().canvas.scroll_offset.x.abs() < 1.0, "Home: the first page");
        press(&mut h, Key::Num2, NONE);
        press(&mut h, Key::Enter, NONE);
        settle(&mut h);
        let x = h.state().canvas.scroll_offset.x;
        assert!(x > 1.0 && x < max - 1.0, "2, Enter: the second page ({x})");
        press(&mut h, Key::Escape, NONE);
        assert!(!h.state().session.view.read_mode, "Escape leaves Read Mode");
    }
}
