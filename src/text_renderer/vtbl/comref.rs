use enums::*;
use error::DWResult;
use font_face::FontFace;
use text_renderer::{
    Context, DrawGlyphRun, DrawInlineObject, DrawStrikethrough, DrawUnderline, TextRendererComRef,
};

use std::panic::catch_unwind;
use std::slice;

use winapi::ctypes::c_void;
use winapi::shared::guiddef::{IsEqualIID, REFIID};
use winapi::shared::minwindef::{BOOL, FLOAT, ULONG};
use winapi::shared::winerror::{E_FAIL, E_NOTIMPL, HRESULT, SUCCEEDED, S_OK};
use winapi::um::dcommon::*;
use winapi::um::dwrite::*;
use winapi::um::unknwnbase::*;
use winapi::Interface;

pub static TEXT_RENDERER_COMREF_VTBL: IDWriteTextRendererVtbl = IDWriteTextRendererVtbl {
    parent: IDWritePixelSnappingVtbl {
        parent: IUnknownVtbl {
            QueryInterface: query_interface,
            AddRef: add_ref,
            Release: release,
        },
        GetCurrentTransform: get_current_transform,
        GetPixelsPerDip: get_pixels_per_dip,
        IsPixelSnappingDisabled: is_pixel_snapping_disabled,
    },
    DrawGlyphRun: draw_glyph_run,
    DrawInlineObject: draw_inline_object,
    DrawStrikethrough: draw_strikethrough,
    DrawUnderline: draw_underline,
};

