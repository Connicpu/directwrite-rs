use winapi::um::dwrite::DWRITE_OVERHANG_METRICS;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Indicates how much any visible DIPs (device independent pixels) overshoot each side of the
/// layout or inline objects.
///
/// Positive overhangs indicate that the visible area extends outside the layout box or inline
/// object, while negative values mean there is whitespace inside. The returned values are
/// unaffected by rendering transforms or pixel snapping. Additionally, they may not exactly
/// match the final target's pixel bounds after applying grid fitting and hinting.
pub struct OverhangMetrics {
    /// The distance from the left-most visible DIP to its left-alignment edge.
    pub left: f32,

    /// The distance from the top-most visible DIP to its top alignment edge.
    pub top: f32,

    /// The distance from the right-most visible DIP to its right-alignment edge.
    pub right: f32,

    /// The distance from the bottom-most visible DIP to its lower-alignment edge.
    pub bottom: f32,
}

impl From<DWRITE_OVERHANG_METRICS> for OverhangMetrics {
    fn from(metrics: DWRITE_OVERHANG_METRICS) -> Self {
        unsafe { std::mem::transmute(metrics) }
    }
}
