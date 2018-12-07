//! Custom effects that may be applied to individual runs of text in a layout.

#[doc(inline)]
pub use self::client_effect::ClientEffect;
#[doc(inline)]
pub use self::drawing_effect::DrawingEffect;

#[doc(hidden)]
pub mod client_effect;
#[doc(hidden)]
pub mod drawing_effect;
