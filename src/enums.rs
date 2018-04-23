use std::marker::PhantomData;

pub trait CheckedEnum: Sized + Copy + 'static {
    fn to_u32(self) -> u32;
    fn from_u32(value: u32) -> Option<Self>;
}

macro_rules! dw_enums {
    ($(
        pub enum $ename:ident {
            $($ekey:ident = $eval:expr,)*
        }
    )*) => {$(
        #[repr(u32)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $ename {
            $($ekey = $eval,)*
        }

        impl $ename {
            #[inline(always)]
            pub fn to_u32(self) -> u32 {
                self as u32
            }
            
            pub fn from_u32(value: u32) -> Option<Self> {
                match value {
                    $($eval => Some($ename :: $ekey),)*
                    _ => None,
                }
            }
        }

        impl CheckedEnum for $ename {
            #[inline(always)]
            fn to_u32(self) -> u32 {
                $ename :: to_u32(self)
            }
            #[inline(always)]
            fn from_u32(value: u32) -> Option<Self> {
                $ename :: from_u32(value)
            }
        }
    )*};
}

dw_enums! {
    pub enum FontWeight {
        Thin = 100,
        ExtraLight = 200,
        Light = 300,
        SemiLight = 350,
        Normal = 400,
        Medium = 500,
        SemiBold = 600,
        Bold = 700,
        ExtraBold = 800,
        Black = 900,
        ExtraBlack = 950,
    }

    pub enum FontStyle {
        Normal = 0,
        Oblique = 1,
        Italic = 2,
    }

    pub enum FontStretch {
        Undefined = 0,
        UltraCondensed = 1,
        ExtraCondensed = 2,
        Condensed = 3,
        SemiCondensed = 4,
        Normal = 5,
        SemiExpanded = 6,
        Expanded = 7,
        ExtraExpanded = 8,
        UltraExpanded = 9,
    }

    pub enum BreakCondition {
        Neutral = 0,
        CanBreak = 1,
        MayNotBreak = 2,
        MustBreak = 3,
    }

    pub enum FlowDirection {
        TopToBottom = 0,
        BottomToTop = 1,
        LeftToRight = 2,
        RightToLeft = 3,
    }

    pub enum LineSpacingMethod {
        Default = 0,
        Uniform = 1,
        Proportional = 2,
    }

    pub enum ParagraphAlignment {
        Near = 0,
        Far = 1,
        Center = 2,
    }

    pub enum ReadingDirection {
        LeftToRight = 0,
        RightToLeft = 1,
        TopToBottom = 2,
        BottomToTop = 3,
    }

    pub enum TextAlignment {
        Leading = 0,
        Trailing = 1,
        Center = 2,
        Justified = 3,
    }

    pub enum WordWrapping {
        Wrap = 0,
        NoWrap = 1,
        EmergencyBreak = 2,
        WholeWord = 3,
        Character = 4,
    }

    pub enum MeasuringMode {
        Natural = 0,
        GdiClassic = 1,
        GdiNatural = 2,
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UncheckedEnum<T: CheckedEnum> {
    pub value: u32,
    _marker: PhantomData<T>,
}

impl<T> UncheckedEnum<T>
where
    T: CheckedEnum,
{
    pub fn new(value: u32) -> Self {
        UncheckedEnum {
            value,
            _marker: PhantomData,
        }
    }

    pub fn as_enum(self) -> Option<T> {
        T::from_u32(self.value)
    }
}
