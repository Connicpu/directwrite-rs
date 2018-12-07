use crate::error::DWResult;
use crate::font_file::loader::file_timestamp;
use crate::font_file::loader::{FileStream, MmapStream, OwnedDataStream};
use crate::font_file::loader::{FontFileLoader, FontFileStream};

use std::borrow::Cow;
use std::fs::File;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

/// Represents a FontFileStream that may be constructed from a `File`
pub trait FileBackedStream: FontFileStream {
    /// Construct the stream.
    fn from_file(file: File) -> DWResult<Self>;
}

/// A simple implementation of FontFileLoader that loads files from paths.
pub struct FileLoader<S: FileBackedStream> {
    /// The base directory which passed paths are joined onto, if it exists.
    pub base_dir: Option<PathBuf>,
    _marker: PhantomData<S>,
}

impl<S> FileLoader<S>
where
    S: FileBackedStream,
{
    /// Creates a FileLoader that just loads from paths passed as keys directly.
    pub fn new() -> Self {
        FileLoader {
            base_dir: None,
            _marker: PhantomData,
        }
    }

    /// Creates a FileLoader with a base direction that passed keys are joined onto.
    pub fn with_base_dir(base_dir: impl Into<PathBuf>) -> Self {
        FileLoader {
            base_dir: Some(base_dir.into()),
            _marker: PhantomData,
        }
    }
}

impl<S> FontFileLoader for FileLoader<S>
where
    S: FileBackedStream,
{
    type Key = Path;
    type Stream = S;

    fn create_stream(&self, key: &Path) -> DWResult<S> {
        let path = if let Some(base_dir) = &self.base_dir {
            Cow::Owned(base_dir.join(key))
        } else {
            Cow::Borrowed(key)
        };

        let file = File::open(&path)?;
        let stream = S::from_file(file)?;

        Ok(stream)
    }
}

impl FileBackedStream for FileStream {
    fn from_file(file: File) -> DWResult<Self> {
        FileStream::new(file)
    }
}

impl FileBackedStream for MmapStream {
    fn from_file(file: File) -> DWResult<Self> {
        MmapStream::map(&file)
    }
}

impl FileBackedStream for OwnedDataStream {
    fn from_file(mut file: File) -> DWResult<Self> {
        use std::io::Read;
        let meta = file.metadata()?;
        let last_write = file_timestamp(&meta)?;

        let mut buf = Vec::with_capacity(meta.len() as usize);
        file.read_to_end(&mut buf)?;

        Ok(OwnedDataStream {
            data: buf.into(),
            last_write,
        })
    }
}
