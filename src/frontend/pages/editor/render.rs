use gpui::{Context, Render, Window, div, prelude::*, rgb, px, Div};
use json::schema::OdysseyMsg;

use super::Editor;
use crate::frontend::components::leftbar::leftbar;
use crate::frontend::assets::fonts::FontFace;

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .text_color(rgb(0xFFFFFF))
            .font_family("Cal Sans UI")
            .child(leftbar(self.nav.clone()))
            .child(self.sidebar(cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .child(
                                match self.selected_index {
                                    None => {
                                        no_message_selected()
                                    },
                                    Some(index) => {
                                        self.message_selected(index)
                                    }
                                }
                            )
                    ),
            )
    }
}

impl Editor {
    /// Screen for when a message is selected.
    fn message_selected(&self, index: usize) -> Div {
        let text: String = match self.file.message(index) {
            OdysseyMsg::Can(message) => {
                message.id.clone()
            },
            OdysseyMsg::Meta(message) => {
                message.desc.clone()
            }
        };
        div()
            .child(text)
            .font_face(crate::frontend::assets::fonts::SatoshiLight)
            .text_size(px(100.0))
            .text_color(rgb(0xCCCCCC))
    }
}

/// No message selected screen.
fn no_message_selected() -> Div {
    const NO_MESSAGE_SELECTED_ICON_COLOR: u32 = 0x181818;
    const NO_MESSAGE_SELECTED_TEXT_COLOR: u32 = 0x181818;
    div()
        .flex()
        .flex_col()
        .text_center()
        .justify_center()
        .items_center()
        .child(
            crate::frontend::assets::icons::Messages::get()
                .size(px(250.0))
                .text_color(rgb(NO_MESSAGE_SELECTED_ICON_COLOR)),
        )
}