#[repr(u32)]
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

impl FontWeight {
    pub fn from_u32(weight: u32) -> Option<Self> {
        match weight {
            100 => Some(FontWeight::Thin),
            200 => Some(FontWeight::ExtraLight),
            300 => Some(FontWeight::Light),
            350 => Some(FontWeight::SemiLight),
            400 => Some(FontWeight::Normal),
            500 => Some(FontWeight::Medium),
            600 => Some(FontWeight::SemiBold),
            700 => Some(FontWeight::Bold),
            800 => Some(FontWeight::ExtraBold),
            900 => Some(FontWeight::Black),
            950 => Some(FontWeight::ExtraBlack),
            _ => None,
        }
    }
}

#[repr(u32)]
pub enum FontStyle {
    Normal = 0,
    Oblique = 1,
    Italic = 2,
}

impl FontStyle {
    pub fn from_u32(style: u32) -> Option<Self> {
        match style {
            0 => Some(FontStyle::Normal),
            1 => Some(FontStyle::Oblique),
            2 => Some(FontStyle::Italic),
            _ => None,
        }
    }
}

#[repr(u32)]
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

impl FontStretch {
    pub fn from_u32(stretch: u32) -> Option<Self> {
        match stretch {
            0 => Some(FontStretch::Undefined),
            1 => Some(FontStretch::UltraCondensed),
            2 => Some(FontStretch::ExtraCondensed),
            3 => Some(FontStretch::Condensed),
            4 => Some(FontStretch::SemiCondensed),
            5 => Some(FontStretch::Normal),
            6 => Some(FontStretch::SemiExpanded),
            7 => Some(FontStretch::Expanded),
            8 => Some(FontStretch::ExtraExpanded),
            9 => Some(FontStretch::UltraExpanded),
            _ => None,
        }
    }
}
