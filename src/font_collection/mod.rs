use error::DWResult;
use factory::Factory;
use font::Font;
use font_face::FontFace;
use font_family::FontFamily;

use std::ptr;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontCollection;
use wio::com::ComPtr;
use wio::wide::ToWide;

pub use font_collection::builder::FontCollectionBuilder;
pub mod builder;
pub mod loader;

pub struct FontCollection {
    ptr: ComPtr<IDWriteFontCollection>,
}

impl FontCollection {
    /// Finds the font family with the specified family name and returns its index
    pub fn find_family_name(&self, family_name: &str) -> DWResult<Option<u32>> {
        unsafe {
            let family = family_name.to_wide_null();
            let mut index = 0;
            let mut exists = 0;
            let hr = self.ptr.FindFamilyName(family.as_ptr(), &mut index, &mut exists);
            if SUCCEEDED(hr){
                Ok(if exists != 0{ Some(index) } else{ None })
            } else {
                Err(hr.into())
            }
        }
    }
    
    pub unsafe fn from_raw(raw: *mut IDWriteFontCollection) -> Self {
        FontCollection {
            ptr: ComPtr::from_raw(raw),
        }
    }

    /// Creates a FontFamily object given a zero-based font family index
    pub fn get_font_family(&self, index: u32) -> DWResult<FontFamily> {
        unsafe {
            let mut ff = ptr::null_mut();
            let hr = self.ptr.GetFontFamily(index, &mut ff);
            if SUCCEEDED(hr){
                Ok(FontFamily::from_raw(ff))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets the number of font families in the collection
    pub fn get_font_family_count(&self) -> DWResult<u32> {
        unsafe {
           Ok(self.ptr.GetFontFamilyCount())
        }
    }

    /// Gets the font object that corresponds to the same physical font as the specified font face object. 
    /// The specified physical font must belong to the font collection. 
    pub fn get_font_from_font_face(&self, font_face: &FontFace) -> DWResult<Font> {
        unsafe {
            let mut f = ptr::null_mut();
            let hr = self.ptr.GetFontFromFontFace(font_face.get_raw(), &mut f);
            if SUCCEEDED(hr){
                Ok(Font::from_raw(f))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Gets a FontCollection object which represents the set of installed fonts. 
    pub fn get_system_font_collection(factory: &Factory, check_for_updates: bool) -> DWResult<FontCollection> {
        unsafe {
            let mut fc = ptr::null_mut();
            let check = if check_for_updates{ 1 } else { 0 };
            let factory_ptr = &*(factory.get_raw());
            let hr = factory_ptr.GetSystemFontCollection(&mut fc, check);
            if SUCCEEDED(hr){
                Ok(FontCollection::from_raw(fc))    
            } else {
                Err(hr.into())
            }
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFontCollection {
        self.ptr.as_raw()
    }
}

unsafe impl Send for FontCollection {}
unsafe impl Sync for FontCollection {}