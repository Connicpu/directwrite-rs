use font_file::loader::{CreateStreamFromKey, FontFileLoaderComRef, FontFileStreamComRef, ReadFileFragment, ReleaseFileFragment};

use std::ffi::CStr;
use std::panic::catch_unwind;

use winapi::Interface;
use winapi::ctypes::c_void;
use winapi::shared::guiddef::{IsEqualIID, REFIID};
use winapi::shared::minwindef::ULONG;
use winapi::shared::winerror::{E_FAIL, E_NOTIMPL, HRESULT, SUCCEEDED, S_OK};
use winapi::um::dwrite::*;
use winapi::um::unknwnbase::*;

pub static FF_LOADER_COMREF_VTBL: IDWriteFontFileLoaderVtbl = IDWriteFontFileLoaderVtbl {
    parent: IUnknownVtbl {
        QueryInterface: query_interface_loader,
        AddRef: add_ref,
        Release: release,
    },
    CreateStreamFromKey: create_stream_from_key,
};

pub static FF_STREAM_COMREF_VTBL: IDWriteFontFileStreamVtbl = IDWriteFontFileStreamVtbl {
    parent: IUnknownVtbl {
        QueryInterface: query_interface_stream,
        AddRef: add_ref,
        Release: release,
    },
    GetFileSize: get_file_size,
    GetLastWriteTime: get_last_write_time,
    ReadFileFragment: read_file_fragment,
    ReleaseFileFragment: release_file_fragment,
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

    if IsEqualIID(&*iid, &IDWriteFontFileLoader::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    return E_NOTIMPL;
}

pub unsafe extern "system" fn query_interface_stream(
    this: *mut IUnknown,
    iid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if IsEqualIID(&*iid, &IUnknown::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    if IsEqualIID(&*iid, &IDWriteFontFileStream::uuidof()) {
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

pub unsafe extern "system" fn create_stream_from_key(
    this: *mut IDWriteFontFileLoader,
    reference_key: *const c_void,
    _reference_key_size: u32,
    font_file_stream: *mut *mut IDWriteFontFileStream
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &mut *(this as *mut FontFileLoaderComRef);
        let context = CreateStreamFromKey{
            reference_key: CStr::from_ptr(reference_key as *const _).to_str().unwrap()
        };
        match comref.obj.create_stream_from_key(&context) {
            Ok(mut stream) => *font_file_stream = stream.to_raw(),
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

pub unsafe extern "system" fn read_file_fragment(
    this: *mut IDWriteFontFileStream,
    fragment_start: *mut *const c_void,
    file_offset: u64,
    fragment_size: u64,
    fragment_context: *mut *mut c_void
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &mut *(this as *mut FontFileStreamComRef);
        let context = ReadFileFragment{
            file_offset: file_offset,
            fragment_size: fragment_size
        };
        match comref.obj.read_file_fragment(&context) {
            Ok(mut fragment) => {
                *fragment_start = fragment.0.as_mut_slice() as *mut _ as *mut c_void;
                *fragment_context = fragment.1.into_owned().as_mut_str() as *mut _ as *mut c_void;
            }
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

pub unsafe extern "system" fn release_file_fragment(
    this: *mut IDWriteFontFileStream,
    fragment_context: *mut c_void
) -> () {
    let comref = &mut *(this as *mut FontFileStreamComRef);
    let context = ReleaseFileFragment{
        fragment_context: CStr::from_ptr(fragment_context as *mut _).to_str().unwrap()
    };
    comref.obj.release_file_fragment(&context);
}

pub unsafe extern "system" fn get_file_size(
    this: *mut IDWriteFontFileStream,
    file_size: *mut u64,
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &*(this as *mut FontFileStreamComRef);
        match comref.obj.get_font_size() {
            Ok(sz) => *file_size = sz,
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

pub unsafe extern "system" fn get_last_write_time(
    this: *mut IDWriteFontFileStream,
    write_time: *mut u64,
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &*(this as *mut FontFileStreamComRef);
        match comref.obj.get_last_write_time() {
            Ok(tm) => *write_time = tm,
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}
