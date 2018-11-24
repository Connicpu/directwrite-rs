#[auto_enum(u32, checked)]
/// Text granularity used to trim text overflowing the layout box.
pub enum TrimmingGranularity {
    /// No trimming occurs. Text flows beyond the layout width.
    None,

    /// Trimming occurs at character cluster boundary.
    Character,

    /// Trimming occurs at word boundary.
    Word,
}
