use winapi::um::dwrite::DWRITE_TEXT_METRICS;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TextMetrics {
    /// A value that indicates the left-most point of formatted text relative to the layout box,
    /// while excluding any glyph overhang.
    pub left: f32,

    /// A value that indicates the top-most point of formatted text relative to the layout box,
    /// while excluding any glyph overhang.
    pub top: f32,

    /// A value that indicates the width of the formatted text, while ignoring trailing whitespace
    /// at the end of each line.
    pub width: f32,

    /// The width of the formatted text, taking into account the trailing whitespace at the end of
    /// each line.
    pub width_including_trailing_whitespace: f32,

    /// The height of the formatted text. The height of an empty string is set to the same value as
    /// that of the default font.
    pub height: f32,

    /// The initial width given to the layout. It can be either larger or smaller than the text
    /// content width, depending on whether the text was wrapped.
    pub layout_width: f32,

    /// Initial height given to the layout. Depending on the length of the text, it may be larger
    /// or smaller than the text content height.
    pub layout_height: f32,

    /// The maximum reordering count of any line of text, used to calculate the most number of
    /// hit-testing boxes needed. If the layout has no bidirectional text, or no text at all, the
    /// minimum level is 1.
    pub max_bidi_reordering_depth: u32,

    /// Total number of lines.
    pub line_count: u32,
}

impl From<DWRITE_TEXT_METRICS> for TextMetrics {
    fn from(metrics: DWRITE_TEXT_METRICS) -> Self {
        unsafe { std::mem::transmute(metrics) }
    }
}
