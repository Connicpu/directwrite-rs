//! TextRenderer and types for constructing your own application-defined instances.

use crate::error::DWResult;
use crate::pixel_snapping::PixelSnapping;
use crate::text_renderer::custom::{
    DrawGlyphRun, DrawInlineObject, DrawStrikethrough, DrawUnderline,
};

use com_wrapper::ComWrapper;
use dcommon::helpers::deref_com_wrapper;
use dcommon::helpers::unwrap_opt_com;
use winapi::ctypes::c_void;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteTextRenderer;
use wio::com::ComPtr;

pub mod custom;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send)]
/// A generic Text Renderer object.
pub struct TextRenderer {
    ptr: ComPtr<IDWriteTextRenderer>,
}

impl TextRenderer {
    /// Create a text renderer from an application-implemented interface.
    pub fn new(renderer: impl custom::CustomTextRenderer) -> TextRenderer {
        custom::com_renderer::ComRenderer::new(renderer)
    }

    /// Draws a run of glyphs using this text renderer. Normally you won't call this directly,
    /// but it will be called indirectly from [`TextLayout::draw`][1]
    ///
    /// [1]: struct.TextLayout.html#method.draw
    pub fn draw_glyph_run(&mut self, context: &DrawGlyphRun) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.DrawGlyphRun(
                context.context.ptr(),
                context.baseline_origin.x,
                context.baseline_origin.y,
                context.measuring_mode.value,
                &context.glyph_run.into_raw(),
                &context.glyph_run_desc.into_raw(),
                unwrap_opt_com(context.client_effect),
            );
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Draws a section of underline using this text renderer. Normally you won't call this
    /// directly, but it will be called indirectly from [`TextLayout::draw`][1]
    ///
    /// [1]: struct.TextLayout.html#method.draw
    pub fn draw_underline(&mut self, context: &DrawUnderline) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.DrawUnderline(
                context.context.ptr(),
                context.baseline_origin.x,
                context.baseline_origin.y,
                &context.underline.into_raw(),
                unwrap_opt_com(context.client_effect),
            );
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Draws a section of strikethrough using this text renderer. Normally you won't call this
    /// directly, but it will be called indirectly from [`TextLayout::draw`][1]
    ///
    /// [1]: struct.TextLayout.html#method.draw
    pub fn draw_strikethrough(&mut self, context: &DrawStrikethrough) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.DrawStrikethrough(
                context.context.ptr(),
                context.baseline_origin.x,
                context.baseline_origin.y,
                &context.strikethrough.into_raw(),
                unwrap_opt_com(context.client_effect),
            );
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Draws an inline object using this text renderer. Normally you won't call this
    /// directly, but it will be called indirectly from [`TextLayout::draw`][1]
    ///
    /// [1]: struct.TextLayout.html#method.draw
    pub fn draw_inline_object(&mut self, context: &DrawInlineObject) -> DWResult<()> {
        unsafe {
            let hr = self.ptr.DrawInlineObject(
                context.context.ptr(),
                context.origin.x,
                context.origin.y,
                context.inline_object.get_raw(),
                context.is_sideways as i32,
                context.is_right_to_left as i32,
                unwrap_opt_com(context.client_effect),
            );
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }
}

impl std::ops::Deref for TextRenderer {
    type Target = PixelSnapping;
    fn deref(&self) -> &PixelSnapping {
        unsafe { deref_com_wrapper(self) }
    }
}

#[derive(Copy, Clone)]
/// A context that can be passed through draw calls. This may only be constructed unsafely
/// as any text renderer could be behind a TextRenderer and it may do anything with the
/// context passed.
pub struct DrawContext(usize);

impl DrawContext {
    /// Construct the context from an integer value.
    pub unsafe fn from_usize(value: usize) -> Self {
        DrawContext(value)
    }

    /// Construct the context from a void pointer.
    pub unsafe fn from_ptr(value: *mut c_void) -> Self {
        DrawContext(value as usize)
    }

    /// Interpret the context as an integer.
    pub fn value(&self) -> usize {
        self.0
    }

    /// Interpret the context as a void pointer.
    pub unsafe fn ptr(&self) -> *mut c_void {
        self.0 as *mut c_void
    }
}

impl std::fmt::Debug for DrawContext {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_tuple("DrawContext")
            .field(&format_args!("{:x}", self.0))
            .finish()
    }
}
