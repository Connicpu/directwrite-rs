use font_collection::loader::{CreateEnumeratorFromKey, FontFileEnumeratorComRef, FontCollectionLoaderComRef};

use std::ffi::CStr;
use std::panic::catch_unwind;

use winapi::Interface;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::{IsEqualIID, REFIID};
use winapi::shared::minwindef::{BOOL, ULONG};
use winapi::shared::winerror::{E_FAIL, E_NOTIMPL, HRESULT, SUCCEEDED, S_OK};
use winapi::um::dwrite::*;
use winapi::um::unknwnbase::*;

pub static FC_LOADER_COMREF_VTBL: IDWriteFontCollectionLoaderVtbl = IDWriteFontCollectionLoaderVtbl {
    parent: IUnknownVtbl {
        QueryInterface: query_interface_loader,
        AddRef: add_ref,
        Release: release,
    },
    CreateEnumeratorFromKey: create_enumerator_from_key,
};

pub static FF_ENUMERATOR_COMREF_VTBL: IDWriteFontFileEnumeratorVtbl = IDWriteFontFileEnumeratorVtbl {
    parent: IUnknownVtbl {
        QueryInterface: query_interface_enumerator,
        AddRef: add_ref,
        Release: release,
    },
    GetCurrentFontFile: get_current_font_file,
    MoveNext: move_next,
};

pub unsafe extern "system" fn query_interface_loader(
    this: *mut IUnknown,
    iid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if IsEqualIID(&*iid, &IUnknown::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    if IsEqualIID(&*iid, &IDWriteFontCollectionLoader::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    return E_NOTIMPL;
}

pub unsafe extern "system" fn query_interface_enumerator(
    this: *mut IUnknown,
    iid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if IsEqualIID(&*iid, &IUnknown::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    if IsEqualIID(&*iid, &IDWriteFontFileEnumerator::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    return E_NOTIMPL;
}

pub unsafe extern "system" fn add_ref(_this: *mut IUnknown) -> ULONG {
    2
}

pub unsafe extern "system" fn release(_this: *mut IUnknown) -> ULONG {
    1
}

pub unsafe extern "system" fn create_enumerator_from_key(
    this: *mut IDWriteFontCollectionLoader,
    factory: *mut IDWriteFactory,
    collection_key: *const c_void,
    _collection_key_size: u32,
    font_file_enumerator: *mut *mut IDWriteFontFileEnumerator,
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &mut *(this as *mut FontCollectionLoaderComRef);
        let context = CreateEnumeratorFromKey{
            factory: factory,
            collection_key: CStr::from_ptr(collection_key as *const _).to_str().unwrap(),
        };
        match comref.obj.create_enumerator_from_key(&context) {
            Ok(mut enumerator) => *font_file_enumerator = enumerator.to_raw(),
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

pub unsafe extern "system" fn get_current_font_file(
    this: *mut IDWriteFontFileEnumerator,
    font_file: *mut *mut IDWriteFontFile
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &mut *(this as *mut FontFileEnumeratorComRef);
        match comref.obj.get_current_font_file() {
            Ok(ff) => *font_file = ff.get_raw(),
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }    
}

pub unsafe extern "system" fn move_next(
    this: *mut IDWriteFontFileEnumerator,
    has_current_file: *mut BOOL
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &mut *(this as *mut FontFileEnumeratorComRef);
        *has_current_file = comref.obj.move_next() as BOOL;
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }    
}