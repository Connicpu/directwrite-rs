//! Types and traits used for implementing the loading of custom FontFiles.
//!
//! # Example
//!
//! ```
//! # extern crate directwrite; extern crate winapi;
//! # fn main() {
//! # use directwrite::font_file::loader::{StaticDataStream, FontFileLoader};
//! # use directwrite::error::DWResult;
//! # use directwrite::{Factory, FontFile};
//! # use winapi::shared::winerror::{HRESULT_FROM_WIN32, ERROR_NOT_FOUND};
//! const OPENSANS_REGULAR: StaticDataStream = StaticDataStream {
//!     // Sunday, November 11, 2018 18:30:45
//!     last_modified: 636775578456076107,
//! # /*
//!     data: include_bytes!("OpenSans-Regular.ttf"),
//! # */
//! #   data: include_bytes!("../../../tests/test_fonts/OpenSans-Regular.ttf"),
//! };
//!
//! pub struct DataFileLoader;
//! impl FontFileLoader for DataFileLoader {
//!     type Key = str;
//!     type Stream = StaticDataStream;
//!
//!     fn create_stream(&self, key: &str) -> DWResult<StaticDataStream> {
//!         match key {
//!             "OpenSans-Regular" => Ok(OPENSANS_REGULAR),
//!             _ => Err(HRESULT_FROM_WIN32(ERROR_NOT_FOUND).into()),
//!         }
//!     }
//! }
//!
//! let factory = Factory::new().unwrap();
//! let file_loader = DataFileLoader.register(&factory).unwrap();
//!
//! let file = FontFile::create(&factory)
//!     .with_loader(&file_loader)
//!     .with_key("OpenSans-Regular")
//!     .build()
//!     .unwrap();
//! # let _ = file;
//! # }
//! ```

use descriptions::FontKey;
use error::DWResult;
use factory::Factory;

use std::fs::Metadata;
use std::time::UNIX_EPOCH;

#[doc(inline)]
pub use font_file::loader::file_stream::FileStream;
#[doc(inline)]
pub use font_file::loader::handle::FileLoaderHandle;
#[doc(inline)]
pub use font_file::loader::mmap_stream::MmapStream;
#[doc(inline)]
pub use font_file::loader::owned_stream::OwnedDataStream;
#[doc(inline)]
pub use font_file::loader::set_loader::SetLoader;
#[doc(inline)]
pub use font_file::loader::shared_stream::SharedDataStream;
#[doc(inline)]
pub use font_file::loader::static_stream::StaticDataStream;

pub(crate) mod com_loader;
pub(crate) mod com_stream;

#[doc(hidden)]
pub mod file_loader;
#[doc(hidden)]
pub mod file_stream;
#[doc(hidden)]
pub mod handle;
#[doc(hidden)]
pub mod mmap_stream;
#[doc(hidden)]
pub mod owned_stream;
#[doc(hidden)]
pub mod set_loader;
#[doc(hidden)]
pub mod shared_stream;
#[doc(hidden)]
pub mod static_stream;

/// Facilitates the loading of FontFiles by giving streams to the
/// runtime for custom font files.
pub trait FontFileLoader: Sized + Send + Sync + 'static {
    /// The Key data used to identify the file that is to be loaded
    type Key: FontKey + ?Sized;

    /// The type of streams created by this Loader. If you need dynamic
    /// dispatch, use `Box<FontFileStream>`.
    type Stream: FontFileStream;

    /// Try to create a stream for a file associated with the given key.
    fn create_stream(&self, key: &Self::Key) -> DWResult<Self::Stream>;

    /// Register this file loader in the factory
    fn register(self, factory: &Factory) -> DWResult<FileLoaderHandle<Self::Key>>
    where
        Self: Sized,
    {
        FileLoaderHandle::register(factory, self)
    }
}

/// Describes the value that should be returned from `last_write_time` if the file was
/// last written on the unix epoch.
pub const UNIX_EPOCH_IN_WRITE_TIME: u64 = 621355968000000000;

