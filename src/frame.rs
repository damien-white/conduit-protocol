use bytes::{BufMut, Bytes, BytesMut};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

// Signature: 0b10001000

#[derive(Debug)]
pub struct Frame {
    /// Magic signature used as a
    pub nonce: Nonce,
    /// `u8` protocol version and direction of frame (request / response)
    pub version: Version,
    /// `u24` `FrameId` used to track the frame ID and create unique identifier
    pub frame_id: FrameId,
    /// `u8` opcode; determines the command (GET, SET, HEALTH, etc.)
    pub opcode: Opcode,
    /// `u16` field to determine length of the body
    pub length_field: u16,
    /// Variable length payload determined by the `length_field` value
    pub payload: BytesMut,
}

#[derive(Debug)]
pub struct Nonce(Bytes);

impl Nonce {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let signature = (&mut rng)
            .sample_iter(Alphanumeric)
            .take(16)
            .collect::<Vec<_>>();
        Nonce(Bytes::from(signature))
    }

    pub fn value(&self) -> &Bytes {
        &self.0
    }
}

impl Default for Nonce {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique `FrameId` that points to a particular frame.
///
/// The Saint protocol heavily relies upon asynchronous operations and this
/// field is used to identify the particular frame so that clients can receive
/// responses in order.
#[derive(Debug)]
pub struct FrameId {
    local_key: u8,
    remote_key: u16,
}

#[derive(Debug)]
pub enum Version {
    Request,
    Response,
}

impl Version {
    pub fn to_u8(&self) -> u8 {
        match self {
            Version::Request => 0x01,
            Version::Response => 0x81,
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    Get,
    Set,
    Status,
    Error,
    Message,
}

#[cfg(test)]
mod tests {
    use nom::AsBytes;

    use super::*;

    #[test]
    fn signature_header_is_generated() {
        let signature = Nonce::new();
        println!("{:?}", signature);
        assert!(!signature.value().is_empty())
    }

    #[test]
    fn requests_types_are_correctly_parsed() {
        let mut buf = BytesMut::new();
        let nonce = Nonce::new();
        let nonce_value = nonce.value().as_bytes();
        let payload = "\x01".as_bytes();
        buf.put_slice(nonce_value);
        buf.put_slice(payload);
        println!("{:?}", buf);
        assert!(!buf.is_empty())
    }

    #[test]
    fn response_types_are_correctly_parsed() {}
}
