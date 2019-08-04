use crate::font::Font;
use crate::font_collection::FontCollection;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontList;
use wio::com::ComPtr;

#[derive(Clone, ComWrapper, PartialEq)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// Represents a list of fonts.
pub struct FontList {
    ptr: ComPtr<IDWriteFontList>,
}

impl FontList {
    /// Get an iterator over all fonts in the list.
    pub fn all_fonts<'a>(&'a self) -> impl Iterator<Item = Font> + Clone + 'a {
        (0..self.count()).filter_map(move |i| self.get(i))
    }
}

pub unsafe trait IFontList {
    /// The number of fonts in the list
    fn count(&self) -> u32 {
        unsafe { self.raw_fontlist().GetFontCount() }
    }

    /// The collection that owns these fonts
    fn collection(&self) -> Option<FontCollection> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.raw_fontlist().GetFontCollection(&mut ptr);
            if SUCCEEDED(hr) {
                Some(FontCollection::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Get a specific font in the list at the given index.
    fn get(&self, i: u32) -> Option<Font> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.raw_fontlist().GetFont(i, &mut ptr);
            if SUCCEEDED(hr) {
                Some(Font::from_raw(ptr))
            } else {
                None
            }
        }
    }

    unsafe fn raw_fontlist(&self) -> &IDWriteFontList;
}

unsafe impl IFontList for FontList {
    unsafe fn raw_fontlist(&self) -> &IDWriteFontList {
        &self.ptr
    }
}
