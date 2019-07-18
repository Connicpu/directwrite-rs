//! Safe bindings for DirectWrite in Rust. Allows for the loading of fonts, laying out of text,
//! and rendering text and glyphs to TextRenderers.

#![cfg(windows)]
//#![warn(missing_docs)]

pub use crate::error::DWResult;
pub use crate::factory::Factory;
pub use crate::font::Font;
pub use crate::font_collection::FontCollection;
pub use crate::font_face::FontFace;
pub use crate::font_family::FontFamily;
pub use crate::font_file::FontFile;
pub use crate::font_list::FontList;
pub use crate::geometry_sink::GeometrySink;
pub use crate::inline_object::InlineObject;
pub use crate::rendering_params::RenderingParams;
pub use crate::text_format::TextFormat;
pub use crate::text_layout::TextLayout;
pub use crate::text_renderer::TextRenderer;
pub use crate::typography::Typography;

pub mod descriptions;
pub mod effects;
pub mod enums;
pub mod error;
pub mod factory;
pub mod font;
pub mod font_collection;
pub mod font_face;
pub mod font_family;
pub mod font_file;
pub mod font_list;
pub mod geometry_sink;
pub mod inline_object;
pub mod localized_strings;
pub mod metrics;
pub mod number_substitution;
pub mod pixel_snapping;
pub mod rendering_params;
pub mod text_analysis;
pub mod text_format;
pub mod text_layout;
pub mod text_renderer;
pub mod typography;

/// Shortcut to initialize a Factory, which is required to access all other
/// functionality in the library.
pub fn initialize() -> error::DWResult<Factory> {
    Factory::new()
}
