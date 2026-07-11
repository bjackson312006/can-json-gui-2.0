use gpui::{Div, ElementId, MouseButton, Stateful, div, prelude::*};

/// Builds a button starting point: an interactive `Stateful<Div>` with an id
/// and a `mouse_down` handler that stops propagation. Callers chain their own
/// styling and `on_click` on top.
pub fn button(id: impl Into<ElementId>) -> Stateful<Div> {
    div()
        .id(id)
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}
