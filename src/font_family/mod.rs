use enums::{FontStretch, FontStyle, FontWeight};
use font::Font;
use font_list::FontList;
use localized_strings::LocalizedStrings;

use std::ptr;

use com_wrapper::ComWrapper;
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
    /// Creates a localized strings object that contains the family names for the font family,
    /// indexed by locale name.
    pub fn family_name(&self) -> Option<LocalizedStrings> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetFamilyNames(&mut ptr);
            if SUCCEEDED(hr) {
                Some(LocalizedStrings::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Gets the font that best matches the specified properties.
    pub fn first_matching_font(
        &self,
        weight: FontWeight,
        stretch: FontStretch,
        style: FontStyle,
    ) -> Option<Font> {
        unsafe {
            let mut font_ptr = ptr::null_mut();
            let hr = self.ptr.GetFirstMatchingFont(
                weight.0,
                stretch as u32,
                style as u32,
                &mut font_ptr,
            );
            if SUCCEEDED(hr) {
                Some(Font::from_raw(font_ptr))
            } else {
                None
            }
        }
    }

    /// Gets a list of fonts in the font family ranked in order of how well they match the
    /// specified properties.
    pub fn matching_fonts(
        &self,
        weight: FontWeight,
        stretch: FontStretch,
        style: FontStyle,
    ) -> Option<FontList> {
        unsafe {
            let mut list = ptr::null_mut();
            let hr = self
                .ptr
                .GetMatchingFonts(weight.0, stretch as u32, style as u32, &mut list);
            if SUCCEEDED(hr) {
                Some(FontList::from_raw(list))
            } else {
                None
            }
        }
    }
}
