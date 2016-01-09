use error::DWriteError;
use winapi::*;

pub unsafe trait FromParams: Sized {
    type Params;
    
    fn from_params(factory: &mut IDWriteFactory, params: Self::Params) -> Result<Self, DWriteError>;
}
