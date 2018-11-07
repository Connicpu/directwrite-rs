use geometry_sink::ComGeometrySink;
use geometry_sink::GeometrySink;
use winapi::shared::winerror::E_NOINTERFACE;

use std::ptr;
use std::sync::atomic::Ordering;

use math2d::BezierSegment;
use math2d::Point2f;
use winapi::ctypes::c_void;
use winapi::shared::basetsd::UINT32;
use winapi::shared::guiddef::IsEqualIID;
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::{HRESULT, S_OK};
use winapi::um::d2d1::{ID2D1SimplifiedGeometrySink, D2D1_BEZIER_SEGMENT, D2D1_POINT_2F};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::Interface;

use winapi::um::d2d1::ID2D1SimplifiedGeometrySinkVtbl;

pub fn vtable_for<T>() -> &'static ID2D1SimplifiedGeometrySinkVtbl
where
    T: GeometrySink,
{
    &ID2D1SimplifiedGeometrySinkVtbl {
        parent: IUnknownVtbl {
            AddRef: add_ref::<T>,
            Release: release::<T>,
            QueryInterface: query_interface,
        },
        AddBeziers: add_beziers::<T>,
        AddLines: add_lines::<T>,
        BeginFigure: begin_figure::<T>,
        Close: close::<T>,
        EndFigure: end_figure::<T>,
        SetFillMode: set_fill_mode::<T>,
        SetSegmentFlags: set_segment_flags::<T>,
    }
}

unsafe extern "system" fn add_ref<T>(this: *mut IUnknown) -> u32
where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    this.safety_refcount.fetch_add(1, Ordering::SeqCst) as u32 + 1
}

unsafe extern "system" fn release<T>(this: *mut IUnknown) -> u32
where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    this.safety_refcount.fetch_add(1, Ordering::SeqCst) as u32 + 1
}

unsafe extern "system" fn query_interface(
    this: *mut IUnknown,
    rrid: *const GUID,
    ptr: *mut *mut c_void,
) -> HRESULT {
    if IsEqualIID(&*rrid, &IUnknown::uuidof())
        || IsEqualIID(&*rrid, &ID2D1SimplifiedGeometrySink::uuidof())
    {
        *ptr = this as *mut c_void;
    } else {
        *ptr = ptr::null_mut();
        return E_NOINTERFACE;
    }

    S_OK
}

unsafe extern "system" fn begin_figure<T>(
    this: *mut ID2D1SimplifiedGeometrySink,
    start: D2D1_POINT_2F,
    begin_flag: u32,
) where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    this.sink.begin_figure(start.into(), begin_flag);
}

unsafe extern "system" fn end_figure<T>(this: *mut ID2D1SimplifiedGeometrySink, end_flag: u32)
where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    this.sink.end_figure(end_flag);
}

unsafe extern "system" fn set_fill_mode<T>(this: *mut ID2D1SimplifiedGeometrySink, mode: u32)
where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    this.sink.set_fill_mode(mode);
}

unsafe extern "system" fn set_segment_flags<T>(this: *mut ID2D1SimplifiedGeometrySink, flags: u32)
where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    this.sink.set_segment_flags(flags);
}

unsafe extern "system" fn add_beziers<T>(
    this: *mut ID2D1SimplifiedGeometrySink,
    beziers: *const D2D1_BEZIER_SEGMENT,
    count: UINT32,
) where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    let slice = std::slice::from_raw_parts(beziers as *const BezierSegment, count as usize);
    this.sink.add_beziers(slice)
}

unsafe extern "system" fn add_lines<T>(
    this: *mut ID2D1SimplifiedGeometrySink,
    beziers: *const D2D1_POINT_2F,
    count: UINT32,
) where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    let slice = std::slice::from_raw_parts(beziers as *const Point2f, count as usize);
    this.sink.add_lines(slice)
}

unsafe extern "system" fn close<T>(this: *mut ID2D1SimplifiedGeometrySink) -> HRESULT
where
    T: GeometrySink,
{
    let this = &mut *(this as *mut ComGeometrySink<T>);
    this.sink.close().map(|_| S_OK).unwrap_or_else(|e| e.0)
}
