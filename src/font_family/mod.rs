use enums::{FontStretch, FontStyle, FontWeight};
use error::DWResult;
use font::Font;
use font_list::FontList;
use localized_strings::LocalizedStrings;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{IDWriteFontFamily};
use wio::com::ComPtr;

pub struct FontFamily{
    ptr: ComPtr<IDWriteFontFamily>,
}

impl FontFamily{
    pub unsafe fn from_raw(raw: *mut IDWriteFontFamily) -> Self {
        FontFamily {
            ptr: ComPtr::from_raw(raw),
        }
    }

    /// Creates a localized strings object that contains the family names for the font family, indexed by locale name. 
    pub fn get_family_names(&self) -> DWResult<LocalizedStrings>{
        unsafe{
            let mut strings = ptr::null_mut();
            let hr = self.ptr.GetFamilyNames(&mut strings);
            if SUCCEEDED(hr){
                Ok(LocalizedStrings::from_raw(strings))
            } else {
                Err(hr.into())
            }   
        }     
    }    

    /// Gets the font that best matches the specified properties.
    pub fn get_first_matching_font(&self, weight: FontWeight, stretch: FontStretch, style: FontStyle) -> DWResult<Font>{
        unsafe{
            let mut font = ptr::null_mut();
            let hr = self.ptr.GetFirstMatchingFont(weight as u32, stretch as u32, style as u32, &mut font);
            if SUCCEEDED(hr){
                Ok(Font::from_raw(font))
            } else {
                Err(hr.into())
            }
        }   
    }

    /// Gets a list of fonts in the font family ranked in order of how well they match the specified properties.
    pub fn get_matching_fonts(&self, weight: FontWeight, stretch: FontStretch, style: FontStyle) -> DWResult<FontList>{
        unsafe{
            let mut list = ptr::null_mut();
            let hr = self.ptr.GetMatchingFonts(weight as u32, stretch as u32, style as u32, &mut list);
            if SUCCEEDED(hr){
                Ok(FontList::from_raw(list))
            } else {
                Err(hr.into())
            }  
        }      
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFontFamily {
        self.ptr.as_raw()
    }
}

unsafe impl Send for FontFamily {}
unsafe impl Sync for FontFamily {}