//! TextFormat and types for building new ones.

use crate::descriptions::Trimming;
use crate::enums::*;
use crate::factory::Factory;
use crate::font_collection::FontCollection;
use crate::inline_object::InlineObject;

use std::ffi::OsString;
use std::ptr;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteTextFormat;
use wio::com::ComPtr;
use wio::wide::FromWide;

#[doc(inline)]
pub use self::builder::TextFormatBuilder;

#[doc(hidden)]
pub mod builder;

#[derive(ComWrapper, PartialEq)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// Represents a format for laying out text. You can think of this like a Font with all of the
/// little customization boxes filled in.
pub struct TextFormat {
    ptr: ComPtr<IDWriteTextFormat>,
}

impl TextFormat {
    /// Get a builder for creating a new text format.
    pub fn create<'a>(factory: &'a Factory) -> TextFormatBuilder<'a> {
        unsafe { TextFormatBuilder::new(&*factory.get_raw()) }
    }

    /// Get the flow direction of text in this format.
    pub fn flow_direction(&self) -> UncheckedEnum<FlowDirection> {
        unsafe { self.ptr.GetFlowDirection().into() }
    }

    /// Get the font collection this format loaded its font from.
    pub fn font_collection(&self) -> Option<FontCollection> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetFontCollection(&mut ptr);
            if SUCCEEDED(hr) && ptr != ptr::null_mut() {
                Some(FontCollection::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Get the name of the font family specified for this format.
    pub fn font_family_name(&self) -> Option<String> {
        unsafe {
            let len = self.ptr.GetFontFamilyNameLength();
            let mut buf = Vec::with_capacity(len as usize + 1);
            let hr = self.ptr.GetFontFamilyName(buf.as_mut_ptr(), len + 1);
            if SUCCEEDED(hr) {
                buf.set_len(len as usize);
                let osstr = OsString::from_wide(&buf);
                let ff_name = osstr.to_string_lossy().into_owned();
                Some(ff_name)
            } else {
                None
            }
        }
    }

    /// Get the font size associated with this format.
    pub fn font_size(&self) -> f32 {
        unsafe { self.ptr.GetFontSize() }
    }

    /// Get the stretch applied to this format.
    pub fn font_stretch(&self) -> UncheckedEnum<FontStretch> {
        unsafe { self.ptr.GetFontStretch().into() }
    }

    /// Get the style applied to this format.
    pub fn font_style(&self) -> UncheckedEnum<FontStyle> {
        unsafe { self.ptr.GetFontStyle().into() }
    }

    /// Get the weight applied to this format.
    pub fn font_weight(&self) -> FontWeight {
        unsafe { FontWeight(self.ptr.GetFontWeight()) }
    }

    /// Get the incremental tabstop size for this format.
    pub fn incremental_tabstop(&self) -> f32 {
        unsafe { self.ptr.GetIncrementalTabStop() }
    }

    /// Get the line spacing information for this format.
    pub fn line_spacing(&self) -> Result<LineSpacing, Error> {
        unsafe {
            let mut method = 0;
            let mut spacing = 0.0;
            let mut baseline = 0.0;
            let hr = self
                .ptr
                .GetLineSpacing(&mut method, &mut spacing, &mut baseline);
            if SUCCEEDED(hr) {
                let method = method.into();
                Ok(LineSpacing {
                    method,
                    spacing,
                    baseline,
                })
            } else {
                Err(hr.into())
            }
        }
    }

    /// Get the locale used for this format.
    pub fn locale_name(&self) -> Result<String, Error> {
        unsafe {
            let len = self.ptr.GetLocaleNameLength();
            let mut buf = Vec::with_capacity(len as usize + 1);
            let hr = self.ptr.GetLocaleName(buf.as_mut_ptr(), len + 1);
            if SUCCEEDED(hr) {
                buf.set_len(len as usize);
                let osstr = OsString::from_wide(&buf);
                let loc_name = osstr
                    .into_string()
                    .unwrap_or_else(|e| e.to_string_lossy().into_owned());
                Ok(loc_name)
            } else {
                Err(hr.into())
            }
        }
    }

    /// Get the paragraph alignment of text under this format.
    pub fn paragraph_alignment(&self) -> UncheckedEnum<ParagraphAlignment> {
        unsafe { self.ptr.GetParagraphAlignment().into() }
    }

    /// Get the reading direction of text under this format.
    pub fn reading_direction(&self) -> UncheckedEnum<ReadingDirection> {
        unsafe { self.ptr.GetReadingDirection().into() }
    }

    /// Get the alignment of text under this format.
    pub fn text_alignment(&self) -> UncheckedEnum<TextAlignment> {
        unsafe { self.ptr.GetTextAlignment().into() }
    }

    /// Gets the trimming options for text that overflows the layout box.
    ///
    /// The inline object is an omission sign that will be rendered to show that
    /// text was omitted.
    pub fn trimming(&self) -> Result<(Trimming, Option<InlineObject>), Error> {
        unsafe {
            let mut trimming = std::mem::zeroed();
            let mut ptr = std::ptr::null_mut();
            let hr = self.ptr.GetTrimming(&mut trimming, &mut ptr);
            if SUCCEEDED(hr) {
                let obj = if !ptr.is_null() {
                    Some(InlineObject::from_raw(ptr))
                } else {
                    None
                };
                Ok((trimming.into(), obj))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Get the word wrapping for text under this format.
    pub fn word_wrapping(&self) -> UncheckedEnum<WordWrapping> {
        unsafe { self.ptr.GetWordWrapping().into() }
    }

    /// Set the flow direction for text under this format.
    pub fn set_flow_direction(&mut self, value: FlowDirection) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetFlowDirection(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the incremental tabstop value for text under this format.
    pub fn set_incremental_tabstop(&mut self, value: f32) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetIncrementalTabStop(value);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the line spacing metrics for text under this format.
    pub fn set_line_spacing(
        &mut self,
        method: LineSpacingMethod,
        spacing: f32,
        baseline: f32,
    ) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetLineSpacing(method as u32, spacing, baseline);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the paragraph alignment for text under this format.
    pub fn set_paragraph_alignment(&mut self, value: ParagraphAlignment) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetParagraphAlignment(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the reading direction used to lay out text under this format.
    pub fn set_reading_direction(&mut self, value: ReadingDirection) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetReadingDirection(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the text alignment for this format.
    pub fn set_text_alignment(&mut self, value: TextAlignment) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetTextAlignment(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Sets trimming options for text overflowing the layout width.
    pub fn set_trimming(
        &self,
        trimming: &Trimming,
        omission_sign: Option<&InlineObject>,
    ) -> Result<(), Error> {
        unsafe {
            let omission_sign = match omission_sign {
                Some(sign) => sign.get_raw(),
                None => ptr::null_mut(),
            };
            let hr = self
                .ptr
                .SetTrimming(trimming as *const _ as *const _, omission_sign);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Set the word wrapping for text under this format.
    pub fn set_word_wrapping(&mut self, value: WordWrapping) -> Result<(), Error> {
        unsafe {
            let hr = self.ptr.SetWordWrapping(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }
}

/// Information about the line spacing of a format.
pub struct LineSpacing {
    /// The method used for line spacing in a text layout.
    pub method: UncheckedEnum<LineSpacingMethod>,

    /// The amount of spacing to use.
    pub spacing: f32,

    /// The distance from top of line to baseline. A reasonable ratio to `spacing` is 80 percent.
    pub baseline: f32,
}
