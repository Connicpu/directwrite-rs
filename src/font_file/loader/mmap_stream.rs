use crate::error::DWResult;
use crate::font_file::loader::{file_timestamp, FontFileStream, Fragment};

use std::fs::File;

use memmap::Mmap;
use winapi::shared::winerror::E_FAIL;

/// A FontFileStream implementation backed by a memory mapped file.
pub struct MmapStream {
    mmap: Mmap,
    last_write: u64,
}

impl MmapStream {
    /// Attempts to create an mmap-ed file stream.
    pub fn map(file: &File) -> DWResult<MmapStream> {
        let mmap = unsafe { Mmap::map(file)? };
        let last_write = file_timestamp(&file.metadata()?)?;

        Ok(MmapStream { mmap, last_write })
    }
}

impl FontFileStream for MmapStream {
    fn file_size(&self) -> u64 {
        self.mmap.len() as u64
    }

    fn last_write_time(&self) -> u64 {
        self.last_write
    }

    fn read_fragment(&self, offset: u64, length: u64) -> DWResult<Fragment> {
        let len64 = self.mmap.len() as u64;
        if offset > len64 || length > len64 || offset + length > len64 {
            return Err(E_FAIL.into());
        }

        let fragment = unsafe { Fragment::new(0, &self.mmap[offset as usize]) };
        Ok(fragment)
    }

    fn release_fragment(&self, _key: usize) {}
}
