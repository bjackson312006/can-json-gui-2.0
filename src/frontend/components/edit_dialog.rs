//! The "Edit" dropdown menu that opens from the titlebar.

use gpui::{
    BoxShadow, Context, Div, Hsla, Stateful, deferred, div, point, prelude::*, px, rgb,
};

use crate::frontend::components::button;
use crate::frontend::window::AppWindow;
use crate::frontend::window::TITLEBAR_HEIGHT_PX;

const MENU_BG: u32 = 0x1F1F1F;
const MENU_BORDER: u32 = 0x3A3A3A;
const ITEM_HOVER_BG: u32 = 0x2D2D2D;
const LABEL_COLOR: u32 = 0xCCCCCC;
const SHORTCUT_COLOR: u32 = 0x808080;
const DISABLED_COLOR: u32 = 0x5A5A5A;
const MENU_ROUNDED_PX: f32 = 4.0;
const MENU_WIDTH_PX: f32 = 160.0;

/// Edit dropdown menu.
pub fn edit_menu(undo_enabled: bool, cx: &mut Context<AppWindow>) -> gpui::AnyElement {
    deferred(
        div()
            .absolute()
            .top(px(TITLEBAR_HEIGHT_PX))
            .left_0()
            .w(px(MENU_WIDTH_PX))
            .occlude()
            .flex()
            .flex_col()
            .p(px(4.0))
            .rounded(px(MENU_ROUNDED_PX))
            .bg(rgb(MENU_BG))
            .border_1()
            .border_color(rgb(MENU_BORDER))
            .font_family("Cal Sans UI")
            .shadow(vec![BoxShadow {
                color: Hsla { h: 0., s: 0., l: 0., a: 0.3 },
                blur_radius: px(3.0),
                spread_radius: px(0.0),
                offset: point(px(0.0), px(2.0)),
            }])
            .child(item("em-undo", "Undo", "Ctrl+Z", undo_enabled, |w, cx| w.undo(cx), cx)),
    )
    // Draw on top of the dismiss backdrop
    .with_priority(1)
    .into_any_element()
}

/// Menu row in the edit dialog.
fn item(
    id: &'static str,
    label: &'static str,
    shortcut: &'static str,
    enabled: bool,
    action: impl Fn(&mut AppWindow, &mut Context<AppWindow>) + 'static,
    cx: &mut Context<AppWindow>,
) -> Stateful<Div> {
    let (label_color, shortcut_color) = if enabled {
        (LABEL_COLOR, SHORTCUT_COLOR)
    } else {
        (DISABLED_COLOR, DISABLED_COLOR)
    };

    let row = button::button(id)
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .gap(px(24.0))
        .px(px(10.0))
        .py(px(4.0))
        .rounded(px(4.0))
        .text_size(px(12.0))
        .font_weight(gpui::FontWeight(100.0))
        .child(div().text_color(rgb(label_color)).child(label))
        .child(div().text_color(rgb(shortcut_color)).child(shortcut));

    if enabled {
        row.cursor_pointer()
            .hover(|s| s.bg(rgb(ITEM_HOVER_BG)))
            .on_click(cx.listener(move |app, _, _, cx| action(app, cx)))
    } else {
        row.cursor_default()
    }
}
