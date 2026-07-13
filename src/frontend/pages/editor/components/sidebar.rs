//! Sidebar that displays the big list of CAN messages in the editor page.

use gpui::{Context, Div, Hsla, Pixels, Stateful, div, point, prelude::*, px, rgb};

use super::super::Editor;
use crate::frontend::components::button;

const SIDEBAR_WIDTH_PX: f32 = 280.0;
const SIDEBAR_BG_COLOR: u32 = 0x181818;
const SIDEBAR_BORDER_COLOR: u32 = 0x2D2D2D;

impl Editor {
    /// The sidebar.
    pub(in crate::frontend::pages::editor) fn sidebar(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .h_full()
            .w(px(SIDEBAR_WIDTH_PX))
            .flex_none()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(0.0))
            .bg(rgb(SIDEBAR_BG_COLOR))
            .border_r_1()
            .border_color(rgb(SIDEBAR_BORDER_COLOR))
            .child(self.top_menu(cx))
            .child(self.cardlist(cx))
    }

    /// Top menu (thing that sits above the big list of messages)
    fn top_menu(&self, cx: &Context<Self>) -> impl IntoElement {
        const TOP_MENU_SHADOW_SIZE: Pixels = px(4.0);
        div()
            .bg(rgb(SIDEBAR_BG_COLOR))
            .w_full()
            .h(px(30.0))
            .pl(px(10.0))
            .pr(px(10.0))
            .flex()
            .items_center()
            .child(
                // Left-justified
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .font_weight(gpui::FontWeight(50.0))
                    .text_size(px(10.0))
                    .child("MESSAGES")
                    .text_color(rgb(0xCCCCCC))
                    .font_family("Cal Sans UI"),
            )
            .child(
                // Right-justified
                div()
                    .flex_none()
                    .pl(px(8.0))
                    .child(self.create_button(cx)),
            )
            .shadow(vec![gpui::BoxShadow {
                color: Hsla { h: 0., s: 0., l: 0., a: 0.4 },
                blur_radius: TOP_MENU_SHADOW_SIZE / 2.0,
                spread_radius: -TOP_MENU_SHADOW_SIZE / 2.0,
                offset: point(px(0.0), TOP_MENU_SHADOW_SIZE / 2.0),
            }])
    }

    /// Button for creating new messages.
    fn create_button(&self, cx: &Context<Self>) -> Stateful<Div> {
        const ICON_TEXT_COLOR: u32 = 0xb5b5b5;
        const ICON_HOVER_BACKGROUND: u32 = 0x2D2D2D;

        let editor = cx.entity().downgrade();
        button::button("create-message")
            .rounded(px(5.0))
            .p(px(3.0))
            .text_size(px(12.0))
            .text_color(rgb(ICON_TEXT_COLOR))
            .hover(|s| s.bg(rgb(ICON_HOVER_BACKGROUND)).text_color(rgb(ICON_TEXT_COLOR)))
            .child(
                crate::frontend::assets::icons::Create::get()
                    .size(px(13.0))
                    .text_color(rgb(ICON_TEXT_COLOR)),
            )
            .on_click(move |_ev, _window, cx| {
                if let Some(editor) = editor.upgrade() {
                    editor.update(cx, |editor, cx| {
                        editor.add_message();
                        cx.notify();
                    });
                }
            })
    }
}