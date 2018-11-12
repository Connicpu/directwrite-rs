use error::DWResult;
use factory::Factory;
use font::Font;
use font_face::FontFace;
use font_family::FontFamily;
use key::FontKey;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontCollection;
use wio::com::ComPtr;
use wio::wide::ToWide;

#[doc(inline)]
pub use font_collection::builder::FontCollectionBuilder;

#[doc(hidden)]
pub mod builder;
pub mod loader;

#[derive(Clone, ComWrapper, PartialEq)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct FontCollection {
    ptr: ComPtr<IDWriteFontCollection>,
}

impl FontCollection {
    pub fn create<'a, K>(factory: &'a Factory) -> FontCollectionBuilder<'a, K>
    where
        K: FontKey,
    {
        FontCollectionBuilder::new(factory)
    }

    /// Finds the font family with the specified family name and returns its index
    pub fn find_family_by_name(&self, family_name: &str) -> Option<u32> {
        unsafe {
            let family = family_name.to_wide_null();
            let family = family.as_ptr();
            let mut idx = 0;
            let mut exists = 0;
            let hr = self.ptr.FindFamilyName(family, &mut idx, &mut exists);
            if SUCCEEDED(hr) && exists != 0 {
                Some(idx)
            } else {
                None
            }
        }
    }

    /// Gets the number of font families in the collection
    pub fn family_count(&self) -> u32 {
        unsafe { self.ptr.GetFontFamilyCount() }
    }

    /// Gets a FontFamily object given a zero-based font family index
    pub fn family(&self, index: u32) -> Option<FontFamily> {
        unsafe {
            let mut ff = ptr::null_mut();
            let hr = self.ptr.GetFontFamily(index, &mut ff);
            if SUCCEEDED(hr) {
                Some(FontFamily::from_raw(ff))
            } else {
                None
            }
        }
    }

    /// Get an iterator of all font families in this collection
    pub fn all_families<'a>(&'a self) -> impl Iterator<Item = FontFamily> + 'a {
        (0..self.family_count()).filter_map(move |i| self.family(i))
    }

    /// Gets the font object that corresponds to the same physical font as the specified font face object.
    /// The specified physical font must belong to the font collection.
    pub fn font_from_face(&self, face: &FontFace) -> Option<Font> {
        unsafe {
            let mut f = ptr::null_mut();
            let hr = self.ptr.GetFontFromFontFace(face.get_raw(), &mut f);
            if SUCCEEDED(hr) {
                Some(Font::from_raw(f))
            } else {
                None
            }
        }
    }

    /// Gets a FontCollection object which represents the set of installed fonts.
    pub fn system_font_collection(
        factory: &Factory,
        check_for_updates: bool,
    ) -> DWResult<FontCollection> {
        unsafe {
            let mut fc = ptr::null_mut();
            let check = if check_for_updates { 1 } else { 0 };
            let factory_ptr = &*(factory.get_raw());
            let hr = factory_ptr.GetSystemFontCollection(&mut fc, check);
            if SUCCEEDED(hr) {
                Ok(FontCollection::from_raw(fc))
            } else {
                Err(hr.into())
            }
        }
    }
}
