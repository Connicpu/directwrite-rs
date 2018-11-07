use enums::font_feature_tag::FontFeatureTag;
use enums::{FontFaceType, FontSimulations, MeasuringMode, RenderingMode};
use error::DWResult;
use factory::Factory;
use font_file::FontFile;
use geometry_sink::{ComGeometrySink, GeometrySink};
use glyphs::GlyphOffset;
use metrics::{FontMetrics, GlyphMetrics};
use rendering_params::RenderingParams;

use std::{mem, ptr, u32};

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use math2d::Matrix3x2f;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{IDWriteFontFace, IDWriteFontFile, DWRITE_GLYPH_METRICS};
use wio::com::ComPtr;

#[doc(inline)]
pub use self::builder::FontFaceBuilder;
#[doc(inline)]
pub use self::table::FontTable;

#[doc(hidden)]
pub mod builder;
#[doc(hidden)]
pub mod table;

#[derive(ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct FontFace {
    ptr: ComPtr<IDWriteFontFace>,
}

impl FontFace {
    pub fn create<'a, 'b>(factory: &'a Factory) -> FontFaceBuilder<'a, 'b> {
        unsafe { FontFaceBuilder::new(&*factory.get_raw()) }
    }

    /// Obtains ideal (resolution-independent) glyph metrics in font design units.
    pub fn design_glyph_metrics(
        &self,
        glyph_indices: &[u16],
        is_sideways: bool,
    ) -> DWResult<Vec<GlyphMetrics>> {
        unsafe {
            let mut metrics = vec![mem::uninitialized(); glyph_indices.len()];
            let hr = self.ptr.GetDesignGlyphMetrics(
                glyph_indices.as_ptr(),
                glyph_indices.len() as u32,
                metrics.as_mut_ptr(),
                if is_sideways { 1 } else { 0 },
            );
            if SUCCEEDED(hr) {
                assert_eq!(
                    mem::size_of::<DWRITE_GLYPH_METRICS>(),
                    mem::size_of::<GlyphMetrics>(),
                );

                let metrics = mem::transmute::<Vec<_>, Vec<GlyphMetrics>>(metrics);
                Ok(metrics)
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains the font files representing a font face.
    pub fn files(&self) -> DWResult<Vec<FontFile>> {
        unsafe {
            let mut count = 0;
            let hr = self.ptr.GetFiles(&mut count, ptr::null_mut());
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            let mut native_files = vec![ptr::null_mut(); count as usize];
            let hr = self.ptr.GetFiles(&mut count, native_files.as_mut_ptr());
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            assert_eq!(
                mem::size_of::<*mut IDWriteFontFile>(),
                mem::size_of::<FontFile>()
            );

            Ok(mem::transmute::<Vec<*mut _>, Vec<FontFile>>(native_files))
        }
    }

    /// Obtains the number of glyphs in the font face.
    pub fn glyph_count(&self) -> u16 {
        unsafe { self.ptr.GetGlyphCount() }
    }

    /// Returns the nominal mapping of UCS4 Unicode code points to glyph indices as defined by the
    /// font 'CMAP' table.
    pub fn glyph_indices(&self, code_points: &[u32]) -> DWResult<Vec<u16>> {
        unsafe {
            let mut indices: Vec<u16> = Vec::with_capacity(code_points.len());
            let hr = self.ptr.GetGlyphIndices(
                code_points.as_ptr(),
                code_points.len() as u32,
                indices.as_mut_ptr(),
            );
            if SUCCEEDED(hr) {
                Ok(indices)
            } else {
                Err(hr.into())
            }
        }
    }

    /// Computes the outline of a run of glyphs by calling back to the outline sink interface.
    /// If glyph_advances and/or glyph_offsets are provided, they must be the same length as
    /// glyph_indices, or the function will panic.
    pub fn glyph_run_outline(
        &self,
        em_size: f32,
        glyph_indices: &[u16],
        glyph_advances: Option<&[f32]>,
        glyph_offsets: Option<&[GlyphOffset]>,
        is_sideways: bool,
        is_rtl: bool,
        geometry_sink: impl GeometrySink,
    ) -> DWResult<()> {
        let gi = glyph_indices;
        assert!(glyph_advances.map(|g| g.len() == gi.len()).unwrap_or(true));
        assert!(glyph_offsets.map(|g| g.len() == gi.len()).unwrap_or(true));

        let mut geometry_sink = ComGeometrySink::new(geometry_sink);

        unsafe {
            let hr = self.ptr.GetGlyphRunOutline(
                em_size,
                glyph_indices.as_ptr(),
                match glyph_advances {
                    Some(g) => g.as_ptr(),
                    None => ptr::null(),
                },
                match glyph_offsets {
                    Some(g) => g.as_ptr() as *const _,
                    None => ptr::null(),
                },
                glyph_indices.len() as u32,
                if is_sideways { 1 } else { 0 },
                if is_rtl { 1 } else { 0 },
                geometry_sink.as_raw_sink(),
            );
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains the index of a font face in the context of its font files.
    pub fn index(&self) -> u32 {
        unsafe { self.ptr.GetIndex() }
    }

    /// Obtains design units and common metrics for the font face.
    /// These metrics are applicable to all the glyphs within a font face
    /// and are used by applications for layout calculations.
    pub fn metrics(&self) -> FontMetrics {
        unsafe {
            let mut metrics = mem::uninitialized();
            self.ptr.GetMetrics(&mut metrics);
            metrics.into()
        }
    }

    pub fn recommended_rendering_mode(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        measuring_mode: MeasuringMode,
        params: &RenderingParams,
    ) -> DWResult<UncheckedEnum<RenderingMode>> {
        unsafe {
            let mut mode = 0;
            let hr = self.ptr.GetRecommendedRenderingMode(
                em_size,
                pixels_per_dip,
                measuring_mode as u32,
                params.get_raw(),
                &mut mode,
            );

            if SUCCEEDED(hr) {
                Ok(mode.into())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains the file format type of a font face.
    pub fn font_type(&self) -> UncheckedEnum<FontFaceType> {
        unsafe { self.ptr.GetType().into() }
    }

    /// Obtains the algorithmic style simulation flags of a font face.
    pub fn simulations(&self) -> FontSimulations {
        unsafe { FontSimulations(self.ptr.GetSimulations()) }
    }

    /// Determines whether the font is a symbol font.
    pub fn is_symbol_font(&self) -> bool {
        unsafe { self.ptr.IsSymbolFont() > 0 }
    }

    /// Obtains glyph metrics in font design units with the return values compatible with what GDI
    /// would produce.
    pub fn gdi_compatible_glyph_metrics(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        transform: Option<&Matrix3x2f>,
        use_gdi_natural: bool,
        glyph_indices: &[u16],
        is_sideways: bool,
    ) -> DWResult<Vec<GlyphMetrics>> {
        unsafe {
            let mut metrics = vec![mem::uninitialized(); glyph_indices.len()];
            let hr = self.ptr.GetGdiCompatibleGlyphMetrics(
                em_size,
                pixels_per_dip,
                match transform {
                    Some(x) => x as *const Matrix3x2f as *const _,
                    None => ptr::null(),
                },
                if use_gdi_natural { 1 } else { 0 },
                glyph_indices.as_ptr(),
                glyph_indices.len() as u32,
                metrics.as_mut_ptr(),
                if is_sideways { 1 } else { 0 },
            );
            if SUCCEEDED(hr) {
                assert_eq!(
                    mem::size_of::<DWRITE_GLYPH_METRICS>(),
                    mem::size_of::<GlyphMetrics>(),
                );

                let metrics = mem::transmute::<Vec<_>, Vec<GlyphMetrics>>(metrics);
                Ok(metrics)
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains design units and common metrics for the font face.
    /// These metrics are applicable to all the glyphs within a fontface and are used
    /// by applications for layout calculations.
    pub fn gdi_compatible_metrics(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        transform: Option<&Matrix3x2f>,
    ) -> DWResult<FontMetrics> {
        unsafe {
            let mut metrics = mem::uninitialized();
            let hr = self.ptr.GetGdiCompatibleMetrics(
                em_size,
                pixels_per_dip,
                match transform {
                    Some(x) => x as *const Matrix3x2f as *const _,
                    None => ptr::null(),
                },
                &mut metrics,
            );
            if SUCCEEDED(hr) {
                Ok(metrics.into())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Finds the specified OpenType font table if it exists and returns a pointer to it. The
    /// function accesses the underlying font data through the IDWriteFontFileStream interface
    /// implemented by the font file loader.
    pub fn font_table<'a>(&'a self, tag: impl Into<FontFeatureTag>) -> Option<FontTable<'a>> {
        unsafe {
            let mut data = ptr::null();
            let mut size = 0;
            let mut context = ptr::null_mut();
            let mut exists = 0;

            let hr = self.ptr.TryGetFontTable(
                tag.into().0,
                &mut data,
                &mut size,
                &mut context,
                &mut exists,
            );

            if SUCCEEDED(hr) && exists != 0 {
                let data = std::slice::from_raw_parts(data as *const u8, size as usize);

                Some(FontTable {
                    face: &self.ptr,
                    context,
                    data,
                })
            } else {
                None
            }
        }
    }
}
