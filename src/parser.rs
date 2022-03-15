use bytes::Buf;
use nom::bytes::streaming::{tag, take};
use nom::combinator::verify;
use nom::error::{Error, ErrorKind};
use nom::number::streaming::be_u8;
use nom::IResult;

// TODO: 1. Handle Opcodes, Length field and Message body; 2. revisit validation
use crate::types::{Version, MAGIC_SEQUENCE};

pub fn parse_header(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (input, _magic) = parse_magic(input)?;
    let (input, _identifier) = parse_identifier(input)?;
    let (input, _version) = parse_version(input)?;
    Ok((input, input))
}

pub fn parse_magic(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(&MAGIC_SEQUENCE)(input)
}

pub fn parse_identifier(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, mut identifier) = verify(take4, |parsed: &[u8]| parsed.len() == 4)(input)?;
    Ok((input, identifier.get_u32()))
}

pub fn parse_version(input: &[u8]) -> IResult<&[u8], Version> {
    let (input, value) = parse_u8(input)?;
    let version = match value {
        0x01 => Version::Request,
        0x81 => Version::Response,
        _ => return Err(nom::Err::Failure(Error::new(input, ErrorKind::OneOf))),
    };

    Ok((input, version))
}

pub fn take4(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(4usize)(input)
}

pub fn parse_u8(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, version) = be_u8(input)?;
    Ok((input, version))
}
