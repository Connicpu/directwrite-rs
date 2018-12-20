use crate::descriptions::wide_str::WideCStr;
use crate::enums::{FlowDirection, MeasuringMode, ReadingDirection};

use checked_enum::UncheckedEnum;
use winapi::um::dwrite::DWRITE_UNDERLINE;

#[repr(C)]
#[derive(Copy, Clone)]
/// Contains information about the size and placement of underlines. All coordinates are in device
/// independent pixels (DIPs).
pub struct Underline<'a> {
    /// Width of the underline, measured parallel to the baseline.
    pub width: f32,

    /// Thickness of the underline, measured perpendicular to the
    /// baseline.
    pub thickness: f32,

    /// Offset of the underline from the baseline.
    /// A positive offset represents a position below the baseline and
    /// a negative offset is above.
    pub offset: f32,

    /// Height of the tallest run where the underline applies.
    pub run_height: f32,

    /// Reading direction of the text associated with the underline.  This
    /// value is used to interpret whether the width value runs horizontally
    /// or vertically.
    pub reading_direction: UncheckedEnum<ReadingDirection>,

    /// Flow direction of the text associated with the underline.  This value
    /// is used to interpret whether the thickness value advances top to
    /// bottom, left to right, or right to left.
    pub flow_direction: UncheckedEnum<FlowDirection>,

    /// Locale of the text the underline is being drawn under. Can be
    /// pertinent where the locale affects how the underline is drawn.
    /// For example, in vertical text, the underline belongs on the
    /// left for Chinese but on the right for Japanese.
    /// This choice is completely left up to higher levels.
    pub locale_name: &'a WideCStr,

    /// The measuring mode can be useful to the renderer to determine how
    /// underlines are rendered, e.g. rounding the thickness to a whole pixel
    /// in GDI-compatible modes.
    pub measuring_mode: UncheckedEnum<MeasuringMode>,
}

#[cfg(test)]
dcommon::member_compat_test! {
    underline_bin_compat:
    Underline <=> DWRITE_UNDERLINE {
        width <=> width,
        thickness <=> thickness,
        offset <=> offset,
        run_height <=> runHeight,
        reading_direction <=> readingDirection,
        flow_direction <=> flowDirection,
        locale_name <=> localeName,
        measuring_mode <=> measuringMode,
    }
}

impl<'a> Underline<'a> {
    pub(crate) unsafe fn from_raw(desc: &'a DWRITE_UNDERLINE) -> Underline<'a> {
        std::mem::transmute(*desc)
    }

    pub(crate) unsafe fn into_raw(&self) -> DWRITE_UNDERLINE {
        std::mem::transmute(*self)
    }
}
