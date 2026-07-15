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
    use gpui::{Font, FontWeight, Styled};

    /// Helper macro for declaring fonts.
    macro_rules! font {
        ($name:ident, $family:literal, $weight:expr, $path:literal) => {
            const _: &'static str = $path;
            #[allow(dead_code)]
            #[doc = concat!("\"", stringify!($name), "\" font.")]
            pub struct $name;
            impl $name {
                /// The font family name this face belongs to.
                #[allow(dead_code)]
                pub const FAMILY: &'static str = $family;

                /// The weight of this face.
                #[allow(dead_code)]
                pub const WEIGHT: FontWeight = $weight;

                #[inline(always)]
                #[allow(dead_code)]
                #[doc = concat!("Gets the bytes of the \"", stringify!($name), "\" font. This type is how GPUI expects fonts.")]
                pub const fn get() -> std::borrow::Cow<'static, [u8]> {
                    std::borrow::Cow::Borrowed(include_bytes!($path))
                }

                // Builds a GPUI front for this font. Can be used with the normal GPUI `.font()` call.
                #[allow(dead_code)]
                pub fn font() -> Font {
                    let mut font = gpui::font($family);
                    font.weight = $weight;
                    font
                }
            }

            impl From<$name> for Font {
                fn from(_: $name) -> Font {
                    $name::font()
                }
            }
        };
    }

    font!(CalSansUiLight, "Cal Sans UI", FontWeight(300.0), "fonts/cal-sans-ui/CalSansUI-Light.ttf");
    font!(CalSansUiRegular, "Cal Sans UI", FontWeight(400.0), "fonts/cal-sans-ui/CalSansUI-Regular.ttf");
    font!(CalSansUiMedium, "Cal Sans UI", FontWeight(500.0), "fonts/cal-sans-ui/CalSansUI-Medium.ttf");
    font!(CalSansUiSemiBold, "Cal Sans UI", FontWeight(600.0), "fonts/cal-sans-ui/CalSansUI-SemiBold.ttf");
    font!(CalSansUiBold, "Cal Sans UI", FontWeight(700.0), "fonts/cal-sans-ui/CalSansUI-Bold.ttf");

    font!(SatoshiLight, "Satoshi", FontWeight(300.0), "fonts/satoshi/Satoshi-Light.ttf");
    font!(SatoshiRegular, "Satoshi", FontWeight(400.0), "fonts/satoshi/Satoshi-Regular.ttf");
    font!(SatoshiMedium, "Satoshi", FontWeight(500.0), "fonts/satoshi/Satoshi-Medium.ttf");
    font!(SatoshiBold, "Satoshi", FontWeight(700.0), "fonts/satoshi/Satoshi-Bold.ttf");
    font!(SatoshiBlack, "Satoshi", FontWeight(900.0), "fonts/satoshi/Satoshi-Black.ttf");

    /// Extension trait that lets you pass a font marker type directly to a
    /// styling call, e.g. `div().font_face(CalSansUiBold)`.
    #[allow(dead_code)]
    pub trait FontFace: Styled + Sized {
        fn font_face(self, face: impl Into<Font>) -> Self {
            self.font(face.into())
        }
    }
    impl<T: Styled + Sized> FontFace for T {}
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
    icon!(Trash, "tabler-icons/outline/trash.svg");
    icon!(Create, "tabler-icons/outline/square-rounded-plus.svg");
    icon!(Messages, "tabler-icons/outline/messages.svg");
    icon!(Circle, "tabler-icons/filled/circle.svg");
    icon!(FolderOpen, "tabler-icons/outline/folder-open.svg");
    icon!(SquareRoundedPlus, "tabler-icons/outline/square-rounded-plus.svg");
    icon!(Duplicate, "tabler-icons/outline/copy.svg");
    icon!(Reload, "tabler-icons/outline/reload.svg");
}