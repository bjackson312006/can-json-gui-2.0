use gpui::{Context, Render, Window, div, prelude::*, px, rgb};

use super::{Page, Editor};
use crate::frontend::components::button;
use crate::frontend::components::message_card::message_card;
use crate::frontend::components::sidebar::sidebar;

impl Render for Editor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_row()
            .text_color(rgb(0xFFFFFF))
            .font_family("Cal Sans UI")
            .child(sidebar(self.nav.clone(), self.file.messages()))
    }
}
