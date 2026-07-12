use gpui::{Context, Render, Window, div, prelude::*, rgb};

use super::Editor;
use crate::frontend::components::sidebar::sidebar;
use crate::frontend::components::leftbar::leftbar;

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
    }
}
