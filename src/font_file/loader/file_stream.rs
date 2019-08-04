use crate::font_file::loader::{file_timestamp, FontFileStream, Fragment};

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::sync::Mutex;

use dcommon::Error;

/// A FontFileStream implementation backed by a file. Seeks to positions and reads them into
/// a new buffer for every time `read_fragment` is called.
pub struct FileStream {
    file: Mutex<File>,
    last_write: u64,
    len: u64,
}

impl FileStream {
    /// Construct a new FileStream from a given File.
    pub fn new(file: File) -> Result<FileStream, Error> {
        let meta = file.metadata()?;

        let stream = FileStream {
            file: Mutex::new(file),
            last_write: file_timestamp(&meta)?,
            len: meta.len(),
        };

        Ok(stream)
    }
}

impl FontFileStream for FileStream {
    fn file_size(&self) -> u64 {
        self.len
    }

    fn last_write_time(&self) -> u64 {
        self.last_write
    }

    fn read_fragment(&self, offset: u64, length: u64) -> Result<Fragment, Error> {
        assert!(length < std::isize::MAX as u64);

        // Seek to the position
        let mut file = self.file.lock().unwrap();
        file.seek(SeekFrom::Start(offset))?;

        // Read the data into a buffer
        let mut buf = Vec::new();
        buf.resize(length as usize, 0);
        file.read_exact(&mut buf)?;

        // Release the mutex
        drop(file);

        let buf = Box::new(buf);
        let ptr = buf.as_ptr();

        // Typed as a sanity check
        let buf: *mut Vec<u8> = Box::into_raw(buf);

        unsafe { Ok(Fragment::new(buf as usize, ptr)) }
    }

    fn release_fragment(&self, key: usize) {
        unsafe {
            Box::from_raw(key as *mut Vec<u8>);
        }
    }
}
