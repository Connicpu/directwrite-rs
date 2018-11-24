//! Typography and types for building new ones.

use descriptions::FontFeature;
use factory::Factory;

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteTypography;
use wio::com::ComPtr;

#[doc(inline)]
pub use self::builder::TypographyBuilder;

#[doc(hidden)]
pub mod builder;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync)]
/// Represents a font typography setting.
pub struct Typography {
    ptr: ComPtr<IDWriteTypography>,
}

impl Typography {
    /// Create a new Typography object
    pub fn create(factory: &Factory) -> TypographyBuilder {
        TypographyBuilder::new(factory)
    }

    /// Get the number of font features included in this typography object.
    pub fn feature_count(&self) -> u32 {
        unsafe { self.ptr.GetFontFeatureCount() }
    }

    /// Get the font feature at the specified index.
    pub fn feature(&self, index: u32) -> Option<FontFeature> {
        unsafe {
            let mut feature = std::mem::zeroed();
            let hr = self.ptr.GetFontFeature(index, &mut feature);
            if SUCCEEDED(hr) {
                Some(feature.into())
            } else {
                None
            }
        }
    }

    /// Get an iterator over all of the features stored in this typography object.
    pub fn all_features<'a>(&'a self) -> impl Iterator<Item = FontFeature> + 'a {
        (0..self.feature_count()).filter_map(move |i| self.feature(i))
    }
}

impl std::fmt::Debug for Typography {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        struct DebugFeatures<'a>(&'a Typography);
        impl<'a> std::fmt::Debug for DebugFeatures<'a> {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                fmt.debug_map()
                    .entries(self.0.all_features().map(|f| (f.name_tag, f.parameter)))
                    .finish()
            }
        }
        fmt.debug_struct(stringify!(Typography))
            .field("features", &DebugFeatures(self))
            .finish()
    }
}
