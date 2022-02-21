//! This module contains the main error handling data structure for this crate.

/// FrameError enumerates all possible errors returned by this library.
#[derive(Debug)]
pub enum CodecError {
    /// Represents a failure due to missing data.
    MissingData,
    /// Represents a failure to read from the input source.
    ReadError { source: std::io::Error },
    /// Catch-all error for any errors related to `std::io::Error`.
    IOError(std::io::Error),
}

impl std::fmt::Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CodecError::MissingData => {
                write!(f, "Missing expected data")
            }
            CodecError::ReadError { .. } => {
                write!(f, "Could not read from input source")
            }
            CodecError::IOError(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for CodecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            CodecError::MissingData => None,
            CodecError::ReadError { ref source } => Some(source),
            CodecError::IOError(_) => None,
        }
    }
}

impl From<std::io::Error> for CodecError {
    fn from(err: std::io::Error) -> CodecError {
        CodecError::IOError(err)
    }
}
