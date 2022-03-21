pub const FRAME_HEADER_LENGTH: usize = 12;
pub const MAX_FRAME_LENGTH: usize = 65536;
/// This sequence of bytes tells the parser we have a new frame.

/// The protocol version, coupled with the direction of the frame.
///
/// The "direction" of the frame indicates whether the frame is a `Request` or
/// a `Response` variant. This will help keep track of traffic flow later.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Version {
    Request,
    Response,
}

/// Opcodes are used to map parsed bytes (codes) to operations recognized by the
/// receiving service.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    /// Gets an entry pair and its associated TTL if it exists.
    Get,
    /// Inserts an entry the key-value store.
    Set,
    /// Removes an existing entry and clears any data with the key.
    Delete,
    /// Lists all key-value pairs that match a given criteria.
    List,
    /// Checks the availability of the service with a health check.
    Health,
    /// Sends a shutdown signal to the service, initiating a graceful shut down.
    Shutdown,
    /// Represents all unknown or illegal commands.
    Illegal,
}
