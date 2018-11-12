#[doc(inline)]
pub use metrics::cluster::ClusterMetrics;
#[doc(inline)]
pub use metrics::font::FontMetrics;
#[doc(inline)]
pub use metrics::glyph::GlyphMetrics;
#[doc(inline)]
pub use metrics::hit_test::HitTestMetrics;
#[doc(inline)]
pub use metrics::line::LineMetrics;
#[doc(inline)]
pub use metrics::overhang::OverhangMetrics;
#[doc(inline)]
pub use metrics::text::TextMetrics;

#[doc(hidden)]
pub mod cluster;
#[doc(hidden)]
pub mod font;
#[doc(hidden)]
pub mod glyph;
#[doc(hidden)]
pub mod hit_test;
#[doc(hidden)]
pub mod line;
#[doc(hidden)]
pub mod overhang;
#[doc(hidden)]
pub mod text;
