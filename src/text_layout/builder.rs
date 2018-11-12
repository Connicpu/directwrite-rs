use error::DWriteError;
use {TextFormat, TextLayout};

use std::borrow::Cow;
use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::*;
use wio::com::ComPtr;
use wio::wide::ToWide;

pub struct TextLayoutBuilder<'a> {
    factory: &'a IDWriteFactory,
    text: Option<Cow<'a, [u16]>>,
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
            let text = self.text.expect("`text` must be specified");
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

    /// Specify the text from a UTF-8 string.
    /// 
    /// Be aware that all of the text positions returned from the directwrite APIs will use text
    /// positions as if this text was converted to UTF-16.
    pub fn with_str(mut self, text: &str) -> Self {
        self.text = Some(text.to_wide().into());
        self
    }

    /// Specify the text from a UTF-16 string.
    pub fn with_text(mut self, text: &'a [u16]) -> Self {
        self.text = Some(Cow::Borrowed(text));
        self
    }

    /// Specify the text format (Font) used with this text.
    pub fn with_format(mut self, format: &'a TextFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Specify the maximum layout width in DIPs
    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Specify the maximum layout height in DIPs
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    /// Specify the maximum layout width and height in DIPs
    pub fn with_size(self, width: f32, height: f32) -> Self {
        self.with_width(width).with_height(height)
    }

    /// Specify whether the text will be centered within the layout
    pub fn with_centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }
}
