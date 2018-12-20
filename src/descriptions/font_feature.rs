use crate::enums::font_feature_tag::FontFeatureTag;

use winapi::um::dwrite::DWRITE_FONT_FEATURE;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Specifies properties used to identify and execute typographic features in the current font face.
///
/// ### Remarks
///
/// The OpenType standard provides access to typographic features available in the font by
/// means of a feature tag with the associated parameters. The OpenType feature tag is a
/// 4-byte identifier of the registered name of a feature. For example, the 'kern' feature
/// name tag is used to identify the 'Kerning' feature in OpenType font. Similarly, the
/// OpenType feature tag for 'Standard Ligatures' and 'Fractions' is 'liga' and 'frac'
/// respectively. Since a single run can be associated with more than one typographic features,
/// the Text String API accepts typographic settings for a run as a list of features and are
/// executed in the order they are specified.
///
/// The value of the tag member represents the OpenType name tag of the feature, while the
/// param value represents additional parameter for the execution of the feature referred by
/// the tag member. Both nameTag and parameter are stored as little endian, the same convention
/// followed by GDI. Most features treat the Param value as a binary value that indicates
/// whether to turn the execution of the feature on or off, with it being off by default in the
/// majority of cases. Some features, however, treat this value as an integral value
/// representing the integer index to the list of alternate results it may produce during the
/// execution; for instance, the feature 'Stylistic Alternates' or 'salt' uses the parameter
/// value as an index to the list of alternate substituting glyphs it could produce for a
/// specified glyph.
pub struct FontFeature {
    /// The feature's OpenType name identifier.
    pub name_tag: FontFeatureTag,

    /// The execution parameter of the feature.
    ///
    /// A non-zero value generally enables the feature execution, while the zero value disables it.
    /// A feature requiring a selector uses this value to indicate the selector index.
    pub parameter: u32,
}

#[cfg(test)]
dcommon::member_compat_test! {
    font_feature_compat:
    FontFeature <=> DWRITE_FONT_FEATURE {
        name_tag <=> nameTag,
        parameter <=> parameter,
    }
}

impl From<FontFeature> for DWRITE_FONT_FEATURE {
    fn from(feature: FontFeature) -> DWRITE_FONT_FEATURE {
        DWRITE_FONT_FEATURE {
            nameTag: feature.name_tag.0,
            parameter: feature.parameter,
        }
    }
}

impl From<DWRITE_FONT_FEATURE> for FontFeature {
    fn from(feature: DWRITE_FONT_FEATURE) -> FontFeature {
        FontFeature {
            name_tag: feature.nameTag.into(),
            parameter: feature.parameter,
        }
    }
}
