use enums::font_simulations::FontSimulations;
use enums::font_stretch::FontStretch;
use enums::font_style::FontStyle;
use enums::font_weight::FontWeight;
use enums::InformationalStringId;
use error::DWResult;
use font_face::FontFace;
use font_family::FontFamily;
use localized_strings::LocalizedStrings;
use metrics::font::FontMetrics;

use std::mem;
use std::ptr;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFont;
use wio::com::ComPtr;

#[derive(ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// Represents a physical font in a font collection. This interface is used to
/// create font faces from physical fonts, or to retrieve information such as
/// font face metrics or face names from existing font faces.
pub struct Font {
    ptr: ComPtr<IDWriteFont>,
}

impl Font {
    /// Creates a font face object for the font.
    pub fn create_face(&self) -> DWResult<FontFace> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.CreateFontFace(&mut ptr);
            if SUCCEEDED(hr) {
                Ok(FontFace::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    /// It is unclear in what situations this method may fail to return a face
    /// names collection, and so is returned as an Option to be safe.
    pub fn face_name(&self) -> Option<LocalizedStrings> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetFaceNames(&mut ptr);
            if SUCCEEDED(hr) {
                Some(LocalizedStrings::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Gets the font family to which the specified font belongs.
    pub fn font_family(&self) -> Option<FontFamily> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let hr = self.ptr.GetFontFamily(&mut ptr);
            if SUCCEEDED(hr) {
                Some(FontFamily::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Gets a localized strings collection containing the specified
    /// informational strings, indexed by locale name.
    pub fn informational_strings(&self, id: InformationalStringId) -> Option<LocalizedStrings> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let mut exists = 0;
            let hr = self
                .ptr
                .GetInformationalStrings(id as u32, &mut ptr, &mut exists);
            if SUCCEEDED(hr) && exists != 0 {
                Some(LocalizedStrings::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Get metric information for this Font.
    pub fn metrics(&self) -> FontMetrics {
        unsafe {
            let mut metrics = mem::uninitialized();
            self.ptr.GetMetrics(&mut metrics);
            metrics.into()
        }
    }

    /// Get simulations applied to this Font.
    pub fn simulations(&self) -> FontSimulations {
        unsafe { FontSimulations(self.ptr.GetSimulations()) }
    }

    /// Get the stretch value of this Font.
    pub fn stretch(&self) -> UncheckedEnum<FontStretch> {
        unsafe { self.ptr.GetStretch().into() }
    }

    /// Get the style of this Font (Norma, Oblique, Italic).
    pub fn style(&self) -> UncheckedEnum<FontStyle> {
        unsafe { self.ptr.GetStyle().into() }
    }

    /// Get the weight of this Font.
    pub fn weight(&self) -> FontWeight {
        unsafe { FontWeight(self.ptr.GetWeight()) }
    }

    /// Check if a unicode codepoint is supported by this Font.
    pub fn has_character(&self, c: char) -> bool {
        unsafe {
            let mut exists = 0;
            let hr = self.ptr.HasCharacter(c as u32, &mut exists);
            SUCCEEDED(hr) && exists != 0
        }
    }

    /// Determines if this Font is a "Symbol" Font.
    pub fn is_symbol_font(&self) -> bool {
        unsafe { self.ptr.IsSymbolFont() != 0 }
    }
}
