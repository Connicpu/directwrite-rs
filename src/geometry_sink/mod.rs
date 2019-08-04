use dcommon::Error;
use math2d::BezierSegment;
use math2d::Point2f;

pub(crate) mod com_sink;

/// A sink for geometry made of straight lines and cubic bezier curves.
pub trait GeometrySink: Sized {
    /// Sets the fill mode to be used for any figures that come after this method call until
    /// this method is called again. See [`direct2d::enums::FillMode`][1] for interpreting
    /// the mode parameter.
    ///
    /// [1]: https://docs.rs/direct2d/*/direct2d/enums/enum.FillMode.html
    fn set_fill_mode(&mut self, mode: u32);

    /// Specifies stroke and join options to be applied to new segments added to the geometry
    /// sink. After this method is called, the specified segment flags are applied to each segment
    /// subsequently added to the sink. The segment flags are applied to every additional segment
    /// until this method is called again and a different set of segment flags is specified. See
    /// [`direct2d::enums::PathSegment`][1] for interpreting the flags parameter.
    ///
    /// [1]: https://docs.rs/direct2d/*/direct2d/enums/struct.PathSegment.html
    fn set_segment_flags(&mut self, flags: u32);

    /// Called at the beginning of a new figure. See [`direct2d::enums::FigureBegin`][1]
    /// for interpreting the flag parameter.
    ///
    /// [1]: https://docs.rs/direct2d/*/direct2d/enums/enum.FigureBegin.html
    fn begin_figure(&mut self, start: Point2f, begin_flag: u32);

    /// Adds a list of cubic bezier segments. Each segment begins at the last point passed
    /// to the current figure. If there are no lines or curves added yet, the last point will be
    /// the point passed to `begin_figure`.
    fn add_beziers(&mut self, beziers: &[BezierSegment]);

    /// Adds a list of straight lines to the figure. The start point for each line segment is the
    /// last point on the previous segment. If there are no lines or curves added yet, the last
    /// point will be the point passed to `begin_figure`.
    fn add_lines(&mut self, points: &[Point2f]);

    /// Called to end a figure. See [`direct2d::enums::FigureEnd`][1]
    /// for interpreting the flag parameter.
    ///
    /// [1]: https://docs.rs/direct2d/*/direct2d/enums/enum.FigureEnd.html
    fn end_figure(&mut self, end_flag: u32);

    /// Closes the geometry sink, indicates whether it is in an error state, and resets the
    /// sink's error state.
    fn close(&mut self) -> Result<(), Error>;
}

impl<'a, T> GeometrySink for &'a mut T
where
    T: GeometrySink,
{
    fn set_fill_mode(&mut self, mode: u32) {
        T::set_fill_mode(*self, mode)
    }

    fn set_segment_flags(&mut self, flags: u32) {
        T::set_segment_flags(*self, flags)
    }

    fn begin_figure(&mut self, start: Point2f, begin_flag: u32) {
        T::begin_figure(*self, start, begin_flag)
    }

    fn add_beziers(&mut self, beziers: &[BezierSegment]) {
        T::add_beziers(*self, beziers)
    }

    fn add_lines(&mut self, points: &[Point2f]) {
        T::add_lines(*self, points)
    }

    fn end_figure(&mut self, end_flag: u32) {
        T::end_figure(*self, end_flag)
    }

    fn close(&mut self) -> Result<(), Error> {
        T::close(*self)
    }
}