pub unsafe extern "system" fn query_interface(
    this: *mut IUnknown,
    iid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if IsEqualIID(&*iid, &IUnknown::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    if IsEqualIID(&*iid, &IDWritePixelSnapping::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    if IsEqualIID(&*iid, &IDWriteTextRenderer::uuidof()) {
        add_ref(this);
        *ppv = this as *mut _;
        return S_OK;
    }

    return E_NOTIMPL;
}

pub unsafe extern "system" fn add_ref(_this: *mut IUnknown) -> ULONG {
    2
}

pub unsafe extern "system" fn release(_this: *mut IUnknown) -> ULONG {
    1
}

pub unsafe extern "system" fn get_current_transform(
    this: *mut IDWritePixelSnapping,
    context: *mut c_void,
    transform: *mut DWRITE_MATRIX,
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &*(this as *mut TextRendererComRef);
        match comref.obj.current_transform(Context(context)) {
            Ok(matrix) => *transform = matrix,
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

pub unsafe extern "system" fn get_pixels_per_dip(
    this: *mut IDWritePixelSnapping,
    context: *mut c_void,
    pixels_per_dip: *mut FLOAT,
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &*(this as *mut TextRendererComRef);
        match comref.obj.pixels_per_dip(Context(context)) {
            Ok(ppd) => *pixels_per_dip = ppd,
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

pub unsafe extern "system" fn is_pixel_snapping_disabled(
    this: *mut IDWritePixelSnapping,
    context: *mut c_void,
    is_disabled: *mut BOOL,
) -> HRESULT {
    match catch_unwind(move || {
        let comref = &*(this as *mut TextRendererComRef);
        match comref.obj.is_pixel_snapping_disabled(Context(context)) {
            Ok(disabled) => *is_disabled = disabled as BOOL,
            Err(err) if !SUCCEEDED(err.0) => return err.0,
            Err(_) => return E_FAIL,
        }
        S_OK
    }) {
        Ok(result) => result,
        Err(_) => E_FAIL,
    }
}

pub unsafe extern "system" fn draw_glyph_run(
    this: *mut IDWriteTextRenderer,
    context: *mut c_void,
    baseline_origin_x: FLOAT,
    baseline_origin_y: FLOAT,
    measuring_mode: DWRITE_MEASURING_MODE,
    glyph_run: *const DWRITE_GLYPH_RUN,
    glyph_run_desc: *const DWRITE_GLYPH_RUN_DESCRIPTION,
    client_effect: *mut IUnknown,
) -> HRESULT {
    match catch_unwind(move || -> DWResult<()> {
        let comref = &mut *(this as *mut TextRendererComRef);
        let run = &*glyph_run;
        let run_desc = &*glyph_run_desc;

        assert!(!run.fontFace.is_null());
        (*run.fontFace).AddRef();

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
            context: Context(context),
            baseline_origin_x,
            baseline_origin_y,
            measuring_mode: MeasuringMode::from_u32(measuring_mode).ok_or(E_FAIL)?,
            font_face: FontFace::from_raw(run.fontFace),
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
            client_effect: if client_effect.is_null() {
                None
            } else {
                Some(&*client_effect)
            },
        };

        comref.obj.draw_glyph_run(&data)
    }) {
        Ok(Ok(())) => S_OK,
        Ok(Err(err)) if !SUCCEEDED(err.0) => err.0,
        _ => E_FAIL,
    }
}

pub unsafe extern "system" fn draw_inline_object(
    this: *mut IDWriteTextRenderer,
    context: *mut c_void,
    origin_x: FLOAT,
    origin_y: FLOAT,
    inline_object: *mut IDWriteInlineObject,
    is_sideways: BOOL,
    is_rtl: BOOL,
    client_effect: *mut IUnknown,
) -> HRESULT {
    match catch_unwind(move || -> DWResult<()> {
        let comref = &mut *(this as *mut TextRendererComRef);

        let data = DrawInlineObject {
            context: Context(context),
            origin_x,
            origin_y,
            inline_object: &*inline_object,
            is_sideways: is_sideways != 0,
            is_right_to_left: is_rtl != 0,
            client_effect: if client_effect.is_null() {
                None
            } else {
                Some(&*client_effect)
            },
        };

        comref.obj.draw_inline_object(&data)
    }) {
        Ok(Ok(())) => S_OK,
        Ok(Err(err)) if !SUCCEEDED(err.0) => err.0,
        _ => E_FAIL,
    }
}

pub unsafe extern "system" fn draw_strikethrough(
    this: *mut IDWriteTextRenderer,
    context: *mut c_void,
    baseline_origin_x: FLOAT,
    baseline_origin_y: FLOAT,
    strikethrough: *const DWRITE_STRIKETHROUGH,
    client_effect: *mut IUnknown,
) -> HRESULT {
    match catch_unwind(move || -> DWResult<()> {
        let comref = &mut *(this as *mut TextRendererComRef);
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
            context: Context(context),
            baseline_origin_x,
            baseline_origin_y,
            width: desc.width,
            thickness: desc.thickness,
            offset: desc.offset,
            reading_direction: ReadingDirection::from_u32(desc.readingDirection).ok_or(E_FAIL)?,
            flow_direction: FlowDirection::from_u32(desc.flowDirection).ok_or(E_FAIL)?,
            locale_name: locale,
            measuring_mode: MeasuringMode::from_u32(desc.measuringMode).ok_or(E_FAIL)?,
            client_effect: if client_effect.is_null() {
                None
            } else {
                Some(&*client_effect)
            },
        };

        comref.obj.draw_strikethrough(&data)
    }) {
        Ok(Ok(())) => S_OK,
        Ok(Err(err)) if !SUCCEEDED(err.0) => err.0,
        _ => E_FAIL,
    }
}

pub unsafe extern "system" fn draw_underline(
    this: *mut IDWriteTextRenderer,
    context: *mut c_void,
    baseline_origin_x: FLOAT,
    baseline_origin_y: FLOAT,
    underline: *const DWRITE_UNDERLINE,
    client_effect: *mut IUnknown,
) -> HRESULT {
    match catch_unwind(move || -> DWResult<()> {
        let comref = &mut *(this as *mut TextRendererComRef);
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
            context: Context(context),
            baseline_origin_x,
            baseline_origin_y,
            width: desc.width,
            thickness: desc.thickness,
            offset: desc.offset,
            reading_direction: ReadingDirection::from_u32(desc.readingDirection).ok_or(E_FAIL)?,
            flow_direction: FlowDirection::from_u32(desc.flowDirection).ok_or(E_FAIL)?,
            locale_name: locale,
            measuring_mode: MeasuringMode::from_u32(desc.measuringMode).ok_or(E_FAIL)?,
            client_effect: if client_effect.is_null() {
                None
            } else {
                Some(&*client_effect)
            },
        };

        comref.obj.draw_underline(&data)
    }) {
        Ok(Ok(())) => S_OK,
        Ok(Err(err)) if !SUCCEEDED(err.0) => err.0,
        _ => E_FAIL,
    }
}
