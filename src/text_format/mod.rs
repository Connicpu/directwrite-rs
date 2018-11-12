use enums::*;
use error::DWResult;
use factory::Factory;
use font_collection::FontCollection;

use std::ffi::OsString;
use std::ptr;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteTextFormat;
use wio::com::ComPtr;
use wio::wide::FromWide;

pub use self::builder::TextFormatBuilder;

pub mod builder;

#[derive(ComWrapper, PartialEq)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct TextFormat {
    ptr: ComPtr<IDWriteTextFormat>,
}

impl TextFormat {
    pub fn create<'a>(factory: &'a Factory) -> TextFormatBuilder<'a> {
        unsafe { TextFormatBuilder::new(&*factory.get_raw()) }
    }

    pub fn flow_direction(&self) -> UncheckedEnum<FlowDirection> {
        unsafe { self.ptr.GetFlowDirection().into() }
    }

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

    pub fn font_size(&self) -> f32 {
        unsafe { self.ptr.GetFontSize() }
    }

    pub fn font_stretch(&self) -> UncheckedEnum<FontStretch> {
        unsafe { self.ptr.GetFontStretch().into() }
    }

    pub fn font_style(&self) -> UncheckedEnum<FontStyle> {
        unsafe { self.ptr.GetFontStyle().into() }
    }

    pub fn font_weight(&self) -> FontWeight {
        unsafe { FontWeight(self.ptr.GetFontWeight()) }
    }

    pub fn incremental_tabstop(&self) -> f32 {
        unsafe { self.ptr.GetIncrementalTabStop() }
    }

    pub fn line_spacing(&self) -> DWResult<LineSpacing> {
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

    pub fn locale_name(&self) -> DWResult<String> {
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

    pub fn paragraph_alignment(&self) -> UncheckedEnum<ParagraphAlignment> {
        unsafe { self.ptr.GetParagraphAlignment().into() }
    }

    pub fn reading_direction(&self) -> UncheckedEnum<ReadingDirection> {
        unsafe { self.ptr.GetReadingDirection().into() }
    }

    pub fn text_alignment(&self) -> UncheckedEnum<TextAlignment> {
        unsafe { self.ptr.GetTextAlignment().into() }
    }

    // TODO: pub fn trimming

    pub fn word_wrapping(&self) -> UncheckedEnum<WordWrapping> {
        unsafe { self.ptr.GetWordWrapping().into() }
    }

    pub fn set_flow_direction(&mut self, value: FlowDirection) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetFlowDirection(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_incremental_tabstop(&mut self, value: f32) -> DWResult<()> {
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
        &mut self,
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

    pub fn set_paragraph_alignment(&mut self, value: ParagraphAlignment) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetParagraphAlignment(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_reading_direction(&mut self, value: ReadingDirection) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetReadingDirection(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_text_alignment(&mut self, value: TextAlignment) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.SetTextAlignment(value as u32);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn set_word_wrapping(&mut self, value: WordWrapping) -> DWResult<()> {
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

pub struct LineSpacing {
    pub method: UncheckedEnum<LineSpacingMethod>,
    pub spacing: f32,
    pub baseline: f32,
}
