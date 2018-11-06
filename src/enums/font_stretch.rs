#[auto_enum(u32, checked)]
/// Represents the degree to which a font has been stretched compared to a
/// font's normal aspect ratio.The enumerated values correspond to the
/// usWidthClass definition in the OpenType specification. The usWidthClass
/// represents an integer value between 1 and 9—lower values indicate narrower
/// widths; higher values indicate wider widths.
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
