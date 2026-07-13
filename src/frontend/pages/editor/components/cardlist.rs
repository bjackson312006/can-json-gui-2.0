//! Big list of message cards, with a scrollbar in its own column to the right.

use gpui::{Context, div, prelude::*, px};

use super::super::Editor;
use crate::frontend::components::scrollbar::Scrollbar;

impl Editor {
    /// Big list of message cards, with a scrollbar in its own column to the right.
    pub(super) fn cardlist(&self, cx: &Context<Self>) -> impl IntoElement {
        let messages = self.file.messages();

        if messages.is_empty() {
            div()
                .flex()
                .flex_row()
                .flex_1()
                .min_h_0()
                .w_full()
                .items_center()
                .text_center()
                .justify_center()
                .child("No messages")
                .font_weight(gpui::FontWeight(50.0))
                .text_size(px(12.0))
        } else {
            div()
                .flex()
                .flex_row()
                .flex_1()
                .min_h_0()
                .w_full()
                .px(px(12.0))
                .child(
                    div()
                        .id("message-list")
                        .flex_1()
                        .min_w_0()
                        .h_full()
                        .overflow_y_scroll()
                        .track_scroll(self.scroll.handle())
                        .children(messages.iter().enumerate().map(|(i, _message)| {
                            self.message_card(("message-card", i), i, cx)
                        }))
                        .pb(px(20.0)),
                )
                .child(Scrollbar::new(self.scroll.clone()))
        }
    }
}
