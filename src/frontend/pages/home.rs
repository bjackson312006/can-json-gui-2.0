use std::path::{Path, PathBuf};

use gpui::{Context, Render, Window, div, prelude::*, px, rgb, point, Hsla};

use super::Navigator;
use crate::frontend::{assets::fonts::FontFace, components::button};

const BACKGROUND_COLOR: u32 = 0x1F1F1F;
const BOX_COLOR: u32 = 0x2A2A2A;
const ACCENT_BLUE: u32 = 0x1473E6;
const ACCENT_BLUE_HOVER: u32 = 0x2D7FF9;
const BORDER_COLOR: u32 = 0x454545;
const SUBTLE_TEXT_COLOR: u32 = 0xCCCCCC;
const MUTED_TEXT_COLOR: u32 = 0x808080;
const HOVER_COLOR: u32 = 0x3A3A3A;

/// debug button
fn check_update_button() -> impl IntoElement {
    button::button("home-check-update")
        .flex()
        .items_center()
        .justify_center()
        .px(px(16.0))
        .py(px(8.0))
        .rounded(px(10.0))
        .bg(rgb(BOX_COLOR))
        .border(px(1.0))
        .border_color(rgb(BORDER_COLOR))
        .hover(|s| s.bg(rgb(HOVER_COLOR)))
        .text_color(rgb(SUBTLE_TEXT_COLOR))
        .text_size(px(12.0))
        .child("Check for updates (debug)")
        .on_click(|_, _, cx| {
            cx.background_spawn(async {
                use crate::frontend::update::{check_for_update, UpdateStatus};
                match check_for_update() {
                    Ok(UpdateStatus::UpToDate) => println!("we're up to date!!!"),
                    Ok(UpdateStatus::Available(update_info)) => {
                        println!("update available: {:?}", update_info);
                    }
                    Err(err) => println!("update error: {}", err),
                }
            })
            .detach();
        })
}

pub struct HomePage {
    nav: Navigator,

    /// List of recently-opened files
    recent: Vec<PathBuf>,
}

impl HomePage {
    pub fn new(nav: Navigator) -> Self {
        Self {
            nav,
            recent: crate::backend::recent::load(),
        }
    }

    /// Removes `path` from the recent list, both on disk and in the live view.
    fn remove_recent(&mut self, path: &Path, cx: &mut Context<Self>) {
        crate::backend::recent::remove(path);
        self.recent.retain(|existing| existing != path);
        cx.notify();
    }
}

