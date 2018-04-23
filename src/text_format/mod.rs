use enums::*;
use error::DWResult;
use factory::Factory;
use font_collection::FontCollection;

use std::ffi::OsString;
use std::ptr;

use winapi::shared::winerror::{E_UNEXPECTED, SUCCEEDED};
use winapi::um::dwrite::IDWriteTextFormat;
use wio::com::ComPtr;
use wio::wide::FromWide;

pub use self::builder::TextFormatBuilder;

pub mod builder;

#[derive(Clone, PartialEq)]
pub struct TextFormat {
    ptr: ComPtr<IDWriteTextFormat>,
}

impl TextFormat {
    pub fn create<'a>(factory: &'a Factory) -> TextFormatBuilder<'a> {
        unsafe { TextFormatBuilder::new(&*factory.get_raw()) }
    }

    pub fn get_flow_direction(&self) -> DWResult<FlowDirection> {
        unsafe { FlowDirection::from_u32(self.ptr.GetFlowDirection()).ok_or(E_UNEXPECTED.into()) }
    }

    pub fn get_font_collection(&self) -> Option<FontCollection> {
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

    pub fn get_font_family_name(&self) -> DWResult<String> {
        unsafe {
            let len = self.ptr.GetFontFamilyNameLength();
            let mut buf = Vec::with_capacity(len as usize + 1);
            let hr = self.ptr.GetFontFamilyName(buf.as_mut_ptr(), len + 1);
            if SUCCEEDED(hr) {
                buf.set_len(len as usize);
                let osstr = OsString::from_wide(&buf);
                let ff_name = osstr
                    .into_string()
                    .unwrap_or_else(|e| e.to_string_lossy().into_owned());
                Ok(ff_name)
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn get_font_size(&self) -> f32 {
        unsafe { self.ptr.GetFontSize() }
    }

    pub fn get_font_stretch(&self) -> DWResult<FontStretch> {
        unsafe { FontStretch::from_u32(self.ptr.GetFontStretch()).ok_or(E_UNEXPECTED.into()) }
    }

    pub fn get_font_style(&self) -> DWResult<FontStyle> {
        unsafe { FontStyle::from_u32(self.ptr.GetFontStyle()).ok_or(E_UNEXPECTED.into()) }
    }

    pub fn get_font_weight(&self) -> DWResult<FontWeight> {
        unsafe { FontWeight::from_u32(self.ptr.GetFontWeight()).ok_or(E_UNEXPECTED.into()) }
    }

    pub fn get_incremental_tabstop(&self) -> f32 {
        unsafe { self.ptr.GetIncrementalTabStop() }
    }

    pub fn get_line_spacing(&self) -> DWResult<LineSpacing> {
        unsafe {
            let mut method = 0;
            let mut spacing = 0.0;
            let mut baseline = 0.0;
            let hr = self.ptr
                .GetLineSpacing(&mut method, &mut spacing, &mut baseline);
            if SUCCEEDED(hr) {
                let method = LineSpacingMethod::from_u32(method).ok_or(E_UNEXPECTED)?;
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

    pub fn get_locale_name(&self) -> DWResult<String> {
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

    pub fn get_paragraph_alignment(&self) -> DWResult<ParagraphAlignment> {
        unsafe {
            ParagraphAlignment::from_u32(self.ptr.GetParagraphAlignment())
                .ok_or(E_UNEXPECTED.into())
        }
    }

    pub fn get_reading_direction(&self) -> DWResult<ReadingDirection> {
        unsafe {
            ReadingDirection::from_u32(self.ptr.GetReadingDirection()).ok_or(E_UNEXPECTED.into())
        }
    }

    pub fn get_text_alignment(&self) -> DWResult<TextAlignment> {
        unsafe { TextAlignment::from_u32(self.ptr.GetTextAlignment()).ok_or(E_UNEXPECTED.into()) }
    }

    // TODO: pub fn get_trimming

    pub fn get_word_wrapping(&self) -> DWResult<WordWrapping> {
        unsafe { WordWrapping::from_u32(self.ptr.GetWordWrapping()).ok_or(E_UNEXPECTED.into()) }
    }

    pub fn set_flow_direction(&self, value: FlowDirection) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetFlowDirection(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_incremental_tabstop(&self, value: f32) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetIncrementalTabStop(value);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_line_spacing(
        &self,
        method: LineSpacingMethod,
        spacing: f32,
        baseline: f32,
    ) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetLineSpacing(method as u32, spacing, baseline);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_paragraph_alignment(&self, value: ParagraphAlignment) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetParagraphAlignment(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_reading_direction(&self, value: ReadingDirection) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetReadingDirection(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_text_alignment(&self, value: ReadingDirection) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetTextAlignment(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_word_wrapping(&self, value: WordWrapping) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetWordWrapping(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub unsafe fn from_raw(ptr: *mut IDWriteTextFormat) -> Self {
        TextFormat {
            ptr: ComPtr::from_raw(ptr),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteTextFormat {
        self.ptr.as_raw()
    }
}

pub struct LineSpacing {
    pub method: LineSpacingMethod,
    pub spacing: f32,
    pub baseline: f32,
}
