use error::DWriteError;
use {TextFormat, TextLayout};

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::*;
use wio::com::ComPtr;
use wio::wide::ToWide;

pub struct TextLayoutBuilder<'a> {
    factory: &'a IDWriteFactory,
    text: Option<&'a str>,
    format: Option<&'a TextFormat>,
    width: Option<f32>,
    height: Option<f32>,
    centered: bool,
}

impl<'a> TextLayoutBuilder<'a> {
    pub fn new(factory: &'a IDWriteFactory) -> TextLayoutBuilder<'a> {
        TextLayoutBuilder {
            factory,
            text: None,
            format: None,
            width: None,
            height: None,
            centered: false,
        }
    }

    pub fn build(self) -> Result<TextLayout, DWriteError> {
        unsafe {
            let text = self.text.expect("`text` must be specified").to_wide_null();
            let format = self.format.expect("`format` must be specified");
            let width = self.width.expect("`width` or `size` must be specified");
            let height = self.height.expect("`height` or `size` must be specified");

            let mut ptr: *mut IDWriteTextLayout = ptr::null_mut();
            let result = self.factory.CreateTextLayout(
                text.as_ptr(),
                text.len() as u32,
                format.get_raw(),
                width,
                height,
                &mut ptr,
            );

            if SUCCEEDED(result) {
                let ptr = ComPtr::from_raw(ptr);
                if self.centered {
                    ptr.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER);
                }

                Ok(TextLayout { ptr: ptr })
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn with_text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    pub fn with_font(mut self, font: &'a TextFormat) -> Self {
        self.format = Some(font);
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_size(self, width: f32, height: f32) -> Self {
        self.with_width(width).with_height(height)
    }

    pub fn with_centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }
}
