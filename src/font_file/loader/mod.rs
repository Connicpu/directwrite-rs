use error::DWResult;

use std::borrow::Cow;

use winapi::um::dwrite::*;

pub mod vtbl;

pub trait FontFileLoader {
    fn create_stream_from_key(&mut self, context: &CreateStreamFromKey) -> DWResult<Box<FontFileStream>>;
}

pub trait FontFileStream {
    fn get_font_size(&self) -> DWResult<u64>;
    fn get_last_write_time(&self) -> DWResult<u64>;

    fn read_file_fragment(&mut self, context: &ReadFileFragment) -> DWResult<(Vec<u8>, Cow<str>)>;
    fn release_file_fragment(&mut self, context: &ReleaseFileFragment) -> ();
    fn to_raw(&mut self) -> *mut IDWriteFontFileStream;
}

pub struct CreateStreamFromKey<'a> {
    pub reference_key: &'a str,
}

pub struct ReadFileFragment {
    pub file_offset: u64,
    pub fragment_size: u64,
}

pub struct ReleaseFileFragment<'a> {
    pub fragment_context: &'a str,
}

#[repr(C)]
pub struct FontFileLoaderComRef<'a> {
    vtbl: *const IDWriteFontFileLoaderVtbl,
    obj: &'a mut FontFileLoader,
}

impl<'a> FontFileLoaderComRef<'a> {
    pub fn new(loader: &'a mut FontFileLoader) -> FontFileLoaderComRef<'a> {
        FontFileLoaderComRef {
            vtbl: &vtbl::FF_LOADER_COMREF_VTBL,
            obj: loader,
        }
    }

    pub unsafe fn as_raw(&mut self) -> &mut IDWriteFontFileLoader {
        &mut *(self as *mut _ as *mut _)
    }
}

#[repr(C)]
pub struct FontFileStreamComRef<'a> {
    vtbl: *const IDWriteFontFileStreamVtbl,
    obj: &'a mut FontFileStream,
}

impl<'a> FontFileStreamComRef<'a> {
    pub fn new(stream: &'a mut FontFileStream) -> FontFileStreamComRef<'a> {
        FontFileStreamComRef {
            vtbl: &vtbl::FF_STREAM_COMREF_VTBL,
            obj: stream,
        }
    }

    pub unsafe fn as_raw(&mut self) -> &mut IDWriteFontFileLoader {
        &mut *(self as *mut _ as *mut _)
    }
}