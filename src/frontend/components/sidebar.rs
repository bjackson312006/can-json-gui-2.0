use gpui::{Div, ElementId, MouseButton, Axis, Stateful, div, prelude::*, px, rgb, svg};
use crate::frontend::Navigator;
use json::schema::{OdysseyMsg};
use crate::frontend::{Page, components::button};

const SIDEBAR_WIDTH_PX: f32 = 280.0;
const SIDEBAR_BG_COLOR: u32 = 0x181818;
const SIDEBAR_BORDER_COLOR: u32 = 0x2D2D2D;
const SIDEBAR_ICON_ACTIVE_COLOR: u32 = 0xFFFFFF;
const SIDEBAR_ICON_INACTIVE_COLOR: u32 = 0xCCCCCC;
const SIDEBAR_BG_ACTIVE_COLOR: u32 = 0x3B3B3B;
const SIDEBAR_BG_HOVER_COLOR: u32 = 0x2D2D2D;

pub fn sidebar(nav: Navigator, messages: &[OdysseyMsg]) -> impl IntoElement {
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
        .child(back(nav.clone()))
        .child(
            div()
                .w(px(38.0))
                .h(px(1.0))
                .my(px(6.0))
                .bg(rgb(SIDEBAR_BORDER_COLOR))
        )
        .child(crate::frontend::components::cardlist::cardlist(messages))
}

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

/// Scrollbar thumb (one of those little things you grab onto when scrolling)
fn scrollbar_thumb(
    group_name: &'static str,
    axis: Axis,
    thumb_offset: f32,
    thumb_thickness: f32,
    is_dragging: bool,
) -> impl IntoElement {
    const SCROLLBAR_THICKNESS_PX: f32 = 12.0;
    const SCROLLBAR_PADDING_PX: f32 = 2.0;
    const SCROLLBAR_THUMB_LENGTH_PX: f32 = 60.0;
    const SCROLLBAR_TRACK_COLOR: u32 = 0x1A1A1A;
    const SCROLLBAR_THUMB_COLOR: u32 = 0x4A4A4A;
    const SCROLLBAR_THUMB_HIGHLIGHT_COLOR: u32 = 0x6A6A6A;

    let pad = SCROLLBAR_PADDING_PX;
    div()
        .absolute()
        .map(|d| match axis {
            Axis::Horizontal => d
                .top(px(pad))
                .left(px(thumb_offset))
                .w(px(SCROLLBAR_THUMB_LENGTH_PX))
                .h(px(thumb_thickness)),
            Axis::Vertical => d
                .left(px(pad))
                .top(px(thumb_offset))
                .w(px(thumb_thickness))
                .h(px(SCROLLBAR_THUMB_LENGTH_PX)),
        })
        .rounded(px(thumb_thickness * 0.5))
        .bg(rgb(SCROLLBAR_THUMB_COLOR))
        .when(is_dragging, |s| s.bg(rgb(SCROLLBAR_THUMB_HIGHLIGHT_COLOR)))
        .group_hover(group_name, |s| s.bg(rgb(SCROLLBAR_THUMB_HIGHLIGHT_COLOR)))
}