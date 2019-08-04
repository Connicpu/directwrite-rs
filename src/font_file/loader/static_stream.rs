use crate::font_file::loader::{FontFileStream, Fragment};

use dcommon::Error;
use winapi::shared::winerror::E_FAIL;

#[derive(Clone)]
/// A simple FontFileStream implementation for when you have the font file embedded
/// as a static array in your program.
pub struct StaticDataStream {
    /// The last time the file was modified in 100-nanosecond intervals since
    /// January 1, 1601 (UTC).
    pub last_write: u64,

    /// The contents of the file
    pub data: &'static [u8],
}

impl StaticDataStream {
    pub fn new(data: &'static [u8], last_write: u64) -> Self {
        StaticDataStream { data, last_write }
    }
}

impl FontFileStream for StaticDataStream {
    fn file_size(&self) -> u64 {
        self.data.len() as u64
    }

    fn last_write_time(&self) -> u64 {
        self.last_write
    }

    fn read_fragment(&self, offset: u64, length: u64) -> Result<Fragment, Error> {
        let len64 = self.data.len() as u64;
        if offset > len64 || length > len64 || offset + length > len64 {
            return Err(E_FAIL.into());
        }

        unsafe {
            let ptr = self.data.as_ptr().offset(offset as isize);
            let frag = Fragment::new(0, ptr);
            Ok(frag)
        }
    }

    fn release_fragment(&self, _key: usize) {}
}
