use super::{Navigator, Page};
use json::CanJson;

mod render;

pub struct Editor {
    nav: Navigator,
    file: CanJson,
}

impl Editor {
    pub fn new(nav: Navigator, file: CanJson) -> Self {
        Self { nav, file }
    }
}