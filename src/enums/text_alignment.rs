#[auto_enum::auto_enum(u32, checked)]
/// Specifies the alignment of paragraph text along the reading direction axis,
/// relative to the leading and trailing edge of the layout box.
pub enum TextAlignment {
    /// The leading edge of the paragraph text is aligned to the leading edge
    /// of the layout box.
    Leading,

    /// The trailing edge of the paragraph text is aligned to the trailing edge
    /// of the layout box.
    Trailing,

    /// The center of the paragraph text is aligned to the center of the layout box.
    Center,

    /// Align text to the leading side, and also justify text to fill the lines.
    Justified,
}
