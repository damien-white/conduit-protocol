<div align="center">
  <h1>amalgam</h1>

[![Crates.io](https://img.shields.io/crates/v/amalgam.svg)](https://crates.io/crates/amalgam)
[![Docs.rs](https://docs.rs/amalgam/badge.svg)](https://docs.rs/amalgam)
[![CI](https://github.com/dark-fusion/amalgam/workflows/CI/badge.svg)](https://github.com/dark-fusion/amalgam/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/amalgam/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/amalgam?branch=main)

</div>

## Project Setup

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* This project uses [just](https://github.com/casey/just) for running commonly used commands.
* Run `just compile` to make sure the project compiles

## Description

This crate contains the building blocks for `amalgam`. In particular, it provides an API that allows
access to a simple codec and (de)serialization routines.

## Type System

The type system is currently based upon [serde's data model][serde-rs]
Lucy Types system can essentially be broken up into two main categories: `primitive` types
and `structural` types.

### Primitives:

- `boolean` => `1`
- `int` => `2`
- `uint` => `3`
- `float` => `4`
- `double` => `5`
- `string` => `6`

### Structural

- `Entry` => `7`
    - associated pair / key-value entry represented as a `Tuple`
- `Array` => `8`
    - fixed size `array` type / [T]
- `List` => `9`
    - growable array type / `Vector`
- `Map` => `10`
    - unordered list of key-value pairs / `HashMap`

## Framing

Lucy's types are represented on the wire in a very particular way.

<!-- TODO: Come up with a proper framing structure -->

- **Frame header structure will be match in-memory alignment**

- Magic sequence: 12 bytes
- Source address: 6-8 bytes
- Version: 1 byte
- Length field: 2-3 bytes
- Destination address: 6-8 bytes: ((u8, u8, u8, u8) u16) -> aligns to 8 bytes

```text
      0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 
   0 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                                                               |
   4 +                                                               +
     |                Handshake data / Negotiation data              |
   8 +                                                               +
     |                                                               |
  12 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                         Source Address                        |     
  16 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+     
     |                       Destination Address                     |     
  16 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |    Version    |                   Length                      |
  20 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                                                               |
  16 +                       Payload / Bytes                         +
     |                                                               |
 ... +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

[Byte order / Endianness](https://en.wikipedia.org/wiki/Endianness): All values use big endian
order, or "network order".

### Header Fields

#### Magic

4-byte sequence that indicates the beginning of a frame. Note that the presence of this sequence is
meant to help synchronize framing but is not sufficient by itself to define a frame boundary.

#### Version

1-byte integer containing the protocol version number and the "direction" of the frame. Simple
bitwise operations are used to extract and validate the received value.

#### Tag

1-byte integer value which indicates the type of payload, or value to expect.

#### Length

2-byte integer representing the length (in bytes) of the value field that follows.<br/>

- Frame size is currently capped at **65536** bytes. This limit exists to prevent denial-of-service
  attacks.

### Value

Variable-length byte stream containing arbitrary data. The data may or may not be valid.

The value contains the bulk of almost all types of frames as it contains the payload data of a given
request or response.

This field may contain any, or a combination of, the following:

- Command / Procedure with optional parameters
- Error message
- Status code(s)
- Simple Signal

## License

This work is licensed under the [MIT license](/LICENSE). You may also view this license
at http://opensource.org/licenses/MIT.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
