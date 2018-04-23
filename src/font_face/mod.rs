use winapi::um::dwrite::IDWriteFontFace;
use wio::com::ComPtr;

pub struct FontFace {
    ptr: ComPtr<IDWriteFontFace>,
}

impl FontFace {
    pub unsafe fn from_raw(raw: *mut IDWriteFontFace) -> Self {
        FontFace {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFontFace {
        self.ptr.as_raw()
    }
}

unsafe impl Send for FontFace {}
unsafe impl Sync for FontFace {}