impl Render for HomePage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const BUTTONS_ROUNDED: f32 = 10.0;

        let open_button = button::button("home-open")
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .px(px(24.0))
            .py(px(8.0))
            .rounded(px(BUTTONS_ROUNDED))
            .bg(rgb(ACCENT_BLUE))
            .hover(|s| s.bg(rgb(ACCENT_BLUE_HOVER)))
            .text_color(rgb(0xFFFFFF))
            .text_size(px(13.0))
            .font_face(crate::frontend::assets::fonts::CalSansUiBold)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        crate::frontend::assets::icons::FolderOpen::get()
                            .size(px(16.0))
                            .text_color(rgb(0xFFFFFF)),
                    )
                    .child("Open")
                    .line_height(gpui::relative(1.0))
            )
            .on_click({
                let nav = self.nav.clone();
                move |_, _, cx| {
                    nav.with_app(cx, |app, cx| app.file_open(cx));
                }
            });

        let new_button = button::button("home-new")
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .px(px(24.0))
            .py(px(8.0))
            .rounded(px(BUTTONS_ROUNDED))
            .bg(rgb(BACKGROUND_COLOR))
            .border(px(1.0))
            .border_color(rgb(BORDER_COLOR))
            .hover(|s| s.bg(rgb(0x2D2D2D)))
            .text_color(rgb(SUBTLE_TEXT_COLOR))
            .text_size(px(13.0))
            .font_face(crate::frontend::assets::fonts::CalSansUiBold)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        crate::frontend::assets::icons::SquareRoundedPlus::get()
                            .size(px(16.0))
                            .text_color(rgb(SUBTLE_TEXT_COLOR)),
                    )
                    .child("New")
                    .line_height(gpui::relative(1.0))
            )
            .on_click({
                let nav = self.nav.clone();
                move |_, _, cx| {
                    nav.with_app(cx, |app, cx| app.file_new(cx));
                }
            });

        let left_section = div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .min_w(px(140.0))
            .child(open_button)
            .child(new_button);

        let recent_list = if self.recent.is_empty() {
            div().child(
                div()
                    .text_color(rgb(MUTED_TEXT_COLOR))
                    .text_size(px(12.0))
                    .child("No recent files"),
            )
        } else {
            let home = cx.entity().downgrade();
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .children(self.recent.iter().enumerate().map(|(ix, path)| {
                    let filename: String = match path.file_name() {
                        Some(filename) => { 
                            match filename.to_str() {
                                Some(string) => string.into(),
                                None => "UNKNOWN FILENAME".into(),
                            }
                        },
                            None => "UNKNOWN FILENAME".into(),
                    };
                    button::button(("recent-file", ix))
                        .flex()
                        .items_center()
                        .justify_between()
                        .gap(px(8.0))
                        .w_full()
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(BUTTONS_ROUNDED))
                        .text_color(rgb(SUBTLE_TEXT_COLOR))
                        .text_size(px(12.0))
                        .hover(|s| s.bg(rgb(HOVER_COLOR)))
                        .child(
                            div()
                                .min_w_0()
                                .truncate()
                                .child(filename),
                        )
                        .child(
                            button::button(("recent-remove", ix))
                                .rounded(px(5.0))
                                .p(px(3.0))
                                .text_color(rgb(MUTED_TEXT_COLOR))
                                .hover(|s| s.bg(rgb(0x4A4949)).text_color(rgb(SUBTLE_TEXT_COLOR)))
                                .child(
                                    crate::frontend::assets::icons::Close::get()
                                        .size(px(10.0))
                                        .text_color(rgb(MUTED_TEXT_COLOR)),
                                )
                                .on_click({
                                    let home = home.clone();
                                    let path = path.clone();
                                    move |_, _, cx| {
                                        if let Some(home) = home.upgrade() {
                                            home.update(cx, |home, cx| home.remove_recent(&path, cx));
                                        }
                                    }
                                }),
                        )
                        .on_click({
                            let nav = self.nav.clone();
                            let path = path.clone();
                            move |_, _, cx| {
                                nav.with_app(cx, |app, cx| app.open_path(path.clone(), cx));
                            }
                        })
                }))
        };

        let right_section = div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .min_w(px(180.0))
            .child(
                div()
                    .text_color(rgb(MUTED_TEXT_COLOR))
                    .text_size(px(11.0))
                    .child("RECENT"),
            )
            .child(recent_list);
        
        const ACTION_BOX_BORDER_COLOR: u32 = 0x454545;
        let action_box = div()
            .flex()
            .flex_row()
            .border(px(2.0))
            .border_color(rgb(ACTION_BOX_BORDER_COLOR))
            .gap(px(24.0))
            .p(px(20.0))
            .rounded(px(BUTTONS_ROUNDED))
            .bg(rgb(BOX_COLOR))
            .child(left_section)
            .child(right_section);

        const COLUMN_GAP: f32 = 300.0;

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .text_color(rgb(0xFFFFFF))
            .font_family("Cal Sans UI")
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(px(COLUMN_GAP))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_start()
                            .gap(px(16.0))
                            .child(
                                div()
                                    .text_color(rgb(0xEEEEEE))
                                    .text_size(px(40.0))
                                    .font_face(crate::frontend::assets::fonts::CalSansUiBold)
                                    .child("can-json-gui"),
                            )
                            .child(action_box),
                    )
                    // Right col thats empty for rn
                    .child(check_update_button()),
            )
    }
}
