use enums::{FontStretch, FontStyle, FontWeight};
use error::DWResult;
use font::Font;
use font_list::FontList;
use helpers::get_system_locale;

use std::ptr;

use winapi::ctypes::wchar_t;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontFamily;
use wio::com::ComPtr;

#[derive(ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct FontFamily {
    ptr: ComPtr<IDWriteFontFamily>,
}

impl FontFamily {
    /// Shortcut method based on GetFamilyNames to get the family name in the current locale (or EN-US or the first locale available, if not found)
    pub fn get_family_name(&self) -> DWResult<String> {
        unsafe {
            let mut ls = ptr::null_mut();
            let hr = self.ptr.GetFamilyNames(&mut ls);
            if SUCCEEDED(hr) {
                let ptr = ComPtr::from_raw(ls);
                let mut locale_idx: u32 = 0;
                let mut exists: i32 = 0;
                let mut hr = ptr.FindLocaleName(
                    (*get_system_locale()).as_ptr(),
                    &mut locale_idx,
                    &mut exists,
                );
                if !SUCCEEDED(hr) || exists == 0 {
                    locale_idx = 0;
                }
                let mut len: u32 = 0;
                hr = ptr.GetStringLength(locale_idx, &mut len);
                if !SUCCEEDED(hr) {
                    return Err(hr.into());
                }
                let mut name: Vec<wchar_t> = Vec::with_capacity(len as usize + 1);
                hr = ptr.GetString(locale_idx, name.as_mut_ptr(), len + 1);
                if !SUCCEEDED(hr) {
                    return Err(hr.into());
                }
                name.set_len(len as usize);
                Ok(String::from_utf16(&name).ok().unwrap())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the font that best matches the specified properties.
    pub fn get_first_matching_font(
        &self,
        weight: FontWeight,
        stretch: FontStretch,
        style: FontStyle,
    ) -> DWResult<Font> {
        unsafe {
            let mut font = ptr::null_mut();
            let hr = self.ptr.GetFirstMatchingFont(
                weight.0,
                stretch as u32,
                style as u32,
                &mut font,
            );
            if SUCCEEDED(hr) {
                Ok(Font::from_raw(font))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets a list of fonts in the font family ranked in order of how well they match the specified properties.
    pub fn get_matching_fonts(
        &self,
        weight: FontWeight,
        stretch: FontStretch,
        style: FontStyle,
    ) -> DWResult<FontList> {
        unsafe {
            let mut list = ptr::null_mut();
            let hr =
                self.ptr
                    .GetMatchingFonts(weight.0, stretch as u32, style as u32, &mut list);
            if SUCCEEDED(hr) {
                Ok(FontList::from_raw(list))
            } else {
                Err(hr.into())
            }
        }
    }
}
