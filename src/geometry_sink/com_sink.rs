use geometry_sink::GeometrySink;

use com_impl::{Refcount, VTable};
use math2d::{BezierSegment, Point2f};
use winapi::shared::winerror::{HRESULT, S_OK};
use winapi::um::d2d1::D2D1_BEZIER_SEGMENT;
use winapi::um::d2d1::D2D1_POINT_2F;
use winapi::um::d2d1::{ID2D1SimplifiedGeometrySink, ID2D1SimplifiedGeometrySinkVtbl};
use winapi::um::dwrite::IDWriteGeometrySink;
use wio::com::ComPtr;

#[repr(C)]
#[derive(ComImpl)]
pub struct ComGeometrySink<T>
where
    T: GeometrySink,
{
    vtbl: VTable<ID2D1SimplifiedGeometrySinkVtbl>,
    refcount: Refcount,
    sink: T,
}

impl<T> ComGeometrySink<T>
where
    T: GeometrySink,
{
    pub fn new(sink: T) -> ComPtr<IDWriteGeometrySink> {
        let ptr = Self::create_raw(sink);
        let ptr = ptr as *mut IDWriteGeometrySink;
        unsafe { ComPtr::from_raw(ptr) }
    }
}

#[com_impl]
unsafe impl<T> ID2D1SimplifiedGeometrySink for ComGeometrySink<T>
where
    T: GeometrySink,
{
    unsafe fn begin_figure(&mut self, start: D2D1_POINT_2F, begin_flag: u32) {
        self.sink.begin_figure(start.into(), begin_flag);
    }

    unsafe fn end_figure(&mut self, end_flag: u32) {
        self.sink.end_figure(end_flag);
    }

    unsafe fn set_fill_mode(&mut self, mode: u32) {
        self.sink.set_fill_mode(mode);
    }

    unsafe fn set_segment_flags(&mut self, flags: u32) {
        self.sink.set_segment_flags(flags);
    }

    unsafe fn add_beziers(&mut self, beziers: *const D2D1_BEZIER_SEGMENT, count: u32) {
        let slice = std::slice::from_raw_parts(beziers as *const BezierSegment, count as usize);
        self.sink.add_beziers(slice)
    }

    unsafe fn add_lines(&mut self, beziers: *const D2D1_POINT_2F, count: u32) {
        let slice = std::slice::from_raw_parts(beziers as *const Point2f, count as usize);
        self.sink.add_lines(slice)
    }

    unsafe fn close(&mut self) -> HRESULT {
        self.sink.close().map(|_| S_OK).unwrap_or_else(|e| e.0)
    }
}
