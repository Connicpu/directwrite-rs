use client_effect::ClientEffect;
use inline_object::InlineObject;
use enums::*;
use error::DWResult;
use font_face::FontFace;
use glyphs::GlyphOffset;

use checked_enum::UncheckedEnum;
use math2d::Matrix3x2f;

pub mod com_renderer;

pub trait CustomTextRenderer: Send + 'static {
    fn current_transform(&self, context: usize) -> DWResult<Matrix3x2f>;
    fn pixels_per_dip(&self, context: usize) -> DWResult<f32>;
    fn is_pixel_snapping_disabled(&self, context: usize) -> DWResult<bool>;

    fn draw_glyph_run(&mut self, context: &DrawGlyphRun) -> DWResult<()>;
    fn draw_inline_object(&mut self, context: &DrawInlineObject) -> DWResult<()>;
    fn draw_strikethrough(&mut self, context: &DrawStrikethrough) -> DWResult<()>;
    fn draw_underline(&mut self, context: &DrawUnderline) -> DWResult<()>;
}

pub struct DrawGlyphRun<'a> {
    pub context: usize,
    pub baseline_origin_x: f32,
    pub baseline_origin_y: f32,
    pub measuring_mode: UncheckedEnum<MeasuringMode>,
    pub font_face: &'a FontFace,
    pub font_em_size: f32,
    pub glyph_count: u32,
    pub glyph_indices: &'a [u16],
    pub glyph_advances: &'a [f32],
    pub glyph_offsets: &'a [GlyphOffset],
    pub is_sideways: bool,
    pub bidi_level: u32,
    pub locale_name: &'a [u16],
    pub string: &'a [u16],
    pub cluster_map: &'a [u16],
    pub text_position: u32,
    pub client_effect: Option<&'a ClientEffect>,
}

pub struct DrawInlineObject<'a> {
    pub context: usize,
    pub origin_x: f32,
    pub origin_y: f32,
    pub inline_object: &'a InlineObject,
    pub is_sideways: bool,
    pub is_right_to_left: bool,
    pub client_effect: Option<&'a ClientEffect>,
}

pub struct DrawStrikethrough<'a> {
    pub context: usize,
    pub baseline_origin_x: f32,
    pub baseline_origin_y: f32,
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub reading_direction: UncheckedEnum<ReadingDirection>,
    pub flow_direction: UncheckedEnum<FlowDirection>,
    pub locale_name: &'a [u16],
    pub measuring_mode: UncheckedEnum<MeasuringMode>,
    pub client_effect: Option<&'a ClientEffect>,
}

pub struct DrawUnderline<'a> {
    pub context: usize,
    pub baseline_origin_x: f32,
    pub baseline_origin_y: f32,
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub reading_direction: UncheckedEnum<ReadingDirection>,
    pub flow_direction: UncheckedEnum<FlowDirection>,
    pub locale_name: &'a [u16],
    pub measuring_mode: UncheckedEnum<MeasuringMode>,
    pub client_effect: Option<&'a ClientEffect>,
}
