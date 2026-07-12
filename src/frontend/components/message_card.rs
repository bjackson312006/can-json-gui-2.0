//! Message card!

use gpui::{Div, ElementId, MouseButton, Stateful, div, prelude::*, px, rgb, Pixels, Hsla, point};
use json::schema::{OdysseyMsg, MetaMsg, CANMsg};

fn meta_card(message: &MetaMsg) -> Div {
    div().child("meta mesage").child(message.desc.clone())
}

fn can_card(message: &CANMsg) -> Div {
    div().child("can mesage").child(message.id.clone())
}

const CARD_BORDER_COLOR: u32 = 0x454545;
const CARD_BORDER_SIZE: Pixels = px(1.0);
const CARD_SHADOW_SIZE: Pixels = px(4.0);
const CARD_HOVER_COLOR: u32 = 0x363636;
const CARD_BACKGROUND_COLOR: u32 = 0x2D2D2D;

/// Message card!
pub fn message_card(id: impl Into<ElementId>, message: &OdysseyMsg) -> Stateful<Div> {
    div()
        .id(id)
        .rounded(px(10.0))
        .text_size(px(12.0))
        .font_weight(gpui::FontWeight(100.0))
        .py(px(5.0))
        .px(px(25.0))
        .border(CARD_BORDER_SIZE)
        .border_color(rgb(CARD_BORDER_COLOR))
        .my(px(10.0))
        .bg(rgb(CARD_BACKGROUND_COLOR))
        .hover(|s| s.bg(rgb(CARD_HOVER_COLOR)))
        .child(
            match message {
                OdysseyMsg::Can(message) => can_card(message),
                OdysseyMsg::Meta(message) => meta_card(message),
            }
        )
        .shadow(
            vec![gpui::BoxShadow {
                color: Hsla { h: 0., s: 0., l: 0., a: 0.4 },
                blur_radius: CARD_SHADOW_SIZE / 2.,
                spread_radius: px(0.),
                offset: point(px(0.0), px(0.0)),
        }])
}
