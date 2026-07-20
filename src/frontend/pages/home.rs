use std::path::{Path, PathBuf};

use gpui::{App, Context, Render, Window, div, prelude::*, px, rgb, rgba, point, Hsla};

use super::Navigator;
use crate::frontend::{assets::fonts::FontFace, components::button, update::{UpdateStatus, UpdateError}};
use chrono::{DateTime, Utc, Local};

const BACKGROUND_COLOR: u32 = 0x1F1F1F;
const RIGHT_COLUMN_COLOR: u32 = 0x2A2A2A;
const BOX_COLOR: u32 = 0x2A2A2A;
const ACCENT_BLUE: u32 = 0x1473E6;
const ACCENT_BLUE_HOVER: u32 = 0x2D7FF9;
const ACCENT_BLUE_DISABLED: u32 = 0x1473E659;
const DISABLED_TEXT_COLOR: u32 = 0xFFFFFF80;
const BORDER_COLOR: u32 = 0x454545;
const SUBTLE_TEXT_COLOR: u32 = 0xCCCCCC;
const MUTED_TEXT_COLOR: u32 = 0x808080;
const HOVER_COLOR: u32 = 0x3A3A3A;

// Which state the last update check is in.
enum LastChecked {
    /// A check completed at this time.
    Time(std::time::SystemTime),

    /// We are actively in the process of checking
    Checking,

    /// The last check attempt ended in an error
    Error(UpdateError),
}

/// The app's update checking state.
pub struct UpdateState {
    status: UpdateStatus,
    last_checked: LastChecked,
}

impl UpdateState {
    pub fn new() -> Self {
        Self {
            status: UpdateStatus::UpToDate,
            last_checked: LastChecked::Checking,
        }
    }

    /// Whether a newer release is available to download.
    pub fn update_available(&self) -> bool {
        matches!(self.status, UpdateStatus::Available(_))
    }

    /// Link to the available release's page, if an update is available.
    pub fn release_url(&self) -> Option<&str> {
        match &self.status {
            UpdateStatus::Available(info) => Some(&info.release_url),
            UpdateStatus::UpToDate => None,
        }
    }

    /// One-line availability summary shown in the Updates box.
    pub fn availability_text(&self) -> String {
        match &self.status {
            UpdateStatus::Available(update) => format!(
                "Update available! (current version: v{}, new version: v{})",
                env!("CARGO_PKG_VERSION"),
                update.version
            ),
            UpdateStatus::UpToDate => format!(
                "You are up to date! (current version: v{})",
                env!("CARGO_PKG_VERSION")
            ),
        }
    }

    /// The "last checked" time as text.
    pub fn last_checked_text(&self) -> String {
        match &self.last_checked {
            LastChecked::Time(time) => {
                let time: DateTime<Local> = (*time).into();
                time.format("%Y/%m/%d %I:%M %p").to_string()
            }
            LastChecked::Checking => String::from("Checking..."),
            LastChecked::Error(err) => match err {
                UpdateError::Network(_) => String::from("Error occured while checking: Network"),
                UpdateError::Parse(_) => String::from("Error occured while checking: Parse"),
                UpdateError::Semver(_) => String::from("Error occured while checking: Semver"),
            },
        }
    }

    /// Marks that a check is now in progress.
    pub fn set_checking(&mut self) {
        self.last_checked = LastChecked::Checking;
    }

    /// Records a completed check, stamped with the current time.
    pub fn set_result(&mut self, status: UpdateStatus) {
        self.status = status;
        self.last_checked = LastChecked::Time(std::time::SystemTime::now());
    }

    /// Records a failed check.
    pub fn set_error(&mut self, err: UpdateError) {
        self.last_checked = LastChecked::Error(err);
    }
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

    /// Opens the update's GitHub release page in the user's default web browser.
    /// The release to open comes from the update state cached on the AppWindow.
    fn download_update(&self, cx: &App) {
        let url = self
            .nav
            .read_app(cx, |app| app.update_state().release_url().map(str::to_string))
            .flatten();
        let Some(url) = url else { return };
        if let Err(err) = webbrowser::open(&url) {
            println!("failed to open the release page in a browser: {err}");
        }
    }

    /// Button that lets you download an update from GitHub if possible. When no update is available, button is disabled.
    fn download_update_button(&self, update_available: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let base = button::button("home-download-update")
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .px(px(24.0))
            .py(px(8.0))
            .rounded(px(10.0))
            .text_size(px(13.0))
            .font_face(crate::frontend::assets::fonts::CalSansUiBold)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child("Download (via GitHub)")
                    .line_height(gpui::relative(1.0)),
            );

