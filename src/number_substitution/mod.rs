//! Describes how numberic digits should be substituted.

use factory::Factory;

use winapi::um::dwrite::IDWriteNumberSubstitution;
use wio::com::ComPtr;

#[doc(inline)]
pub use self::builder::NumberSubstitutionBuilder;

#[doc(hidden)]
pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper, Clone)]
#[com(send, sync, debug)]
/// Holds the appropriate digits and numeric punctuation for a given locale.
pub struct NumberSubstitution {
    ptr: ComPtr<IDWriteNumberSubstitution>,
}

impl NumberSubstitution {
    /// Initialize a builder.
    pub fn create(factory: &Factory) -> NumberSubstitutionBuilder {
        NumberSubstitutionBuilder::new(factory)
    }
}

