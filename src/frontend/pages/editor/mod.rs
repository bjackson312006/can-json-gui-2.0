use super::Navigator;
use json::CanJson;

use crate::frontend::components::scrollbar::ScrollbarState;

mod render;

pub struct Editor {
    nav: Navigator,
    file: CanJson,
    /// Scroll + drag state for the message list's scrollbar. Lives here because
    /// it must persist across renders; the stateless list/scrollbar components
    /// borrow it each frame.
    scroll: ScrollbarState,
}

impl Editor {
    pub fn new(nav: Navigator, file: CanJson) -> Self {
        Self {
            nav,
            file,
            scroll: ScrollbarState::new(),
        }
    }
}