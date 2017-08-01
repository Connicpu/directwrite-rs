use error::DWriteError;
use winapi::um::dwrite::*;

pub unsafe trait FromParams: Sized {
    type Params;
    
    fn from_params(factory: &mut IDWriteFactory, params: Self::Params) -> Result<Self, DWriteError>;
}
