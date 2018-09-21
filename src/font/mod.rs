use winapi::um::dwrite::IDWriteFont;
use wio::com::ComPtr;

pub struct Font {
    ptr: ComPtr<IDWriteFont>,
}

impl Font {
    pub unsafe fn from_raw(raw: *mut IDWriteFont) -> Self {
        Font {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFont {
        self.ptr.as_raw()
    }
}

unsafe impl Send for Font {}
unsafe impl Sync for Font {}
