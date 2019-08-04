use crate::descriptions::{FontKey, KeyPayload};
use crate::font_file::loader::handle::FileLoaderHandle;
use crate::font_file::FontFile;

use std::ptr;

use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::minwindef::FILETIME;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFactory;
use wio::wide::ToWide;

#[must_use]
/// Facilitates construction of FontFiles.
///
/// You can construct a FontFile from either a custom font loader or a file path.
///
/// If you specify `file_path` you may optionally also specify `last_write_time` to let
/// DirectWrite know what you believe the last-modified time of the file to be.
///
/// If you specify `loader` you must also specify a `key` to pass to the custom font loader.
///
/// `file_path` and `last_write_time` are mutually exclusive with `loader` and `key`.
pub struct FontFileBuilder<'a, K: FontKey + ?Sized> {
    factory: &'a IDWriteFactory,
    source: Source<'a, K>,
}

enum Source<'a, K: FontKey + ?Sized> {
    Unspecified,
    File {
        path: Option<&'a str>,
        last_write: Option<FILETIME>,
    },
    Custom {
        loader: Option<&'a FileLoaderHandle<K>>,
        key: Option<&'a K>,
    },
}

impl<'a, K: FontKey + ?Sized> FontFileBuilder<'a, K> {
    /// Initializes a builder for a FontFile.
    pub fn new(factory: &'a IDWriteFactory) -> Self {
        FontFileBuilder {
            factory,
            source: Source::Unspecified,
        }
    }

    /// Finalizes the builder and constructs the FontFile.
    pub fn build(self) -> Result<FontFile, Error> {
        unsafe {
            match self.source {
                Source::Unspecified => {
                    panic!("You must specify some parameters to load a FontFile.")
                }
                Source::File { path, last_write } => {
                    let path = path.expect("`file_path` must be specified").to_wide_null();

                    let last_write_time = match last_write {
                        Some(t) => &t,
                        None => ptr::null(),
                    };

                    let mut ptr = ptr::null_mut();
                    let hr = self.factory.CreateFontFileReference(
                        path.as_ptr(),
                        last_write_time,
                        &mut ptr,
                    );

                    if SUCCEEDED(hr) {
                        Ok(FontFile::from_raw(ptr))
                    } else {
                        Err(hr.into())
                    }
                }
                Source::Custom { loader, key } => {
                    let loader = loader.expect("`loader` must be specified");
                    let key = KeyPayload::new(key.expect("`key` must be specified"));
                    let key_size = std::mem::size_of_val(&key) as u32;

                    let mut ptr = ptr::null_mut();
                    let hr = self.factory.CreateCustomFontFileReference(
                        &key as *const _ as *const _,
                        key_size,
                        loader.get_raw(),
                        &mut ptr,
                    );

                    if SUCCEEDED(hr) {
                        Ok(FontFile::from_raw(ptr))
                    } else {
                        Err(hr.into())
                    }
                }
            }
        }
    }
}

impl<'a> FontFileBuilder<'a, ()> {
    /// Specify the font file path used to construct the font.
    ///
    /// Once this method has been called it is an error to call `with_loader` or `with_key`.
    pub fn with_file_path(mut self, file_path: &'a str) -> Self {
        self.source = match self.source {
            Source::Unspecified => Source::File {
                path: Some(file_path),
                last_write: None,
            },
            Source::File { last_write, .. } => Source::File {
                path: Some(file_path),
                last_write,
            },
            Source::Custom { .. } => panic!("Custom font loaders take keys, not paths."),
        };
        self
    }

    /// Specify the last-modified time the application believes the file should have. This is
    /// entirely optional and if omitted the runtime will read the last-write time at the time
    /// of construction.
    ///
    /// If you do specify this value, it should be the number of 100-nanosecond ticks elapsed
    /// since `January 1, 1601 00:00:00 UTC`
    ///
    /// Once this method has been called it is an error to call `with_loader` or `with_key`.
    pub fn with_last_write_time(mut self, last_write: u64) -> Self {
        let last_write = FILETIME {
            dwLowDateTime: last_write as u32,
            dwHighDateTime: (last_write >> 32) as u32,
        };
        self.source = match self.source {
            Source::Unspecified => Source::File {
                path: None,
                last_write: Some(last_write),
            },
            Source::File { path, .. } => Source::File {
                path,
                last_write: Some(last_write),
            },
            Source::Custom { .. } => panic!("Custom font loaders don't take write times."),
        };
        self
    }
}

impl<'a, K: FontKey + ?Sized> FontFileBuilder<'a, K> {
    /// Specify the custom file loader that this file should be loaded from.
    pub fn with_loader(mut self, loader: &'a FileLoaderHandle<K>) -> Self {
        self.source = match self.source {
            Source::Unspecified => Source::Custom {
                loader: Some(loader),
                key: None,
            },
            Source::Custom { key, .. } => Source::Custom {
                loader: Some(loader),
                key,
            },
            Source::File { .. } => panic!("System font files don't use loaders."),
        };
        self
    }

    /// Specify the key value to be passed to the custom font file loader.
    pub fn with_key(mut self, key: &'a K) -> Self {
        self.source = match self.source {
            Source::Unspecified => Source::Custom {
                loader: None,
                key: Some(key),
            },
            Source::Custom { loader, .. } => Source::Custom {
                loader,
                key: Some(key),
            },
            Source::File { .. } => panic!("System fonts take paths, not keys."),
        };
        self
    }
}
