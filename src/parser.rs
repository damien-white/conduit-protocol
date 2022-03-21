use std::io;
use std::io::Cursor;

use bytes::Buf;

pub const MAGIC_SEQUENCE: [u8; 4] = [0x4c, 0x75, 0x63, 0x79];

pub fn parse_byte(buf: &mut Cursor<&[u8]>) -> io::Result<u8> {
    if !buf.has_remaining() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Missing or invalid data",
        ));
    }

    let value = buf.get_u8();
    Ok(value)
}

// fn peek_byte(buf: &mut Cursor<&[u8]>) -> io::Result<u8> {
//     if !buf.has_remaining() {
//         return Err(io::Error::new(
//             io::ErrorKind::InvalidData,
//             "Missing or invalid data",
//         ));
//     }
//
//     Ok(buf.chunk()[0])
// }

pub fn parse_magic(buf: &mut Cursor<&[u8]>) -> io::Result<[u8; 4]> {
    if !buf.remaining() <= 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Missing or invalid data",
        ));
    }

    let value = buf.get_u32();
    let magic: [u8; 4] = value.to_be_bytes();
    if magic != MAGIC_SEQUENCE {
        Err(std::io::Error::new(
            io::ErrorKind::InvalidData,
            "Missing or invalid data",
        ))
    } else {
        Ok(magic)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::parser::{parse_magic, MAGIC_SEQUENCE};

    #[test]
    fn parses_frame_header_magic_sequence() {
        let data = b"\x4c\x75\x63\x79";

        let mut reader = Cursor::new(&data[..]);

        let res = parse_magic(&mut reader).unwrap();
        for (i, val) in data.iter().enumerate() {
            println!("{i}: {}", *val);
        }

        assert_ne!(res.len(), 0);
        assert!(!res.is_empty(), "reader is empty!");
        assert_eq!(res, MAGIC_SEQUENCE);
    }
}
