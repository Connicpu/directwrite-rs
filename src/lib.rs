#![cfg(windows)]

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
extern crate winapi;
extern crate wio;

#[doc(inline)]
pub use client_effect::ClientEffect;
#[doc(inline)]
pub use drawing_effect::DrawingEffect;
#[doc(inline)]
pub use factory::Factory;
#[doc(inline)]
pub use font::Font;
#[doc(inline)]
pub use font_collection::FontCollection;
#[doc(inline)]
pub use font_face::FontFace;
#[doc(inline)]
pub use font_family::FontFamily;
#[doc(inline)]
pub use font_file::FontFile;
#[doc(inline)]
pub use font_list::FontList;
#[doc(inline)]
pub use geometry_sink::GeometrySink;
#[doc(inline)]
pub use inline_object::InlineObject;
#[doc(inline)]
pub use localized_strings::LocalizedStrings;
#[doc(inline)]
pub use rendering_params::RenderingParams;
#[doc(inline)]
pub use text_format::TextFormat;
#[doc(inline)]
pub use text_layout::TextLayout;
#[doc(inline)]
pub use text_range::TextRange;
#[doc(inline)]
pub use text_renderer::TextRenderer;

#[doc(hidden)]
pub mod client_effect;
#[doc(hidden)]
pub mod drawing_effect;
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
pub mod glyphs;
pub mod inline_object;
pub mod key;
pub mod localized_strings;
pub mod metrics;
pub mod rendering_params;
pub mod text_format;
pub mod text_layout;
#[doc(hidden)]
pub mod text_range;
pub mod text_renderer;

mod helpers;
