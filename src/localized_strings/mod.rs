use winapi::um::dwrite::IDWriteLocalizedStrings;
use wio::com::ComPtr;

pub struct LocalizedStrings {
    ptr: ComPtr<IDWriteLocalizedStrings>,
}

impl LocalizedStrings {
    pub unsafe fn from_raw(raw: *mut IDWriteLocalizedStrings) -> Self {
        LocalizedStrings {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteLocalizedStrings {
        self.ptr.as_raw()
    }
}

unsafe impl Send for LocalizedStrings {}
unsafe impl Sync for LocalizedStrings {}