        if update_available {
            base.bg(rgb(ACCENT_BLUE))
                .hover(|s| s.bg(rgb(ACCENT_BLUE_HOVER)))
                .text_color(rgb(0xFFFFFF))
                .cursor_pointer()
                .on_click(cx.listener(|this, _ev, _window, cx| {
                    this.download_update(cx);
                }))
        } else {
            base.bg(rgba(ACCENT_BLUE_DISABLED))
                .text_color(rgba(DISABLED_TEXT_COLOR))
        }
    }

    /// Small icon button that re-runs the update check.
    fn recheck_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        const ICON_TEXT_COLOR: u32 = 0xB5B5B5;
        const ICON_HOVER_BACKGROUND: u32 = 0x4A4949;

        button::button("home-recheck-update")
            .rounded(px(5.0))
            .p(px(3.0))
            .text_color(rgb(ICON_TEXT_COLOR))
            .hover(|s| s.bg(rgb(ICON_HOVER_BACKGROUND)).text_color(rgb(ICON_TEXT_COLOR)))
            .child(
                crate::frontend::assets::icons::Reload::get()
                    .size(px(11.0))
                    .text_color(rgb(ICON_TEXT_COLOR)),
            )
            .on_click(cx.listener(|this, _ev, _window, cx| {
                this.nav.with_app(cx, |app, cx| app.check_update(cx));
            }))
    }
}

impl Render for HomePage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const BUTTONS_ROUNDED: f32 = 10.0;

        // Below this window width, hide the right column
        const RIGHT_COLUMN_MIN_WIDTH: f32 = 1200.0;
        let show_right_column = window.viewport_size().width >= px(RIGHT_COLUMN_MIN_WIDTH);
        let left_column_width = if show_right_column {
            gpui::relative(0.6)
        } else {
            gpui::relative(1.0)
        };

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

        
        // Update state is cached on the AppWindow so it survives navigation.
        let (update_available, availability_text, last_checked_text) = self
            .nav
            .read_app(cx, |app| {
                let state = app.update_state();
                (
                    state.update_available(),
                    state.availability_text(),
                    state.last_checked_text(),
                )
            })
            .unwrap_or_else(|| (false, String::new(), String::from("Checking...")));
        let updates_box = div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .min_w(px(460.0))
            .border(px(2.0))
            .border_color(rgb(BORDER_COLOR))
            .p(px(20.0))
            .rounded(px(BUTTONS_ROUNDED))
            .bg(rgb(BOX_COLOR))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        crate::frontend::assets::icons::ExclamationCircle::get()
                            .size(px(18.0))
                            .text_color(rgb(0xEEEEEE)),
                    )
                    .child(
                        div()
                            .text_color(rgb(0xEEEEEE))
                            .text_size(px(15.0))
                            .font_face(crate::frontend::assets::fonts::CalSansUiBold)
                            .child("Check for updates"),
                    )
                    .child(self.recheck_button(cx)),
            )
            .child(
                div()
                    .text_color(rgb(MUTED_TEXT_COLOR))
                    .text_size(px(12.0))
                    .line_height(gpui::relative(1.3))
                    .child(availability_text)
            )
            .child(
                div()
                    .text_color(rgb(MUTED_TEXT_COLOR))
                    .text_size(px(12.0))
                    .line_height(gpui::relative(1.3))
                    .child(format!("Last checked: {}", last_checked_text)),
            )
            .child(self.download_update_button(update_available, cx));
        div()
            .size_full()
            .flex()
            .flex_row()
            .text_color(rgb(0xFFFFFF))
            .font_family("Cal Sans UI")
            .child(
                div()
                    .w(left_column_width)
                    .h_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .bg(rgb(BACKGROUND_COLOR))
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
                                    .child("can-json-gui-2.0"),
                            )
                            .child(action_box),
                    ),
            )
            .children(show_right_column.then(|| {
                div()
                    .w(gpui::relative(0.4))
                    .h_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .bg(rgb(RIGHT_COLUMN_COLOR))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_start()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_color(rgb(0xEEEEEE))
                                    .text_size(px(25.0))
                                    .font_face(crate::frontend::assets::fonts::CalSansUiBold)
                                    .child("Updates"),
                            )
                            .child(updates_box),
                    )
            }))
    }
}
