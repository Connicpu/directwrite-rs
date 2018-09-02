use error::DWResult;
use factory::Factory;
use font_file::FontFile;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{IDWriteFactory, IDWriteFontFileEnumerator, IDWriteFontFileEnumeratorVtbl, IDWriteFontCollectionLoader, IDWriteFontCollectionLoaderVtbl};

pub mod vtbl;

pub trait FontFileEnumerator{
    fn get_current_font_file(&mut self) -> DWResult<FontFile>;
    fn move_next(&mut self) -> bool;
    fn to_raw(&mut self) -> *mut IDWriteFontFileEnumerator;
}

pub trait FontCollectionLoader{
    fn create_enumerator_from_key(&mut self, context: &CreateEnumeratorFromKey) -> DWResult<Box<FontFileEnumerator>>;
    fn to_raw(&mut self) -> *mut IDWriteFontCollectionLoader;
}

pub struct CreateEnumeratorFromKey<'a> {
    pub factory: *mut IDWriteFactory,
    pub collection_key: &'a str
}

/// Registers a custom font collection loader with DirectWrite. Must be called once and only once
/// before creating an object with a given FontCollectionLoader.
pub fn register_font_collection_loader(factory: Factory, mut loader: Box<FontCollectionLoader>) -> DWResult<()> {
    unsafe {
        let ptr = &*(factory.get_raw());
        let hr = ptr.RegisterFontCollectionLoader(loader.to_raw());
        if SUCCEEDED(hr) {
            return Ok(())
        } else {
            return Err(hr.into())
        }
    }
}

/// Unregisters a custom font collection loader with DirectWrite.
pub fn unregister_font_collection_loader(factory: Factory, mut loader: Box<FontCollectionLoader>) -> DWResult<()> {
    unsafe {
        let ptr = &*(factory.get_raw());
        let hr = ptr.RegisterFontCollectionLoader(loader.to_raw());
        if SUCCEEDED(hr) {
            return Ok(())
        } else {
            return Err(hr.into())
        }
    }
}

#[repr(C)]
pub struct FontCollectionLoaderComRef<'a> {
    vtbl: *const IDWriteFontCollectionLoaderVtbl,
    obj: &'a mut FontCollectionLoader,
}

impl<'a> FontCollectionLoaderComRef<'a> {
    pub fn new(loader: &'a mut FontCollectionLoader) -> FontCollectionLoaderComRef<'a> {
        FontCollectionLoaderComRef {
            vtbl: &vtbl::FC_LOADER_COMREF_VTBL,
            obj: loader,
        }
    }

    pub unsafe fn as_raw(&mut self) -> &mut IDWriteFontCollectionLoader {
        &mut *(self as *mut _ as *mut _)
    }
}

#[repr(C)]
pub struct FontFileEnumeratorComRef<'a> {
    vtbl: *const IDWriteFontFileEnumeratorVtbl,
    obj: &'a mut FontFileEnumerator,
}

impl<'a> FontFileEnumeratorComRef<'a> {
    pub fn new(stream: &'a mut FontFileEnumerator) -> FontFileEnumeratorComRef<'a> {
        FontFileEnumeratorComRef {
            vtbl: &vtbl::FF_ENUMERATOR_COMREF_VTBL,
            obj: stream,
        }
    }

    pub unsafe fn as_raw(&mut self) -> &mut IDWriteFontFileEnumerator {
        &mut *(self as *mut _ as *mut _)
    }
}