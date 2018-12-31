use crate::font_face::FontFace;

use std::slice::from_raw_parts;

use com_wrapper::ComWrapper;
use dcommon::helpers::wrap_ref_to_raw_com;
use dcommon::helpers::{WideCStr, WideStr};
use winapi::um::dwrite::{DWRITE_GLYPH_RUN, DWRITE_GLYPH_RUN_DESCRIPTION};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// The optional adjustment to a glyph's position.
///
/// A glyph offset changes the position of a glyph without affecting the pen position. Offsets
/// are in logical, pre-transform units.
pub struct GlyphOffset {
    /// The offset in the advance direction of the run. A positive advance offset moves the glyph
    /// to the right (in pre-transform coordinates) if the run is left-to-right or to the left if
    /// the run is right-to-left.
    pub advance_offset: f32,

    /// The offset in the ascent direction, that is, the direction ascenders point. A positive
    /// ascender offset moves the glyph up (in pre-transform coordinates). A negative ascender
    /// offset moves the glyph down.
    pub ascender_offset: f32,
}

#[cfg(test)]
dcommon::member_compat_test! {
    glyph_offset_compat_test:
    GlyphOffset <=> winapi::um::dwrite::DWRITE_GLYPH_OFFSET {
        advance_offset <=> advanceOffset,
        ascender_offset <=> ascenderOffset,
    }
}

/// Contains the information needed by renderers to draw glyph runs. All coordinates are in device
/// independent pixels (DIPs).
pub struct GlyphRun<'a> {
    /// The physical font face object to draw with.
    pub font_face: &'a FontFace,

    /// The logical size of the font in DIPs (equals 1/96 inch), not points.
    pub font_em_size: f32,

    /// An array of indices to render for the glyph run.
    pub glyph_indices: &'a [u16],

    /// An array containing glyph advance widths for the glyph run.
    pub glyph_advances: &'a [f32],

    /// An array containing glyph offsets for the glyph run.
    pub glyph_offsets: &'a [GlyphOffset],

    /// If true, specifies that glyphs are rotated 90 degrees to the left and vertical metrics are
    /// used. Vertical writing is achieved by specifying is_sideways = true and rotating the entire
    /// run 90 degrees to the right via a rotate transform.
    pub is_sideways: bool,

    /// The implicit resolved bidi level of the run. Odd levels indicate right-to-left languages
    /// like Hebrew and Arabic, while even levels indicate left-to-right languages like English
    /// and Japanese (when written horizontally). For right-to-left languages, the text origin is
    /// on the right, and text should be drawn to the left.
    pub bidi_level: u32,
}

impl<'a> GlyphRun<'a> {
    pub(crate) unsafe fn from_raw(run: &'a DWRITE_GLYPH_RUN) -> GlyphRun<'a> {
        let len = run.glyphCount as usize;
        GlyphRun {
            font_face: wrap_ref_to_raw_com(&run.fontFace),
            font_em_size: run.fontEmSize,
            glyph_indices: from_raw_parts(run.glyphIndices, len),
            glyph_advances: from_raw_parts(run.glyphAdvances, len),
            glyph_offsets: from_raw_parts(run.glyphOffsets as *const GlyphOffset, len),
            is_sideways: run.isSideways != 0,
            bidi_level: run.bidiLevel,
        }
    }

    pub(crate) unsafe fn into_raw(&self) -> DWRITE_GLYPH_RUN {
        DWRITE_GLYPH_RUN {
            fontFace: self.font_face.get_raw(),
            fontEmSize: self.font_em_size,
            glyphCount: self.glyph_indices.len() as u32,
            glyphIndices: self.glyph_indices.as_ptr(),
            glyphAdvances: self.glyph_advances.as_ptr(),
            glyphOffsets: self.glyph_offsets.as_ptr() as *const _,
            isSideways: self.is_sideways as i32,
            bidiLevel: self.bidi_level,
        }
    }
}

/// Contains additional properties related to those in [`GlyphRun`][1].
///
/// [1]: struct.GlyphRun.html
pub struct GlyphRunDescription<'a> {
    /// A string for the locale name associated with this run.
    pub locale_name: &'a WideCStr,

    /// The string associated with the glyphs in this run.
    pub string: WideStr<'a>,

    /// An array of indices to the glyph indices array, of the first glyphs of all the glyph
    /// clusters of the glyphs to render. This is the same length as `string` and associates
    /// the characters in the string with the glyphs in the run.
    pub cluster_map: &'a [u16],

    /// Corresponding text position in the string this glyph run came from. This is relative to
    /// the beginning of the string represented by the TextLayout object.
    pub text_position: u32,
}

impl<'a> GlyphRunDescription<'a> {
    pub(crate) unsafe fn from_raw(
        desc: &'a DWRITE_GLYPH_RUN_DESCRIPTION,
    ) -> GlyphRunDescription<'a> {
        let len = desc.stringLength as usize;
        GlyphRunDescription {
            locale_name: WideCStr::from_ptr(desc.localeName),
            string: WideStr::from_raw(desc.string, len),
            cluster_map: from_raw_parts(desc.clusterMap, len),
            text_position: desc.textPosition,
        }
    }

    pub(crate) unsafe fn into_raw(&self) -> DWRITE_GLYPH_RUN_DESCRIPTION {
        DWRITE_GLYPH_RUN_DESCRIPTION {
            localeName: self.locale_name.as_ptr(),
            string: self.string.data.as_ptr(),
            stringLength: self.string.data.len() as u32,
            clusterMap: self.cluster_map.as_ptr(),
            textPosition: self.text_position,
        }
    }
}
