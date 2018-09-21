use winapi::um::dwrite::IDWriteFontList;
use wio::com::ComPtr;

#[repr(C)]
#[derive(Clone)]
pub struct FontList {
    ptr: ComPtr<IDWriteFontList>,
}

impl FontList {
    pub unsafe fn from_raw(raw: *mut IDWriteFontList) -> Self {
        FontList {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFontList {
        self.ptr.as_raw()
    }
}

unsafe impl Send for FontList {}
unsafe impl Sync for FontList {}
