//! FontFace and types for examining its contents and building new ones.

use crate::descriptions::GlyphOffset;
use crate::enums::font_feature_tag::FontFeatureTag;
use crate::enums::{FontFaceType, FontSimulations, MeasuringMode, RenderingMode};
use crate::factory::IFactory;
use crate::font_file::FontFile;
use crate::geometry_sink::{self, GeometrySink};
use crate::metrics::{FontMetrics, GlyphMetrics};
use crate::rendering_params::IRenderingParams;

use std::{mem, ptr, u32};

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::Error;
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

#[repr(transparent)]
#[derive(Clone, ComWrapper, PartialEq)]
#[com(send, sync, debug)]
/// Represents an absolute reference to a font face which contains font face type, appropriate
/// file references, face identification data and various font data such as metrics, names and
/// glyph outlines.
pub struct FontFace {
    ptr: ComPtr<IDWriteFontFace>,
}

impl FontFace {
    /// Initializes a builder for creating a FontFace
    pub fn create<'a, 'b>(factory: &'a dyn IFactory) -> FontFaceBuilder<'a, 'b> {
        unsafe { FontFaceBuilder::new(factory.raw_f()) }
    }
}

pub unsafe trait IFontFace {
    /// Obtains ideal (resolution-independent) glyph metrics in font design units.
    fn design_glyph_metrics(
        &self,
        glyph_indices: &[u16],
        is_sideways: bool,
    ) -> Result<Vec<GlyphMetrics>, Error> {
        unsafe {
            let mut metrics = vec![mem::uninitialized(); glyph_indices.len()];
            let hr = self.raw_fontface().GetDesignGlyphMetrics(
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
    fn files(&self) -> Result<Vec<FontFile>, Error> {
        unsafe {
            let mut count = 0;
            let hr = self.raw_fontface().GetFiles(&mut count, ptr::null_mut());
            if !SUCCEEDED(hr) {
                return Err(hr.into());
            }

            let mut native_files = vec![ptr::null_mut(); count as usize];
            let hr = self
                .raw_fontface()
                .GetFiles(&mut count, native_files.as_mut_ptr());
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
    fn glyph_count(&self) -> u16 {
        unsafe { self.raw_fontface().GetGlyphCount() }
    }

    /// Returns the nominal mapping of UCS4 Unicode code points to glyph indices as defined by the
    /// font 'CMAP' table.
    fn glyph_indices(&self, code_points: &[u32]) -> Result<Vec<u16>, Error> {
        unsafe {
            let mut indices: Vec<u16> = Vec::with_capacity(code_points.len());
            let hr = self.raw_fontface().GetGlyphIndices(
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
    fn glyph_run_outline(
        &self,
        em_size: f32,
        glyph_indices: &[u16],
        glyph_advances: Option<&[f32]>,
        glyph_offsets: Option<&[GlyphOffset]>,
        is_sideways: bool,
        is_rtl: bool,
        geometry_sink: impl GeometrySink,
    ) -> Result<(), Error> {
        let gi = glyph_indices;
        assert!(glyph_advances.map(|g| g.len() == gi.len()).unwrap_or(true));
        assert!(glyph_offsets.map(|g| g.len() == gi.len()).unwrap_or(true));

        unsafe {
            let geometry_sink = geometry_sink::com_sink::ComGeometrySink::create(geometry_sink);

            let hr = self.raw_fontface().GetGlyphRunOutline(
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
                is_sideways as i32,
                is_rtl as i32,
                geometry_sink.as_raw(),
            );
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains the index of a font face in the context of its font files.
    fn index(&self) -> u32 {
        unsafe { self.raw_fontface().GetIndex() }
    }

    /// Obtains design units and common metrics for the font face.
    /// These metrics are applicable to all the glyphs within a font face
    /// and are used by applications for layout calculations.
    fn metrics(&self) -> FontMetrics {
        unsafe {
            let mut metrics = mem::uninitialized();
            self.raw_fontface().GetMetrics(&mut metrics);
            metrics.into()
        }
    }

    /// Attempt to determine the recommended rendering mode for this font face
    /// with the given parameters.
    fn recommended_rendering_mode(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        measuring_mode: MeasuringMode,
        params: &dyn IRenderingParams,
    ) -> Result<UncheckedEnum<RenderingMode>, Error> {
        unsafe {
            let mut mode = 0;
            let hr = self.raw_fontface().GetRecommendedRenderingMode(
                em_size,
                pixels_per_dip,
                measuring_mode as u32,
                params.raw_rp() as *const _ as *mut _,
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
    fn font_type(&self) -> UncheckedEnum<FontFaceType> {
        unsafe { self.raw_fontface().GetType().into() }
    }

    /// Obtains the algorithmic style simulation flags of a font face.
    fn simulations(&self) -> FontSimulations {
        unsafe { FontSimulations(self.raw_fontface().GetSimulations()) }
    }

    /// Determines whether the font is a symbol font.
    fn is_symbol_font(&self) -> bool {
        unsafe { self.raw_fontface().IsSymbolFont() > 0 }
    }

    /// Obtains glyph metrics in font design units with the return values compatible with what GDI
    /// would produce.
    fn gdi_compatible_glyph_metrics(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        transform: Option<&Matrix3x2f>,
        use_gdi_natural: bool,
        glyph_indices: &[u16],
        is_sideways: bool,
    ) -> Result<Vec<GlyphMetrics>, Error> {
        unsafe {
            let mut metrics = vec![mem::uninitialized(); glyph_indices.len()];
            let hr = self.raw_fontface().GetGdiCompatibleGlyphMetrics(
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
    fn gdi_compatible_metrics(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        transform: Option<&Matrix3x2f>,
    ) -> Result<FontMetrics, Error> {
        unsafe {
            let mut metrics = mem::uninitialized();
            let hr = self.raw_fontface().GetGdiCompatibleMetrics(
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
    fn font_table<'a>(&'a self, tag: FontFeatureTag) -> Option<FontTable<'a>> {
        unsafe {
            let mut data = ptr::null();
            let mut size = 0;
            let mut context = ptr::null_mut();
            let mut exists = 0;

            let hr = self.raw_fontface().TryGetFontTable(
                tag.0,
                &mut data,
                &mut size,
                &mut context,
                &mut exists,
            );

            if SUCCEEDED(hr) && exists != 0 {
                let data = std::slice::from_raw_parts(data as *const u8, size as usize);

                Some(FontTable {
                    face: &self.raw_fontface(),
                    context,
                    data,
                })
            } else {
                None
            }
        }
    }

    fn as_font_face(&self) -> FontFace {
        unsafe {
            let ptr = self.raw_fontface();
            ptr.AddRef();
            FontFace::from_raw(ptr as *const _ as *mut _)
        }
    }

    unsafe fn raw_fontface(&self) -> &IDWriteFontFace;
}

unsafe impl IFontFace for FontFace {
    unsafe fn raw_fontface(&self) -> &IDWriteFontFace {
        &self.ptr
    }
}
