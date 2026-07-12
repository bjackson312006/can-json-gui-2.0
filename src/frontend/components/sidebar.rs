//! Sidebar that displays the big list of CAN messages in the editor page.

use gpui::{div, prelude::*, px, rgb, Pixels, Hsla, point};
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

/// The sidebar.
pub fn sidebar(scroll: &ScrollbarState, messages: &[OdysseyMsg]) -> impl IntoElement {
    div()
        .h_full()
        .w(px(SIDEBAR_WIDTH_PX))
        .flex_none()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(2.0))
        .bg(rgb(SIDEBAR_BG_COLOR))
        .border_r_1()
        .border_color(rgb(SIDEBAR_BORDER_COLOR))
        .child(top_menu())
        .child(crate::frontend::components::cardlist::cardlist(scroll, messages))
}

/// Top menu (thing that sits above the big list of messages)
fn top_menu() -> impl IntoElement {
    const TOP_MENU_SHADOW_SIZE: Pixels = px(4.0);
    div()
        .bg(rgb(SIDEBAR_BG_COLOR))
        .w_full()
        .h(px(30.0))
        .pl(px(10.0))
        .flex()
        .items_center()
        .child(
            div()
                .font_weight(gpui::FontWeight(50.0))
                .text_size(px(10.0))
                .child("MESSAGES")
                .text_color(rgb(0xCCCCCC))
                .font_family("Cal Sans UI")
        )
        .shadow(
            vec![gpui::BoxShadow {
                color: Hsla { h: 0., s: 0., l: 0., a: 0.4 },
                blur_radius: TOP_MENU_SHADOW_SIZE / 2.0,
                spread_radius: -TOP_MENU_SHADOW_SIZE / 2.0,
                offset: point(px(0.0), TOP_MENU_SHADOW_SIZE / 2.0),
        }])
}