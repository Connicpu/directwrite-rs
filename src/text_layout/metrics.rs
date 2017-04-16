use winapi::{DWRITE_TEXT_METRICS, DWRITE_CLUSTER_METRICS, DWRITE_LINE_METRICS};
use helpers::InternalConstructor;
use std::mem;

#[derive(Copy, Clone)]
pub struct Metrics {
    metrics: DWRITE_TEXT_METRICS,
}

impl InternalConstructor for Metrics {
    type Arguments = DWRITE_TEXT_METRICS;
    fn build(args: DWRITE_TEXT_METRICS) -> Self {
        Metrics { metrics: args }
    }
}

impl Metrics {
    /// A value that indicates the left-most point of formatted text relative to the layout box,
    /// while excluding any glyph overhang.
    pub fn left(&self) -> f32 {
        self.metrics.left
    }

    /// A value that indicates the top-most point of formatted text relative to the layout box,
    /// while excluding any glyph overhang.
    pub fn top(&self) -> f32 {
        self.metrics.top
    }

    /// A value that indicates the width of the formatted text, while ignoring trailing whitespace
    /// at the end of each line.
    pub fn width(&self) -> f32 {
        self.metrics.width
    }

    /// The width of the formatted text, taking into account the trailing whitespace at the end of
    /// each line.
    pub fn width_including_trailing_whitespace(&self) -> f32 {
        self.metrics.widthIncludingTrailingWhitespace
    }

    /// The height of the formatted text. The height of an empty string is set to the same value as
    /// that of the default font.
    pub fn height(&self) -> f32 {
        self.metrics.height
    }

    /// The initial width given to the layout. It can be either larger or smaller than the text
    /// content width, depending on whether the text was wrapped.
    pub fn layout_width(&self) -> f32 {
        self.metrics.layoutWidth
    }

    /// Initial height given to the layout. Depending on the length of the text, it may be larger
    /// or smaller than the text content height.
    pub fn layout_height(&self) -> f32 {
        self.metrics.layoutHeight
    }

    /// The maximum reordering count of any line of text, used to calculate the most number of
    /// hit-testing boxes needed. If the layout has no bidirectional text, or no text at all, the
    /// minimum level is 1.
    pub fn max_bidi_reordering_depth(&self) -> u32 {
        self.metrics.maxBidiReorderingDepth
    }

    /// Total number of lines.
    pub fn line_count(&self) -> u32 {
        self.metrics.lineCount
    }
}

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
    fn build(args: DWRITE_CLUSTER_METRICS) -> Self {
        ClusterMetrics { metrics: args }
    }
}

impl ClusterMetrics {
    /// The total advance width of all glyphs in the cluster.
    pub fn width(&self) -> f32 {
        self.metrics.width
    }

    /// The number of text positions in the cluster.
    pub fn length(&self) -> u16 {
        self.metrics.length
    }

    /// Indicates whether a line can be broken right after the cluster.
    pub fn can_wrap_line_after(&self) -> bool {
        self.metrics.canWrapLineAfter() != 0
    }

    /// Indicates whether the cluster corresponds to a whitespace character.
    pub fn is_whitespace(&self) -> bool {
        self.metrics.isWhitespace() != 0
    }

    /// Indicates whether the cluster corresponds to a newline character.
    pub fn is_newline(&self) -> bool {
        self.metrics.isNewline() != 0
    }

    /// Indicates whether the cluster corresponds to a soft hyphen character.
    pub fn is_soft_hyphen(&self) -> bool {
        self.metrics.isSoftHyphen() != 0
    }

    /// Indicates whether the cluster is read from right to left.
    pub fn is_right_to_left(&self) -> bool {
        self.metrics.isRightToLeft() != 0
    }
}

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
    fn build(args: DWRITE_LINE_METRICS) -> Self {
        LineMetrics { metrics: args }
    }
}

impl LineMetrics {
    /// The number of text positions in the text line. This includes any trailing whitespace and
    /// newline characters.
    pub fn length(&self) -> u32 {
        self.metrics.length
    }

    /// The number of whitespace positions at the end of the text line. Newline sequences are
    /// considered whitespace.
    pub fn trailing_whitespace_length(&self) -> u32 {
        self.metrics.trailingWhitespaceLength
    }

    /// The number of characters in the newline sequence at the end of the text line. If the count
    /// is zero, then the text line was either wrapped or it is the end of the text.
    pub fn newline_length(&self) -> u32 {
        self.metrics.newlineLength
    }

    /// The height of the text line.
    pub fn height(&self) -> f32 {
        self.metrics.height
    }

    /// The distance from the top of the text line to its baseline.
    pub fn baseline(&self) -> f32 {
        self.metrics.baseline
    }

    /// The line is trimmed.
    pub fn is_trimmed(&self) -> bool {
        self.metrics.isTrimmed != 0
    }
}
