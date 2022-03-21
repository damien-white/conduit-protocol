use std::io;

use bytes::{Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::types::MAX_FRAME_LENGTH;

pub struct ConduitCodec;

impl ConduitCodec {}

// TODO: Implement Decoder trait
impl Decoder for ConduitCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, _src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

// TODO: Implement Encoder trait
impl Encoder<Bytes> for ConduitCodec {
    type Error = io::Error;

    fn encode(&mut self, data: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = data.len();

        if len > MAX_FRAME_LENGTH {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "payload exceeds maximum allowed frame size",
            ));
        }

        // TODO: Bounds checking; reserve capacity in buffer to fit frame

        dst.extend_from_slice(&data[..]);

        Ok(())
    }
}
