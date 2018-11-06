#[auto_enum(u32, checked)]
/// Indicates the file format of a complete font face.
pub enum FontFaceType {
    /// OpenType font face with CFF outlines.
    CFF,

    /// OpenType font face with TrueType outlines.
    TrueType,
    
    /// 
    OpenTypeCollection,
    
    /// A Type 1 font face.
    Type1,

    /// A vector .FON format font face.
    Vector,
    
    /// A bitmap .FON format font face.
    Bitmap,
    
    /// Font face type is not recognized by the DirectWrite font system.
    Unknown,
    
    /// The font data includes only the CFF table from an OpenType CFF font.
    /// This font face type can be used only for embedded fonts (i.e., custom
    /// font file loaders) and the resulting font face object supports only
    /// the minimum functionality necessary to render glyphs.
    RawCFF,
}
