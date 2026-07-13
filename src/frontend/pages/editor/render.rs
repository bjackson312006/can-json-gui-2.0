use gpui::{Context, Render, Window, div, prelude::*, rgb, px};
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
                                    None => "No message selected".to_string(),
                                    Some(index) => {
                                        match self.file.message(index) {
                                            OdysseyMsg::Can(message) => {
                                                message.id.clone()
                                            },
                                            OdysseyMsg::Meta(message) => {
                                                message.desc.clone()
                                            }
                                        }
                                    }
                                }
                            )
                            .font_face(crate::frontend::assets::fonts::SatoshiLight)
                            .text_size(px(100.0))
                            .text_color(rgb(0xCCCCCC)),
                    ),
            )
    }
}
