use factory::Factory;

use com_wrapper::ComWrapper;
use winapi::um::dwrite::IDWriteFontFile;
use wio::com::ComPtr;

pub use self::builder::FontFileBuilder;

pub mod builder;

#[derive(ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct FontFile {
    ptr: ComPtr<IDWriteFontFile>,
}

impl FontFile {
    pub fn create(factory: &Factory) -> FontFileBuilder {
        unsafe { FontFileBuilder::new(&*factory.get_raw()) }
    }
}
