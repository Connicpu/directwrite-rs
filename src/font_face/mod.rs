use enums::{FontFaceType, FontSimulations};
use error::DWResult;
use factory::Factory;
use font_file::FontFile;
use helpers::InternalConstructor;

use std::{mem, ptr, u32};

use winapi::shared::winerror::{E_UNEXPECTED, SUCCEEDED};
use winapi::um::dwrite::{DWRITE_GLYPH_OFFSET, DWRITE_MATRIX, IDWriteFontFace, IDWriteGeometrySink};
use wio::com::ComPtr;

pub use self::builder::FontFaceBuilder;
pub use self::metrics::FontMetrics;
pub use self::metrics::GlyphMetrics;
pub mod builder;
pub mod metrics;

pub struct FontFace {
    ptr: ComPtr<IDWriteFontFace>,
}

impl FontFace {
    pub fn create(factory: &Factory) -> FontFaceBuilder {
        unsafe { FontFaceBuilder::new(&*factory.get_raw()) }
    }
    pub unsafe fn from_raw(raw: *mut IDWriteFontFace) -> Self {
        FontFace {
            ptr: ComPtr::from_raw(raw),
        }
    }

    /// Obtains ideal (resolution-independent) glyph metrics in font design units. 
    pub fn get_design_glyph_metrics(&self, glyph_indices: &[u16], is_sideways: bool) -> DWResult<Vec<GlyphMetrics>> {
        unsafe {
            let mut metrics = vec![mem::uninitialized(); glyph_indices.len()];
            let hr = self.ptr.GetDesignGlyphMetrics(glyph_indices.as_ptr(), glyph_indices.len() as u32, metrics.as_mut_ptr(), 
                                                    if is_sideways{ 1 } else{ 0 });
            if SUCCEEDED(hr) {
                Ok(metrics.iter().map(|m| GlyphMetrics::build(*m)).collect())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains the font files representing a font face.
    pub fn get_files(&self) -> DWResult<Vec<FontFile>> {
        unsafe {
            let mut count = 0;
            let mut hr = self.ptr.GetFiles(&mut count, ptr::null_mut());
            if SUCCEEDED(hr) {
                let mut native_files = vec![ptr::null_mut(); count as usize];
                hr = self.ptr.GetFiles(&mut count, native_files.as_mut_ptr());
                if SUCCEEDED(hr){
                    Ok(native_files.iter().map(|ptr| FontFile::from_raw(*ptr) ).collect())
                } else {
                    Err(hr.into())
                }
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains glyph metrics in font design units with the return values compatible with what GDI would produce.
    pub fn get_gdi_compatible_glyph_metrics(&self, em_size: f32, pixels_per_dip: f32, transform: Option<&DWRITE_MATRIX>,
                                             use_gdi_natural: bool, glyph_indices: &[u16], is_sideways: bool) -> DWResult<Vec<GlyphMetrics>> {
        unsafe {
            let mut metrics = vec![mem::uninitialized(); glyph_indices.len()];
            let hr = self.ptr.GetGdiCompatibleGlyphMetrics(em_size, pixels_per_dip, 
                                                            match transform{ Some(x) => { x } None => { ptr::null() }},
                                                            if use_gdi_natural{ 1 } else{ 0 }, 
                                                            glyph_indices.as_ptr(), glyph_indices.len() as u32, metrics.as_mut_ptr(), 
                                                            if is_sideways{ 1 } else{ 0 });
            if SUCCEEDED(hr) {
                Ok(metrics.iter().map(|m| GlyphMetrics::build(*m)).collect())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains design units and common metrics for the font face. 
    /// These metrics are applicable to all the glyphs within a fontface and are used by applications for layout calculations.
    pub fn get_gdi_compatible_metrics(&self, em_size: f32, pixels_per_dip: f32, transform: Option<&DWRITE_MATRIX>) -> DWResult<FontMetrics> {
        unsafe {
            let mut metrics = mem::uninitialized();
            let hr = self.ptr.GetGdiCompatibleMetrics(em_size, pixels_per_dip, match transform{ Some(x) => { x } None => { ptr::null() }}, &mut metrics);
            if SUCCEEDED(hr) {
                Ok(FontMetrics::build(metrics))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains the number of glyphs in the font face. 
    pub fn get_glyph_count(&self) -> u16{
        unsafe {
            self.ptr.GetGlyphCount()
        }
    }

    /// Returns the nominal mapping of UCS4 Unicode code points to glyph indices as defined by the font 'CMAP' table. 
    pub fn get_glyph_indices(&self, code_points: &[u32]) -> DWResult<Vec<u16>> {
        unsafe {
            let mut indices : Vec<u16> = Vec::with_capacity(code_points.len());
            let hr = self.ptr.GetGlyphIndices(code_points.as_ptr(), code_points.len() as u32, indices.as_mut_ptr());
            if SUCCEEDED(hr) {
                Ok(indices)
            } else {
                Err(hr.into())
            }
        }
    }

    /// Computes the outline of a run of glyphs by calling back to the outline sink interface.
    /// If glyph_advances and/or glyph_offsets are provided, they must be the same length as glyph_indices, or the function will panic.
    pub fn get_glyph_run_outline(&self, em_size: f32, glyph_indices: &[u16], glyph_advances: Option<&[f32]>,
                                   glyph_offsets: Option<&[DWRITE_GLYPH_OFFSET]>, is_sideways: bool,
                                   is_rtl: bool, geometry_sink: *mut IDWriteGeometrySink) -> DWResult<()> {
        unsafe {
            assert!(glyph_advances.is_none() || glyph_advances.unwrap().len() == glyph_indices.len());
            assert!(glyph_offsets.is_none() || glyph_offsets.unwrap().len() == glyph_indices.len());
            let hr = self.ptr.GetGlyphRunOutline(em_size, glyph_indices.as_ptr(), 
                                                  match glyph_advances{ Some(g) => { g.as_ptr() } None => { ptr::null() }},
                                                  match glyph_offsets{ Some(g) => { g.as_ptr() } None => { ptr::null() }},
                                                  glyph_indices.len() as u32, if is_sideways{ 1 } else { 0 },
                                                  if is_rtl{ 1 } else { 0 }, geometry_sink);
            if SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(hr.into())
            }
        }
    }

    /// Obtains the index of a font face in the context of its font files.
    pub fn get_index(&self) -> u32 {
        unsafe {
            self.ptr.GetIndex()
        }
    }

    /// Obtains design units and common metrics for the font face. 
    /// These metrics are applicable to all the glyphs within a font face 
    /// and are used by applications for layout calculations. 
    pub fn get_metrics(&self) -> FontMetrics {
        unsafe {
            let mut metrics = mem::uninitialized();
            self.ptr.GetMetrics(&mut metrics);
            FontMetrics::build(metrics)
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFontFace {
        self.ptr.as_raw()
    }

    /// Obtains the file format type of a font face. 
    pub fn get_type(&self) -> FontFaceType {
        unsafe {
            FontFaceType::from_u32(self.ptr.GetType()).unwrap()
        }
    }

    /// Obtains the algorithmic style simulation flags of a font face.
    pub fn get_simulations(&self) -> DWResult<FontSimulations> {
        unsafe {
            FontSimulations::from_u32(self.ptr.GetSimulations()).ok_or(E_UNEXPECTED.into())
        }
    }

    /// Determines whether the font is a symbol font.
    pub fn is_symbol_font(&self) -> bool {
        unsafe {
            self.ptr.IsSymbolFont() > 0
        }
    }
}

unsafe impl Send for FontFace {}
unsafe impl Sync for FontFace {}
