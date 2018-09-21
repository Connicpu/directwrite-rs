use helpers::InternalConstructor;

use std::mem;

use winapi::um::dwrite::*;

/// Contains the metrics associated with text after layout. All coordinates are in device independent pixels (DIPs).
#[derive(Copy, Clone)]
pub struct Metrics {
    metrics: DWRITE_TEXT_METRICS,
}

impl InternalConstructor for Metrics {
    type Arguments = DWRITE_TEXT_METRICS;
    #[inline]
    fn build(args: DWRITE_TEXT_METRICS) -> Self {
        Metrics { metrics: args }
    }
}

impl Metrics {
    /// A value that indicates the left-most point of formatted text relative to the layout box,
    /// while excluding any glyph overhang.
    #[inline]
    pub fn left(&self) -> f32 {
        self.metrics.left
    }

    /// A value that indicates the top-most point of formatted text relative to the layout box,
    /// while excluding any glyph overhang.
    #[inline]
    pub fn top(&self) -> f32 {
        self.metrics.top
    }

    /// A value that indicates the width of the formatted text, while ignoring trailing whitespace
    /// at the end of each line.
    #[inline]
    pub fn width(&self) -> f32 {
        self.metrics.width
    }

    /// The width of the formatted text, taking into account the trailing whitespace at the end of
    /// each line.
    #[inline]
    pub fn width_including_trailing_whitespace(&self) -> f32 {
        self.metrics.widthIncludingTrailingWhitespace
    }

    /// The height of the formatted text. The height of an empty string is set to the same value as
    /// that of the default font.
    #[inline]
    pub fn height(&self) -> f32 {
        self.metrics.height
    }

    /// The initial width given to the layout. It can be either larger or smaller than the text
    /// content width, depending on whether the text was wrapped.
    #[inline]
    pub fn layout_width(&self) -> f32 {
        self.metrics.layoutWidth
    }

    /// Initial height given to the layout. Depending on the length of the text, it may be larger
    /// or smaller than the text content height.
    #[inline]
    pub fn layout_height(&self) -> f32 {
        self.metrics.layoutHeight
    }

    /// The maximum reordering count of any line of text, used to calculate the most number of
    /// hit-testing boxes needed. If the layout has no bidirectional text, or no text at all, the
    /// minimum level is 1.
    #[inline]
    pub fn max_bidi_reordering_depth(&self) -> u32 {
        self.metrics.maxBidiReorderingDepth
    }

    /// Total number of lines.
    #[inline]
    pub fn line_count(&self) -> u32 {
        self.metrics.lineCount
    }
}

/// Contains information about a glyph cluster.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClusterMetrics {
    metrics: DWRITE_CLUSTER_METRICS,
}

impl Default for ClusterMetrics {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl InternalConstructor for ClusterMetrics {
    type Arguments = DWRITE_CLUSTER_METRICS;
    #[inline]
    fn build(args: DWRITE_CLUSTER_METRICS) -> Self {
        ClusterMetrics { metrics: args }
    }
}

impl ClusterMetrics {
    /// The total advance width of all glyphs in the cluster.
    #[inline]
    pub fn width(&self) -> f32 {
        self.metrics.width
    }

    /// The number of text positions in the cluster.
    #[inline]
    pub fn length(&self) -> u16 {
        self.metrics.length
    }

    /// Indicates whether a line can be broken right after the cluster.
    #[inline]
    pub fn can_wrap_line_after(&self) -> bool {
        self.metrics.canWrapLineAfter() != 0
    }

    /// Indicates whether the cluster corresponds to a whitespace character.
    #[inline]
    pub fn is_whitespace(&self) -> bool {
        self.metrics.isWhitespace() != 0
    }

    /// Indicates whether the cluster corresponds to a newline character.
    #[inline]
    pub fn is_newline(&self) -> bool {
        self.metrics.isNewline() != 0
    }

    /// Indicates whether the cluster corresponds to a soft hyphen character.
    #[inline]
    pub fn is_soft_hyphen(&self) -> bool {
        self.metrics.isSoftHyphen() != 0
    }

    /// Indicates whether the cluster is read from right to left.
    #[inline]
    pub fn is_right_to_left(&self) -> bool {
        self.metrics.isRightToLeft() != 0
    }
}

/// Contains information about a formatted line of text.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LineMetrics {
    metrics: DWRITE_LINE_METRICS,
}

