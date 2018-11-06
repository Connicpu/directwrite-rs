#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GlyphOffset {
    pub advance_offset: f32,
    pub ascender_offset: f32,
}
