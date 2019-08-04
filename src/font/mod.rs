use crate::enums::font_simulations::FontSimulations;
use crate::enums::font_stretch::FontStretch;
use crate::enums::font_style::FontStyle;
use crate::enums::font_weight::FontWeight;
use crate::enums::InformationalStringId;
use crate::font_face::FontFace;
use crate::font_family::FontFamily;
use crate::localized_strings::LocalizedStrings;
use crate::metrics::font::FontMetrics;

use std::mem::MaybeUninit;

use checked_enum::UncheckedEnum;
use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFont;
use wio::com::ComPtr;

#[derive(Clone, ComWrapper, PartialEq)]
#[com(send, sync, debug)]
#[repr(transparent)]
/// Represents a physical font in a font collection. This interface is used to
/// create font faces from physical fonts, or to retrieve information such as
/// font face metrics or face names from existing font faces.
pub struct Font {
    ptr: ComPtr<IDWriteFont>,
}

pub unsafe trait IFont {
    /// Creates a font face object for the font.
    fn create_face(&self) -> Result<FontFace, Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_font().CreateFontFace(&mut ptr);
            if SUCCEEDED(hr) {
                Ok(FontFace::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    /// It is unclear in what situations this method may fail to return a face
    /// names collection, and so is returned as an Option to be safe.
    fn face_name(&self) -> Option<LocalizedStrings> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_font().GetFaceNames(&mut ptr);
            if SUCCEEDED(hr) {
                Some(LocalizedStrings::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Gets the font family to which the specified font belongs.
    fn font_family(&self) -> Option<FontFamily> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = self.raw_font().GetFontFamily(&mut ptr);
            if SUCCEEDED(hr) {
                Some(FontFamily::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Gets a localized strings collection containing the specified
    /// informational strings, indexed by locale name.
    fn informational_strings(&self, id: InformationalStringId) -> Option<LocalizedStrings> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let mut exists = 0;
            let hr = self
                .raw_font()
                .GetInformationalStrings(id as u32, &mut ptr, &mut exists);
            if SUCCEEDED(hr) && exists != 0 {
                Some(LocalizedStrings::from_raw(ptr))
            } else {
                None
            }
        }
    }

    /// Get metric information for this Font.
    fn metrics(&self) -> FontMetrics {
        unsafe {
            let mut metrics = MaybeUninit::uninit();
            self.raw_font().GetMetrics(metrics.as_mut_ptr());
            metrics.assume_init().into()
        }
    }

    /// Get simulations applied to this Font.
    fn simulations(&self) -> FontSimulations {
        unsafe { FontSimulations(self.raw_font().GetSimulations()) }
    }

    /// Get the stretch value of this Font.
    fn stretch(&self) -> UncheckedEnum<FontStretch> {
        unsafe { self.raw_font().GetStretch().into() }
    }

    /// Get the style of this Font (Norma, Oblique, Italic).
    fn style(&self) -> UncheckedEnum<FontStyle> {
        unsafe { self.raw_font().GetStyle().into() }
    }

    /// Get the weight of this Font.
    fn weight(&self) -> FontWeight {
        unsafe { FontWeight(self.raw_font().GetWeight()) }
    }

    /// Check if a unicode codepoint is supported by this Font.
    fn has_character(&self, c: char) -> bool {
        unsafe {
            let mut exists = 0;
            let hr = self.raw_font().HasCharacter(c as u32, &mut exists);
            SUCCEEDED(hr) && exists != 0
        }
    }

    /// Determines if this Font is a "Symbol" Font.
    fn is_symbol_font(&self) -> bool {
        unsafe { self.raw_font().IsSymbolFont() != 0 }
    }

    unsafe fn raw_font(&self) -> &IDWriteFont;
}

unsafe impl IFont for Font {
    unsafe fn raw_font(&self) -> &IDWriteFont {
        &self.ptr
    }
}
