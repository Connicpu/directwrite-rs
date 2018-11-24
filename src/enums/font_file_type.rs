#[auto_enum(u32, checked)]
/// The type of a font represented by a single font file. Font formats that consist of multiple
/// files, for example Type 1 .PFM and .PFB, have separate enum values for each of the file types.
pub enum FontFileType {
    /// Font type is not recognized by the DirectWrite font system.
    Unknown,

    /// OpenType font with CFF outlines.
    Cff,

    /// OpenType font with TrueType outlines.
    TrueType,

    /// OpenType font that contains a TrueType collection.
    OpenTypeCollection,

    /// Type 1 PFM font.
    Type1Pfm,

    /// Type 1 PFB font.
    Type1Pfb,

    /// Vector .FON font.
    Vector,

    /// Bitmap .FON font.
    Bitmap,
}
