use descriptions::DBool;

use math2d::{Point2f, Sizef};
use winapi::um::dwrite::DWRITE_HIT_TEST_METRICS;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Describes the region obtained by a hit test.
pub struct HitTestMetrics {
    /// The first text position within the hit region.
    pub text_position: u32,
    /// The number of text positions within the hit region.
    pub length: u32,
    /// The upper-left corner of the hit region.
    pub position: Point2f,
    /// The size of the hit region.
    pub size: Sizef,
    /// The BIDI level of the text positions within the hit region.
    pub bidi_level: u32,
    /// Non-zero if the hit region contains text; otherwise, `0`.
    pub is_text: DBool,
    /// Non-zero if the text range is trimmed; otherwise, `0`.
    pub is_trimmed: DBool,
}

impl From<DWRITE_HIT_TEST_METRICS> for HitTestMetrics {
    fn from(metrics: DWRITE_HIT_TEST_METRICS) -> Self {
        unsafe { std::mem::transmute(metrics) }
    }
}
