use error::DWResult;
use descriptions::key::FontKey;
use font_file::loader::{FontFileLoader, FontFileStream};

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use winapi::shared::winerror::{HRESULT_FROM_WIN32, ERROR_FILE_NOT_FOUND};

/// Represents a loader from a set of preloaded streams which may be cloned.
pub struct SetLoader<K, S, Key>
where
    K: Hash + Eq + Borrow<Key> + Send + Sync + 'static,
    S: FontFileStream + Clone,
    Key: FontKey + Hash + Eq + ?Sized,
{
    /// The streams from which this loader loads requested resources.
    pub streams: HashMap<K, S>,
    _marker: PhantomData<Key>,
}

impl<K, S, Key> SetLoader<K, S, Key>
where
    K: Hash + Eq + Borrow<Key> + Send + Sync + 'static,
    S: FontFileStream + Clone,
    Key: FontKey + Hash + Eq + ?Sized,
{
    /// Initialize the loader from a set of streams.
    pub fn new(streams: HashMap<K, S>) -> Self {
        SetLoader {
            streams,
            _marker: PhantomData,
        }
    }
}

impl<K, S, Key> FontFileLoader for SetLoader<K, S, Key>
where
    K: Hash + Eq + Borrow<Key> + Send + Sync + 'static,
    S: FontFileStream + Clone,
    Key: FontKey + Hash + Eq + ?Sized,
{
    type Key = Key;
    type Stream = S;

    fn create_stream(&self, key: &Key) -> DWResult<S> {
        match self.streams.get(key) {
            Some(stream) => Ok(stream.clone()),
            None => Err(HRESULT_FROM_WIN32(ERROR_FILE_NOT_FOUND).into())
        }
    }
}
