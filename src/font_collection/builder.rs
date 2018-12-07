use crate::descriptions::{FontKey, KeyPayload};
use crate::error::DWResult;
use crate::factory::Factory;
use crate::font_collection::loader::CollectionLoaderHandle;
use crate::font_collection::FontCollection;

use std::mem;
use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;

#[must_use]
/// Builder for a FontCollection
///
/// `loader` and `key` are both required.
pub struct FontCollectionBuilder<'a, K>
where
    K: FontKey + ?Sized,
{
    factory: &'a Factory,
    loader: Option<&'a CollectionLoaderHandle<K>>,
    key: Option<&'a K>,
}

impl<'a, K> FontCollectionBuilder<'a, K>
where
    K: FontKey + ?Sized,
{
    pub(super) fn new(factory: &'a Factory) -> Self {
        FontCollectionBuilder {
            factory,
            loader: None,
            key: None,
        }
    }

    /// Finalize the builder, attempting to create the FontCollection with the
    /// specified parameters.
    pub fn build(self) -> DWResult<FontCollection> {
        let loader = self.loader.expect("Font Loader must be specified");
        let key = KeyPayload::new(self.key.expect("Key must be specified"));

        unsafe {
            let f = &*self.factory.get_raw();

            let mut ptr = ptr::null_mut();
            let hr = f.CreateCustomFontCollection(
                loader.get_raw(),
                &key as *const _ as *const _,
                mem::size_of_val(&key) as u32,
                &mut ptr,
            );

            if SUCCEEDED(hr) {
                Ok(FontCollection::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }
}

impl<'a, K> FontCollectionBuilder<'a, K>
where
    K: FontKey + ?Sized,
{
    /// Specify the collection loader that should be used in creating this collection
    pub fn with_loader(mut self, loader: &'a CollectionLoaderHandle<K>) -> Self {
        self.loader = Some(loader);
        self
    }

    /// Specify the key passed to the collection. This is required.
    pub fn with_key(mut self, key: &'a K) -> Self {
        self.key = Some(key);
        self
    }
}
