#![cfg(windows)]

extern crate winapi;
extern crate kernel32;
extern crate uuid;

pub use factory::Factory;
pub use text_format::TextFormat;
pub use text_layout::TextLayout;

#[macro_use]
mod macros;

pub mod comptr;
pub mod drawing_effect;
pub mod error;
pub mod enums;
pub mod factory;
pub mod internal;
pub mod text_format;
pub mod text_layout;

mod helpers;
mod load_dll;
