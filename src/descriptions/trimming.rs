use crate::enums::trimming_granularity::TrimmingGranularity;

use checked_enum::UncheckedEnum;
use winapi::um::dwrite::DWRITE_TRIMMING;

#[repr(C)]
#[derive(Copy, Clone)]
/// Specifies the trimming option for text overflowing the layout box.
pub struct Trimming {
    /// Text granularity of which trimming applies.
    pub granularity: UncheckedEnum<TrimmingGranularity>,

    /// Character code used as the delimiter signaling the beginning of the portion of text to be
    /// preserved, most useful for path ellipsis, where the delimiter would be a slash. Leave this
    /// zero if there is no delimiter.
    pub delimiter: u32,
    
    /// How many occurrences of the delimiter to step back. Leave this zero if there is no
    /// delimiter.
    pub delimiter_count: u32,
}

#[cfg(test)]
member_compat_test! {
    trimming_compat:
    Trimming <=> DWRITE_TRIMMING {
        granularity <=> granularity,
        delimiter <=> delimiter,
        delimiter_count <=> delimiterCount,
    }
}

impl From<DWRITE_TRIMMING> for Trimming {
    fn from(trim: DWRITE_TRIMMING) -> Trimming {
        unsafe { std::mem::transmute(trim) }
    }
}
