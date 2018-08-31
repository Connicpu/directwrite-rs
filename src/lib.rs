#![cfg(windows)]

extern crate winapi;
extern crate wio;

pub use factory::Factory;
pub use text_format::TextFormat;
pub use text_layout::TextLayout;

pub mod drawing_effect;
pub mod error;
pub mod enums;
pub mod factory;
pub mod font;
pub mod font_collection;
pub mod font_face;
pub mod font_family;
pub mod font_file;
pub mod font_list;
pub mod inline_object;
pub mod text_format;
pub mod text_layout;
pub mod text_renderer;

mod helpers;
