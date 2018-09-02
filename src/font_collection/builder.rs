use error::DWriteError;
use font_collection::FontCollection;
use font_collection::loader::FontCollectionLoader;

use std::ptr;

use winapi::ctypes::c_void;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{IDWriteFactory, IDWriteFontCollection};
use wio::com::ComPtr;

pub struct FontCollectionBuilder<'a> {
    factory: &'a IDWriteFactory,
    collection_loader: Option<Box<FontCollectionLoader>>,
    collection_key: Option<&'a str>
}

impl<'a> FontCollectionBuilder<'a> {
    pub fn new(factory: &'a IDWriteFactory) -> FontCollectionBuilder<'a> {
        FontCollectionBuilder {
            factory,
            collection_loader: None,
            collection_key: None
        }
    }

    pub fn build(self) -> Result<FontCollection, DWriteError> {
        unsafe {
            let mut collection_loader = self.collection_loader.expect("`collection_loader` must be specified");
            let collection_key = self.collection_key.expect("`collection_key` must be specified");

            let mut ptr: *mut IDWriteFontCollection = ptr::null_mut();
            let result = self.factory.CreateCustomFontCollection(
                    &mut *collection_loader.to_raw(),
                    collection_key as *const _ as *const c_void,
                    collection_key.len() as u32,
                    &mut ptr
            );

            if SUCCEEDED(result) {
                let ptr = ComPtr::from_raw(ptr);
                Ok(FontCollection { ptr: ptr })            
            } else{
                Err(From::from(result))
            }
        }
    }   

    pub fn with_collection_loader(mut self, collection_loader: Box<FontCollectionLoader>) -> Self {
        self.collection_loader = Some(collection_loader);
        self
    }

    pub fn with_collection_key(mut self, collection_key: &'a str) -> Self {
        self.collection_key = Some(collection_key);
        self
    }
}