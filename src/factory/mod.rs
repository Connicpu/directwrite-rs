use com_wrapper::ComWrapper;
use dcommon::Error;
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::dwrite::{DWriteCreateFactory, IDWriteFactory, DWRITE_FACTORY_TYPE_SHARED};
use winapi::um::unknwnbase::IUnknown;
use winapi::Interface;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(Clone, ComWrapper)]
#[com(send, sync, debug)]
/// The root type required to access all directwrite functionality.
pub struct Factory {
    ptr: ComPtr<IDWriteFactory>,
}

impl Factory {
    /// Initializes a new Factory.
    pub fn new() -> Result<Factory, Error> {
        unsafe {
            let mut ptr: *mut IDWriteFactory = std::ptr::null_mut();
            let hr = DWriteCreateFactory(
                DWRITE_FACTORY_TYPE_SHARED,
                &IDWriteFactory::uuidof(),
                &mut ptr as *mut _ as *mut *mut IUnknown,
            );

            if SUCCEEDED(hr) {
                Ok(Factory::from_raw(ptr))
            } else {
                Err(hr.into())
            }
        }
    }
}

pub unsafe trait IFactory {
    unsafe fn raw_f(&self) -> &IDWriteFactory;
}

unsafe impl IFactory for Factory {
    unsafe fn raw_f(&self) -> &IDWriteFactory {
        &self.ptr
    }
}
