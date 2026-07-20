use gpui::{
    App, Context, CursorStyle, Decorations, FocusHandle, HitboxBehavior, Hsla, KeyBinding,
    MouseButton, Pixels, Point, ResizeEdge, Size, Window, WindowControlArea, black, canvas, div,
    point, prelude::*, px, rgb, svg, transparent_black, IntoElement, Div
};

use crate::frontend::{
    assets::{fonts::FontFace, icons}, components::{button, file_dialog, edit_dialog}, pages::{Editor, Navigator, Page, UpdateState}, update::check_for_update,
};
use json::CanJson;

gpui::actions!(file_menu, [NewFile, OpenFile, SaveFile, SaveFileAs, GoHome, QuitApp]);
gpui::actions!(edit_menu, [Undo]);

/// Keybinds
pub fn bind_app_keys(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("ctrl-n", NewFile, None),
        KeyBinding::new("ctrl-o", OpenFile, None),
        KeyBinding::new("ctrl-s", SaveFile, None),
        KeyBinding::new("ctrl-shift-s", SaveFileAs, None),
        KeyBinding::new("ctrl-h", GoHome, None),
        KeyBinding::new("ctrl-q", QuitApp, None),
        KeyBinding::new("ctrl-z", Undo, None),
    ]);
}

pub struct AppWindow {
    page: Page,
    update_state: UpdateState,
    file_menu_open: bool,
    edit_menu_open: bool,
    focus_handle: FocusHandle,
}

impl AppWindow {
    pub fn new(page: Page, cx: &mut Context<Self>) -> Self {
        Self { page, update_state: UpdateState::new(), file_menu_open: false, edit_menu_open: false, focus_handle: cx.focus_handle() }
    }

    /// The cached update-check state.
    pub fn update_state(&self) -> &UpdateState {
        &self.update_state
    }

