use crate::descriptions::key::FontKey;
use crate::factory::Factory;
use crate::font_collection::loader::FontCollectionLoader;
use crate::font_file::loader::handle::FileLoaderHandle;
use crate::font_file::FontFile;

use std::borrow::Borrow;
use std::sync::Arc;

use dcommon::Error;

/// A FontCollection loader that loads FontFiles from a predefined list of keys
/// which key passed to a loader.
pub struct KeyLoader<K, IK>
where
    K: FontKey + Borrow<IK>,
    IK: FontKey + ?Sized,
{
    /// The list of keys which are passed to the file loader.
    pub keys: Arc<[K]>,

    /// The handle to the file loader.
    pub file_source: FileLoaderHandle<IK>,
}

impl<K, IK> KeyLoader<K, IK>
where
    K: FontKey + Borrow<IK>,
    IK: FontKey + ?Sized,
{
    /// Initialize a new KeyLoader from its parts.
    pub fn new(keys: impl Into<Arc<[K]>>, file_source: FileLoaderHandle<IK>) -> Self {
        let keys = keys.into();
        KeyLoader { keys, file_source }
    }
}

impl<K, IK> FontCollectionLoader for KeyLoader<K, IK>
where
    K: FontKey + Borrow<IK>,
    IK: FontKey + ?Sized,
{
    /// This collection loader does not have a key. It only loads 1 collection.
    type Key = ();
    type Iter = KeyLoaderIter<K, IK>;

    fn get_iterator(&self, factory: &Factory, _key: &()) -> Result<Self::Iter, Error> {
        Ok(KeyLoaderIter {
            pos: 0,
            keys: self.keys.clone(),
            source: self.file_source.clone(),
            factory: factory.clone(),
        })
    }
}

pub struct KeyLoaderIter<K, IK>
where
    K: FontKey + Borrow<IK>,
    IK: FontKey + ?Sized,
{
    pos: usize,
    keys: Arc<[K]>,
    source: FileLoaderHandle<IK>,
    factory: Factory,
}

impl<K, IK> Iterator for KeyLoaderIter<K, IK>
where
    K: FontKey + Borrow<IK>,
    IK: FontKey + ?Sized,
{
    type Item = Result<FontFile, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.keys.len() {
            return None;
        }

        let key = self.keys[self.pos].borrow();
        let result = FontFile::create(&self.factory)
            .with_loader(&self.source)
            .with_key(key)
            .build();

        Some(result)
    }
}
