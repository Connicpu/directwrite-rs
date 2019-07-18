#[auto_enum::auto_enum(u32, checked)]
/// Represents a method of rendering glyphs.
pub enum RenderingMode {
    /// Specifies that the rendering mode is determined automatically, based on the font and size.
    Default,

    /// Specifies that no anti-aliasing is performed. Each pixel is either set to the foreground
    /// color of the text or retains the color of the background.
    Aliased,

    /// Specifies that antialiasing is performed in the horizontal direction and the appearance of
    /// glyphs is layout-compatible with GDI using `CLEARTYPE_QUALITY`. Use `GdiClassic` to get
    /// glyph advances. The antialiasing may be either ClearType or grayscale depending on the text
    /// antialiasing mode.
    GdiClassic,

    /// Specifies that antialiasing is performed in the horizontal direction and the appearance of
    /// glyphs is layout-compatible with GDI using `CLEARTYPE_NATURAL_QUALITY`. Glyph advances are
    /// close to the font design advances, but are still rounded to whole pixels. Use `GDI_NATURAL`
    /// to get glyph advances. The antialiasing may be either ClearType or grayscale depending on
    /// the text antialiasing mode.
    GdiNatural,

    /// Specifies that antialiasing is performed in the horizontal direction. This rendering
    /// mode allows glyphs to be positioned with subpixel precision and is therefore suitable
    /// for natural (i.e., resolution-independent) layout. The antialiasing may be either
    /// ClearType or grayscale depending on the text antialiasing mode.
    Natural,

    /// Similar to natural mode except that antialiasing is performed in both the horizontal
    /// and vertical directions. This is typically used at larger sizes to make curves and
    /// diagonal lines look smoother. The antialiasing may be either ClearType or grayscale
    /// depending on the text antialiasing mode.
    NaturalSymmetric,
    
    /// Specifies that rendering should bypass the rasterizer and use the outlines directly. 
    /// This is typically used at very large sizes.
    Outline,
}
