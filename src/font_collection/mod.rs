use winapi::um::dwrite::IDWriteFontCollection;
use wio::com::ComPtr;

pub struct FontCollection {
    ptr: ComPtr<IDWriteFontCollection>,
}

impl FontCollection {
    pub unsafe fn from_raw(raw: *mut IDWriteFontCollection) -> Self {
        FontCollection {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFontCollection {
        self.ptr.as_raw()
    }
}
