use comptr::ComPtr;
use error::DWriteError;
use text_format;
use load_dll;
use winapi::*;

pub struct Factory {
    ptr: ComPtr<IDWriteFactory>,
}

impl Factory {
    pub fn create() -> Result<Factory, DWriteError> {
        let dwrite = try!(load_dll::DWrite::load());
        let ptr = try!(dwrite.create_factory(false));
        Ok(Factory { ptr: ptr })
    }
    
    pub fn create_text_format(
        &self,
        params: text_format::Params,
    ) -> Result<text_format::TextFormat, DWriteError> {
        use internal::FromParams;
        FromParams::from_params(unsafe { &mut *self.ptr.raw_value() }, params)
    }
}
