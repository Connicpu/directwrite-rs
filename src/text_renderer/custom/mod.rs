//! Types and traits for creating application-implemented TextRenderer objects.

use crate::descriptions::GlyphRun;
use crate::descriptions::GlyphRunDescription;
use crate::descriptions::Strikethrough;
use crate::descriptions::Underline;
use crate::effects::ClientEffect;
use crate::enums::MeasuringMode;
use crate::error::DWResult;
use crate::inline_object::InlineObject;
use crate::text_renderer::DrawContext;

use checked_enum::UncheckedEnum;
use math2d::Matrix3x2f;
use math2d::Point2f;

#[doc(inline)]
pub use crate::text_renderer::custom::com_renderer::ComRenderer;

#[doc(hidden)]
pub mod com_renderer;

/// An application-implemented TextRenderer that can be passed to DirectWrite to receive
/// glyphs and inline objects from a TextLayout to perform customized rendering.
pub trait CustomTextRenderer: Send + 'static {
    /// Determines whether pixel snapping is disabled. The recommended default is false,
    /// unless doing animation that requires subpixel vertical placement.
    fn pixel_snapping_disabled(&self, context: DrawContext) -> bool;

    /// Gets the current transform that maps abstract coordinates to DIPs,
    /// which may disable pixel snapping upon any rotation or shear.
    fn current_transform(&self, context: DrawContext) -> Matrix3x2f;

    /// Gets the number of physical pixels per DIP. A DIP (device-independent pixel) is 1/96 inch,
    /// so the pixelsPerDip value is the number of logical pixels per inch divided by 96 (yielding
    /// a value of 1 for 96 DPI and 1.25 for 120).
    fn pixels_per_dip(&self, context: DrawContext) -> f32;

    /// [`TextLayout::draw`][1] calls this function to instruct the client to
    /// render a run of glyphs.
    ///
    /// [1]: ../../struct.TextLayout.html#method.draw
    fn draw_glyph_run(&mut self, context: &DrawGlyphRun) -> DWResult<()>;

    /// [`TextLayout::draw`][1] calls this function to instruct the client to draw
    /// an underline.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Remarks**
    /// A single underline can be broken into multiple calls, depending on
    /// how the formatting changes attributes. If font sizes/styles change
    /// within an underline, the thickness and offset will be averaged
    /// weighted according to characters.
    /// To get the correct top coordinate of the underline rect, add underline::offset
    /// to the baseline's Y. Otherwise the underline will be immediately under the text.
    /// The x coordinate will always be passed as the left side, regardless
    /// of text directionality. This simplifies drawing and reduces the
    /// problem of round-off that could potentially cause gaps or a double
    /// stamped alpha blend. To avoid alpha overlap, round the end points
    /// to the nearest device pixel.
    ///
    /// </div>
    ///
    /// [1]: ../../struct.TextLayout.html#method.draw
    fn draw_underline(&mut self, context: &DrawUnderline) -> DWResult<()>;

    /// [`TextLayout::draw`][1] calls this function to instruct the client to draw
    /// an inline object.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Remarks**
    /// The right-to-left flag is a hint for those cases where it would look
    /// strange for the image to be shown normally (like an arrow pointing to
    /// right to indicate a submenu).
    ///
    /// </div>
    ///
    /// [1]: ../../struct.TextLayout.html#method.draw
    fn draw_strikethrough(&mut self, context: &DrawStrikethrough) -> DWResult<()>;

    /// [`TextLayout::draw`][1] calls this function to instruct the client to draw
    /// a strikethrough.
    ///
    /// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
    ///
    /// **Remarks**
    /// A single strikethrough can be broken into multiple calls, depending on
    /// how the formatting changes attributes. Strikethrough is not averaged
    /// across font sizes/styles changes.
    /// To get the correct top coordinate of the strikethrough rect,
    /// add strikethrough::offset to the baseline's Y.
    /// Like underlines, the x coordinate will always be passed as the left side,
    /// regardless of text directionality.
    ///
    /// </div>
    ///
    /// [1]: ../../struct.TextLayout.html#method.draw
    fn draw_inline_object(&mut self, context: &DrawInlineObject) -> DWResult<()>;
}

/// All of the contextual information required to draw a run of glyphs.
pub struct DrawGlyphRun<'a> {
    /// The context passed to [`TextLayout::draw`][1]
    ///
    /// [1]: ../../struct.TextLayout.html#method.draw
    pub context: DrawContext,

    /// Origin of the baseline for this run of glyphs. This is like the line you're writing
    /// on in a ruled notebook.
    pub baseline_origin: Point2f,

    /// The measuring method for glyphs in the run, used with the other properties to determine
    /// the rendering mode.
    pub measuring_mode: UncheckedEnum<MeasuringMode>,

    /// The glyph run that should be drawn.
    pub glyph_run: GlyphRun<'a>,

    /// Additional description of the properties of this run.
    pub glyph_run_desc: GlyphRunDescription<'a>,

    /// An application-defined effect applied to this text. In Direct2D convention this is
    /// often a Brush.
    pub client_effect: Option<&'a ClientEffect>,
}

/// All of the contextual information required to draw a section of underline.
pub struct DrawUnderline<'a> {
    /// The context passed to [`TextLayout::draw`][1]
    ///
    /// [1]: ../../struct.TextLayout.html#method.draw
    pub context: DrawContext,

    /// Origin of the baseline for this run of glyphs. This is like the line you're writing
    /// on in a ruled notebook.
    pub baseline_origin: Point2f,

    /// A description of the underline to be drawn.
    pub underline: Underline<'a>,

    /// An application-defined effect applied to this text. In Direct2D convention this is
    /// often a Brush.
    pub client_effect: Option<&'a ClientEffect>,
}

/// All of the contextual information required to draw a section of strikethrough.
pub struct DrawStrikethrough<'a> {
    /// The context passed to [`TextLayout::draw`][1]
    ///
    /// [1]: ../../struct.TextLayout.html#method.draw
    pub context: DrawContext,

    /// Origin of the baseline for this run of glyphs. This is like the line you're writing
    /// on in a ruled notebook.
    pub baseline_origin: Point2f,

    /// A description of the strikethrough to be drawn.
    pub strikethrough: Strikethrough<'a>,

    /// An application-defined effect applied to this text. In Direct2D convention this is
    /// often a Brush.
    pub client_effect: Option<&'a ClientEffect>,
}

/// All of the contextual information required to draw an inline object.
pub struct DrawInlineObject<'a> {
    /// The context passed to [`TextLayout::draw`][1]
    ///
    /// [1]: ../../struct.TextLayout.html#method.draw
    pub context: DrawContext,

    /// Top-left corner of where the inline object should be rendered.
    pub origin: Point2f,

    /// The inline object being drawn.
    pub inline_object: &'a InlineObject,

    /// Whether the text is indended to be drawn vertically.
    pub is_sideways: bool,

    /// A hint for drawing the inline object that it's surrounding text is rendered RTL
    /// and it may wish to flip its rendering direction depending on what it represents.
    pub is_right_to_left: bool,

    /// An application-defined effect applied to this text. In Direct2D convention this is
    /// often a Brush.
    pub client_effect: Option<&'a ClientEffect>,
}
