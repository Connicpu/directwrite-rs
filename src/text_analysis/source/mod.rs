use crate::enums::reading_direction::ReadingDirection;
use crate::number_substitution::NumberSubstitution;

use com_wrapper::ComWrapper;
use winapi::um::dwrite::IDWriteTextAnalysisSource;
use wio::com::ComPtr;

pub mod custom;

#[repr(transparent)]
#[derive(ComWrapper)]
pub struct TextAnalysisSource {
    ptr: ComPtr<IDWriteTextAnalysisSource>,
}

impl TextAnalysisSource {
    pub fn new<T: TextAnalysisProvider>(provider: T) -> Self {
        custom::CustomTextAnalysisSource::create(provider)
    }
}

pub trait TextAnalysisProvider: 'static {
    /// Returns the locale name of the specified position in text. Must be
    /// null terminated or E_FAIL will be returned to the DWrite runtime.
    ///
    /// Also returns the number of utf-16 words for which this locale
    /// is the same starting at `position`.
    fn locale_name(&self, position: u32) -> (&[u16], u32);

    /// Get the number substitution method used at the specified position in text,
    /// along with the number of utf-16 words for which the substitution is the same.
    fn number_substitution(&self, position: u32) -> (NumberSubstitution, u32);

    /// Gets the paragraph reading direction used for this source of text.
    fn paragraph_reading_direction(&self) -> ReadingDirection;

    /// Get a block of text starting at the specified position. As many
    /// code units may be returned as the implementation would like.
    fn text_at(&self, position: u32) -> Option<&[u16]>;

    /// Gets a block of text immediately preceding the specified position.
    fn text_before(&self, position: u32) -> Option<&[u16]>;
}
