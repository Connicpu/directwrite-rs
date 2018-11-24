//! Various structures that define things passed to and from directwrite APIs.

#[doc(inline)]
pub use self::dbool::DBool;
#[doc(inline)]
pub use self::font_feature::FontFeature;
#[doc(inline)]
pub use self::glyphs::{GlyphOffset, GlyphRun, GlyphRunDescription};
#[doc(inline)]
pub use self::key::FontKey;
pub(crate) use self::key::KeyPayload;
#[doc(inline)]
pub use self::strikethrough::Strikethrough;
#[doc(inline)]
pub use self::text_range::TextRange;
#[doc(inline)]
pub use self::trimming::Trimming;
#[doc(inline)]
pub use self::underline::Underline;
#[doc(inline)]
pub use self::wide_str::{WideCStr, WideStr};

#[doc(hidden)]
pub mod dbool;
#[doc(hidden)]
pub mod font_feature;
#[doc(hidden)]
pub mod glyphs;
#[doc(hidden)]
pub mod key;
#[doc(hidden)]
pub mod strikethrough;
#[doc(hidden)]
pub mod text_range;
#[doc(hidden)]
pub mod trimming;
#[doc(hidden)]
pub mod underline;
#[doc(hidden)]
pub mod wide_str;
