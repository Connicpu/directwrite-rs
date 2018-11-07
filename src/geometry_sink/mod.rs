use error::DWResult;

use std::sync::atomic::{AtomicUsize, Ordering};

use math2d::BezierSegment;
use math2d::Point2f;
use winapi::um::d2d1::{ID2D1SimplifiedGeometrySink, ID2D1SimplifiedGeometrySinkVtbl};

mod vtbl;

pub trait GeometrySink: Sized {
    fn begin_figure(&mut self, start: Point2f, begin_flag: u32);
    fn end_figure(&mut self, end_flag: u32);

    fn set_fill_mode(&mut self, mode: u32);
    fn set_segment_flags(&mut self, flags: u32);

    fn add_beziers(&mut self, beziers: &[BezierSegment]);
    fn add_lines(&mut self, points: &[Point2f]);

    fn close(&mut self) -> DWResult<()>;
}

#[repr(C)]
pub struct ComGeometrySink<T>
where
    T: GeometrySink,
{
    vtbl: *const ID2D1SimplifiedGeometrySinkVtbl,
    safety_refcount: AtomicUsize,
    sink: T,
}

impl<T> ComGeometrySink<T>
where
    T: GeometrySink,
{
    pub fn new(sink: T) -> Self {
        ComGeometrySink {
            vtbl: vtbl::vtable_for::<T>(),
            safety_refcount: AtomicUsize::new(1),
            sink,
        }
    }

    pub unsafe fn as_raw_sink(&mut self) -> *mut ID2D1SimplifiedGeometrySink {
        self as *mut _ as *mut _
    }
}

impl<T> Drop for ComGeometrySink<T>
where
    T: GeometrySink,
{
    fn drop(&mut self) {
        assert_eq!(self.safety_refcount.load(Ordering::SeqCst), 1);
    }
}

impl<'a, T> GeometrySink for &'a mut T
where
    T: GeometrySink,
{
    fn begin_figure(&mut self, start: Point2f, begin_flag: u32) {
        T::begin_figure(*self, start, begin_flag)
    }

    fn end_figure(&mut self, end_flag: u32) {
        T::end_figure(*self, end_flag)
    }

    fn set_fill_mode(&mut self, mode: u32) {
        T::set_fill_mode(*self, mode)
    }

    fn set_segment_flags(&mut self, flags: u32) {
        T::set_segment_flags(*self, flags)
    }

    fn add_beziers(&mut self, beziers: &[BezierSegment]) {
        T::add_beziers(*self, beziers)
    }

    fn add_lines(&mut self, points: &[Point2f]) {
        T::add_lines(*self, points)
    }
    
    fn close(&mut self) -> DWResult<()> {
        T::close(*self)
    }
}
