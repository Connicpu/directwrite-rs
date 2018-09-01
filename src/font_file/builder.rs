use error::DWriteError;
use font_file::FontFile;
use font_file::loader::{FontFileLoader, FontFileLoaderComRef};

use std::ptr;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::FILETIME;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{IDWriteFactory, IDWriteFontFile};
use wio::com::ComPtr;
use wio::wide::ToWide;

pub struct FontFileBuilder<'a> {
    factory: &'a IDWriteFactory,
    file_path: Option<&'a str>,
    last_write_time: Option<FILETIME>
}

impl<'a> FontFileBuilder<'a> {
    pub fn new(factory: &'a IDWriteFactory) -> FontFileBuilder<'a> {
        FontFileBuilder {
            factory,
            file_path: None,
            last_write_time: None,
        }
    }

    pub fn build(self) -> Result<FontFile, DWriteError> {
        unsafe {
            let file_path = self.file_path.expect("`file_path` must be specified").to_wide_null();
            let last_write_time = match self.last_write_time{ Some(t) => { &t }
                                                              None => { ptr::null() }};

            let mut ptr: *mut IDWriteFontFile = ptr::null_mut();
            let result = self.factory.CreateFontFileReference(
                file_path.as_ptr(),
                last_write_time,
                &mut ptr,
            );

            if SUCCEEDED(result) {
                let ptr = ComPtr::from_raw(ptr);
                Ok(FontFile { ptr: ptr })
            } else {
                Err(From::from(result))
            }
        }
    }

    pub fn from_loader(self, mut loader: Box<FontFileLoader>, key: &str) -> Result<FontFile, DWriteError> {
        unsafe {
            let mut comref = FontFileLoaderComRef::new(&mut *loader);
            let mut ptr: *mut IDWriteFontFile = ptr::null_mut();
            let result = self.factory.CreateCustomFontFileReference(
                key.as_ptr() as *const c_void,
                key.len() as u32,
                comref.as_raw(),
                &mut ptr,
            );
            if SUCCEEDED(result) {
                Ok(FontFile::from_raw(ptr))
            } else {
                Err(From::from(result))
            }          
        }
    }

    pub fn with_file_path(mut self, file_path: &'a str) -> Self {
        self.file_path = Some(file_path);
        self
    }
}