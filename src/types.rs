pub const FRAME_HEADER_LENGTH: usize = 12;
pub const MAX_FRAME_LENGTH: usize = 65536;
/// This sequence of bytes tells the parser we have a new frame.
pub const MAGIC_SEQUENCE: [u8; 4] = [0x0C, 0x09, 0x8D, 0x0B];

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
    /// Renews the TTL for a specific key.
    Renew,
    /// Checks the availability of the service with a health check.
    Health,
    /// Sends a shutdown signal to the service, initiating a graceful shut down.
    Shutdown,
    /// Represents all unknown or illegal commands.
    Illegal,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Opcode {
        match value {
            0x00 => Opcode::Get,
            0x01 => Opcode::Set,
            0x02 => Opcode::Delete,
            0x03 => Opcode::List,
            0x04 => Opcode::Renew,
            0x05 => Opcode::Health,
            0x06 => Opcode::Shutdown,
            _ => Opcode::Illegal,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> u8 {
        match opcode {
            Opcode::Get => 0x00,
            Opcode::Set => 0x01,
            Opcode::Delete => 0x02,
            Opcode::List => 0x03,
            Opcode::Renew => 0x04,
            Opcode::Health => 0x05,
            Opcode::Shutdown => 0x06,
            Opcode::Illegal => opcode as u8,
        }
    }
}
