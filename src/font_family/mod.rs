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

    pub unsafe fn get_raw(&self) -> *mut IDWriteFontFamily {
        self.ptr.as_raw()
    }
}

unsafe impl Send for FontFamily {}
unsafe impl Sync for FontFamily {}