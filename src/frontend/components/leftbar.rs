//! Like the sidebar but even further left

use gpui::{div, prelude::*, px, rgb};
use crate::frontend::Navigator;
use json::schema::{OdysseyMsg};
use crate::frontend::{Page, components::button};
use crate::frontend::components::scrollbar::ScrollbarState;

const LEFTBAR_WIDTH_PX: f32 = 48.0;
const LEFTBAR_BG_COLOR: u32 = 0x181818;
const LEFTBAR_BORDER_COLOR: u32 = 0x2D2D2D;
const LEFTBAR_ICON_ACTIVE_COLOR: u32 = 0xFFFFFF;
const LEFTBAR_ICON_INACTIVE_COLOR: u32 = 0xCCCCCC;
const LEFTBAR_BG_ACTIVE_COLOR: u32 = 0x3B3B3B;
const LEFTBAR_BG_HOVER_COLOR: u32 = 0x2D2D2D;

/// That bar on the left side of the window.
pub fn leftbar(nav: Navigator) -> impl IntoElement {
    div()
        .h_full()
        .w(px(LEFTBAR_WIDTH_PX))
        .flex_none()
        .flex()
        .flex_col()
        .items_center()
        .py(px(8.0))
        .gap(px(2.0))
        .bg(rgb(LEFTBAR_BG_COLOR))
        .border_r_1()
        .border_color(rgb(LEFTBAR_BORDER_COLOR))
        .child(back(nav.clone()))
        .child(
            div()
                .w(px(24.0))
                .h(px(1.0))
                .my(px(6.0))
                .bg(rgb(LEFTBAR_BORDER_COLOR))
        )
}

/// The back button on the leftbar.
fn back(nav: Navigator) -> impl IntoElement {
    const BACK_ICON_TEXT_COLOR: u32 = 0xCCCCCC;
    const BACK_BG_HOVER_COLOR: u32 = 0x2D2D2D;
    button::button("back-to-home")
        .w(px(36.0))
        .h(px(36.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(6.0))
        .hover(|s| s.bg(rgb(BACK_BG_HOVER_COLOR)))
        .on_click(move |_, _, cx| {
            let next = Page::home(nav.clone(), cx);
            nav.navigate(next, cx);
        })
            .child(
                crate::frontend::assets::icons::ArrowLeft::get()
                .size(px(16.0))
                .text_color(rgb(BACK_ICON_TEXT_COLOR))
        )
}