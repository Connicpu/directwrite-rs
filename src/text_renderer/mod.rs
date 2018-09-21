use enums::*;
use error::DWResult;
use font_face::FontFace;

use winapi::ctypes::c_void;
use winapi::um::dwrite::{
    IDWriteInlineObject, IDWriteTextRenderer, IDWriteTextRendererVtbl, DWRITE_MATRIX,
};
use winapi::um::unknwnbase::IUnknown;

pub mod vtbl;

pub struct Context(pub *mut c_void);

pub trait TextRenderer {
    fn current_transform(&self, context: Context) -> DWResult<DWRITE_MATRIX>;
    fn pixels_per_dip(&self, context: Context) -> DWResult<f32>;
    fn is_pixel_snapping_disabled(&self, context: Context) -> DWResult<bool>;

    fn draw_glyph_run(&mut self, context: &DrawGlyphRun) -> DWResult<()>;
    fn draw_inline_object(&mut self, context: &DrawInlineObject) -> DWResult<()>;
    fn draw_strikethrough(&mut self, context: &DrawStrikethrough) -> DWResult<()>;
    fn draw_underline(&mut self, context: &DrawUnderline) -> DWResult<()>;
}

pub struct DrawGlyphRun<'a> {
    pub context: Context,
    pub baseline_origin_x: f32,
    pub baseline_origin_y: f32,
    pub measuring_mode: MeasuringMode,
    pub font_face: FontFace,
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
    pub client_effect: Option<&'a IUnknown>,
}

pub struct DrawInlineObject<'a> {
    pub context: Context,
    pub origin_x: f32,
    pub origin_y: f32,
    pub inline_object: &'a IDWriteInlineObject,
    pub is_sideways: bool,
    pub is_right_to_left: bool,
    pub client_effect: Option<&'a IUnknown>,
}

pub struct DrawStrikethrough<'a> {
    pub context: Context,
    pub baseline_origin_x: f32,
    pub baseline_origin_y: f32,
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub reading_direction: ReadingDirection,
    pub flow_direction: FlowDirection,
    pub locale_name: &'a [u16],
    pub measuring_mode: MeasuringMode,
    pub client_effect: Option<&'a IUnknown>,
}

pub struct DrawUnderline<'a> {
    pub context: Context,
    pub baseline_origin_x: f32,
    pub baseline_origin_y: f32,
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub reading_direction: ReadingDirection,
    pub flow_direction: FlowDirection,
    pub locale_name: &'a [u16],
    pub measuring_mode: MeasuringMode,
    pub client_effect: Option<&'a IUnknown>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlyphOffset {
    pub advance_offset: f32,
    pub ascender_offset: f32,
}

#[repr(C)]
pub struct TextRendererComRef<'a> {
    vtbl: *const IDWriteTextRendererVtbl,
    obj: &'a mut TextRenderer,
}

impl<'a> TextRendererComRef<'a> {
    pub fn new(renderer: &'a mut TextRenderer) -> TextRendererComRef<'a> {
        TextRendererComRef {
            vtbl: &vtbl::TEXT_RENDERER_COMREF_VTBL,
            obj: renderer,
        }
    }

    pub unsafe fn as_raw(&mut self) -> &mut IDWriteTextRenderer {
        &mut *(self as *mut _ as *mut _)
    }
}
