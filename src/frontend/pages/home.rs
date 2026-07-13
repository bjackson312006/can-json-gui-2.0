use std::path::{Path, PathBuf};

use gpui::{Context, Render, Window, div, prelude::*, px, rgb};

use super::{Navigator, Page};
use crate::frontend::{assets::fonts::FontFace, components::button};
use json::CanJson;

const BACKGROUND_COLOR: u32 = 0x1F1F1F;
const BOX_COLOR: u32 = 0x2A2A2A;
const ACCENT_BLUE: u32 = 0x1473E6;
const ACCENT_BLUE_HOVER: u32 = 0x2D7FF9;
const BORDER_COLOR: u32 = 0x454545;
const SUBTLE_TEXT_COLOR: u32 = 0xCCCCCC;
const MUTED_TEXT_COLOR: u32 = 0x808080;
const HOVER_COLOR: u32 = 0x3A3A3A;

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

    /// Opens `file` in the editor, or logs the error and stays put.
    fn open_file(
        nav: &Navigator,
        cx: &mut gpui::App,
        file: Result<CanJson, impl std::fmt::Debug>,
    ) {
        match file {
            Ok(file) => {
                let next = Page::editor(nav.clone(), cx, file);
                nav.navigate(next, cx);
            }
            Err(err) => eprintln!("Failed to open file: {err:?}"),
        }
    }
}

impl Render for HomePage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(crate::frontend::assets::icons::FolderOpen::get())
                        .size(px(13.0))
                        .text_color(rgb(0xCCCCCC))
                    .child("Open")
                )
            .font_face(crate::frontend::assets::fonts::CalSansUiBold)
            .on_click({
                let nav = self.nav.clone();
                move |_, _, cx| Self::open_file(&nav, cx, CanJson::open())
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
            .child("New")
            .on_click({
                let nav = self.nav.clone();
                move |_, _, cx| Self::open_file(&nav, cx, CanJson::new())
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
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(5.0))
                        .text_color(rgb(SUBTLE_TEXT_COLOR))
                        .text_size(px(12.0))
                        .hover(|s| s.bg(rgb(HOVER_COLOR)))
                        .child(filename)
                        .on_click({
                            let nav = self.nav.clone();
                            let path = path.clone();
                            move |_, _, cx| {
                                Self::open_file(&nav, cx, CanJson::read(path.clone()))
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
                    .child("Recent"),
            )
            .child(recent_list);

        let action_box = div()
            .flex()
            .flex_row()
            .gap(px(24.0))
            .p(px(20.0))
            .rounded(px(12.0))
            .bg(rgb(BOX_COLOR))
            .child(left_section)
            .child(right_section);

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
    }
}
