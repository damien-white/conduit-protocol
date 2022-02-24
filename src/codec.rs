use bytes::{Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::error::FrameError;
use crate::frame::MAX_FRAME_LEN;

// Bitmask used to check if a frame is a request
pub(crate) const REQUEST_MASK: u8 = 0b00000001;
// Bitmask used to verify that a frame ia a response
pub(crate) const RESPONSE_MASK: u8 = 0b10000001;

pub struct ConduitCodec;

impl ConduitCodec {}

// TODO: Implement Decoder trait
impl Decoder for ConduitCodec {
    type Item = BytesMut;
    type Error = FrameError;

    fn decode(&mut self, _src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

// TODO: Implement Encoder trait
impl Encoder<Bytes> for ConduitCodec {
    type Error = FrameError;

    fn encode(&mut self, data: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = data.len();

        if len > MAX_FRAME_LEN {
            return invalid_input("exceeds maximum allowed frame length");
        }

        // TODO: Bounds checking; reserve capacity in buffer to fit frame

        dst.extend_from_slice(&data[..]);

        Ok(())
    }
}

pub const fn is_request(value: u8) -> bool {
    (REQUEST_MASK ^ value) == 0
}

pub const fn is_response(value: u8) -> bool {
    (RESPONSE_MASK ^ value) == 0
}

pub fn invalid_input(message: &str) -> Result<(), FrameError> {
    Err(FrameError::IOError(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        message,
    )))
}
