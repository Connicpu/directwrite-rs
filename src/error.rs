use std::error::Error;
use std::fmt::{self, Display};

use winapi::shared::ntdef::HRESULT;

#[derive(Clone, Debug)]
pub enum DWriteError {
    ComError {
        hr: HRESULT,
        desc: Option<String>
    },
}

impl From<HRESULT> for DWriteError {
    fn from(hr: HRESULT) -> DWriteError {
        use helpers::*;
        DWriteError::ComError {
            hr: hr,
            desc: hresult_to_string(hr),
        }
    }
}

impl Display for DWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for DWriteError {
    fn description(&self) -> &str {
        match *self {
            DWriteError::ComError { ref desc, .. } => match *desc {
                Some(ref desc) => desc,
                None => "Unknown COM Error Description",
            }
        }
    }
}

