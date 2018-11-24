use winapi::um::dwrite::DWRITE_LINE_METRICS;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
/// Contains information about a formatted line of text.
pub struct LineMetrics {
    /// The number of text positions in the text line. This includes any trailing whitespace and
    /// newline characters.
    pub length: u32,

    /// The number of whitespace positions at the end of the text line. Newline sequences are
    /// considered whitespace.
    pub trailing_whitespace_length: u32,

    /// The number of characters in the newline sequence at the end of the text line. If the count
    /// is zero, then the text line was either wrapped or it is the end of the text.
    pub newline_length: u32,

    /// The height of the text line.
    pub height: f32,

    /// The distance from the top of the text line to its baseline.
    pub baseline: f32,

    /// The line is trimmed.
    pub is_trimmed: bool,
}

impl From<DWRITE_LINE_METRICS> for LineMetrics {
    fn from(metrics: DWRITE_LINE_METRICS) -> Self {
        unsafe { std::mem::transmute(metrics) }
    }
}
