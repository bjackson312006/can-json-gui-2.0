use gpui::{
    Context, CursorStyle, Decorations, HitboxBehavior, Hsla, MouseButton, Pixels, Point,
    ResizeEdge, Size, Window, WindowControlArea, black, canvas, div, point, prelude::*, px, rgb,
    svg, transparent_black,
};

use crate::frontend::{
    components::button, pages::Page, assets::icons,
};

pub struct AppWindow {
    page: Page,
}

impl AppWindow {
    pub fn new(page: Page) -> Self {
        Self { page }
    }

    pub fn set_page(&mut self, page: Page, cx: &mut Context<Self>) {
        self.page = page;
        cx.notify();
    }
}

impl Render for AppWindow {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
        let window_title: String = "template-window".to_string();
        let close_icon = icons::Close::get();
        let maximize_icon = if window.is_maximized() {icons::TwoSquares::get()} else {icons::OneSquare::get()};
        let minimize_icon = icons::Minus::get();

        window.set_client_inset(shadow_size);

        div()
            .id("window-backdrop")
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
                            .h(px(32.0))
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
                                            .hover(|s| s.bg(rgb(0x2D2D2D))),
                                    )
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
                                            .hover(|s| s.bg(rgb(0x2D2D2D))),
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
                                            .child(window_title),
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
                        // Content area.
                        div().flex_1().min_h_0().child(self.page.into_view()),
                    ),
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
