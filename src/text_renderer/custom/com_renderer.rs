use helpers::wrap_opt_ref_to_raw_com;
use helpers::wrap_ref_to_raw_com;
use text_renderer::custom::CustomTextRenderer;
use text_renderer::custom::DrawGlyphRun;
use text_renderer::custom::DrawInlineObject;
use text_renderer::custom::DrawStrikethrough;
use text_renderer::custom::DrawUnderline;
use text_renderer::TextRenderer;

use std::slice;

use com_impl::Refcount;
use com_impl::VTable;
use com_wrapper::ComWrapper;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::FLOAT;
use winapi::shared::winerror::HRESULT;
use winapi::shared::winerror::S_OK;
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
#[derive(ComImpl)]
#[interfaces(IDWritePixelSnapping, IDWriteTextRenderer)]
pub struct ComRenderer<T: CustomTextRenderer> {
    vtable: VTable<IDWriteTextRendererVtbl>,
    refcount: Refcount,
    renderer: T,
}

impl<T: CustomTextRenderer> ComRenderer<T> {
    pub fn new(renderer: T) -> TextRenderer {
        let ptr = Self::create_raw(renderer);
        let ptr = ptr as *mut IDWriteTextRenderer;
        unsafe { TextRenderer::from_raw(ptr) }
    }
}

#[com_impl]
unsafe impl<T: CustomTextRenderer> IDWritePixelSnapping for ComRenderer<T> {
    //#[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn get_current_transform(
        &self,
        context: *mut c_void,
        transform: *mut DWRITE_MATRIX,
    ) -> HRESULT {
        match self.renderer.current_transform(context as usize) {
            Ok(matrix) => {
                *transform = matrix.into();
                S_OK
            }
            Err(e) => e.0,
        }
    }

    //#[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn get_pixels_per_dip(
        &self,
        context: *mut c_void,
        pixels_per_dip: *mut f32,
    ) -> HRESULT {
        match self.renderer.pixels_per_dip(context as usize) {
            Ok(ppd) => {
                *pixels_per_dip = ppd;
                S_OK
            }
            Err(e) => e.0,
        }
    }

    //#[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn is_pixel_snapping_disabled(
        &self,
        context: *mut c_void,
        is_disabled: *mut BOOL,
    ) -> HRESULT {
        match self.renderer.is_pixel_snapping_disabled(context as usize) {
            Ok(disabled) => {
                *is_disabled = disabled as BOOL;
                S_OK
            }
            Err(e) => e.0,
        }
    }
}

#[com_impl]
unsafe impl<T: CustomTextRenderer> IDWriteTextRenderer for ComRenderer<T> {
    //#[panic(result = "E_FAIL")]
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
        let run = &*glyph_run;
        let run_desc = &*glyph_run_desc;

        let locale = if run_desc.localeName.is_null() {
            &[]
        } else {
            let mut p = run_desc.localeName;
            let mut i = 0;
            let len = loop {
                if *p == 0 {
                    break i;
                }
                p = p.offset(1);
            };
            slice::from_raw_parts(run_desc.localeName, len)
        };

        let gcount = run.glyphCount as usize;
        let data = DrawGlyphRun {
            context: context as usize,
            baseline_origin_x,
            baseline_origin_y,
            measuring_mode: measuring_mode.into(),
            font_face: wrap_ref_to_raw_com(&run.fontFace),
            font_em_size: run.fontEmSize,
            glyph_count: run.glyphCount,
            glyph_indices: slice::from_raw_parts(run.glyphIndices, gcount),
            glyph_advances: slice::from_raw_parts(run.glyphAdvances, gcount),
            glyph_offsets: slice::from_raw_parts(run.glyphOffsets as *const _, gcount),
            is_sideways: run.isSideways != 0,
            bidi_level: run.bidiLevel,
            locale_name: locale,
            string: slice::from_raw_parts(run_desc.string, run_desc.stringLength as usize),
            cluster_map: slice::from_raw_parts(run_desc.clusterMap, run_desc.stringLength as usize),
            text_position: run_desc.textPosition,
            client_effect: wrap_opt_ref_to_raw_com(&client_effect),
        };

        match self.renderer.draw_glyph_run(&data) {
            Ok(_) => S_OK,
            Err(e) => e.0,
        }
    }

    //#[panic(result = "E_FAIL")]
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
            context: context as usize,
            origin_x,
            origin_y,
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

    //#[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn draw_strikethrough(
        &mut self,
        context: *mut c_void,
        baseline_origin_x: FLOAT,
        baseline_origin_y: FLOAT,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        client_effect: *mut IUnknown,
    ) -> HRESULT {
        let desc = &*strikethrough;

        let locale = if desc.localeName.is_null() {
            &[]
        } else {
            let mut p = desc.localeName;
            let mut i = 0;
            let len = loop {
                if *p == 0 {
                    break i;
                }
                p = p.offset(1);
            };
            slice::from_raw_parts(desc.localeName, len)
        };

        let data = DrawStrikethrough {
            context: context as usize,
            baseline_origin_x,
            baseline_origin_y,
            width: desc.width,
            thickness: desc.thickness,
            offset: desc.offset,
            reading_direction: desc.readingDirection.into(),
            flow_direction: desc.flowDirection.into(),
            locale_name: locale,
            measuring_mode: desc.measuringMode.into(),
            client_effect: wrap_opt_ref_to_raw_com(&client_effect),
        };

        match self.renderer.draw_strikethrough(&data) {
            Ok(_) => S_OK,
            Err(e) => e.0,
        }
    }

    //#[panic(result = "E_FAIL")]
    pub unsafe extern "system" fn draw_underline(
        &mut self,
        context: *mut c_void,
        baseline_origin_x: FLOAT,
        baseline_origin_y: FLOAT,
        underline: *const DWRITE_UNDERLINE,
        client_effect: *mut IUnknown,
    ) -> HRESULT {
        let desc = &*underline;

        let locale = if desc.localeName.is_null() {
            &[]
        } else {
            let mut p = desc.localeName;
            let mut i = 0;
            let len = loop {
                if *p == 0 {
                    break i;
                }
                p = p.offset(1);
            };
            slice::from_raw_parts(desc.localeName, len)
        };

        let data = DrawUnderline {
            context: context as usize,
            baseline_origin_x,
            baseline_origin_y,
            width: desc.width,
            thickness: desc.thickness,
            offset: desc.offset,
            reading_direction: desc.readingDirection.into(),
            flow_direction: desc.flowDirection.into(),
            locale_name: locale,
            measuring_mode: desc.measuringMode.into(),
            client_effect: wrap_opt_ref_to_raw_com(&client_effect),
        };

        match self.renderer.draw_underline(&data) {
            Ok(_) => S_OK,
            Err(e) => e.0,
        }
    }
}
