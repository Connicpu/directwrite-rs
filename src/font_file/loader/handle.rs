use crate::descriptions::FontKey;
use crate::error::DWResult;
use crate::factory::Factory;
use crate::font_file::loader::com_loader::ComFontFileLoader;
use crate::font_file::loader::FontFileLoader;

use com_wrapper::ComWrapper;
use std::marker::PhantomData;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFontFileLoader;
use wio::com::ComPtr;

#[repr(C)]
/// A handle to a registered file loader. You can use this to load custom font files.
pub struct FileLoaderHandle<K: FontKey + ?Sized> {
    pub(crate) ptr: ComPtr<IDWriteFontFileLoader>,
    _marker: PhantomData<K>,
}

impl<K: FontKey + ?Sized> FileLoaderHandle<K> {
    /// Register a new file loader into the factory and get a handle
    /// that you can use to load custom font files.
    pub fn register<T>(factory: &Factory, loader: T) -> DWResult<Self>
    where
        T: FontFileLoader<Key = K>,
    {
        unsafe {
            let com = ComFontFileLoader::new(loader);
            let hr = (*factory.get_raw()).RegisterFontFileLoader(com.as_raw());
            if SUCCEEDED(hr) {
                Ok(FileLoaderHandle::from_ptr(com))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Unregister the loader. This will invalidate all handles to this loader.
    pub fn unregister(self, factory: &Factory) {
        unsafe {
            (*factory.get_raw()).UnregisterFontFileLoader(self.ptr.as_raw());
        }
    }
}

impl<K: FontKey + ?Sized> Clone for FileLoaderHandle<K> {
    fn clone(&self) -> Self {
        FileLoaderHandle {
            ptr: self.ptr.clone(),
            _marker: PhantomData,
        }
    }
}

unsafe impl<K: FontKey + ?Sized> Send for FileLoaderHandle<K> {}
unsafe impl<K: FontKey + ?Sized> Sync for FileLoaderHandle<K> {}

impl<K: FontKey + ?Sized> ComWrapper for FileLoaderHandle<K> {
    type Interface = IDWriteFontFileLoader;

    unsafe fn get_raw(&self) -> *mut IDWriteFontFileLoader {
        self.ptr.as_raw()
    }

    unsafe fn into_raw(self) -> *mut IDWriteFontFileLoader {
        self.ptr.into_raw()
    }

    unsafe fn from_raw(raw: *mut IDWriteFontFileLoader) -> Self {
        Self::from_ptr(ComPtr::from_raw(raw))
    }

    unsafe fn from_ptr(ptr: ComPtr<IDWriteFontFileLoader>) -> Self {
        FileLoaderHandle {
            ptr,
            _marker: PhantomData,
        }
    }

    unsafe fn into_ptr(self) -> ComPtr<IDWriteFontFileLoader> {
        self.ptr
    }
}
