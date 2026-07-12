use gpui::{Stateful, Context, Div, Render, Window, div, prelude::*, px, rgb};
use json::schema::OdysseyMsg;

/// Big list of message cards!
pub fn cardlist(messages: &[OdysseyMsg]) -> Stateful<Div> {
    div()
        .id("message-list")
        .flex_1()
        .min_h_0()
        .w(px(180.0))
        .overflow_y_scroll()
        .children(
            messages.iter().enumerate().map(|(i, message)| {
                super::message_card::message_card(("message-card", i), message)
            }),
        )
}