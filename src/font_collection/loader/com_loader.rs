use factory::Factory;
use font_collection::loader::com_enumerator::ComEnumerator;
use font_collection::loader::FontCollectionLoader;
use key::KeyPayload;

use std::mem;

use com_impl::{Refcount, VTable};
use winapi::ctypes::c_void;
use winapi::shared::winerror::{E_INVALIDARG, HRESULT, S_OK};
use winapi::um::dwrite::IDWriteFactory;
use winapi::um::dwrite::IDWriteFontFileEnumerator;
use winapi::um::dwrite::{IDWriteFontCollectionLoader, IDWriteFontCollectionLoaderVtbl};
use wio::com::ComPtr;

#[repr(C)]
#[derive(ComImpl)]
pub struct ComFontCollectionLoader<T>
where
    T: FontCollectionLoader,
{
    vtbl: VTable<IDWriteFontCollectionLoaderVtbl>,
    refcount: Refcount,
    loader: T,
}

impl<T> ComFontCollectionLoader<T>
where
    T: FontCollectionLoader,
{
    pub fn new(loader: T) -> ComPtr<IDWriteFontCollectionLoader> {
        let ptr = Self::create_raw(loader);
        let ptr = ptr as *mut IDWriteFontCollectionLoader;
        unsafe { ComPtr::from_raw(ptr) }
    }
}

#[com_impl]
unsafe impl<T> IDWriteFontCollectionLoader for ComFontCollectionLoader<T>
where
    T: FontCollectionLoader,
{
    unsafe fn create_enumerator_from_key(
        &self,
        factory: *mut IDWriteFactory,
        key: *const c_void,
        key_size: u32,
        out_enum: *mut *mut IDWriteFontFileEnumerator,
    ) -> HRESULT {
        if key_size as usize != mem::size_of::<KeyPayload<T::Key>>() {
            return E_INVALIDARG;
        }

        let factory = mem::transmute::<&*mut _, &Factory>(&factory);
        let key = &*(key as *const KeyPayload<T::Key>);

        if !key.valid() {
            return E_INVALIDARG;
        }

        let iter = self.loader.get_iterator(factory, &key.data);
        let iter = match iter {
            Ok(iter) => iter,
            Err(e) => return e.0,
        };

        let enumer = ComEnumerator::new(iter);

        *out_enum = enumer.into_raw();
        S_OK
    }
}

unsafe impl<T> Send for ComFontCollectionLoader<T> where T: FontCollectionLoader {}
unsafe impl<T> Sync for ComFontCollectionLoader<T> where T: FontCollectionLoader {}
