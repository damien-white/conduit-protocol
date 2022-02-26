// TODO: Update this solution to something more optimal and easy to reason about
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    // Unknown command; returns None or an Error
    Unknown = 0x00,
    // Get a key-value pair if it exists
    Get = 0x01,
    // Set a new key-value pair with optional expiration
    Set = 0x02,
    // Remove a cache entry
    Delete = 0x03,
    // List key-value pairs that match a given pattern
    List = 0x04,
    // Renew cache entry
    Refresh = 0x05,
    // Status signal
    Status = 0x81,
    // Shutdown signal
    Shutdown = 0x82,
}

impl Opcode {
    pub fn kind(value: u8) -> Self {
        match value {
            0x00 => Opcode::Unknown,
            0x01 => Opcode::Get,
            0x02 => Opcode::Set,
            0x03 => Opcode::Delete,
            0x04 => Opcode::List,
            0x05 => Opcode::Refresh,
            0x80 => Opcode::Status,
            0x82 => Opcode::Shutdown,
            _ => Opcode::Unknown,
        }
    }

    pub fn into_byte(self) -> u8 {
        match self {
            Opcode::Unknown => 0x00,
            Opcode::Get => 0x01,
            Opcode::Set => 0x02,
            Opcode::Delete => 0x03,
            Opcode::List => 0x04,
            Opcode::Refresh => 0x05,
            Opcode::Status => 0x80,
            Opcode::Shutdown => 0x82,
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        opcode.into_byte()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_code_for_discriminant() {
        println!("Name of command for discriminant");
        let data = 0x04_u8;

        let name = Opcode::kind(data);
        let value: u8 = name.into();
        println!("{name:?}; {value}");

        assert_eq!(name, Opcode::List);
        assert_eq!(value, 4);
    }
}
