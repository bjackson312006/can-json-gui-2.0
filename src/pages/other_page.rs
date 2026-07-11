use gpui::{Context, Render, Window, div, prelude::*, px, rgb};

use super::{Navigator, Page};
use crate::components::button;

pub struct OtherPage {
    nav: Navigator,
}

impl OtherPage {
    pub fn new(nav: Navigator) -> Self {
        Self { nav }
    }
}

impl Render for OtherPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .text_color(rgb(0xFFFFFF))
            .font_family("Cal Sans UI")
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(gpui::FontWeight(100.0))
                    .child("Other Page"),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x9C9C9C))
                    .child("nothing here but the buttons work"),
            )
            .child(
                button::button("back-to-home")
                    .rounded(px(10.0))
                    .text_size(px(12.0))
                    .font_weight(gpui::FontWeight(100.0))
                    .py(px(5.0))
                    .px(px(25.0))
                    .my(px(10.0))
                    .bg(rgb(0x2D2D2D))
                    .hover(|s| s.bg(rgb(0x3B3B3B)))
                    .on_click({
                        let nav = self.nav.clone();
                        move |_, _, cx| {
                            let next = Page::home(nav.clone(), cx);
                            nav.navigate(next, cx);
                        }
                    })
                    .child("back"),
            )
    }
}
