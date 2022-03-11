//! This module contains the main error handling data structure for this crate.

use thiserror::Error;

/// FrameError represents the possible errors returned by this library.
#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Insufficient or missing frame data.")]
    MissingData,
    #[error(
        "Payload exceeds the maximum allowed value. Received: {received}; Expected: {expected}"
    )]
    ExceedsMaxLength { received: usize, expected: usize },
    #[error("An unknown or unexpected error occurred while reading from the input source.")]
    Invalid(#[from] std::io::Error),
}

/// Generic `Result` type that uses the custom `ProtocolError` to make handling
/// errors simple and consistent.
pub type Result<T> = std::result::Result<T, ProtocolError>;
