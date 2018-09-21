use enums::{FontStretch, FontStyle, FontWeight};
use error::DWriteError;
use font_collection::FontCollection;
use text_format::TextFormat;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{IDWriteFactory, IDWriteTextFormat};
use wio::com::ComPtr;
use wio::wide::ToWide;

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
    pub fn new(factory: &'a IDWriteFactory) -> TextFormatBuilder<'a> {
        TextFormatBuilder {
            factory,
            family: None,
            collection: None,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            stretch: FontStretch::Normal,
            size: None,
            locale: None,
        }
    }

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
                self.weight as u32,
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

    pub fn with_family(mut self, family: &'a str) -> Self {
        self.family = Some(family);
        self
    }

    pub fn with_font_collection(mut self, collection: &'a FontCollection) -> Self {
        self.collection = Some(collection);
        self
    }

    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_stretch(mut self, stretch: FontStretch) -> Self {
        self.stretch = stretch;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_locale(mut self, locale: &'a str) -> Self {
        self.locale = Some(locale);
        self
    }
}
