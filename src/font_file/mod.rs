use factory::Factory;
use winapi::um::dwrite::IDWriteFontFile;
use wio::com::ComPtr;

pub use self::builder::FontFileBuilder;
pub mod builder;

#[repr(C)]
#[derive(Clone)]
pub struct FontFile {
    ptr: ComPtr<IDWriteFontFile>,
}

impl FontFile {
    pub fn create(factory: &Factory) -> FontFileBuilder {
        unsafe { FontFileBuilder::new(&*factory.get_raw()) }
    }

    pub unsafe fn from_raw(raw: *mut IDWriteFontFile) -> Self {
        FontFile {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFontFile {
        self.ptr.as_raw()
    }
}

unsafe impl Send for FontFile {}
unsafe impl Sync for FontFile {}
