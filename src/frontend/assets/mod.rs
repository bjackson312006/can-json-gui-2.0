//! Contains all the assets for the frontend (e.g., icons, fonts).

use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/frontend/assets/"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::get(path).map(|f| f.data))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|p| p.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}

/// Module for fonts.
pub mod fonts {
    /// Helper macro for declaring fonts.
    macro_rules! font {
        ($name:ident, $path:literal) => {
            use std::borrow::Cow;
            const _: &'static str = $path;
            const BYTES: &[u8] = include_bytes!($path);
            #[allow(dead_code)]
            #[doc = concat!("\"", stringify!($name), "\" font.")]
            pub struct $name {}
            impl $name {
                #[inline(always)]
                #[allow(dead_code)]
                #[doc = concat!("Gets the bytes of the \"", stringify!($name), "\" font. This type is how GPUI expects fonts.")]
                pub const fn get() -> Cow<'static, [u8]> {
                    Cow::Borrowed(BYTES)
                }
            }
        };
    }

    font!(CalSansUi, "fonts/cal-sans-ui/CalSansUI.wght.GEOM.ttf");
}

/// Module for icons.
pub mod icons {
    use gpui::{svg, Svg};

    /// Helper macro for declaring icons.
    macro_rules! icon {
        ($name:ident, $path:literal) => {
            const _: &'static str = $path;
            const _: &[u8] = include_bytes!($path);
            #[allow(dead_code)]
            #[doc = concat!("\"", stringify!($name), "\" icon.")]
            pub struct $name {}
            impl $name {
                #[inline(always)]
                #[allow(dead_code)]
                #[doc = concat!("Gets the filepath of the \"", stringify!($name), "\" icon. This type is how GPUI expects.")]
                pub fn get() -> Svg {
                    svg().path($path)
                }
            }
        };
    }

    icon!(Close, "tabler-icons/filled/x.svg");
    icon!(TwoSquares, "tabler-icons/outline/squares.svg");
    icon!(OneSquare, "tabler-icons/outline/square.svg");
    icon!(Minus, "tabler-icons/outline/minus.svg");
    icon!(ArrowLeft, "tabler-icons/outline/arrow-left.svg");
}