impl Default for LineMetrics {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl InternalConstructor for LineMetrics {
    type Arguments = DWRITE_LINE_METRICS;
    #[inline]
    fn build(args: DWRITE_LINE_METRICS) -> Self {
        LineMetrics { metrics: args }
    }
}

impl LineMetrics {
    /// The number of text positions in the text line. This includes any trailing whitespace and
    /// newline characters.
    #[inline]
    pub fn length(&self) -> u32 {
        self.metrics.length
    }

    /// The number of whitespace positions at the end of the text line. Newline sequences are
    /// considered whitespace.
    #[inline]
    pub fn trailing_whitespace_length(&self) -> u32 {
        self.metrics.trailingWhitespaceLength
    }

    /// The number of characters in the newline sequence at the end of the text line. If the count
    /// is zero, then the text line was either wrapped or it is the end of the text.
    #[inline]
    pub fn newline_length(&self) -> u32 {
        self.metrics.newlineLength
    }

    /// The height of the text line.
    #[inline]
    pub fn height(&self) -> f32 {
        self.metrics.height
    }

    /// The distance from the top of the text line to its baseline.
    #[inline]
    pub fn baseline(&self) -> f32 {
        self.metrics.baseline
    }

    /// The line is trimmed.
    #[inline]
    pub fn is_trimmed(&self) -> bool {
        self.metrics.isTrimmed != 0
    }
}

/// Indicates how much any visible DIPs (device independent pixels) overshoot each side of the
/// layout or inline objects.
///
/// Positive overhangs indicate that the visible area extends outside the layout box or inline
/// object, while negative values mean there is whitespace inside. The returned values are
/// unaffected by rendering transforms or pixel snapping. Additionally, they may not exactly
/// match the final target's pixel bounds after applying grid fitting and hinting.
#[derive(Copy, Clone)]
pub struct OverhangMetrics {
    metrics: DWRITE_OVERHANG_METRICS,
}

impl InternalConstructor for OverhangMetrics {
    type Arguments = DWRITE_OVERHANG_METRICS;
    #[inline]
    fn build(args: DWRITE_OVERHANG_METRICS) -> Self {
        OverhangMetrics { metrics: args }
    }
}

impl OverhangMetrics {
    /// The distance from the left-most visible DIP to its left-alignment edge.
    #[inline]
    pub fn left(&self) -> f32 {
        self.metrics.left
    }

    /// The distance from the top-most visible DIP to its top alignment edge.
    #[inline]
    pub fn top(&self) -> f32 {
        self.metrics.top
    }

    /// The distance from the right-most visible DIP to its right-alignment edge.
    #[inline]
    pub fn right(&self) -> f32 {
        self.metrics.right
    }

    /// The distance from the bottom-most visible DIP to its lower-alignment edge.
    #[inline]
    pub fn bottom(&self) -> f32 {
        self.metrics.bottom
    }
}

/// Describes the region obtained by a hit test.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HitTestMetrics {
    metrics: DWRITE_HIT_TEST_METRICS,
}

impl Default for HitTestMetrics {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl InternalConstructor for HitTestMetrics {
    type Arguments = DWRITE_HIT_TEST_METRICS;
    #[inline]
    fn build(args: Self::Arguments) -> Self {
        HitTestMetrics { metrics: args }
    }
}

impl HitTestMetrics {
    /// The first text position within the hit region.
    pub fn text_position(&self) -> u32 {
        self.metrics.textPosition
    }

    /// The number of text positions within the hit region.
    pub fn length(&self) -> u32 {
        self.metrics.length
    }

    /// The x-coordinate of the upper-left corner of the hit region.
    pub fn left(&self) -> f32 {
        self.metrics.left
    }

    /// The y-coordinate of the upper-left corner of the hit region.
    pub fn top(&self) -> f32 {
        self.metrics.top
    }

    /// The width of the hit region.
    pub fn width(&self) -> f32 {
        self.metrics.width
    }

    /// The height of the hit region.
    pub fn height(&self) -> f32 {
        self.metrics.height
    }

    /// The BIDI level of the text positions within the hit region.
    pub fn bidi_level(&self) -> u32 {
        self.metrics.bidiLevel
    }

    /// true if the hit region contains text; otherwise, false.
    pub fn is_text(&self) -> bool {
        self.metrics.isText != 0
    }

    /// true if the text range is trimmed; otherwise, false.
    pub fn is_trimmed(&self) -> bool {
        self.metrics.isTrimmed != 0
    }
}
