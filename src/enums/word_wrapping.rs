#[auto_enum(u32, checked)]
/// Specifies the word wrapping to be used in a particular multiline paragraph.
///
/// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
///
/// **Note**
/// `EmergencyBreak`, `WholeWord`, and `Character` are available in Windows 8.1 and later only.
///
/// </div>
pub enum WordWrapping {
    /// Indicates that words are broken across lines to avoid text overflowing
    /// the layout box.
    Wrap,

    /// Indicates that words are kept within the same line even when it
    /// overflows the layout box. This option is often used with scrolling
    /// to reveal overflow text.
    NoWrap,

    /// Words are broken across lines to avoid text overflowing the layout box.
    /// Emergency wrapping occurs if the word is larger than the maximum width.
    EmergencyBreak,

    /// When emergency wrapping, only wrap whole words, never breaking words
    /// when the layout width is too small for even a single word.
    WholeWord,

    ///  Wrap between any valid character clusters.
    Character,
}
