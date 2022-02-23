//! This module contains the main error handling data structure for this crate.

/// FrameError represents the possible errors returned by this library.
#[derive(Debug)]
pub enum FrameError {
    /// Insufficient data or missing required field.
    MissingData,
    /// A failure occurred while trying to read from the input source.
    ReadError { source: std::io::Error },
    /// Any error that can be converted to `std::io::Error`.
    IOError(std::io::Error),
}

impl std::error::Error for FrameError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            FrameError::MissingData => None,
            FrameError::ReadError { ref source } => Some(source),
            FrameError::IOError(_) => None,
        }
    }
}

impl std::fmt::Display for FrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FrameError::MissingData => {
                write!(f, "Missing expected data")
            }
            FrameError::ReadError { .. } => {
                write!(f, "Could not read from input source")
            }
            FrameError::IOError(ref err) => err.fmt(f),
        }
    }
}

impl From<std::io::Error> for FrameError {
    fn from(err: std::io::Error) -> FrameError {
        FrameError::IOError(err)
    }
}
