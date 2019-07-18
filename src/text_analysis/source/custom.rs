use crate::text_analysis::source::{TextAnalysisProvider, TextAnalysisSource};

use com_impl::{Refcount, VTable};
use com_wrapper::ComWrapper;
use winapi::shared::winerror::{E_FAIL, S_OK};
use winapi::um::dwrite::DWRITE_READING_DIRECTION;
use winapi::um::dwrite::{
    IDWriteNumberSubstitution, IDWriteTextAnalysisSource, IDWriteTextAnalysisSourceVtbl,
};

#[repr(C)]
#[derive(com_impl::ComImpl)]
pub struct CustomTextAnalysisSource<S: TextAnalysisProvider> {
    vtbl: VTable<IDWriteTextAnalysisSourceVtbl>,
    refcount: Refcount,
    source: S,
}

impl<S: TextAnalysisProvider> CustomTextAnalysisSource<S> {
    pub fn create(source: S) -> TextAnalysisSource {
        unsafe {
            let ptr: *mut Self = Self::create_raw(source);
            let ptr = ptr as *mut IDWriteTextAnalysisSource;
            TextAnalysisSource::from_raw(ptr)
        }
    }
}

#[com_impl::com_impl]
unsafe impl<S> IDWriteTextAnalysisSource for CustomTextAnalysisSource<S>
where
    S: TextAnalysisProvider,
{
    #[panic(result = "E_FAIL")]
    unsafe fn get_locale_name(&self, pos: u32, len: *mut u32, name: *mut *const u16) -> i32 {
        let (locname, loclen) = self.source.locale_name(pos);
        assert_eq!(locname.last(), Some(&0));
        *len = loclen;
        *name = locname.as_ptr();
        S_OK
    }

    #[panic(result = "E_FAIL")]
    unsafe fn get_number_substitution(
        &self,
        pos: u32,
        len: *mut u32,
        sub: *mut *mut IDWriteNumberSubstitution,
    ) -> i32 {
        let (tsub, tlen) = self.source.number_substitution(pos);
        *len = tlen;
        *sub = tsub.into_raw();
        S_OK
    }

    #[panic(result = "0")]
    unsafe fn get_paragraph_reading_direction(&self) -> DWRITE_READING_DIRECTION {
        self.source.paragraph_reading_direction() as DWRITE_READING_DIRECTION
    }

    #[panic(result = "E_FAIL")]
    unsafe fn get_text_at_position(&self, pos: u32, text: *mut *const u16, len: *mut u32) -> i32 {
        if let Some(data) = self.source.text_at(pos) {
            assert!(data.len() <= std::u32::MAX as usize);
            *text = data.as_ptr();
            *len = data.len() as u32;
            S_OK
        } else {
            *text = std::ptr::null();
            *len = 0;
            S_OK
        }
    }

    #[panic(result = "E_FAIL")]
    unsafe fn get_text_before_position(
        &self,
        pos: u32,
        text: *mut *const u16,
        len: *mut u32,
    ) -> i32 {
        if let Some(data) = self.source.text_before(pos) {
            assert!(data.len() <= std::u32::MAX as usize);
            *text = data.as_ptr();
            *len = data.len() as u32;
            S_OK
        } else {
            *text = std::ptr::null();
            *len = 0;
            S_OK
        }
    }
}
