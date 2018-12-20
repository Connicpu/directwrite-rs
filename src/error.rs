use dcommon::error::Error;

/// Result type that could contain a DWriteError.
pub type DWResult<T> = Result<T, Error>;
