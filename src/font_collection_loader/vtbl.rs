use factory::Factory;
use font_collection_loader::ComEnumerator;
use font_collection_loader::ComFontCollectionLoader;
use font_collection_loader::FontCollectionLoader;
use font_collection_loader::KeyPayload;

use std::mem;
use std::ptr;
use std::sync::Arc;

use winapi::ctypes::c_void;
use winapi::shared::guiddef::IsEqualIID;
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::E_INVALIDARG;
use winapi::shared::winerror::E_NOINTERFACE;
use winapi::shared::winerror::HRESULT;
use winapi::shared::winerror::S_OK;
use winapi::um::dwrite::IDWriteFactory;
use winapi::um::dwrite::IDWriteFontFileEnumerator;
use winapi::um::dwrite::{IDWriteFontCollectionLoader, IDWriteFontCollectionLoaderVtbl};
use winapi::um::unknwnbase::IUnknown;
use winapi::um::unknwnbase::IUnknownVtbl;
use winapi::Interface;

pub fn loader_vtable_for<T>() -> &'static IDWriteFontCollectionLoaderVtbl
where
    T: FontCollectionLoader,
{
    &IDWriteFontCollectionLoaderVtbl {
        parent: IUnknownVtbl {
            AddRef: add_ref::<T>,
            Release: release::<T>,
            QueryInterface: query_interface,
        },
        CreateEnumeratorFromKey: enum_from_key::<T>,
    }
}

unsafe extern "system" fn add_ref<T>(this: *mut IUnknown) -> u32
where
    T: FontCollectionLoader,
{
    let this = Arc::from_raw(this as *const ComFontCollectionLoader<T>);
    mem::forget(this.clone());
    let count = Arc::strong_count(&this);
    mem::forget(this);
    count as u32
}

unsafe extern "system" fn release<T>(this: *mut IUnknown) -> u32
where
    T: FontCollectionLoader,
{
    let this = Arc::from_raw(this as *const ComFontCollectionLoader<T>);
    let old_count = Arc::strong_count(&this);
    drop(this);
    (old_count - 1) as u32
}

unsafe extern "system" fn query_interface(
    this: *mut IUnknown,
    rrid: *const GUID,
    ptr: *mut *mut c_void,
) -> HRESULT {
    if IsEqualIID(&*rrid, &IUnknown::uuidof())
        || IsEqualIID(&*rrid, &IDWriteFontCollectionLoader::uuidof())
    {
        *ptr = this as *mut c_void;
    } else {
        *ptr = ptr::null_mut();
        return E_NOINTERFACE;
    }

    S_OK
}

unsafe extern "system" fn enum_from_key<T>(
    this: *mut IDWriteFontCollectionLoader,
    factory: *mut IDWriteFactory,
    key: *const c_void,
    key_size: u32,
    out_enum: *mut *mut IDWriteFontFileEnumerator,
) -> HRESULT
where
    T: FontCollectionLoader,
{
    let this = &*(this as *const ComFontCollectionLoader<T>);
    if key_size as usize != mem::size_of::<KeyPayload<T::Key>>() {
        return E_INVALIDARG;
    }

    let factory = mem::transmute::<&*mut _, &Factory>(&factory);
    let key = &*(key as *const KeyPayload<T::Key>);

    if !key.valid() {
        return E_INVALIDARG;
    }

    let iter = this.loader.get_iterator(factory, &key.data);
    let iter = match iter {
        Ok(iter) => iter,
        Err(e) => return e.0,
    };

    let enumer = ComEnumerator::new(iter);

    *out_enum = Arc::into_raw(enumer) as *mut _;
    S_OK
}
