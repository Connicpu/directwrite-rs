use error::DWResult;

use math2d::BezierSegment;
use math2d::Point2f;

pub(crate) mod com_sink;

pub trait GeometrySink: Sized {
    fn begin_figure(&mut self, start: Point2f, begin_flag: u32);
    fn end_figure(&mut self, end_flag: u32);

    fn set_fill_mode(&mut self, mode: u32);
    fn set_segment_flags(&mut self, flags: u32);

    fn add_beziers(&mut self, beziers: &[BezierSegment]);
    fn add_lines(&mut self, points: &[Point2f]);

    fn close(&mut self) -> DWResult<()>;
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
