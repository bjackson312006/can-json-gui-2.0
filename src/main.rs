#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod components;
mod pages;
mod window;

use std::borrow::Cow;

use assets::Assets;
use gpui::{
    App, Application, Bounds, TitlebarOptions, WindowBackgroundAppearance, WindowBounds,
    WindowDecorations, WindowOptions, prelude::*, px, size,
};
use pages::{Navigator, Page};
use window::AppWindow;

fn main() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        cx.text_system()
            .add_fonts(vec![
                Cow::Borrowed(include_bytes!("../fonts/cal-sans-ui/CalSansUI.wght.GEOM.ttf")),
            ])
            .unwrap();

        let bounds = Bounds::centered(None, size(px(600.0), px(600.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Opaque,
                window_decorations: Some(WindowDecorations::Client),
                window_min_size: Some(size(px(300.0), px(200.0))),
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    cx.observe_window_appearance(window, |_, window, _| {
                        window.refresh();
                    })
                    .detach();

                    let nav = Navigator::new(cx.weak_entity());
                    let home_page = Page::home(nav, cx);
                    AppWindow::new(home_page)
                })
            },
        )
        .unwrap();
    });
}
