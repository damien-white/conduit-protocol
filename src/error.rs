//! This module contains the main error handling data structure for this crate.

/// FrameError enumerates all possible errors returned by this library.
#[derive(Debug)]
pub enum Error {
    /// Represents a failure due to missing data.
    MissingData,

    /// Represents a failure to read from the input source.
    ReadError { source: std::io::Error },

    /// Catch-all error for any errors related to `std::io::Error`.
    IOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::MissingData => {
                write!(f, "Missing expected data")
            }
            Error::ReadError { .. } => {
                write!(f, "Could not read from input source")
            }
            Error::IOError(ref err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::MissingData => None,
            Error::ReadError { ref source } => Some(source),
            Error::IOError(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}
