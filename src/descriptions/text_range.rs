use winapi::um::dwrite::DWRITE_TEXT_RANGE;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// A text range, represented in UTF-16 code units.
pub struct TextRange {
    /// The first text position in the range
    pub start: u32,
    /// The number of u16 code units that this range covers
    pub length: u32,
}

#[cfg(test)]
dcommon::member_compat_test! {
    test_range_compat:
    TextRange <=> DWRITE_TEXT_RANGE {
        start <=> startPosition,
        length <=> length,
    }
}

impl From<TextRange> for DWRITE_TEXT_RANGE {
    fn from(range: TextRange) -> Self {
        DWRITE_TEXT_RANGE {
            startPosition: range.start,
            length: range.length,
        }
    }
}

impl From<DWRITE_TEXT_RANGE> for TextRange {
    fn from(range: DWRITE_TEXT_RANGE) -> Self {
        TextRange {
            start: range.startPosition,
            length: range.length,
        }
    }
}

impl From<std::ops::RangeFull> for TextRange {
    fn from(_range: std::ops::RangeFull) -> Self {
        TextRange {
            start: 0,
            length: std::u32::MAX,
        }
    }
}

macro_rules! text_range_from {
    ($($t:ident)*) => {

        $(text_range_from!(@ $t);)*
    };
    (@ $t:ident) => {
        #[allow(unused_comparisons)]
        impl From<$t> for TextRange {
            fn from(position: $t) -> Self {
                assert!(position >= 0);
                assert!((position as u64) < std::i32::MAX as u64);
                TextRange {
                    start: position as u32,
                    length: 1,
                }
            }
        }

        #[allow(unused_comparisons)]
        impl From<std::ops::Range<$t>> for TextRange {
            fn from(range: std::ops::Range<$t>) -> Self {
                assert!(range.start >= 0);
                assert!(range.end >= 0);
                assert!((range.start as u64) < std::i32::MAX as u64, "range.start < i32::MAX");
                assert!((range.end as u64) < std::i32::MAX as u64, "range.end < i32::MAX");
                assert!(
                    range.end > range.start,
                    "Range end cannot come before range start"
                );
                TextRange {
                    start: range.start as u32,
                    length: range.end as u32 - range.start as u32,
                }
            }
        }

        #[allow(unused_comparisons)]
        impl From<std::ops::RangeTo<$t>> for TextRange {
            fn from(range: std::ops::RangeTo<$t>) -> Self {
                assert!(range.end >= 0);
                assert!((range.end as u64) < std::i32::MAX as u64, "range.end < i32::MAX");
                TextRange {
                    start: 0,
                    length: range.end as u32,
                }
            }
        }

        #[allow(unused_comparisons)]
        impl From<std::ops::RangeFrom<$t>> for TextRange {
            fn from(range: std::ops::RangeFrom<$t>) -> Self {
                assert!(range.start >= 0);
                assert!((range.start as u64) < std::i32::MAX as u64, "range.start < i32::MAX");
                TextRange {
                    start: range.start as u32,
                    length: std::u32::MAX,
                }
            }
        }

        #[allow(unused_comparisons)]
        impl From<std::ops::RangeInclusive<$t>> for TextRange {
            fn from(range: std::ops::RangeInclusive<$t>) -> Self {
                let start = (*range.start()) as u64;
                let end = (*range.end()) as u64;
                assert!(start >= 0);
                assert!(end >= 0);
                assert!(start < std::i32::MAX as u64, "range.start() < i32::MAX");
                assert!(end < std::i32::MAX as u64, "range.end() < i32::MAX");
                assert!(
                    end >= start,
                    "Range end cannot come before range start"
                );
                TextRange {
                    start: start as u32,
                    length: ((end + 1) - start) as u32,
                }
            }
        }
    }
}

text_range_from!(i8 u8 i16 u16 i32 u32 u64 usize);
