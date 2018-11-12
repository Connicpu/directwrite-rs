use error::DWriteError;
use font_file::loader::handle::FileLoaderHandle;
use font_file::FontFile;
use key::{FontKey, KeyPayload};

use std::ptr;

use com_wrapper::ComWrapper;
use winapi::shared::minwindef::FILETIME;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::IDWriteFactory;
use wio::wide::ToWide;

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
    pub fn new(factory: &'a IDWriteFactory) -> Self {
        FontFileBuilder {
            factory,
            source: Source::Unspecified,
        }
    }

    pub fn build(self) -> Result<FontFile, DWriteError> {
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
