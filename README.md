<div align="center">
  <h1>conduit-protocol</h1>
</div>

[![Crates.io](https://img.shields.io/crates/v/conduit-protocol.svg)](https://crates.io/crates/conduit-protocol)
[![Docs.rs](https://docs.rs/conduit-protocol/badge.svg)](https://docs.rs/conduit-protocol)
[![CI](https://github.com/dark-fusion/conduit-protocol/workflows/CI/badge.svg)](https://github.com/dark-fusion/conduit-protocol/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/conduit-protocol/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/conduit-protocol?branch=main)

## Project Setup

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* This project uses [just](https://github.com/casey/just) for running commonly used commands.
* Run `just compile` to make sure the project compiles

## Description

The conduit-protocol is a byte-oriented network protocol specifically designed
for [Conduit](https://github.com/dark-fusion/conduit), in-memory database backed by a key-value
store. `Conduit` is primarily useful as a cache service and is similar in nature to Redis and
Memcached.

## Specification

This section is a work in progress. Thus, it contains fairly limited information about the protocol
specification. The protocol is entirely open-source and fully defined within this repository.

If you find that you still have questions, would like to help contribute (adding documentation for
this crate would be greatly appreciated!), please take a look at [CONTRIBUTING.md](/CONTRIBUTING.md)
.

### Frame Format

```text
    /       0        |         1       |         2       |        3        |
   /                 |                 |                 |                 |
   | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 |
 0 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                                 MAGIC                                 |
 4 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                              IDENTIFIER                               |
 8 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |     VERSION     |      OPCODE     |               LENGTH              | 
12 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                                                                       |
16 |                                MESSAGE                                |
   |                                                                       |
.. +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

[Byte order / Endianness](https://en.wikipedia.org/wiki/Endianness): All values use big endian
order, or "network order".

### Header Fields

#### Magic

4-byte sequence that provides a "marker" indicating the beginning of a frame. The presence of this
sequence alone is not sufficient to define it as a frame "boundary".

#### Identifier

4-byte value that provides a unique frame identifier. Requests or responses may be received out of
order or not received at all. Frame IDs provide methods for payload synchronization and help
validate that the correct data is received.

#### Version

1-byte integer containing the protocol version number and the "direction" of the frame. Simple
bitwise operations are used to extract and validate the received value.

#### Opcode

1-byte integer that maps to a known, expected instruction set. The opcode is very useful for
determining the exact parameters that should be expected in the message body.

#### Length

2-byte integer that represents the number of bytes to parse from the message body that follows.
Frames may not exceed the maximum frame length of **65536** bytes.

### Message Body

The message contains the body of the message itself. This area of the frame contains the bulk of the
data as it is responsible for holding parameters related to particular opcodes, request and response
messages and other related data.

## License

This work is licensed under the [MIT license](/LICENSE). You may also view this license
at http://opensource.org/licenses/MIT.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
