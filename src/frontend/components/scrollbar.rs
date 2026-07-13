//! A draggable scrollbar for `overflow_*_scroll` regions.
//! Claude wrote this module! I love you claude
//! 
//! Who up scrolling they bar

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gpui::{
    App, IntoElement, Pixels, ScrollHandle, Window, div, point, prelude::*, px, rgb,
};

const SCROLLBAR_WIDTH_PX: f32 = 5.0;
const SCROLLBAR_GAP_PX: f32 = 6.0;
const THUMB_MIN_LEN_PX: f32 = 24.0;
const THUMB_COLOR: u32 = 0x4A4A4A;
const THUMB_HOVER_COLOR: u32 = 0x6A6A6A;

/// Space reserved at the top and bottom of the track that the thumb won't enter.
const THUMB_INSET_PX: f32 = 10.0;

// Marker
struct ThumbDrag;

/// Invisible drag preview
struct DragGhost;

impl Render for DragGhost {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div()
    }
}

#[derive(Clone)]
pub struct ScrollbarState {
    handle: ScrollHandle,
    drag: Rc<RefCell<Option<f32>>>,

    /// Set when a programmatic scroll (e.g. `scroll_to_bottom`) is requested.
    pending_scroll: Rc<Cell<bool>>,
}

impl Default for ScrollbarState {
    fn default() -> Self {
        Self::new()
    }
}

/// Thumb geometry for the current frame, in pixels.
struct Metrics {
    /// Maximum scroll distance.
    max: f32,
    /// Rendered length of the thumb.
    thumb_len: f32,
    /// Track distance the thumb can travel.
    travel: f32,
    /// Current top position of the thumb within the track.
    thumb_top: f32,
}

impl ScrollbarState {
    pub fn new() -> Self {
        Self {
            handle: ScrollHandle::new(),
            drag: Rc::new(RefCell::new(None)),
            pending_scroll: Rc::new(Cell::new(false)),
        }
    }

    /// Attach this to the scroll container: `div().overflow_y_scroll().track_scroll(state.handle())`.
    pub fn handle(&self) -> &ScrollHandle {
        &self.handle
    }

    /// Request a scroll to the bottom of the list.
    pub fn scroll_to_bottom(&self) {
        self.handle.scroll_to_bottom();
        self.pending_scroll.set(true);
    }

    fn is_active(&self) -> bool {
        self.drag.borrow().is_some()
    }

    /// Compute thumb geometry.
    fn metrics(&self) -> Option<Metrics> {
        let viewport = f32::from(self.handle.bounds().size.height);
        let max = f32::from(self.handle.max_offset().height);
        if viewport <= 0.0 || max <= 0.5 {
            return None;
        }
        // Usable track length after reserving an inset at the top and bottom.
        let track = (viewport - 2.0 * THUMB_INSET_PX).max(THUMB_MIN_LEN_PX);
        let thumb_len = (track * (viewport / (viewport + max)))
            .max(THUMB_MIN_LEN_PX)
            .min(track);
        let travel = (track - thumb_len).max(0.0);
        // Offset is negative as you scroll down, so negate to get 0..=max.
        let progress = (-f32::from(self.handle.offset().y) / max).clamp(0.0, 1.0);
        Some(Metrics {
            max,
            thumb_len,
            travel,
            thumb_top: THUMB_INSET_PX + progress * travel,
        })
    }

    /// Convert a window-space Y coordinate into a Y relative to the track top.
    fn track_y(&self, window_y: Pixels) -> f32 {
        f32::from(window_y - self.handle.bounds().origin.y)
    }

    /// Start a drag.
    fn begin_drag(&self, window_y: Pixels) {
        if let Some(m) = self.metrics() {
            *self.drag.borrow_mut() = Some(self.track_y(window_y) - m.thumb_top);
        }
    }

    /// Continue a drag: map the cursor position to a new scroll offset.
    fn drag_to(&self, window_y: Pixels) {
        let grab = match *self.drag.borrow() {
            Some(g) => g,
            None => return,
        };
        let Some(m) = self.metrics() else { return };
        if m.travel <= 0.0 {
            return;
        }
        let thumb_top =
            (self.track_y(window_y) - grab).clamp(THUMB_INSET_PX, THUMB_INSET_PX + m.travel);
        let progress = (thumb_top - THUMB_INSET_PX) / m.travel;
        let x = self.handle.offset().x;
        self.handle.set_offset(point(x, px(-(progress * m.max))));
    }

    fn end_drag(&self) {
        *self.drag.borrow_mut() = None;
    }
}

/// A scrollbar for a scroll region.
#[derive(IntoElement)]
pub struct Scrollbar {
    state: ScrollbarState,
}

impl Scrollbar {
    pub fn new(state: ScrollbarState) -> Self {
        Self { state }
    }
}

impl RenderOnce for Scrollbar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state;

        if state.is_active() && !cx.has_active_drag() {
            state.end_drag();
        }

        if state.pending_scroll.replace(false) {
            window.request_animation_frame();
        }

        let metrics = state.metrics();
        let dragging = state.is_active();

        div()
            .flex_none()
            .w(px(SCROLLBAR_WIDTH_PX))
            .h_full()
            .ml(px(SCROLLBAR_GAP_PX))
            .map(move |this| match metrics {
                None => this,
                Some(m) => this.child(
                    div()
                        .id("scrollbar-thumb")
                        .absolute()
                        .top(px(m.thumb_top))
                        .left_0()
                        .w_full()
                        .h(px(m.thumb_len))
                        .rounded(px(SCROLLBAR_WIDTH_PX * 0.5))
                        .bg(rgb(if dragging {
                            THUMB_HOVER_COLOR
                        } else {
                            THUMB_COLOR
                        }))
                        .hover(|s| s.bg(rgb(THUMB_HOVER_COLOR)))
                        .on_drag(ThumbDrag, {
                            let state = state.clone();
                            move |_, _, window, cx| {
                                state.begin_drag(window.mouse_position().y);
                                cx.new(|_| DragGhost)
                            }
                        })
                        .on_drag_move::<ThumbDrag>({
                            let state = state.clone();
                            move |e, window, _cx| {
                                state.drag_to(e.event.position.y);
                                window.refresh();
                            }
                        }),
                ),
            })
    }
}
