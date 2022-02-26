use bytes::Buf;
use nom::branch::alt;
use nom::bytes::streaming::{tag, take};
use nom::number::streaming::{be_u16, be_u64};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Header {
    version: u8,
    opcode: u8,
    length: u16,
    nonce: u64,
}

#[derive(Debug, PartialEq)]
pub struct Message {
    message: usize,
}

// TODO: Parse message body and tie this logic together with existing modules
pub fn parse_version(input: &[u8]) -> IResult<&[u8], u8> {
    let (i, version) = alt((tag([0x01]), tag([0x81])))(input)?;

    let version = version.chunk()[0];
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

pub fn parse_nonce(input: &[u8]) -> IResult<&[u8], u64> {
    let (i, nonce) = be_u64(input)?;
    Ok((i, nonce))
}
