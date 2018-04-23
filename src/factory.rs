use error::DWriteError;

use std::ptr;

use winapi::Interface;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{DWriteCreateFactory, IDWriteFactory, DWRITE_FACTORY_TYPE_SHARED};
use winapi::um::unknwnbase::IUnknown;
use wio::com::ComPtr;

pub struct Factory {
    ptr: ComPtr<IDWriteFactory>,
}

impl Factory {
    pub unsafe fn from_raw(raw: *mut IDWriteFactory) -> Self {
        Factory {
            ptr: ComPtr::from_raw(raw),
        }
    }

    pub unsafe fn get_raw(&self) -> *mut IDWriteFactory {
        self.ptr.as_raw()
    }

    pub fn new() -> Result<Factory, DWriteError> {
        unsafe {
            let mut ptr: *mut IDWriteFactory = ptr::null_mut();
            let hr = DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &IDWriteFactory::uuidof(),
                &mut ptr as *mut _ as *mut *mut IUnknown,
            );

            if SUCCEEDED(hr) {
                Ok(Factory {
                    ptr: ComPtr::from_raw(ptr),
                })
            } else {
                Err(hr.into())
            }
        }
    }
}

unsafe impl Send for Factory {}
unsafe impl Sync for Factory {}
