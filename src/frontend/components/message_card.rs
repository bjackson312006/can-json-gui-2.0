//! Message card!

use gpui::{Div, ElementId, MouseButton, Stateful, div, prelude::*, px, rgb};
use json::schema::{OdysseyMsg, MetaMsg, CANMsg};

fn meta_card(message: &MetaMsg) -> Div {
    div().child("meta mesage").child(message.desc.clone())
}

fn can_card(message: &CANMsg) -> Div {
    div().child("can mesage").child(message.id.clone())
}

/// Message card!
pub fn message_card(id: impl Into<ElementId>, message: &OdysseyMsg) -> Stateful<Div> {
    div()
        .id(id)
        .rounded(px(10.0))
        .text_size(px(12.0))
        .font_weight(gpui::FontWeight(100.0))
        .py(px(5.0))
        .px(px(25.0))
        .my(px(10.0))
        .bg(rgb(0x2D2D2D))
        .hover(|s| s.bg(rgb(0x3B3B3B)))
        .child(
            match message {
                OdysseyMsg::Can(message) => can_card(message),
                OdysseyMsg::Meta(message) => meta_card(message),
            }
        )
}
