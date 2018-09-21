use inline_object::{DrawingContext, InlineObjectContainer};

use std::mem;
use std::panic::catch_unwind;
use std::sync::Arc;

use winapi::ctypes::c_void;
use winapi::shared::guiddef::{IsEqualIID, REFIID};
use winapi::shared::minwindef::{BOOL, FLOAT, ULONG};
use winapi::shared::winerror::{E_FAIL, E_NOTIMPL, HRESULT, SUCCEEDED, S_OK};
use winapi::um::dwrite::{
    IDWriteInlineObject, IDWriteInlineObjectVtbl, IDWriteTextRenderer, DWRITE_BREAK_CONDITION,
    DWRITE_INLINE_OBJECT_METRICS, DWRITE_OVERHANG_METRICS,
};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::Interface;
use wio::com::ComPtr;

pub static INLINE_OBJECT_VTBL: IDWriteInlineObjectVtbl = IDWriteInlineObjectVtbl {
    parent: IUnknownVtbl {
        QueryInterface: query_interface,
        AddRef: add_ref,
        Release: release,
    },
    Draw: draw,
    GetMetrics: get_metrics,
    GetOverhangMetrics: get_overhang_metrics,
    GetBreakConditions: get_break_conditions,
};

pub unsafe extern "system" fn query_interface(
    this: *mut IUnknown,
    iid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if IsEqualIID(&*iid, &IUnknown::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    if IsEqualIID(&*iid, &IDWriteInlineObject::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    return E_NOTIMPL;
}

unsafe extern "system" fn add_ref(this: *mut IUnknown) -> ULONG {
    let ptr = this as *const InlineObjectContainer;
    let arc = Arc::from_raw(ptr);
    mem::forget(arc.clone());
    let count = Arc::strong_count(&arc);
    mem::forget(arc);
    count as ULONG
}

unsafe extern "system" fn release(this: *mut IUnknown) -> ULONG {
    let ptr = this as *const InlineObjectContainer;
    let arc = Arc::from_raw(ptr);
    let count = Arc::strong_count(&arc);
    mem::drop(arc);
    count as ULONG - 1
}

unsafe extern "system" fn draw(
    this: *mut IDWriteInlineObject,
    ctx: *mut c_void,
    renderer: *mut IDWriteTextRenderer,
    origin_x: FLOAT,
    origin_y: FLOAT,
    is_sideways: BOOL,
    is_rtl: BOOL,
    effect: *mut IUnknown,
) -> HRESULT {
    match catch_unwind(move || {
        let obj = &*(this as *const InlineObjectContainer);

        // Take a reference to the object for working with later
        assert!(!renderer.is_null());
        (*renderer).AddRef();
        let renderer = ComPtr::from_raw(renderer);

        // If there's a client effect, wrap it
        let client_effect = if !effect.is_null() {
            (*effect).AddRef();
            Some(ComPtr::from_raw(effect))
        } else {
            None
        };

        let context = DrawingContext {
            client_context: ctx,
            renderer,
            origin_x,
            origin_y,
            is_sideways: is_sideways != 0,
            is_right_to_left: is_rtl != 0,
            client_effect,
        };

        match obj.obj.draw(&context) {
            Ok(()) => S_OK,
            Err(err) if !SUCCEEDED(err.0) => err.0,
            Err(_) => E_FAIL,
        }
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

unsafe extern "system" fn get_metrics(
    this: *mut IDWriteInlineObject,
    metrics: *mut DWRITE_INLINE_OBJECT_METRICS,
) -> HRESULT {
    match catch_unwind(move || {
        let obj = &*(this as *const InlineObjectContainer);

        let m = match obj.obj.get_metrics() {
            Ok(metrics) => metrics,
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        };

        let metrics = &mut *metrics;
        metrics.width = m.width;
        metrics.height = m.height;
        metrics.baseline = m.baseline;
        metrics.supportsSideways = m.supports_sideways as BOOL;
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

unsafe extern "system" fn get_overhang_metrics(
    this: *mut IDWriteInlineObject,
    metrics: *mut DWRITE_OVERHANG_METRICS,
) -> HRESULT {
    match catch_unwind(move || {
        let obj = &*(this as *const InlineObjectContainer);

        let m = match obj.obj.get_overhang_metrics() {
            Ok(metrics) => metrics,
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        };

        let metrics = &mut *metrics;
        metrics.left = m.left;
        metrics.top = m.top;
        metrics.right = m.right;
        metrics.bottom = m.bottom;
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

unsafe extern "system" fn get_break_conditions(
    this: *mut IDWriteInlineObject,
    before: *mut DWRITE_BREAK_CONDITION,
    after: *mut DWRITE_BREAK_CONDITION,
) -> HRESULT {
    match catch_unwind(move || {
        let obj = &*(this as *const InlineObjectContainer);

        let (b, a) = match obj.obj.get_break_conditions() {
            Ok(result) => result,
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        };

        *before = b as u32;
        *after = a as u32;
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}
