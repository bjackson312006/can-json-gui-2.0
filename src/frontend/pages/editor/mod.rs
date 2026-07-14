use super::Navigator;
use json::CanJson;
use json::schema::OdysseyMsg;
use std::collections::VecDeque;

use crate::frontend::components::scrollbar::ScrollbarState;

mod components;
mod render;

/// Editor state we wanna track for Undo
struct UndoSnapshot {
    messages: Vec<OdysseyMsg>,
    selected_index: Option<usize>,
}

/// The undo history of the editor.
/// This literally just holds a snapshot of the editor state for every change
/// you make, going back `capacity` changes. Many would say this is inefficient.
///
/// Also general rule of thumb: Whenever `CanJson`'s content is about to mutate
/// we gotta push the current editor state list to the undo history first. See
/// `Editor::update_undo_history()`.
struct UndoHistory {
    items: VecDeque<UndoSnapshot>,
    capacity: usize,
}

impl UndoHistory {
    fn new(capacity: usize) -> Self {
        UndoHistory {
            items: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Pushes a snapshot to the front of the undo history
    fn push(&mut self, item: UndoSnapshot) {
        if self.items.len() == self.capacity {
            self.items.pop_back(); // drop the oldest
        }
        self.items.push_front(item); // newest goes to the front
    }

    /// Take back the most recent in the undo history. Returns `None` if there's none left.
    fn undo(&mut self) -> Option<UndoSnapshot> {
        self.items.pop_front() // take back the most recent
    }

    /// Whether there's nothing left to undo.
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

pub struct Editor {
    nav: Navigator,
    file: CanJson,
    undo_history: UndoHistory,
    scroll: ScrollbarState,
    selected_index: Option<usize>,
    sidebar_width: f32,
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

    /// Snapshots the current message list into the undo history.
    fn update_undo_history(&mut self) {
        self.undo_history.push(UndoSnapshot {
            messages: self.file.messages().to_vec(),
            selected_index: self.selected_index,
        });
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
        self.selected_index = match self.selected_index {
            Some(i) if i == index => None,
            Some(i) if i > index => Some(i - 1),
            other => other,
        };
    }

    /// Adds a new message to the end of the JSON, selects it, and scrolls it into view.
    pub fn add_message(&mut self) {
        self.mutate(|file| file.add_message());
        self.selected_index = Some(self.file.messages().len() - 1); // select the new message
        self.scroll.scroll_to_bottom(); // new message renders at the bottom
    }

    /// Duplicates a message.
    pub fn duplicate_message(&mut self, index: usize) {
        self.mutate(|file| file.duplicate_message(index));
        self.selected_index = Some(index + 1);
    }

    /// `Undo` operation.
    pub fn undo(&mut self) {
        if let Some(snapshot) = self.undo_history.undo() {
            self.file.set_messages(snapshot.messages);
            self.selected_index = snapshot.selected_index;
            self.clamp_selection();
        }
    }

    /// Whether there is anything in the undo history to revert.
    pub fn can_undo(&self) -> bool {
        !self.undo_history.is_empty()
    }

    /// Clears `selected_index` if it no longer refers to an existing message.
    fn clamp_selection(&mut self) {
        if let Some(i) = self.selected_index {
            if i >= self.file.messages().len() {
                self.selected_index = None;
            }
        }
    }

    /// Provides read-only access to the current file being edited.
    pub fn file(&self) -> &CanJson {
        &self.file
    }

    /// Saves the open file to its current path.
    /// Don't need to use mutate() here (even tho &mut self) since this doesn't actually change anything inside
    /// the `CanJson` itself.
    pub fn save(&mut self) {
        if let Err(err) = self.file.save() {
            eprintln!("Save failed: {err:?}");
        }
    }

    /// Saves the open file to a new path picked via the OS dialog.
    /// Don't need to use mutate() here (even tho &mut self) since this doesn't actually change anything inside
    /// the `CanJson` itself.
    pub fn save_as(&mut self) {
        if let Err(err) = self.file.save_as() {
            eprintln!("Save-as failed: {err:?}");
        }
    }
}