/// Represents a class for loading the data of a font file so that
/// the runtime can construct a FontFile from it.
pub trait FontFileStream: Sized + Send + Sync + 'static {
    /// The number of bytes in the file.
    fn file_size(&self) -> u64;

    /// The last time the file was modified in 100-nanosecond intervals since
    /// January 1, 1601 (UTC).
    ///
    /// The "last modified time" is used by DirectWrite font selection algorithms to determine
    /// whether one font resource is more up to date than another one.
    fn last_write_time(&self) -> u64;

    /// Called by the runtime to request access to a region of the font file
    /// as an array of bytes. It is up to the implementation of this trait to
    /// manage the memory used for these requests. `release_fragment` will be
    /// called when DirectWrite no longer needs access to the specific region
    /// of memory again.
    ///
    /// ### Unsafe Considerations
    ///
    /// It's up to the implementation to ensure that the pointer returned in the Fragment
    /// points to at least `length` memory. If the request for that length of bytes cannot
    /// be fulfilled, the implementation *must* return an Error.
    ///
    /// The data pointer returned in the fragment should last at least
    /// until one of the following events occurs:
    ///  - `release_fragment` is called with the key returned in the `Fragment`
    ///  - DirectWrite releases its last handle on this `FontFileStream` (`Drop`)
    ///
    /// Implementations are free to keep the memory alive beyond this point, but
    /// it is recommended to free all resources when the last handle is released to
    /// avoid memory leaks.
    ///
    /// ### Threading Considerations
    ///
    /// Because DirectWrite may invoke FontFileStream methods on the same object from multiple
    /// threads simultaneously, this method only takes a shared self. Any internal mutable state
    /// must be protected by a Mutex or similar mechanism, and more complicated logic with files
    /// should ensure `read_fragment` calls are serialized.
    fn read_fragment(&self, offset: u64, length: u64) -> DWResult<Fragment>;

    /// Called when the runtime is finished with a Fragment so that this class may release any
    /// data it allocated when `read_fragment` was called. `key` will be the exact value that the
    /// implementor of this trait returned from `read_fragment` for a given fragment.
    fn release_fragment(&self, key: usize);
}

impl<T> FontFileStream for Box<T>
where
    T: FontFileStream,
{
    fn file_size(&self) -> u64 {
        T::file_size(self)
    }

    fn last_write_time(&self) -> u64 {
        T::last_write_time(self)
    }

    fn read_fragment(&self, offset: u64, length: u64) -> DWResult<Fragment> {
        T::read_fragment(self, offset, length)
    }

    fn release_fragment(&self, key: usize) {
        T::release_fragment(self, key)
    }
}

/// A fragment of memory managed by a `FontFileStream`
pub struct Fragment {
    key: usize,
    data: *const u8,
}

impl Fragment {
    #[inline]
    /// Construct a fragment. This method is unsafe for acknowledgement of the
    /// caller that they are responsible to manage the memory according to the
    /// [unsafe considerations][1] required by `FontFileStream`
    ///
    /// [1]: trait.FontFileStream.html#unsafe-considerations
    pub unsafe fn new(key: usize, data: *const u8) -> Self {
        Fragment { key, data }
    }
}

/// Given a std::fs::Metadata, compute the appropriate timestamp in 100-nanosecond ticks.
pub fn file_timestamp(meta: &Metadata) -> DWResult<u64> {
    let modified = meta.modified()?;
    let (neg, unix_modified) = match modified.duration_since(UNIX_EPOCH) {
        Ok(dur) => (false, dur),
        Err(e) => (true, e.duration()),
    };
    let unix_sec_ticks = unix_modified.as_secs() * 10_000_000;
    let unix_subsec_ticks = unix_modified.subsec_nanos() as u64 / 100;
    let unix_ticks = unix_sec_ticks + unix_subsec_ticks;

    let ticks = if neg {
        UNIX_EPOCH_IN_WRITE_TIME - unix_ticks
    } else {
        UNIX_EPOCH_IN_WRITE_TIME + unix_ticks
    };

    Ok(ticks)
}
