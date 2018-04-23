use helpers::hresult_to_string;

use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::result;

use winapi::shared::ntdef::HRESULT;

pub type DWResult<T> = result::Result<T, DWriteError>;

#[derive(Copy, Clone)]
pub struct DWriteError(pub HRESULT);

impl DWriteError {
    fn message(&self) -> Cow<str> {
        hresult_to_string(self.0)
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed("Unknown COM Error"))
    }
}

impl From<HRESULT> for DWriteError {
    fn from(hr: HRESULT) -> DWriteError {
        DWriteError(hr)
    }
}

impl fmt::Debug for DWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DWriteError({:x}, {:?})", self.0, self.message())
    }
}

impl fmt::Display for DWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message())
    }
}

impl Error for DWriteError {
    fn description(&self) -> &str {
        "DirectWrite Error"
    }
}
