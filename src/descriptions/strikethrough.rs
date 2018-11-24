use descriptions::wide_str::WideCStr;
use enums::{FlowDirection, MeasuringMode, ReadingDirection};

use checked_enum::UncheckedEnum;
use winapi::um::dwrite::DWRITE_STRIKETHROUGH;

#[repr(C)]
#[derive(Copy, Clone)]
/// Contains information regarding the size and placement of strikethroughs.All coordinates are in
/// device independent pixels (DIPs).
pub struct Strikethrough<'a> {
    /// A value that indicates the width of the strikethrough, measured parallel to the baseline.
    pub width: f32,

    /// A value that indicates the thickness of the strikethrough, measured perpendicular to the
    /// baseline.
    pub thickness: f32,

    /// A value that indicates the offset of the strikethrough from the baseline. A positive offset
    /// represents a position below the baseline and a negative offset is above. Typically, the
    /// offset will be negative.
    pub offset: f32,

    /// Reading direction of the text associated with the strikethrough. This value is used to
    /// interpret whether the width value runs horizontally or vertically.
    pub reading_direction: UncheckedEnum<ReadingDirection>,

    /// Flow direction of the text associated with the strikethrough. This value is used to
    /// interpret whether the thickness value advances top to bottom, left to right, or right
    /// to left.
    pub flow_direction: UncheckedEnum<FlowDirection>,

    /// An array of characters containing the locale of the text that is the strikethrough is being
    /// drawn over.
    pub locale_name: &'a WideCStr,

    /// The measuring mode can be useful to the renderer to determine how underlines are rendered,
    /// such as rounding the thickness to a whole pixel in GDI-compatible modes.
    pub measuring_mode: UncheckedEnum<MeasuringMode>,
}

#[cfg(test)]
member_compat_test! {
    strikethrough_bin_compat:
    Strikethrough <=> DWRITE_STRIKETHROUGH {
        width <=> width,
        thickness <=> thickness,
        offset <=> offset,
        reading_direction <=> readingDirection,
        flow_direction <=> flowDirection,
        locale_name <=> localeName,
        measuring_mode <=> measuringMode,
    }
}

impl<'a> Strikethrough<'a> {
    pub(crate) unsafe fn from_raw(desc: &'a DWRITE_STRIKETHROUGH) -> Strikethrough<'a> {
        std::mem::transmute(*desc)
    }

    pub(crate) unsafe fn into_raw(&self) -> DWRITE_STRIKETHROUGH {
        std::mem::transmute(*self)
    }
}
