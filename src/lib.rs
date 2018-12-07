//! Safe bindings for DirectWrite in Rust. Allows for the loading of fonts, laying out of text,
//! and rendering text and glyphs to TextRenderers.

#![cfg(windows)]
#![warn(missing_docs)]

#[macro_use]
extern crate auto_enum;

#[macro_use]
extern crate derive_com_wrapper;

#[macro_use]
extern crate derive_com_impl;

extern crate checked_enum;
extern crate com_impl;
extern crate com_wrapper;
extern crate math2d;
extern crate memmap;
extern crate winapi;
extern crate wio;

#[doc(inline)]
pub use crate::error::{DWResult, DWriteError};
#[doc(inline)]
pub use crate::factory::Factory;
#[doc(inline)]
pub use crate::font::Font;
#[doc(inline)]
pub use crate::font_collection::FontCollection;
#[doc(inline)]
pub use crate::font_face::FontFace;
#[doc(inline)]
pub use crate::font_family::FontFamily;
#[doc(inline)]
pub use crate::font_file::FontFile;
#[doc(inline)]
pub use crate::font_list::FontList;
#[doc(inline)]
pub use crate::geometry_sink::GeometrySink;
#[doc(inline)]
pub use crate::inline_object::InlineObject;
#[doc(inline)]
pub use crate::rendering_params::RenderingParams;
#[doc(inline)]
pub use crate::text_format::TextFormat;
#[doc(inline)]
pub use crate::text_layout::TextLayout;
#[doc(inline)]
pub use crate::text_renderer::TextRenderer;
#[doc(inline)]
pub use crate::typography::Typography;

#[macro_use]
mod helpers;

pub mod descriptions;
pub mod effects;
pub mod enums;
#[doc(hidden)]
pub mod error;
#[doc(hidden)]
pub mod factory;
#[doc(hidden)]
pub mod font;
pub mod font_collection;
pub mod font_face;
#[doc(hidden)]
pub mod font_family;
pub mod font_file;
#[doc(hidden)]
pub mod font_list;
#[doc(hidden)]
pub mod geometry_sink;
pub mod inline_object;
pub mod localized_strings;
pub mod metrics;
pub mod number_substitution;
pub mod pixel_snapping;
#[doc(hidden)]
pub mod rendering_params;
pub mod text_format;
pub mod text_layout;
pub mod text_renderer;
pub mod typography;

/// Shortcut to initialize a Factory, which is required to access all other
/// functionality in the library.
pub fn initialize() -> error::DWResult<Factory> {
    Factory::new()
}
