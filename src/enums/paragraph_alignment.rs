#[auto_enum::auto_enum(u32, checked)]
/// Specifies the alignment of paragraph text along the flow direction axis,
/// relative to the top and bottom of the flow's layout box.
pub enum ParagraphAlignment {
    /// The top of the text flow is aligned to the top edge of the layout box.
    Near,

    /// The bottom of the text flow is aligned to the bottom edge of the
    /// layout box.
    Far,

    /// The center of the flow is aligned to the center of the layout box.
    Center,
}
