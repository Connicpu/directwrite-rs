//! Metrics structs that describe information about various items.

#[doc(inline)]
pub use crate::metrics::cluster::ClusterMetrics;
#[doc(inline)]
pub use crate::metrics::font::FontMetrics;
#[doc(inline)]
pub use crate::metrics::glyph::GlyphMetrics;
#[doc(inline)]
pub use crate::metrics::hit_test::HitTestMetrics;
#[doc(inline)]
pub use crate::metrics::inline_object::InlineObjectMetrics;
#[doc(inline)]
pub use crate::metrics::line::LineMetrics;
#[doc(inline)]
pub use crate::metrics::overhang::OverhangMetrics;
#[doc(inline)]
pub use crate::metrics::text::TextMetrics;

#[doc(hidden)]
pub mod cluster;
#[doc(hidden)]
pub mod font;
#[doc(hidden)]
pub mod glyph;
#[doc(hidden)]
pub mod hit_test;
#[doc(hidden)]
pub mod inline_object;
#[doc(hidden)]
pub mod line;
#[doc(hidden)]
pub mod overhang;
#[doc(hidden)]
pub mod text;
