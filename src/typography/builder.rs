use crate::descriptions::FontFeature;
use crate::enums::FontFeatureTag;
use crate::error::DWResult;
use crate::factory::Factory;
use crate::typography::Typography;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;

/// Builds a Typography object with the listed font features.
pub struct TypographyBuilder<'a> {
    factory: &'a Factory,
    features: FeatureList<'a>,
}

enum FeatureList<'a> {
    Empty,
    Slice(&'a [FontFeature]),
    Owned(Vec<FontFeature>),
    Sublists(Vec<FeatureList<'a>>),
}

impl<'a> TypographyBuilder<'a> {
    pub(crate) fn new(factory: &'a Factory) -> Self {
        TypographyBuilder {
            factory,
            features: FeatureList::Empty,
        }
    }

    /// Build the typography.
    pub fn build(self) -> DWResult<Typography> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            let hr = (*self.factory.get_raw()).CreateTypography(&mut ptr);
            if SUCCEEDED(hr) {
                self.features.for_all(|f| (*ptr).AddFontFeature(f.into()))?;
                Ok(Typography::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }

    /// Add a font feature to the builder.
    pub fn with_feature(self, tag: impl Into<FontFeatureTag>, param: u32) -> Self {
        self.with_feature_desc(FontFeature {
            name_tag: tag.into(),
            parameter: param,
        })
    }

    /// Add a font feature to the builder.
    pub fn with_feature_desc(mut self, feature: FontFeature) -> Self {
        self.features.push(feature);
        self
    }

    /// Add a list of font features to the builder.
    pub fn with_features(mut self, features: &'a [FontFeature]) -> Self {
        self.features.push_slice(features);
        self
    }
}

impl<'a> FeatureList<'a> {
    fn push(&mut self, item: FontFeature) {
        *self = match std::mem::replace(self, FeatureList::Empty) {
            FeatureList::Empty => Self::empty_owned(),
            old @ FeatureList::Slice(_) => Self::list_with_owned(old),
            old @ FeatureList::Owned(_) => old,
            FeatureList::Sublists(mut lists) => {
                lists.push(Self::empty_owned());
                FeatureList::Sublists(lists)
            }
        };

        match self {
            FeatureList::Owned(vec) => vec.push(item),
            FeatureList::Sublists(lists) => match lists.last_mut() {
                Some(FeatureList::Owned(vec)) => vec.push(item),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn push_slice(&mut self, slice: &'a [FontFeature]) {
        *self = match std::mem::replace(self, FeatureList::Empty) {
            FeatureList::Empty => Self::slice(slice),
            old @ FeatureList::Slice(_) => Self::list_with_slice(old, slice),
            old @ FeatureList::Owned(_) => Self::list_with_slice(old, slice),
            FeatureList::Sublists(mut lists) => {
                lists.push(Self::slice(slice));
                FeatureList::Sublists(lists)
            }
        };
    }

    fn empty_owned() -> Self {
        FeatureList::Owned(Vec::with_capacity(4))
    }

    fn slice(slice: &'a [FontFeature]) -> Self {
        FeatureList::Slice(slice)
    }

    fn list_with_owned(item: Self) -> Self {
        FeatureList::Sublists(vec![item, Self::empty_owned()])
    }

    fn list_with_slice(item: Self, slice: &'a [FontFeature]) -> Self {
        FeatureList::Sublists(vec![item, Self::slice(slice)])
    }

    fn for_all(&self, mut f: impl FnMut(FontFeature) -> i32) -> DWResult<()> {
        self.for_all_imp(&mut f)
    }

    fn for_all_imp(&self, f: &mut impl FnMut(FontFeature) -> i32) -> DWResult<()> {
        match self {
            FeatureList::Empty => (),
            FeatureList::Slice(features) => {
                for &feature in *features {
                    let hr = f(feature);
                    if !SUCCEEDED(hr) {
                        return Err(hr.into());
                    }
                }
            }
            FeatureList::Owned(features) => {
                for &feature in features {
                    let hr = f(feature);
                    if !SUCCEEDED(hr) {
                        return Err(hr.into());
                    }
                }
            }
            FeatureList::Sublists(lists) => {
                for list in lists {
                    list.for_all_imp(f)?;
                }
            }
        }
        Ok(())
    }
}
