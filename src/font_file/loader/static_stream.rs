use winapi::shared::winerror::E_FAIL;
use error::DWResult;
use font_file::loader::{FontFileStream, Fragment};

/// A simple FontFileStream implementation for when you have the font file embedded
/// as a static array in your program.
pub struct StaticDataStream {
    /// The last time the file was modified in 100-nanosecond intervals since
    /// January 1, 1601 (UTC).
    pub last_modified: u64,

    /// The contents of the file
    pub data: &'static [u8],
}

impl FontFileStream for StaticDataStream {
    fn file_size(&self) -> u64 {
        self.data.len() as u64
    }

    fn last_write_time(&self) -> u64 {
        self.last_modified
    }

    fn read_fragment(&self, offset: u64, length: u64) -> DWResult<Fragment> {
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
