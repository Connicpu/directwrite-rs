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
/// Represents a list of fonts.
pub struct FontList {
    ptr: ComPtr<IDWriteFontList>,
}

impl FontList {
    /// The number of fonts in the list
    pub fn count(&self) -> u32 {
        unsafe { self.ptr.GetFontCount() }
    }

    /// The collection that owns these fonts
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

    /// Get a specific font in the list at the given index.
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

    /// Get an iterator over all fonts in the list.
    pub fn all_fonts<'a>(&'a self) -> impl Iterator<Item = Font> + Clone + 'a {
        (0..self.count()).filter_map(move |i| self.get(i))
    }
}
