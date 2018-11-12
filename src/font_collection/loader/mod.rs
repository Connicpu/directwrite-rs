use error::DWResult;
use factory::Factory;
use font_file::FontFile;
use key::FontKey;

#[doc(inline)]
pub use font_collection::loader::handle::CollectionLoaderHandle;

#[doc(hidden)]
pub mod handle;

pub(crate) mod com_enumerator;
pub(crate) mod com_loader;

pub trait FontCollectionLoader: Send + Sync + 'static {
    type Key: FontKey + ?Sized;
    type Iter: Iterator<Item = DWResult<FontFile>> + 'static;

    fn get_iterator(&self, factory: &Factory, key: &Self::Key) -> DWResult<Self::Iter>;

    fn register(self, factory: &Factory) -> DWResult<CollectionLoaderHandle<Self::Key>>
    where
        Self: Sized,
    {
        CollectionLoaderHandle::register(factory, self)
    }
}
