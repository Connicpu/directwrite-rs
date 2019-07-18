#[auto_enum::auto_enum(u32, checked)]
/// Indicates the measuring method used for text layout.
pub enum MeasuringMode {
    /// Specifies that text is measured using glyph ideal metrics whose values
    /// are independent to the current display resolution.
    Natural = 0,

    /// Specifies that text is measured using glyph display-compatible metrics
    /// whose values tuned for the current display resolution.
    GdiClassic = 1,

    /// Specifies that text is measured using the same glyph display metrics as
    /// text measured by GDI using a font created with CLEARTYPE_NATURAL_QUALITY.
    GdiNatural = 2,
}
