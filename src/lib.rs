#![cfg(windows)]

#[macro_use]
extern crate auto_enum;

#[macro_use]
extern crate derive_com_wrapper;

extern crate checked_enum;
extern crate com_wrapper;
extern crate math2d;
extern crate winapi;
extern crate wio;

pub use factory::Factory;
pub use text_format::TextFormat;
pub use text_layout::TextLayout;

pub mod drawing_effect;
pub mod enums;
pub mod error;
pub mod factory;
pub mod font;
pub mod font_collection;
pub mod font_collection_loader;
pub mod font_face;
pub mod font_family;
pub mod font_file;
pub mod font_list;
pub mod geometry_sink;
pub mod glyphs;
pub mod inline_object;
pub mod localized_strings;
pub mod metrics;
pub mod rendering_params;
pub mod text_format;
pub mod text_layout;
pub mod text_renderer;

mod helpers;
