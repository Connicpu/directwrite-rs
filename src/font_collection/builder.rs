use error::DWResult;
use factory::Factory;
use font_collection::loader::CollectionLoaderHandle;
use font_collection::FontCollection;
use key::{FontKey, KeyPayload};

use std::mem;
use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::winerror::SUCCEEDED;

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
    pub fn new(factory: &'a Factory) -> Self {
        FontCollectionBuilder {
            factory,
            loader: None,
            key: None,
        }
    }

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

    pub fn with_loader(mut self, loader: &'a CollectionLoaderHandle<K>) -> Self {
        self.loader = Some(loader);
        self
    }

    pub fn with_key(mut self, key: &'a K) -> Self {
        self.key = Some(key);
        self
    }
}
