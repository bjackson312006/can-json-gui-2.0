//! Message card!

use gpui::{Context, Div, ElementId, Hsla, Pixels, Stateful, div, point, prelude::*, px, rgb};
use json::schema::{CANMsg, MetaMsg, OdysseyMsg};

use super::super::Editor;
use crate::frontend::assets::fonts::{CalSansUiBold, CalSansUiLight, FontFace};
use crate::frontend::components::button;

const CARD_BORDER_COLOR: u32 = 0x454545;
const CARD_BORDER_SIZE: Pixels = px(1.0);
const CARD_SHADOW_SIZE: Pixels = px(4.0);
const CARD_HOVER_COLOR: u32 = 0x363636;
const CARD_BACKGROUND_COLOR: u32 = 0x2D2D2D;
const CARD_SELECTED_BACKGROUND_COLOR: u32 = 0x26364A;
const CARD_SELECTED_BORDER_COLOR: u32 = 0x4E7DBA;
const CARD_SELECTED_HOVER_COLOR: u32 = 0x2E4460;

impl Editor {
    /// Outer message card!
    pub(super) fn message_card(&self, id: impl Into<ElementId>, message_index: usize, cx: &Context<Self>) -> Stateful<Div> {
        let selected: bool = self.selected_index == Some(message_index);
        
        let editor = cx.entity().downgrade();
        let border_color = if selected { CARD_SELECTED_BORDER_COLOR } else { CARD_BORDER_COLOR };
        let background_color = if selected { CARD_SELECTED_BACKGROUND_COLOR } else { CARD_BACKGROUND_COLOR };
        let hover_color = if selected { CARD_SELECTED_HOVER_COLOR } else { CARD_HOVER_COLOR };
        let shadow_color = Hsla { h: 0., s: 0., l: 0., a: 0.4 };
        let shadow_blur_radius = CARD_SHADOW_SIZE / 2.0;
        div()
            .id(id)
            .w_full()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .rounded(px(10.0))
            .text_size(px(12.0))
            .font_weight(gpui::FontWeight(100.0))
            .py(px(5.0))
            .px(px(25.0))
            .border(CARD_BORDER_SIZE)
            .border_color(rgb(border_color))
            .my(px(10.0))
            .bg(rgb(background_color))
            .hover(|s| s.bg(rgb(hover_color)))
            .child(
                // Left-justified section
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .child(match self.file.message(message_index) {
                        OdysseyMsg::Can(message) => can_card(message),
                        OdysseyMsg::Meta(message) => meta_card(message),
                    }),
            )
            .child(
                // Right-justified section
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(4.0))
                    .pl(px(8.0))
                    .child(self.duplicate_button(message_index, cx))
                    .child(self.delete_button(message_index, cx))
            )
            .on_click(move |_ev, _window, cx| {
                if let Some(editor) = editor.upgrade() {
                    editor.update(cx, |editor, cx| {
                        if selected {
                            editor.selected_index = None;
                        } else {
                            editor.selected_index = Some(message_index);
                        }
                        cx.notify();
                    });
                }
            })
            .shadow(vec![gpui::BoxShadow {
                color: shadow_color,
                blur_radius: shadow_blur_radius,
                spread_radius: px(0.),
                offset: point(px(0.0), px(0.0)),
            }])
    }

    /// A trash-can button that removes the message at `index` from the file.
    fn delete_button(&self, message_index: usize, cx: &Context<Self>) -> Stateful<Div> {
        const ICON_TEXT_COLOR: u32 = 0xb5b5b5;
        const ICON_HOVER_BACKGROUND: u32 = 0x4a4949;
        const ICON_SELECTED_HOVER_BACKGROUND: u32 = 0x3c5575;

        let selected: bool = self.selected_index == Some(message_index);
        let hover_background = if selected { ICON_SELECTED_HOVER_BACKGROUND } else { ICON_HOVER_BACKGROUND };

        let editor = cx.entity().downgrade();
        button::button(("delete-message", message_index))
            .rounded(px(5.0))
            .p(px(3.0))
            .text_size(px(12.0))
            .text_color(rgb(ICON_TEXT_COLOR))
            .hover(|s| s.bg(rgb(hover_background)).text_color(rgb(ICON_TEXT_COLOR)))
            .child(
                crate::frontend::assets::icons::Close::get()
                    .size(px(11.0))
                    .text_color(rgb(ICON_TEXT_COLOR)),
            )
            .on_click(move |_ev, _window, cx| {
                if let Some(editor) = editor.upgrade() {
                    editor.update(cx, |editor, cx| {
                        editor.remove_message(message_index);
                        cx.notify();
                    });
                }
            })
    }

    fn duplicate_button(&self, message_index: usize, cx: &Context<Self>) -> Stateful<Div> {
        const ICON_TEXT_COLOR: u32 = 0xb5b5b5;
        const ICON_HOVER_BACKGROUND: u32 = 0x4a4949;
        const ICON_SELECTED_HOVER_BACKGROUND: u32 = 0x3c5575;

        let selected: bool = self.selected_index == Some(message_index);
        let hover_background = if selected { ICON_SELECTED_HOVER_BACKGROUND } else { ICON_HOVER_BACKGROUND };

        let editor = cx.entity().downgrade();
        button::button(("duplicate-message", message_index))
            .rounded(px(5.0))
            .p(px(3.0))
            .text_size(px(12.0))
            .text_color(rgb(ICON_TEXT_COLOR))
            .hover(|s| s.bg(rgb(hover_background)).text_color(rgb(ICON_TEXT_COLOR)))
            .child(
                crate::frontend::assets::icons::Duplicate::get()
                    .size(px(11.0))
                    .text_color(rgb(ICON_TEXT_COLOR)),
            )
            .on_click(move |_ev, _window, cx| {
                if let Some(editor) = editor.upgrade() {
                    editor.update(cx, |editor, cx| {
                        editor.duplicate_message(message_index);
                        cx.notify();
                    });
                }
            })
    }
}

fn meta_card(message: &MetaMsg) -> Div {
    div()
        .min_w_0()
        .child(
            div()
                .truncate()
                .child(message.desc.clone())
                .font_face(CalSansUiLight)
                .text_size(px(12.0))
                .text_color(rgb(0xedebeb)),
        )
        .child(
            div()
                .child("(Meta)")
                .font_face(CalSansUiBold)
                .text_size(px(10.0))
                .text_color(rgb(0xb8b8b8)),
        )
}

fn can_card(message: &CANMsg) -> Div {
    div()
        .min_w_0()
        .child(
            div()
                .truncate()
                .child(message.desc.clone())
                .font_face(CalSansUiLight)
                .text_size(px(12.0))
                .text_color(rgb(0xedebeb)),
        )
        .child(
            div()
                .child(message.id.clone())
                .font_face(CalSansUiBold)
                .text_size(px(10.0))
                .text_color(rgb(0xb8b8b8)),
        )
}
