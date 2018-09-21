use winapi::um::dwrite::*;
use helpers::InternalConstructor;

/// Specifies the metrics that are applicable to all glyphs within the font face.
#[derive(Copy, Clone)]
pub struct FontMetrics {
    metrics: DWRITE_FONT_METRICS,
}

impl InternalConstructor for FontMetrics {
    type Arguments = DWRITE_FONT_METRICS;
    #[inline]
    fn build(args: DWRITE_FONT_METRICS) -> Self {
        FontMetrics { metrics: args }
    }
}

impl FontMetrics {
    /// The number of font design units per em unit. 
    /// Font files use their own coordinate system of font design units. 
    /// A font design unit is the smallest measurable unit in the em square, an imaginary square that
    /// is used to size and align glyphs. The concept of em square is used as a reference scale factor 
    /// when defining font size and device transformation semantics. The size of one em square is also
    /// commonly used to compute the paragraph identation value.
    #[inline]
    pub fn design_units_per_em(&self) -> u16 {
        self.metrics.designUnitsPerEm
    }

    /// The ascent value of the font face in font design units. 
    /// Ascent is the distance from the top of font character alignment box to the English baseline.
    #[inline]
    pub fn ascent(&self) -> u16 {
        self.metrics.ascent
    }

    /// The descent value of the font face in font design units. 
    /// Descent is the distance from the bottom of font character alignment box to the English baseline.
    #[inline]
    pub fn descent(&self) -> u16 {
        self.metrics.descent
    }

    /// The line gap in font design units. Recommended additional white space to add between lines to 
    /// improve legibility. The recommended line spacing (baseline-to-baseline distance) is the sum of ascent, 
    /// descent, and lineGap. The line gap is usually positive or zero but can be negative, in which case the 
    /// recommended line spacing is less than the height of the character alignment box.
    #[inline]
    pub fn line_gap(&self) -> i16 {
        self.metrics.lineGap
    }

    /// The cap height value of the font face in font design units. Cap height is the distance from the English 
    /// baseline to the top of a typical English capital. Capital "H" is often used as a reference character for 
    /// the purpose of calculating the cap height value.
    #[inline]
    pub fn cap_height(&self) -> u16 {
        self.metrics.capHeight
    }

    /// The x-height value of the font face in font design units. x-height is the distance from the English baseline 
    /// to the top of lowercase letter "x", or a similar lowercase character.
    #[inline]
    pub fn x_height(&self) -> u16 {
        self.metrics.xHeight
    }

    /// The underline position value of the font face in font design units. Underline position is the position of 
    /// underline relative to the English baseline. The value is usually made negative in order to place the underline 
    /// below the baseline.
    #[inline]
    pub fn underline_position(&self) -> i16 {
        self.metrics.underlinePosition
    }

    /// The suggested underline thickness value of the font face in font design units.
    #[inline]
    pub fn underline_thickness(&self) -> u16 {
        self.metrics.underlineThickness
    }

    /// The strikethrough position value of the font face in font design units. Strikethrough position is the 
    /// position of strikethrough relative to the English baseline. The value is usually made positive in order 
    /// to place the strikethrough above the baseline.
    #[inline]
    pub fn strikethrough_position(&self) -> i16 {
        self.metrics.strikethroughPosition
    }

    /// The suggested strikethrough thickness value of the font face in font design units.
    #[inline]
    pub fn strikethrough_thickness(&self) -> u16 {
        self.metrics.strikethroughThickness
    }
}

/// Specifies the metrics of an individual glyph. The units depend on how the metrics are obtained.
#[derive(Copy, Clone)]
pub struct GlyphMetrics {
    metrics: DWRITE_GLYPH_METRICS,
}

impl InternalConstructor for GlyphMetrics {
    type Arguments = DWRITE_GLYPH_METRICS;
    #[inline]
    fn build(args: DWRITE_GLYPH_METRICS) -> Self {
        GlyphMetrics { metrics: args }
    }
}

impl GlyphMetrics {
    /// Specifies the X offset from the glyph origin to the left edge of the black box. 
    /// The glyph origin is the current horizontal writing position. A negative value means the black 
    /// box extends to the left of the origin (often true for lowercase italic 'f').
    #[inline]
    pub fn left_side_bearing(&self) -> i32 {
        self.metrics.leftSideBearing
    }

    /// Specifies the X offset from the origin of the current glyph to the origin of the next glyph 
    /// when writing horizontally.
    #[inline]
    pub fn advance_width(&self) -> u32 {
        self.metrics.advanceWidth
    }

    /// Specifies the X offset from the right edge of the black box to the origin of the next glyph 
    /// when writing horizontally. The value is negative when the right edge of the black box 
    /// overhangs the layout box.
    #[inline]
    pub fn right_side_bearing(&self) -> i32 {
        self.metrics.rightSideBearing
    }

    /// Specifies the vertical offset from the vertical origin to the top of the black box. 
    /// Thus, a positive value adds whitespace whereas a negative value means the glyph overhangs 
    /// the top of the layout box.
    #[inline]
    pub fn top_side_bearing(&self) -> i32 {
        self.metrics.topSideBearing
    }

    /// Specifies the Y offset from the vertical origin of the current glyph to the vertical origin 
    /// of the next glyph when writing vertically. Note that the term "origin" by itself denotes the horizontal origin. 
    /// The vertical origin is different. Its Y coordinate is specified by verticalOriginY value, and its X coordinate is 
    /// half the advanceWidth to the right of the horizontal origin.
    #[inline]
    pub fn advance_height(&self) -> u32 {
        self.metrics.advanceHeight
    }

    /// Specifies the vertical distance from the bottom edge of the black box to the advance height. 
    /// This is positive when the bottom edge of the black box is within the layout box, 
    /// or negative when the bottom edge of black box overhangs the layout box.
    #[inline]
    pub fn bottom_side_bearing(&self) -> i32 {
        self.metrics.bottomSideBearing
    }

    /// Specifies the Y coordinate of a glyph's vertical origin, in the font's design coordinate system.
    /// The y coordinate of a glyph's vertical origin is the sum of the glyph's top side bearing and the 
    /// top (that is, yMax) of the glyph's bounding box.
    #[inline]
    pub fn vertical_origin_y(&self) -> i32 {
        self.metrics.verticalOriginY
    }
}