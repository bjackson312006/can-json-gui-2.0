use super::Navigator;
use json::CanJson;

use crate::frontend::components::scrollbar::ScrollbarState;

mod components;
mod render;

pub struct Editor {
    nav: Navigator,

    /// The CAN JSON we currently have open. This represents
    /// the state of the file in RAM (as we are actively operating on it), not
    /// the actual filestate written on the disk.
    file: CanJson,

    /// State of the scrollbar.
    scroll: ScrollbarState,

    /// Index of the currently-selected CAN message. If none is selected, then `None`.
    selected_index: Option<usize>,

}

impl Editor {
    pub fn new(nav: Navigator, file: CanJson) -> Self {
        Self {
            nav,
            file,
            scroll: ScrollbarState::new(),
            selected_index: None,
        }
    }

    /// Removes the message at `index` from the open file.
    pub fn remove_message(&mut self, index: usize) {
        self.file.remove_message(index);
    }

    /// Adds a new message to the end of the JSON, selects it, and scrolls it
    /// into view.
    ///
    /// The scrollbar thumb is laid out from the scroll offset during the render
    /// phase, but `scroll_to_bottom` only applies the new offset later in the
    /// same frame's prepaint. The scrollbar handles catching up by requesting a
    /// follow-up frame from within its own render (see `ScrollbarState`).
    pub fn add_message(&mut self) {
        self.file.add_message();
        self.selected_index = Some(self.file.messages().len() - 1); // select the new message
        self.scroll.scroll_to_bottom(); // new message renders at the bottom
    }
}