/// Opcodes are used to map parsed bytes (codes) to operations recognized by the
/// receiving service.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    /// Unknown command; returns None or an Error
    Unknown,
    /// Get a key-value pair if it exists
    Get,
    /// Insert an entry into the cache with an attached TTL
    Set,
    /// Remove an entry if it exists
    Delete,
    /// List all entries that match a given pattern
    List,
    /// Renew the TTL for an existing entry
    Renew,
    /// Status signal
    Status,
    /// Shutdown signal
    Shutdown,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Opcode::Unknown => write!(f, "UNKNOWN"),
            Opcode::Get => write!(f, "GET"),
            Opcode::Set => write!(f, "SET"),
            Opcode::Delete => write!(f, "DELETE"),
            Opcode::List => write!(f, "LIST"),
            Opcode::Renew => write!(f, "RENEW"),
            Opcode::Status => write!(f, "STATUS"),
            Opcode::Shutdown => write!(f, "SHUTDOWN"),
        }
    }
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Opcode {
        match value {
            0x00 => Opcode::Unknown,
            0x01 => Opcode::Get,
            0x02 => Opcode::Set,
            0x03 => Opcode::Delete,
            0x04 => Opcode::List,
            0x05 => Opcode::Renew,
            0x80 => Opcode::Status,
            0x82 => Opcode::Shutdown,
            _ => Opcode::Unknown,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> u8 {
        match opcode {
            Opcode::Unknown => 0x00,
            Opcode::Get => 0x01,
            Opcode::Set => 0x02,
            Opcode::Delete => 0x03,
            Opcode::List => 0x04,
            Opcode::Renew => 0x05,
            Opcode::Status => 0x80,
            Opcode::Shutdown => 0x82,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_code_for_discriminant() {
        let data = 0x04;

        let name = Opcode::from(data);
        let value = u8::from(name);

        assert_eq!(name.to_string(), "LIST");
        assert_eq!(name, Opcode::List);
        assert_eq!(value, data);
    }
}
