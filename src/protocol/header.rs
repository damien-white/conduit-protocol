use bytes::Buf;
use nom::bytes::streaming::take;
use nom::error::ErrorKind;
use nom::number::streaming::{be_u16, be_u64};
use nom::IResult;

use crate::protocol::codec::MAX_FRAME_LEN;

#[derive(Debug, PartialEq)]
pub struct Header {
    pub version: u8,
    pub opcode: u8,
    pub length: u16,
    pub identifier: u64,
}

#[derive(Debug, PartialEq)]
pub struct Message {
    pub message: usize,
}

pub fn parse_version(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, version) = take(1usize)(input)?;

    let version = version[0];
    match version {
        0x01 => Ok((i, version)),
        0x81 => Ok((i, version)),
        _ => panic!("invalid version header"),
    }
}

pub fn parse_opcode(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, opcode) = take(1usize)(input)?;
    Ok((i, opcode.chunk()[0]))
}

pub fn parse_length(input: &[u8]) -> IResult<&[u8], u16> {
    let (i, length) = be_u16(input)?;
    Ok((i, length))
}

pub fn parse_identifier(input: &[u8]) -> IResult<&[u8], u64> {
    let (i, identifier) = be_u64(input)?;
    Ok((i, identifier))
}

pub fn parse_message(input: &[u8], len: usize) -> IResult<&[u8], &[u8]> {
    let (i, message) = take(len)(input)?;
    match message.len() {
        0 => Ok((input, i)),
        1..=MAX_FRAME_LEN => Ok((i, &message[..MAX_FRAME_LEN])),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            message,
            ErrorKind::Eof,
        ))),
    }
}
