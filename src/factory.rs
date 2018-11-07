use error::DWResult;
use font_collection_loader::CollectionLoaderHandle;
use font_collection_loader::ComFontCollectionLoader;
use font_collection_loader::FontCollectionLoader;
use font_collection_loader::FontKey;

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{DWriteCreateFactory, IDWriteFactory, DWRITE_FACTORY_TYPE_SHARED};
use winapi::um::unknwnbase::IUnknown;
use winapi::Interface;
use wio::com::ComPtr;

#[derive(Clone, ComWrapper)]
#[com(send, sync)]
#[repr(transparent)]
pub struct Factory {
    ptr: ComPtr<IDWriteFactory>,
}

impl Factory {
    pub fn new() -> DWResult<Factory> {
        unsafe {
            let mut ptr: *mut IDWriteFactory = ptr::null_mut();
            let hr = DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &IDWriteFactory::uuidof(),
                &mut ptr as *mut _ as *mut *mut IUnknown,
            );

            if SUCCEEDED(hr) {
                Ok(Factory::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    pub fn register_custom_loader<F>(&self, loader: F) -> CollectionLoaderHandle<F::Key>
    where
        F: FontCollectionLoader,
    {
        unsafe {
            let com = ComFontCollectionLoader::new(loader);
            let ptr = ComFontCollectionLoader::to_raw_loader(com);

            self.ptr.RegisterFontCollectionLoader(ptr);

            LoaderHandle::from_raw(ptr)
        }
    }

    pub fn unregister_custom_loader<K>(&self, loader: CollectionLoaderHandle<K>) -> DWResult<()>
    where
        K: FontKey,
    {
        unsafe {
            let hr = self.ptr.UnregisterFontCollectionLoader(loader.get_raw());
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }
}
