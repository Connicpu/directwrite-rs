//! Types related to loading custom font collections.

use crate::descriptions::FontKey;
use crate::factory::Factory;
use crate::font_file::FontFile;

use dcommon::Error;

#[doc(inline)]
pub use crate::font_collection::loader::handle::CollectionLoaderHandle;
#[doc(inline)]
pub use crate::font_collection::loader::key_loader::KeyLoader;

#[doc(hidden)]
pub mod handle;
#[doc(hidden)]
pub mod key_loader;

pub(crate) mod com_enumerator;
pub(crate) mod com_loader;

/// User-defined loader for a FontCollection. This type is responsible for loading the
/// FontFiles that make each FontCollection it loads.
pub trait FontCollectionLoader: Send + Sync + 'static {
    /// The key used to identify each collection that can be loaded
    type Key: FontKey + ?Sized;

    /// Fonts are loaded by DirectWrite from an Iterator of FontFiles. Some Day™ this may
    /// be replaced with an `impl Iterator` but for now it has to be specified. If you want
    /// to just use closures, make it a `Box<Iterator>`. Performance here isn't particularly
    /// important.
    type Iter: Iterator<Item = Result<FontFile, Error>> + 'static;

    /// Called by the runtime to request an enumerator of the font files that are to
    /// be a part of the collection identified by the `key`.
    fn get_iterator(&self, factory: &Factory, key: &Self::Key) -> Result<Self::Iter, Error>;

    /// Shortcut method to more easily register your font loader and get its handle.
    fn register(self, factory: &Factory) -> Result<CollectionLoaderHandle<Self::Key>, Error>
    where
        Self: Sized,
    {
        CollectionLoaderHandle::register(factory, self)
    }
}
