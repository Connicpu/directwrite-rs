//! Types related to loading custom font collections.

use descriptions::FontKey;
use error::DWResult;
use factory::Factory;
use font_file::FontFile;

#[doc(inline)]
pub use font_collection::loader::handle::CollectionLoaderHandle;

#[doc(hidden)]
pub mod handle;

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
    type Iter: Iterator<Item = DWResult<FontFile>> + 'static;

    /// Called by the runtime to request an enumerator of the font files that are to
    /// be a part of the collection identified by the `key`.
    fn get_iterator(&self, factory: &Factory, key: &Self::Key) -> DWResult<Self::Iter>;

    /// Shortcut method to more easily register your font loader and get its handle.
    fn register(self, factory: &Factory) -> DWResult<CollectionLoaderHandle<Self::Key>>
    where
        Self: Sized,
    {
        CollectionLoaderHandle::register(factory, self)
    }
}
