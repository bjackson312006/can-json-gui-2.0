use gpui::{Context, Render, Window, div, prelude::*, rgb, px};

use super::Editor;
use crate::frontend::components::sidebar::sidebar;
use crate::frontend::components::leftbar::leftbar;
use crate::frontend::assets::fonts::FontFace;

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .text_color(rgb(0xFFFFFF))
            .font_family("Cal Sans UI")
            .child(leftbar(self.nav.clone()))
            .child(sidebar(&self.scroll, self.file.messages()))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .justify_center()
                    .items_center()
                    .child(
                        div()
                            .child("Test Text")
                            .font_face(crate::frontend::assets::fonts::SatoshiLight)
                            .text_size(px(100.0))
                            .text_color(rgb(0xCCCCCC)),
                    ),
            )
    }
}
