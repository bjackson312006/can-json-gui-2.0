//! Module for the frontend window's different pages.

mod home;
mod editor;

pub use home::HomePage;
pub use editor::Editor;

use gpui::{AnyView, App, Entity, WeakEntity, prelude::*};

use crate::frontend::window::AppWindow;
use json::CanJson;

/// A handle pages use to swap themselves out for another page.
/// Holds a `WeakEntity<AppWindow>` so pages don't keep the window alive.
#[derive(Clone)]
pub struct Navigator {
    app: WeakEntity<AppWindow>,
}

impl Navigator {
    pub fn new(app: WeakEntity<AppWindow>) -> Self {
        Self { app }
    }

    pub fn navigate(&self, page: Page, cx: &mut App) {
        if let Some(app) = self.app.upgrade() {
            app.update(cx, |app, cx| app.set_page(page, cx));
        }
    }
}

pub enum Page {
    Home(Entity<HomePage>),
    Editor(Entity<Editor>),
}

impl Page {
    pub fn home(nav: Navigator, cx: &mut App) -> Self {
        Page::Home(cx.new(|_| HomePage::new(nav)))
    }

    pub fn editor(nav: Navigator, cx: &mut App, file: CanJson) -> Self {
        Page::Editor(cx.new(|_| Editor::new(nav, file)))
    }

    pub fn into_view(&self) -> AnyView {
        match self {
            Page::Home(entity) => entity.clone().into(),
            Page::Editor(entity) => entity.clone().into(),
        }
    }
}
