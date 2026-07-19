//! Module for the frontend of the app (i.e., all the graphical stuff).

mod assets;
mod components;
mod pages;
mod window;
mod update;

use assets::{Assets, fonts};
use gpui::{
    App, Application, Bounds, TitlebarOptions, WindowBackgroundAppearance, WindowBounds,
    WindowDecorations, WindowOptions, prelude::*, px, size,
};
use pages::{Navigator, Page};
use window::AppWindow;

/// Starts up the frontend application (i.e., opens up a window).
pub fn start() {
    Application::new().with_assets(Assets).run(|cx: &mut App| {
        cx.text_system()
            .add_fonts(vec![
                fonts::CalSansUiLight::get(),
                fonts::CalSansUiRegular::get(),
                fonts::CalSansUiMedium::get(),
                fonts::CalSansUiSemiBold::get(),
                fonts::CalSansUiBold::get(),
                fonts::SatoshiLight::get(),
                fonts::SatoshiRegular::get(),
                fonts::SatoshiMedium::get(),
                fonts::SatoshiBold::get(),
                fonts::SatoshiBlack::get(),
            ])
            .unwrap();

        window::bind_app_keys(cx);

        let bounds = Bounds::centered(None, size(px(1440.0), px(800.0)), cx);
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
                    let mut app_window = AppWindow::new(home_page, cx);
                    app_window.check_update(cx);
                    app_window
                })
            },
        )
        .unwrap();
    });
}