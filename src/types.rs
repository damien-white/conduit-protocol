use std::io;

use bytes::BytesMut;

use crate::error::ProtocolError;

/// The maximum allowed frame length in bytes (including the header).
///
/// This value is sometimes referred to as the MTU, or maximum transmission unit.
pub const MAX_FRAME_LEN: usize = 65536;
/// The length of the entire frame `Header` in bytes.
pub const FRAME_HEADER_LEN: usize = 12;
/// This sequence of bytes tells the parser we have a new frame.
pub const MAGIC_SEQUENCE: [u8; 4] = [0x53, 0x69, 0x4c, 0x6b];

/// The protocol version, coupled with the direction of the frame.
///
/// The "direction" of the frame indicates whether the frame is a `Request` or
/// a `Response` variant. This will help keep track of traffic flow later.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Version {
    Request = 0x01,
    Response = 0x81,
}

impl TryFrom<u8> for Version {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Version::Request),
            0x81 => Ok(Version::Response),
            value => {
                return Err(ProtocolError::Invalid(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("parsed invalid version header: {value}"),
                )));
            }
        }
    }
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

/// The message attached to the frame. Also referred to as the "payload".
#[derive(Debug, PartialEq)]
pub struct Message {
    pub message: BytesMut,
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;
    use rand::Rng;

    use super::*;

    fn generate_id() -> [u8; 4] {
        let mut id = [0u8; 4];
        let mut rng = rand::thread_rng();
        rng.fill(&mut id);
        id
    }

    #[test]
    fn magic_signature_is_recognized() {
        // Arrange
        let mut buf = BytesMut::with_capacity(4);
        let data = MAGIC_SEQUENCE;

        // Act
        buf.reserve(data.len());
        buf.put(&data[..]);

        // Assert
        assert_eq!(data.len(), 4);
        assert_eq!(buf.len(), 4);
        assert!(!buf.is_empty());
    }

    #[test]
    fn frame_id_can_be_decoded() {
        // Arrange
        let mut buf = BytesMut::with_capacity(4);

        // Act
        let mut data = generate_id();
        let original = data.clone();

        buf.reserve(data.len());
        buf.put(&data[..]);

        // Assert
        assert_eq!(data.len(), 4);
        assert_eq!(data.len(), 4);
        assert_eq!(&buf[..], &original[..]);
        assert_eq!(buf.len(), 4);
        assert!(!buf.is_empty());
    }

    #[test]
    fn valid_version_values_pass() {
        let request = Version::Request as u8;
        let response = Version::Response as u8;

        assert_eq!(request, 0x01);
        assert_eq!(response, 0x81);
    }

    #[test]
    fn opcodes_are_recognized() {
        // TODO: Fix implementation for Opcode (error cases) along with other types.
        let data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        for (i, &code) in data.iter().enumerate() {
            let opcode1 = Opcode::from(i as u8);
            let opcode2 = Opcode::from(code as u8);
            println!("{:?} {} {}", opcode2, Opcode::from(code) as u8, code);

            assert_eq!(opcode1, opcode2);
            assert_eq!(opcode1, code.into());
            assert_eq!(opcode2, code.into());
            assert_eq!(u8::from(opcode1), code);
            assert_eq!(u8::from(opcode2), code);
        }
    }

    #[test]
    fn opcode_map_matches_serialized_values() {
        assert_eq!(Opcode::Get, 0x00.into());
        assert_eq!(Opcode::Set, 0x01.into());
        assert_eq!(Opcode::Delete, 0x02.into());
        assert_eq!(Opcode::List, 0x03.into());
        assert_eq!(Opcode::Renew, 0x04.into());
        assert_eq!(Opcode::Health, 0x05.into());
        assert_eq!(Opcode::Shutdown, 0x06.into());
        assert_eq!(Opcode::Illegal, 0x22.into());
    }

    // #[test]
    // fn length_field_recognized_and_produces_message_length() {}
}
