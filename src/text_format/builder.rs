use crate::enums::{FontStretch, FontStyle, FontWeight};
use crate::error::DWriteError;
use crate::font_collection::FontCollection;
use crate::text_format::TextFormat;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{IDWriteFactory, IDWriteTextFormat};
use wio::com::ComPtr;
use wio::wide::ToWide;

#[must_use]
/// Builder for a TextFormat.
pub struct TextFormatBuilder<'a> {
    factory: &'a IDWriteFactory,
    family: Option<&'a str>,
    collection: Option<&'a FontCollection>,
    weight: FontWeight,
    style: FontStyle,
    stretch: FontStretch,
    size: Option<f32>,
    locale: Option<&'a str>,
}

impl<'a> TextFormatBuilder<'a> {
    /// Initialize a new builder.
    pub fn new(factory: &'a IDWriteFactory) -> TextFormatBuilder<'a> {
        TextFormatBuilder {
            factory,
            family: None,
            collection: None,
            weight: FontWeight::NORMAL,
            style: FontStyle::Normal,
            stretch: FontStretch::Normal,
            size: None,
            locale: None,
        }
    }

    /// Finalize the builder. Panics if `family` or `size` is not specified.
    pub fn build(self) -> Result<TextFormat, DWriteError> {
        unsafe {
            let family = self
                .family
                .expect("`family` must be specified")
                .to_wide_null();

            let collection = self
                .collection
                .map(|col| col.get_raw())
                .unwrap_or(ptr::null_mut());

            let size = self.size.expect("`size` must be specified");
            let locale = self.locale.unwrap_or("en-US").to_wide_null();

            let mut ptr: *mut IDWriteTextFormat = ptr::null_mut();
            let result = self.factory.CreateTextFormat(
                family.as_ptr(),
                collection,
                self.weight.0,
                self.style as u32,
                self.stretch as u32,
                size,
                locale.as_ptr(),
                &mut ptr,
            );

            if SUCCEEDED(result) {
                Ok(TextFormat {
                    ptr: ComPtr::from_raw(ptr),
                })
            } else {
                Err(From::from(result))
            }
        }
    }

    /// Specify a font family name.
    pub fn with_family(mut self, family: &'a str) -> Self {
        self.family = Some(family);
        self
    }

    /// Specify a specific font collection to use. This is optional if you are
    /// using a system-installed font.
    pub fn with_collection(mut self, collection: &'a FontCollection) -> Self {
        self.collection = Some(collection);
        self
    }

    /// Specify a font weight. Defaults to [`NORMAL`][1]
    /// 
    /// [1]: ../enums/struct.FontWeight.html#associatedconstant.NORMAL
    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    /// Specify a font style. Defaults to [`Normal`][1]
    /// 
    /// [1]: ../enums/enum.FontStyle.html#variant.Normal
    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.style = style;
        self
    }


    /// Specify a font stretch. Defaults to [`Normal`][1]
    /// 
    /// [1]: ../enums/enum.FontStretch.html#variant.Normal
    pub fn with_stretch(mut self, stretch: FontStretch) -> Self {
        self.stretch = stretch;
        self
    }

    /// Specify a font size to use in DIPs.
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Specify the locale that the font family is named in. Defaults to `en-US`.
    pub fn with_locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }
}
