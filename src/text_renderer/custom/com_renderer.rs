use crate::descriptions::{GlyphRun, GlyphRunDescription};
use crate::descriptions::{Strikethrough, Underline};
use crate::text_renderer::custom::CustomTextRenderer;
use crate::text_renderer::custom::DrawGlyphRun;
use crate::text_renderer::custom::DrawInlineObject;
use crate::text_renderer::custom::DrawStrikethrough;
use crate::text_renderer::custom::DrawUnderline;
use crate::text_renderer::DrawContext;
use crate::text_renderer::TextRenderer;

use com_impl::Refcount;
use com_impl::VTable;
use com_wrapper::ComWrapper;
use dcommon::helpers::wrap_opt_ref_to_raw_com;
use dcommon::helpers::wrap_ref_to_raw_com;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::FLOAT;
use winapi::shared::winerror::{E_FAIL, HRESULT, S_OK};
use winapi::um::dcommon::DWRITE_MEASURING_MODE;
use winapi::um::dwrite::IDWriteInlineObject;
use winapi::um::dwrite::IDWritePixelSnapping;
use winapi::um::dwrite::IDWritePixelSnappingVtbl;
use winapi::um::dwrite::IDWriteTextRenderer;
use winapi::um::dwrite::IDWriteTextRendererVtbl;
use winapi::um::dwrite::DWRITE_GLYPH_RUN;
use winapi::um::dwrite::DWRITE_GLYPH_RUN_DESCRIPTION;
use winapi::um::dwrite::DWRITE_MATRIX;
use winapi::um::dwrite::DWRITE_STRIKETHROUGH;
use winapi::um::dwrite::DWRITE_UNDERLINE;
use winapi::um::unknwnbase::IUnknown;

#[repr(C)]
#[derive(com_impl::ComImpl)]
#[interfaces(IDWritePixelSnapping, IDWriteTextRenderer)]
/// A COM-compatible wrapper of an application-implemented TextRenderer
pub struct ComRenderer<T: CustomTextRenderer> {
    vtable: VTable<IDWriteTextRendererVtbl>,
    refcount: Refcount,
    renderer: T,
}

impl<T: CustomTextRenderer> ComRenderer<T> {
    /// Create a new TextRenderer from a CustomTextRenderer
    pub fn new(renderer: T) -> TextRenderer {
        let ptr = Self::create_raw(renderer);
        let ptr = ptr as *mut IDWriteTextRenderer;
        unsafe { TextRenderer::from_raw(ptr) }
    }
}

#[com_impl::com_impl]
unsafe impl<T: CustomTextRenderer> IDWritePixelSnapping for ComRenderer<T> {
    #[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn is_pixel_snapping_disabled(
        &self,
        context: *mut c_void,
        is_disabled: *mut BOOL,
    ) -> HRESULT {
        let context = DrawContext::from_ptr(context);
        *is_disabled = self.renderer.pixel_snapping_disabled(context) as i32;
        S_OK
    }

    #[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn get_current_transform(
        &self,
        context: *mut c_void,
        transform: *mut DWRITE_MATRIX,
    ) -> HRESULT {
        let context = DrawContext::from_ptr(context);
        *transform = self.renderer.current_transform(context).into();
        S_OK
    }

    #[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn get_pixels_per_dip(
        &self,
        context: *mut c_void,
        pixels_per_dip: *mut f32,
    ) -> HRESULT {
        let context = DrawContext::from_ptr(context);
        *pixels_per_dip = self.renderer.pixels_per_dip(context);
        S_OK
    }
}

#[com_impl::com_impl]
unsafe impl<T: CustomTextRenderer> IDWriteTextRenderer for ComRenderer<T> {
    #[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn draw_glyph_run(
        &mut self,
        context: *mut c_void,
        baseline_origin_x: FLOAT,
        baseline_origin_y: FLOAT,
        measuring_mode: DWRITE_MEASURING_MODE,
        glyph_run: *const DWRITE_GLYPH_RUN,
        glyph_run_desc: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        client_effect: *mut IUnknown,
    ) -> HRESULT {
        let data = DrawGlyphRun {
            context: DrawContext::from_ptr(context),
            baseline_origin: (baseline_origin_x, baseline_origin_y).into(),
            measuring_mode: measuring_mode.into(),
            glyph_run: GlyphRun::from_raw(&*glyph_run),
            glyph_run_desc: GlyphRunDescription::from_raw(&*glyph_run_desc),
            client_effect: wrap_opt_ref_to_raw_com(&client_effect),
        };

        match self.renderer.draw_glyph_run(&data) {
            Ok(_) => S_OK,
            Err(e) => e.0,
        }
    }

    #[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn draw_strikethrough(
        &mut self,
        context: *mut c_void,
        baseline_origin_x: FLOAT,
        baseline_origin_y: FLOAT,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        client_effect: *mut IUnknown,
    ) -> HRESULT {
        let data = DrawStrikethrough {
            context: DrawContext::from_ptr(context),
            baseline_origin: (baseline_origin_x, baseline_origin_y).into(),
            strikethrough: Strikethrough::from_raw(&*strikethrough),
            client_effect: wrap_opt_ref_to_raw_com(&client_effect),
        };

        match self.renderer.draw_strikethrough(&data) {
            Ok(_) => S_OK,
            Err(e) => e.0,
        }
    }

    #[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn draw_underline(
        &mut self,
        context: *mut c_void,
        baseline_origin_x: FLOAT,
        baseline_origin_y: FLOAT,
        underline: *const DWRITE_UNDERLINE,
        client_effect: *mut IUnknown,
    ) -> HRESULT {
        let data = DrawUnderline {
            context: DrawContext::from_ptr(context),
            baseline_origin: (baseline_origin_x, baseline_origin_y).into(),
            underline: Underline::from_raw(&*underline),
            client_effect: wrap_opt_ref_to_raw_com(&client_effect),
        };

        match self.renderer.draw_underline(&data) {
            Ok(_) => S_OK,
            Err(e) => e.0,
        }
    }

    #[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn draw_inline_object(
        &mut self,
        context: *mut c_void,
        origin_x: FLOAT,
        origin_y: FLOAT,
        inline_object: *mut IDWriteInlineObject,
        is_sideways: BOOL,
        is_rtl: BOOL,
        client_effect: *mut IUnknown,
    ) -> HRESULT {
        let data = DrawInlineObject {
            context: DrawContext::from_ptr(context),
            origin: (origin_x, origin_y).into(),
            inline_object: wrap_ref_to_raw_com(&inline_object),
            is_sideways: is_sideways != 0,
            is_right_to_left: is_rtl != 0,
            client_effect: wrap_opt_ref_to_raw_com(&client_effect),
        };

        match self.renderer.draw_inline_object(&data) {
            Ok(_) => S_OK,
            Err(e) => e.0,
        }
    }
}
