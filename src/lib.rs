//! Conduit network protocol
pub mod codec;
pub mod error;
pub mod frame;
pub mod opcode;

/// Generic `Result` type compatible with most conduit-protocol operations.
pub type Result<T> = std::result::Result<T, error::FrameError>;
