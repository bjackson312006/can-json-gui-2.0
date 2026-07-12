use gpui::{div, prelude::*, px};
use json::schema::OdysseyMsg;

use crate::frontend::components::scrollbar::{Scrollbar, ScrollbarState};

/// Big list of message cards, with a scrollbar in its own column to the right.
pub fn cardlist(scroll: &ScrollbarState, messages: &[OdysseyMsg]) -> impl IntoElement {
    if messages.len() == 0 {
        div()
         .flex()
         .flex_row()
         .flex_1()
         .min_h_0()
         .w(px(220.0))
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
        .w(px(220.0))
        .child(
            div()
                .id("message-list")
                .flex_1()
                .min_w_0()
                .h_full()
                .overflow_y_scroll()
                .track_scroll(scroll.handle())
                .children(messages.iter().enumerate().map(|(i, message)| {
                    super::message_card::message_card(("message-card", i), message)
                }))
                .pb(px(20.0)),
        )
        .child(Scrollbar::new(scroll.clone()))
            .mb(px(10.0))
            .mt(px(10.0))
    }
}
