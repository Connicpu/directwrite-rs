#[auto_enum(u32, checked)]
/// Represents the degree to which a font has been stretched compared to a
/// font's normal aspect ratio.The enumerated values correspond to the
/// usWidthClass definition in the OpenType specification. The usWidthClass
/// represents an integer value between 1 and 9—lower values indicate narrower
/// widths; higher values indicate wider widths.
/// 
/// A font stretch describes the degree to which a font form is stretched from its normal aspect
/// ratio, which is the original width to height ratio specified for the glyphs in the font. The
/// following illustration shows an example of Normal and Condensed stretches for the Rockwell
/// Bold typeface.
/// 
/// ![Illustration of font stretching][1]
/// 
/// [1]: https://docs.microsoft.com/en-us/windows/desktop/api/dwrite/images/fontstretch_for_rockwellbold.png
pub enum FontStretch {
    /// Predefined font stretch : Not known (0).
    Undefined = 0,

    /// Predefined font stretch : Ultra-condensed (1).
    UltraCondensed = 1,

    /// Predefined font stretch : Extra-condensed (2).
    ExtraCondensed = 2,

    /// Predefined font stretch : Condensed (3).
    Condensed = 3,

    /// Predefined font stretch : Semi-condensed (4).
    SemiCondensed = 4,

    /// Predefined font stretch : Normal (5).
    Normal = 5,

    /// Predefined font stretch : Semi-expanded (6).
    SemiExpanded = 6,

    /// Predefined font stretch : Expanded (7).
    Expanded = 7,

    /// Predefined font stretch : Extra-expanded (8).
    ExtraExpanded = 8,

    /// Predefined font stretch : Ultra-expanded (9).
    UltraExpanded = 9,
}
