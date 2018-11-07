use error::DWResult;
use font_collection_loader::ComEnumerator;
use font_file::FontFile;

use std::mem;
use std::ptr;
use std::sync::Arc;

use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::IsEqualIID;
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::BOOL;
use winapi::shared::winerror::E_FAIL;
use winapi::shared::winerror::E_NOINTERFACE;
use winapi::shared::winerror::HRESULT;
use winapi::shared::winerror::S_OK;
use winapi::um::dwrite::IDWriteFontFile;
use winapi::um::dwrite::IDWriteFontFileEnumerator;
use winapi::um::dwrite::{IDWriteFontCollectionLoader, IDWriteFontFileEnumeratorVtbl};
use winapi::um::unknwnbase::IUnknown;
use winapi::um::unknwnbase::IUnknownVtbl;
use winapi::Interface;

pub fn enum_vtable_for<I>() -> &'static IDWriteFontFileEnumeratorVtbl
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    &IDWriteFontFileEnumeratorVtbl {
        parent: IUnknownVtbl {
            AddRef: add_ref::<I>,
            Release: release::<I>,
            QueryInterface: query_interface,
        },
        GetCurrentFontFile: get_current_font_file::<I>,
        MoveNext: move_next::<I>,
    }
}

unsafe extern "system" fn add_ref<I>(this: *mut IUnknown) -> u32
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    let this = Arc::from_raw(this as *const ComEnumerator<I>);
    mem::forget(this.clone());
    let count = Arc::strong_count(&this);
    mem::forget(this);
    count as u32
}

unsafe extern "system" fn release<I>(this: *mut IUnknown) -> u32
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    let this = Arc::from_raw(this as *const ComEnumerator<I>);
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

unsafe extern "system" fn get_current_font_file<I>(
    this: *mut IDWriteFontFileEnumerator,
    font_file: *mut *mut IDWriteFontFile,
) -> HRESULT
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    let this = &*(this as *const ComEnumerator<I>);
    if let Some(err) = this.err {
        return err;
    }

    let ptr = this
        .curr
        .clone()
        .map(|f| f.into_raw())
        .unwrap_or(ptr::null_mut());
    *font_file = ptr;

    match ptr.is_null() {
        false => S_OK,
        true => E_FAIL,
    }
}

unsafe extern "system" fn move_next<I>(
    this: *mut IDWriteFontFileEnumerator,
    has_next: *mut BOOL,
) -> HRESULT
where
    I: Iterator<Item = DWResult<FontFile>> + 'static,
{
    let this = &mut *(this as *mut ComEnumerator<I>);
    if let Some(err) = this.err {
        return err;
    }

    let item = this.iter.next();
    let item = match item {
        Some(item) => item,
        None => {
            *has_next = 0;
            return S_OK;
        }
    };

    let item = match item {
        Ok(item) => item,
        Err(e) => {
            this.err = Some(e.0);
            *has_next = 0;
            return e.0;
        }
    };

    this.curr = Some(item);
    *has_next = 1;
    S_OK
}
