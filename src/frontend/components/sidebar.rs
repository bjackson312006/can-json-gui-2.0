use gpui::{div, prelude::*, px, rgb};
use crate::frontend::Navigator;
use json::schema::{OdysseyMsg};
use crate::frontend::{Page, components::button};
use crate::frontend::components::scrollbar::ScrollbarState;

const SIDEBAR_WIDTH_PX: f32 = 280.0;
const SIDEBAR_BG_COLOR: u32 = 0x181818;
const SIDEBAR_BORDER_COLOR: u32 = 0x2D2D2D;
const SIDEBAR_ICON_ACTIVE_COLOR: u32 = 0xFFFFFF;
const SIDEBAR_ICON_INACTIVE_COLOR: u32 = 0xCCCCCC;
const SIDEBAR_BG_ACTIVE_COLOR: u32 = 0x3B3B3B;
const SIDEBAR_BG_HOVER_COLOR: u32 = 0x2D2D2D;

pub fn sidebar(scroll: &ScrollbarState, messages: &[OdysseyMsg]) -> impl IntoElement {
    div()
        .h_full()
        .w(px(SIDEBAR_WIDTH_PX))
        .flex_none()
        .flex()
        .flex_col()
        .items_center()
        .py(px(8.0))
        .gap(px(2.0))
        .bg(rgb(SIDEBAR_BG_COLOR))
        .border_r_1()
        .border_color(rgb(SIDEBAR_BORDER_COLOR))
        .child(crate::frontend::components::cardlist::cardlist(scroll, messages))
}