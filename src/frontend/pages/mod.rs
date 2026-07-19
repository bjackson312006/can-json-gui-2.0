//! Module for the frontend window's different pages.

mod home;
mod editor;

pub use home::HomePage;
pub use editor::Editor;

use gpui::{AnyView, App, Context, Entity, WeakEntity, prelude::*};

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
        self.with_app(cx, |app, cx| app.set_page(page, cx));
    }

    /// Runs `f` against the parent `AppWindow`, if it's still alive.
    /// 
    /// Realistically (I think) this should never return None during normal operation since `AppWindow` is the big guy that owns
    /// all the pages and such. It could probably return None during teardown (i.e., when the user closes the app) but returning None
    /// is fine then since the apps gonna be gone in like a second anyway
    pub fn with_app<R>(
        &self,
        cx: &mut App,
        f: impl FnOnce(&mut AppWindow, &mut Context<AppWindow>) -> R,
    ) -> Option<R> {
        self.app.upgrade().map(|app| app.update(cx, f))
    }
}

pub enum Page {
    Home(Entity<HomePage>),
    Editor(Entity<Editor>),
}

impl Page {
    pub fn home(nav: Navigator, cx: &mut App) -> Self {
        Page::Home(cx.new(|cx| {
            let mut page = HomePage::new(nav);
            // Check for an update at startup so the "Download Update" button
            // reflects whether one is available (it stays disabled otherwise).
            page.check_update(cx);
            page
        }))
    }

    pub fn editor(nav: Navigator, cx: &mut App, file: CanJson) -> Self {
        crate::backend::recent::push(file.path());
        Page::Editor(cx.new(|_| Editor::new(nav, file)))
    }

    pub fn into_view(&self) -> AnyView {
        match self {
            Page::Home(entity) => entity.clone().into(),
            Page::Editor(entity) => entity.clone().into(),
        }
    }
}
