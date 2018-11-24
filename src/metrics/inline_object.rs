use descriptions::DBool;

use math2d::Sizef;
use winapi::um::dwrite::DWRITE_INLINE_OBJECT_METRICS;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Contains properties describing the geometric measurement of an
/// application-defined inline object.
pub struct InlineObjectMetrics {
    /// The width and height of the inline object.
    pub size: Sizef,

    /// The distance from the top of the object to the point where it is lined up with the
    /// adjacent text. If the baseline is at the bottom, then baseline simply equals height.
    pub baseline: f32,

    /// Indicates whether the object is to be placed upright or alongside the text baseline for
    /// vertical text. The value is zero to indicate false, and nonzero to indicate true.
    pub supports_sideways: DBool,
}

impl From<DWRITE_INLINE_OBJECT_METRICS> for InlineObjectMetrics {
    fn from(metrics: DWRITE_INLINE_OBJECT_METRICS) -> Self {
        unsafe { std::mem::transmute(metrics) }
    }
}
