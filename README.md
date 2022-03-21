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
      0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 
   0 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                        Magic Sequence                         |
   4 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |    Version    |     Tag       |            Length             |
  12 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
     |                                                               |
  16 +                            Value                              +
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
