#![allow(missing_docs)]

use winapi::um::dwrite::DWRITE_CLUSTER_METRICS;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Contains information about a glyph cluster.
pub struct ClusterMetrics {
    /// The total advance width of all glyphs in the cluster.
    pub width: f32,

    /// The number of text positions in the cluster.
    pub length: u16,

    pub flags: ClusterMetricsFlags,
}

impl ClusterMetrics {
    #[inline]
    /// Indicates whether a line can be broken right after the cluster.
    pub fn can_wrap_line_after(&self) -> bool {
        self.flags.is_set(ClusterMetricsFlags::CAN_WRAP_LINE_AFTER)
    }

    #[inline]
    /// Indicates whether the cluster corresponds to a whitespace character.
    pub fn is_whitespace(&self) -> bool {
        self.flags.is_set(ClusterMetricsFlags::IS_WHITESPACE)
    }

    #[inline]
    /// Indicates whether the cluster corresponds to a newline character.
    pub fn is_newline(&self) -> bool {
        self.flags.is_set(ClusterMetricsFlags::IS_NEWLINE)
    }

    #[inline]
    /// Indicates whether the cluster corresponds to a soft hyphen character.
    pub fn is_soft_hyphen(&self) -> bool {
        self.flags.is_set(ClusterMetricsFlags::IS_SOFT_HYPHEN)
    }

    #[inline]
    /// Indicates whether the cluster is read from right to left.
    pub fn is_right_to_left(&self) -> bool {
        self.flags.is_set(ClusterMetricsFlags::IS_RIGHT_TO_LEFT)
    }
}

#[auto_enum::enum_flags(u16)]
pub enum ClusterMetricsFlags {
    /// Indicates whether a line can be broken right after the cluster.
    CAN_WRAP_LINE_AFTER = 1 << (16 - 1),
    /// Indicates whether the cluster corresponds to a whitespace character.
    IS_WHITESPACE = 1 << (16 - 2),
    /// Indicates whether the cluster corresponds to a newline character.
    IS_NEWLINE = 1 << (16 - 3),
    /// Indicates whether the cluster corresponds to a soft hyphen character.
    IS_SOFT_HYPHEN = 1 << (16 - 4),
    /// Indicates whether the cluster is read from right to left.
    IS_RIGHT_TO_LEFT = 1 << (16 - 5),
}

impl From<DWRITE_CLUSTER_METRICS> for ClusterMetrics {
    fn from(metrics: DWRITE_CLUSTER_METRICS) -> Self {
        unsafe { std::mem::transmute(metrics) }
    }
}
