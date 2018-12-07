use crate::descriptions::FontKey;
use crate::error::DWResult;
use crate::factory::Factory;
use crate::font_collection::loader::com_loader::ComFontCollectionLoader;
use crate::font_collection::loader::FontCollectionLoader;

use std::marker::PhantomData;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontCollectionLoader;
use wio::com::ComPtr;

#[repr(C)]
/// A handle to a registered FontCollection. Use this for constructing
/// a new FontCollection.
pub struct CollectionLoaderHandle<K: FontKey + ?Sized> {
    pub(crate) ptr: ComPtr<IDWriteFontCollectionLoader>,
    _marker: PhantomData<K>,
}

impl<K: FontKey + ?Sized> CollectionLoaderHandle<K> {
    /// Register the loader with the Factory so that its collections can be loaded.
    pub fn register<T>(factory: &Factory, loader: T) -> DWResult<Self>
    where
        T: FontCollectionLoader<Key = K>,
    {
        unsafe {
            let com = ComFontCollectionLoader::new(loader);
            let hr = (*factory.get_raw()).RegisterFontCollectionLoader(com.as_raw());
            if SUCCEEDED(hr) {
                Ok(CollectionLoaderHandle::from_ptr(com))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Unregister the loader from the factory so that it can be deallocated.
    pub fn unregister(self, factory: &Factory) {
        unsafe {
            (*factory.get_raw()).UnregisterFontCollectionLoader(self.ptr.as_raw());
        }
    }
}

impl<K: FontKey + ?Sized> Clone for CollectionLoaderHandle<K> {
    fn clone(&self) -> Self {
        CollectionLoaderHandle {
            ptr: self.ptr.clone(),
            _marker: PhantomData,
        }
    }
}

unsafe impl<K: FontKey + ?Sized> Send for CollectionLoaderHandle<K> {}
unsafe impl<K: FontKey + ?Sized> Sync for CollectionLoaderHandle<K> {}

impl<K: FontKey + ?Sized> ComWrapper for CollectionLoaderHandle<K> {
    type Interface = IDWriteFontCollectionLoader;

    unsafe fn get_raw(&self) -> *mut IDWriteFontCollectionLoader {
        self.ptr.as_raw()
    }

    unsafe fn into_raw(self) -> *mut IDWriteFontCollectionLoader {
        self.ptr.into_raw()
    }

    unsafe fn from_raw(raw: *mut IDWriteFontCollectionLoader) -> Self {
        Self::from_ptr(ComPtr::from_raw(raw))
    }

    unsafe fn from_ptr(ptr: ComPtr<IDWriteFontCollectionLoader>) -> Self {
        CollectionLoaderHandle {
            ptr,
            _marker: PhantomData,
        }
    }

    unsafe fn into_ptr(self) -> ComPtr<IDWriteFontCollectionLoader> {
        self.ptr
    }
}
