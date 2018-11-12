use font::Font;
use font_collection::FontCollection;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontList;
use wio::com::ComPtr;

#[derive(Clone, ComWrapper, PartialEq)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct FontList {
    ptr: ComPtr<IDWriteFontList>,
}

impl FontList {
    pub fn count(&self) -> u32 {
        unsafe { self.ptr.GetFontCount() }
    }

    pub fn collection(&self) -> Option<FontCollection> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetFontCollection(&mut ptr);
            if SUCCEEDED(hr) {
                Some(FontCollection::from_raw(ptr))
            } else {
                None
            }
        }
    }

    pub fn get(&self, i: u32) -> Option<Font> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetFont(i, &mut ptr);
            if SUCCEEDED(hr) {
                Some(Font::from_raw(ptr))
            } else {
                None
            }
        }
    }

    pub fn all_fonts<'a>(&'a self) -> impl Iterator<Item = Font> + 'a {
        (0..self.count()).filter_map(move |i| self.get(i))
    }
}
