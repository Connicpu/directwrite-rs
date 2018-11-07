use winapi::um::dwrite::DWRITE_GLYPH_METRICS;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Specifies the metrics of an individual glyph.The units depend on how the metrics are obtained.
pub struct GlyphMetrics {
    /// Specifies the X offset from the glyph origin to the left edge of the black box.
    /// The glyph origin is the current horizontal writing position. A negative value means the black
    /// box extends to the left of the origin (often true for lowercase italic 'f').
    pub left_side_bearing: i32,

    /// Specifies the X offset from the origin of the current glyph to the origin of the next glyph
    /// when writing horizontally.
    pub advance_width: u32,

    /// Specifies the X offset from the right edge of the black box to the origin of the next glyph
    /// when writing horizontally. The value is negative when the right edge of the black box
    /// overhangs the layout box.
    pub right_side_bearing: i32,

    /// Specifies the vertical offset from the vertical origin to the top of the black box.
    /// Thus, a positive value adds whitespace whereas a negative value means the glyph overhangs
    /// the top of the layout box.
    pub top_side_bearing: i32,

    /// Specifies the Y offset from the vertical origin of the current glyph to the vertical origin
    /// of the next glyph when writing vertically. Note that the term "origin" by itself denotes the horizontal origin.
    /// The vertical origin is different. Its Y coordinate is specified by verticalOriginY value, and its X coordinate is
    /// half the advanceWidth to the right of the horizontal origin.
    pub advance_height: u32,

    /// Specifies the vertical distance from the bottom edge of the black box to the advance height.
    /// This is positive when the bottom edge of the black box is within the layout box,
    /// or negative when the bottom edge of black box overhangs the layout box.
    pub bottom_side_bearing: i32,

    /// Specifies the Y coordinate of a glyph's vertical origin, in the font's design coordinate system.
    /// The y coordinate of a glyph's vertical origin is the sum of the glyph's top side bearing and the
    /// top (that is, yMax) of the glyph's bounding box.
    pub vertical_origin_y: i32,
}

impl From<DWRITE_GLYPH_METRICS> for GlyphMetrics {
    fn from(metrics: DWRITE_GLYPH_METRICS) -> GlyphMetrics {
        unsafe { std::mem::transmute(metrics) }
    }
}