    /// Runs an update check in the background and refreshes the cached state.
    pub fn check_update(&mut self, cx: &mut Context<Self>) {
        self.update_state.set_checking();
        cx.notify();
        cx.spawn(async move |this, cx| {
            gpui::Timer::after(std::time::Duration::from_secs(1)).await; // rate-limit so you can't request too fast
            let result = cx.background_spawn(async { check_for_update() }).await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(status) => this.update_state.set_result(status),
                    Err(err) => {
                        // just gonna print it out for now but probably have a real GUI error in the future
                        println!("error fetching update status: {}", err);
                        this.update_state.set_error(err);
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    pub fn set_page(&mut self, page: Page, cx: &mut Context<Self>) {
        self.page = page;
        // navigating away dismisses any open titlebar dropdown
        self.file_menu_open = false;
        self.edit_menu_open = false;
        cx.notify();
    }

    /// Are we on the editor page?
    fn page_is_editor(&self) -> bool {
        matches!(self.page, Page::Editor(_))
    }

    /// Whether "Undo" is currently available: on the editor page with a
    /// non-empty undo history. Drives the Edit menu's greyed-out state.
    fn can_undo(&self, cx: &App) -> bool {
        match &self.page {
            Page::Editor(editor) => editor.read(cx).can_undo(),
            Page::Home(_) => false,
        }
    }

    /// Toggles the File dropdown, closing the Edit dropdown (only one open at a time).
    fn toggle_file_menu(&mut self, cx: &mut Context<Self>) {
        self.file_menu_open = !self.file_menu_open;
        self.edit_menu_open = false;
        cx.notify();
    }

    /// Toggles the Edit dropdown, closing the File dropdown (only one open at a time).
    fn toggle_edit_menu(&mut self, cx: &mut Context<Self>) {
        self.edit_menu_open = !self.edit_menu_open;
        self.file_menu_open = false;
        cx.notify();
    }

    /// Closes any open titlebar dropdown (File or Edit).
    fn close_menus(&mut self, cx: &mut Context<Self>) {
        if self.file_menu_open || self.edit_menu_open {
            self.file_menu_open = false;
            self.edit_menu_open = false;
            cx.notify();
        }
    }

    /// Gets the `Entity<Editor>` if we're on the editor page. If we're not, returns `None`.
    fn editor_entity(&self) -> Option<gpui::Entity<Editor>> {
        match &self.page {
            Page::Editor(editor) => Some(editor.clone()),
            Page::Home(_) => None,
        }
    }

    /// "New" button on "File" menu.
    pub fn file_new(&mut self, cx: &mut Context<Self>) {
        self.close_menus(cx);
        match CanJson::new() {
            Ok(file) => {
                let nav = Navigator::new(cx.weak_entity());
                let page = Page::editor(nav, cx, file);
                self.set_page(page, cx);
            }
            Err(err) => eprintln!("New file failed: {err:?}"),
        }
    }

    /// "Open" button on "File" menu.
    pub fn file_open(&mut self, cx: &mut Context<Self>) {
        self.close_menus(cx);
        match CanJson::open() {
            Ok(file) => {
                let nav = Navigator::new(cx.weak_entity());
                let page = Page::editor(nav, cx, file);
                self.set_page(page, cx);
            }
            Err(err) => eprintln!("Open file failed: {err:?}"),
        }
    }

    /// Opens a specific file path in the editor.
    pub fn open_path(&mut self, path: std::path::PathBuf, cx: &mut Context<Self>) {
        self.close_menus(cx);
        match CanJson::read(path) {
            Ok(file) => {
                let nav = Navigator::new(cx.weak_entity());
                let page = Page::editor(nav, cx, file);
                self.set_page(page, cx);
            }
            Err(err) => eprintln!("Open file failed: {err:?}"),
        }
    }

    /// "Save" button on "File" menu.
    /// If not on the editor page, this does nothing.
    pub fn file_save(&mut self, cx: &mut Context<Self>) {
        self.close_menus(cx);
        if let Some(editor) = self.editor_entity() {
            editor.update(cx, |editor, cx| {
                editor.save();
                cx.notify();
            });
        }
    }

    /// "Save As" button on "File" menu.
    /// If not on the editor page, this does nothing.
    pub fn file_save_as(&mut self, cx: &mut Context<Self>) {
        self.close_menus(cx);
        if let Some(editor) = self.editor_entity() {
            editor.update(cx, |editor, cx| {
                editor.save_as();
                cx.notify();
            });
        }
    }

    /// "Home" button on "File" menu.
    pub fn go_home(&mut self, cx: &mut Context<Self>) {
        let nav = Navigator::new(cx.weak_entity());
        let page = Page::home(nav, cx);
        self.set_page(page, cx); // also dismisses the menu
    }

    /// "Exit" button on "File" menu.
    pub fn exit(&mut self, _cx: &mut Context<Self>) {
        _cx.quit();
    }

    /// "Undo" button on "Edit" menu.
    pub fn undo(&mut self, cx: &mut Context<Self>) {
        self.close_menus(cx);
        if let Some(editor) = self.editor_entity() {
            editor.update(cx, |editor, cx| {
                editor.undo();
                cx.notify();
            });
            cx.notify();
        }
    }

    fn window_title(&self, cx: &mut Context<Self>) -> impl IntoElement {
        const WINDOW_TITLE: &'static str = "can-json-gui-2.0";
        const WINDOW_TITLE_COLOR: u32 = 0xCCCCCC;
        fn homepage() -> Div {
            div()
                .flex()
                .flex_row()
                .child(
                    div()
                    .font_face(crate::frontend::assets::fonts::CalSansUiLight)
                    .text_size(px(10.0))
                    .text_color(rgb(WINDOW_TITLE_COLOR))
                    .child(format!("{WINDOW_TITLE} - "))
                )
                .child(
                    div()
                    .font_face(crate::frontend::assets::fonts::CalSansUiLight)
                    .text_size(px(10.0))
                    .text_color(rgb(WINDOW_TITLE_COLOR))
                    .child(format!("Home"))
                )
        }

        fn editor(editor: &Editor) -> Div {
            const UNSAVED_INDICATOR_COLOR: u32 = 0xb8973b;
            div()
                .flex()
                .flex_row()
                .items_center()
                .child(
                    div()
                    .font_face(crate::frontend::assets::fonts::CalSansUiLight)
                    .text_size(px(10.0))
                    .text_color(rgb(WINDOW_TITLE_COLOR))
                    .child(format!("{WINDOW_TITLE} - "))
                )
                .child(
                    div()
                    .font_face(crate::frontend::assets::fonts::CalSansUiLight)
                    .text_size(px(10.0))
                    .text_color(rgb(WINDOW_TITLE_COLOR))
                    .child(format!("Editing {}", editor.file().filename()))
                )
                .child(
                    crate::frontend::assets::icons::Circle::get()
                        .size(px(8.0))
                        .ml(px(4.0))
                        .text_color(rgb(UNSAVED_INDICATOR_COLOR))
                        .opacity(if editor.file().is_mutated() { 1.0 } else { 0.0 }),
                )
        }
        
        div()
            .flex()
            .flex_row()
            .child(
                match &self.page {
                    Page::Home(_) => homepage(),
                    Page::Editor(page) => {
                        editor(page.read(cx))
                    },
                }
            )
    }
}

pub const TITLEBAR_HEIGHT_PX: f32 = 32.0;

impl Render for AppWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        /* Main window settings. */
        let decorations = window.window_decorations();
        let rounding = px(10.0);
        let shadow_size = px(10.0);
        let border_size = px(1.0);
        let border_color = rgb(0x454545);
        let window_color = rgb(0x1f1f1f);
        let text_color = rgb(0xFFFFFF);

        /* Internal border parameters. */
        let inner_border_color = rgb(0x2D2D2D);
        let inner_border_size = px(1.0);

        /* Titlebar settings. */
        let titlebar_color = rgb(0x181818);
        let close_icon = icons::Close::get();
        let maximize_icon = if window.is_maximized() {icons::TwoSquares::get()} else {icons::OneSquare::get()};
        let minimize_icon = icons::Minus::get();

        window.set_client_inset(shadow_size);

        // Keep the window root focused for keyboard shortcuts.
        // Eventually probably should only grab focus on initial load in case we have other
        // child windows (maybe for error popups and the like)
        if !self.focus_handle.is_focused(window) {
            window.focus(&self.focus_handle);
        }

        // Titlebar dropdowns (File / Edit) and their shared dismiss backdrop.
        let file_menu = self
            .file_menu_open
            .then(|| file_dialog::file_menu(self.page_is_editor(), cx));
        let edit_menu = self
            .edit_menu_open
            .then(|| edit_dialog::edit_menu(self.can_undo(cx), cx));
        let menu_backdrop = (self.file_menu_open || self.edit_menu_open).then(|| {
            div()
                .absolute()
                .inset_0()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|app, _, _, cx| {
                        app.close_menus(cx);
                    }),
                )
        });

        div()
            .id("window-backdrop")
            .on_action(cx.listener(|this, _: &NewFile, _, cx| this.file_new(cx)))
            .on_action(cx.listener(|this, _: &OpenFile, _, cx| this.file_open(cx)))
            .on_action(cx.listener(|this, _: &SaveFile, _, cx| this.file_save(cx)))
            .on_action(cx.listener(|this, _: &SaveFileAs, _, cx| this.file_save_as(cx)))
            .on_action(cx.listener(|this, _: &GoHome, _, cx| this.go_home(cx)))
            .on_action(cx.listener(|this, _: &QuitApp, _, cx| this.exit(cx)))
            .on_action(cx.listener(|this, _: &Undo, _, cx| this.undo(cx)))
            .bg(transparent_black())
            .map(|div| match decorations {
                Decorations::Server => div,
                Decorations::Client { tiling, .. } => div
                    .bg(gpui::transparent_black())
                    .child(
                        canvas(
                            |_bounds, window, _cx| {
                                window.insert_hitbox(
                                    gpui::Bounds::new(
                                        point(px(0.0), px(0.0)),
                                        window.window_bounds().get_bounds().size,
                                    ),
                                    HitboxBehavior::Normal,
                                )
                            },
                            move |_bounds, hitbox, window, _cx| {
                                if window.is_maximized() {
                                    return;
                                }
                                let mouse = window.mouse_position();
                                let size = window.window_bounds().get_bounds().size;
                                let Some(edge) = resize_edge(mouse, shadow_size, size, rounding) else {
                                    return;
                                };
                                window.set_cursor_style(
                                    match edge {
                                        ResizeEdge::Top | ResizeEdge::Bottom => {
                                            CursorStyle::ResizeUpDown
                                        }
                                        ResizeEdge::Left | ResizeEdge::Right => {
                                            CursorStyle::ResizeLeftRight
                                        }
                                        ResizeEdge::TopLeft | ResizeEdge::BottomRight => {
                                            CursorStyle::ResizeUpLeftDownRight
                                        }
                                        ResizeEdge::TopRight | ResizeEdge::BottomLeft => {
                                            CursorStyle::ResizeUpRightDownLeft
                                        }
                                    },
                                    &hitbox,
                                );
                            },
                        )
                        .size_full()
                        .absolute(),
                    )
                    .when(!(tiling.top || tiling.right), |div| div.rounded_tr(rounding))
                    .when(!(tiling.top || tiling.left), |div| div.rounded_tl(rounding))
                    .when(!(tiling.bottom || tiling.right), |div| div.rounded_br(rounding))
                    .when(!(tiling.bottom || tiling.left), |div| div.rounded_bl(rounding))
                    .when(!tiling.top, |div| div.pt(shadow_size))
                    .when(!tiling.bottom, |div| div.pb(shadow_size))
                    .when(!tiling.left, |div| div.pl(shadow_size))
                    .when(!tiling.right, |div| div.pr(shadow_size))
                    .on_mouse_move(|_e, window, _cx| window.refresh())
                    .on_mouse_down(MouseButton::Left, move |e, window, _cx| {
                        if window.is_maximized() {
                            return;
                        }
                        let size = window.window_bounds().get_bounds().size;
                        match resize_edge(e.position, shadow_size, size, rounding) {
                            Some(edge) => window.start_window_resize(edge),
                            None => return,
                        }
                    }),
            })
            .size_full()
            .child(
                div()
                    .cursor(CursorStyle::Arrow)
                    .map(|div| match decorations {
                        Decorations::Server => div,
                        Decorations::Client { tiling } => div
                            .border_color(border_color)
                            .when(!(tiling.top || tiling.right), |div| div.rounded_tr(rounding))
                            .when(!(tiling.top || tiling.left), |div| div.rounded_tl(rounding))
                            .when(!(tiling.bottom || tiling.right), |div| div.rounded_br(rounding))
                            .when(!(tiling.bottom || tiling.left), |div| div.rounded_bl(rounding))
                            .border_t(border_size)
                            .border_b(border_size)
                            .border_l(border_size)
                            .border_r(border_size)
                            .when(!tiling.is_tiled(), |div| {
                                div.shadow(vec![gpui::BoxShadow {
                                    color: Hsla { h: 0., s: 0., l: 0., a: 0.4 },
                                    blur_radius: shadow_size / 2.,
                                    spread_radius: px(0.),
                                    offset: point(px(0.0), px(0.0)),
                                }])
                            }),
                    })
                    .on_mouse_move(|_e, _, cx| cx.stop_propagation())
                    .bg(window_color)
                    .size_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .child(
                        // Titlebar
                        div()
                            .w_full()
                            .h(px(TITLEBAR_HEIGHT_PX))
                            .flex()
                            .items_center()
                            .bg(titlebar_color)
                            .overflow_hidden()
                            .border_color(inner_border_color)
                            .border_b(inner_border_size)
                            .map(|div| match decorations {
                                Decorations::Server => div,
                                Decorations::Client { tiling } => div
                                    .when(!(tiling.top || tiling.right), |div| div.rounded_tr(rounding))
                                    .when(!(tiling.top || tiling.left), |div| div.rounded_tl(rounding))
                                    .on_mouse_down(MouseButton::Left, |_e, window, _cx| {
                                        if _e.click_count == 1 { // On a single click, start a window drag.
                                            window.start_window_move();
                                        } else if _e.click_count == 2 { // On a double click, maximize/minimize the window.
                                            window.zoom_window();
                                        }
                                    })
                                    .on_mouse_up(MouseButton::Right, |e, window, _cx| {
                                        window.show_window_menu(e.position);
                                    }),
                            })
                            .child(
                                div()
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .ml(px(8.0))
                                    .items_center()
                                    .when(cfg!(target_os = "macos"), |d| {
                                        d.child(
                                            div().min_w(px(65.0)).bg(black())
                                        )
                                    })
                                    .child(
                                        div()
                                            .relative()
                                            .h_full()
                                            .flex()
                                            .items_center()
                                            .child(
                                                button::button("file-button")
                                                    .text_color(rgb(0xCCCCCC))
                                                    .font_family("Cal Sans UI")
                                                    .p(px(5.0))
                                                    .rounded(px(5.0))
                                                    .font_weight(gpui::FontWeight(50.0))
                                                    .text_size(px(12.0))
                                                    .group("file-button")
                                                    .line_height(gpui::relative(1.0))
                                                    .child("File")
                                                    .hover(|s| s.bg(rgb(0x2D2D2D)))
                                                    .on_click(cx.listener(|app, _, _, cx| {
                                                        app.toggle_file_menu(cx)
                                                    })),
                                            )
                                            .children(file_menu),
                                    )
                                    .child(
                                        div()
                                            .relative()
                                            .h_full()
                                            .flex()
                                            .items_center()
                                            .child(
                                                button::button("edit-button")
                                                    .text_color(rgb(0xCCCCCC))
                                                    .font_family("Cal Sans UI")
                                                    .p(px(5.0))
                                                    .rounded(px(5.0))
                                                    .font_weight(gpui::FontWeight(50.0))
                                                    .text_size(px(12.0))
                                                    .group("edit-button")
                                                    .line_height(gpui::relative(1.0))
                                                    .child("Edit")
                                                    .hover(|s| s.bg(rgb(0x2D2D2D)))
                                                    .on_click(cx.listener(|app, _, _, cx| {
                                                        app.toggle_edit_menu(cx)
                                                    })),
                                            )
                                            .children(edit_menu),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .h_full()
                                            .window_control_area(WindowControlArea::Drag),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .window_control_area(WindowControlArea::Drag)
                                    .justify_center()
                                    .child(
                                        div()
                                            .p(px(5.0))
                                            .rounded(px(3.0))
                                            .max_w(px(300.0))
                                            .truncate()
                                            .text_color(text_color)
                                            .font_family("Cal Sans UI")
                                            .text_size(px(11.0))
                                            .font_weight(gpui::FontWeight(50.0))
                                            .line_height(gpui::relative(1.0))
                                            .window_control_area(WindowControlArea::Drag)
                                            .child(self.window_title(cx)),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .gap(px(1.0))
                                    .pr(px(5.0))
                                    .when(cfg!(not(target_os = "macos")), |div| {
                                        div.min_w(px(100.0))
                                    })
                                    .child(
                                        div()
                                            .flex_1()
                                            .h_full()
                                            .window_control_area(WindowControlArea::Drag),
                                    )
                                    .when(!cfg!(target_os = "macos"), |div| {
                                        // On non-macOS, render the custom window controls.
                                        // On macOS, the native traffic-light buttons handle
                                        // these actions, so we skip the custom set.
                                        div.child(
                                            button::button("minimize-button")
                                                .group("minimize-button")
                                                .w(px(20.0))
                                                .h(px(20.0))
                                                .flex()
                                                .hover(|s| s.bg(rgb(0x2D2D2D)))
                                                .rounded(px(5.0))
                                                .items_center()
                                                .justify_center()
                                                .window_control_area(WindowControlArea::Min)
                                                .on_click(|_, window, _| window.minimize_window())
                                                .child(
                                                    minimize_icon
                                                        .size(px(11.0))
                                                        .text_color(rgb(0xCCCCCC)),
                                                ),
                                        )
                                        .child(
                                            button::button("maximize-button")
                                                .group("maximize-button")
                                                .w(px(20.0))
                                                .h(px(20.0))
                                                .flex()
                                                .hover(|s| s.bg(rgb(0x2D2D2D)))
                                                .rounded(px(5.0))
                                                .items_center()
                                                .justify_center()
                                                .window_control_area(WindowControlArea::Max)
                                                .on_click(|_, window, _| toggle_maximize(window))
                                                .child(
                                                    maximize_icon
                                                        .size(px(11.0))
                                                        .text_color(rgb(0xCCCCCC)),
                                                ),
                                        )
                                        .child(
                                            button::button("close-button")
                                                .group("close-button")
                                                .w(px(20.0))
                                                .h(px(20.0))
                                                .flex()
                                                .hover(|s| s.bg(rgb(0x2D2D2D)))
                                                .rounded(px(5.0))
                                                .items_center()
                                                .justify_center()
                                                .window_control_area(WindowControlArea::Close)
                                                .on_click(|_, window, _| window.remove_window())
                                                .child(
                                                    close_icon
                                                        .size(px(13.0))
                                                        .text_color(rgb(0xCCCCCC)),
                                                ),
                                        )
                                    }),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_h_0()
                            .track_focus(&self.focus_handle) // track_focus() is here so keyboard shortcuts work. If you put track_focus() at the window root it breaks the titlebar on windows for some reason
                            .child(self.page.into_view()),
                    )
                    // Little guy that dismisses the File menu on an outside click
                    .children(menu_backdrop),
            )
    }
}

// This is a custom function for the middle button on the titlebar, necessary because Window's default behavior for WindowControlArea::Max and zoom_window() is weird.
// If you try to just use WindowControlArea::Max or zoom_window() on Windows, clicking the middle titlebar button (the square) will open up the multitasking menu rather than just toggling the window's maximized state like it does on other OSes.
// So, this function manually sends the SC_RESTORE and SC_MAXIMIZE commands when on Windows. If not on Windows, it just falls back to window.zoom_window() as normal.
fn toggle_maximize(window: &mut Window) {
    #[cfg(target_os = "windows")]
    {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        const WM_SYSCOMMAND: u32 = 0x0112;
        const SC_MAXIMIZE: usize = 0xF030;
        const SC_RESTORE: usize = 0xF120;

        #[link(name = "user32")]
        unsafe extern "system" {
            fn PostMessageW(hwnd: isize, msg: u32, wparam: usize, lparam: isize) -> i32;
        }

        let cmd = if window.is_maximized() { SC_RESTORE } else { SC_MAXIMIZE };
        if let Ok(handle) = HasWindowHandle::window_handle(window) {
            if let RawWindowHandle::Win32(win32) = handle.as_raw() {
                unsafe {
                    PostMessageW(win32.hwnd.get(), WM_SYSCOMMAND, cmd, 0);
                }
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        window.zoom_window();
    }
}

fn resize_edge(pos: Point<Pixels>, shadow_size: Pixels, size: Size<Pixels>, rounding: Pixels) -> Option<ResizeEdge> {
    let edge = if pos.y < (shadow_size + rounding) && pos.x < (shadow_size + rounding) {
        ResizeEdge::TopLeft
    } else if pos.y < (shadow_size + rounding) && pos.x > size.width - (shadow_size + rounding) {
        ResizeEdge::TopRight
    } else if pos.y < shadow_size {
        ResizeEdge::Top
    } else if pos.y > size.height - (shadow_size + rounding) && pos.x < (shadow_size + rounding) {
        ResizeEdge::BottomLeft
    } else if pos.y > size.height - (shadow_size + rounding) && pos.x > size.width - (shadow_size + rounding) {
        ResizeEdge::BottomRight
    } else if pos.y > size.height - shadow_size {
        ResizeEdge::Bottom
    } else if pos.x < shadow_size {
        ResizeEdge::Left
    } else if pos.x > size.width - shadow_size {
        ResizeEdge::Right
    } else {
        return None;
    };
    Some(edge)
}
