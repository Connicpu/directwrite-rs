use wio::com::ComPtr;
use error::DWriteError;
use load_dll;

use winapi::um::dwrite::IDWriteFactory;

pub struct Factory {
    ptr: ComPtr<IDWriteFactory>,
}

impl Factory {
    pub fn new() -> Result<Factory, DWriteError> {
        let dwrite = try!(load_dll::DWrite::load());
        let ptr = try!(dwrite.create_factory(false));
        Ok(Factory { ptr: ptr })
    }
    
    pub fn create<T: ::internal::FromParams>(&self, params: T::Params) -> Result<T, DWriteError> {
        T::from_params(unsafe { &mut *self.ptr.as_raw() }, params)
    }
}
