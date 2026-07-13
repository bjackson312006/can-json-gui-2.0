use super::Navigator;
use json::CanJson;
use std::collections::VecDeque;

use crate::frontend::components::scrollbar::ScrollbarState;

mod components;
mod render;

/// The undo history of the editor.
/// This literally just holds the entire state of the file for every change you
/// make, going back `capacity` changes. Many would say this is inefficient
/// 
/// Also general rule of thumb: Whenever `CanJson` mutates itself we gotta
/// push a new CanJson to the undo history. See `Editor::update_undo_history()`.
struct UndoHistory {
    items: VecDeque<CanJson>,
    capacity: usize,
}

impl UndoHistory {
    fn new(capacity: usize) -> Self {
        UndoHistory {
            items: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Pushes a new CanJson to the front of the undo history
    fn push(&mut self, item: CanJson) {
        if self.items.len() == self.capacity {
            self.items.pop_back(); // drop the oldest
        }
        self.items.push_front(item); // newest goes to the front
    }

    /// Take back the most recent in the undo history. Returns `None` if there's none left.
    fn undo(&mut self) -> Option<CanJson> {
        self.items.pop_front() // take back the most recent
    }
}

pub struct Editor {
    nav: Navigator,

    /// The CAN JSON we currently have open. This represents
    /// the state of the file in RAM (as we are actively operating on it), not
    /// the actual filestate written on the disk.
    file: CanJson,

    /// The undo history of the CanJson file.
    undo_history: UndoHistory,

    /// State of the scrollbar.
    scroll: ScrollbarState,

    /// Index of the currently-selected CAN message. If none is selected, then `None`.
    selected_index: Option<usize>,

    /// Current width of the sidebar, in pixels. Adjustable by dragging the
    /// sidebar's right edge.
    sidebar_width: f32,

    /// While the resize handle is being dragged, holds `(mouse_x_at_grab,
    /// sidebar_width_at_grab)` so drag moves resolve to a width delta.
    resize_start: Option<(f32, f32)>,
}

/// Size of the undo history (how many old `CanJson`s to store)
const UNDO_HISTORY_SIZE: usize = 20;

impl Editor {
    pub fn new(nav: Navigator, file: CanJson) -> Self {
        Self {
            nav,
            file,
            undo_history: UndoHistory::new(UNDO_HISTORY_SIZE),
            scroll: ScrollbarState::new(),
            selected_index: None,
            sidebar_width: components::sidebar::SIDEBAR_DEFAULT_WIDTH_PX,
            resize_start: None,
        }
    }

    /// Saves the current `CanJson` to the undo history. Called by `mutate`
    /// prior to each mutation.
    fn update_undo_history(&mut self) {
        self.undo_history.push(self.file.clone());
    }

    /// Runs a mutation against the open file, first snapshotting the current
    /// state into the undo history so the change can be reversed with `undo()`.
    ///
    /// Any mutations to `CanJson` should be routed through here!!
    pub fn mutate<R>(&mut self, f: impl FnOnce(&mut CanJson) -> R) -> R {
        self.update_undo_history();
        f(&mut self.file)
    }

    /// Removes the message at `index` from the open file.
    pub fn remove_message(&mut self, index: usize) {
        self.mutate(|file| file.remove_message(index));
    }

    /// Adds a new message to the end of the JSON, selects it, and scrolls it
    /// into view.
    pub fn add_message(&mut self) {
        self.mutate(|file| file.add_message());
        // Selection and scroll position are UI state, not file state, so they
        // live outside the undo snapshot.
        self.selected_index = Some(self.file.messages().len() - 1); // select the new message
        self.scroll.scroll_to_bottom(); // new message renders at the bottom
    }

    /// `Undo` operation. Replaces the current `CanJson` with the last stored `CanJson`.
    pub fn undo(&mut self) {
        if let Some(last_file) = self.undo_history.undo() {
            self.file = last_file;
        }
    }

    /// Provides read-only access to the current file being edited.
    pub fn file(&self) -> &CanJson {
        &self.file
    }
}