# conduit-protocol

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
for [Conduit](https://github.com/dark-fusion/conduit), in-memory database backed
by a key-value store. `Conduit` is primarily useful as a cache service and is
similar in nature to Redis and Memcached.

## Specification

The official specification has not yet been published. Please check back soon
for updates!

### Frame Diagram

```text
    /       0        |         1       |         2       |        3        |
   /                 |                 |                 |                 |
   | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 |
 0 +-----------------+-----------------+-----------------+-----------------+
   |      VERSION    |     OPCODE      |               LENGTH              |
 4 +-----------------+-----------------+-----------------+-----------------+
   |                                                                       |
 8 +                                FRAME ID                               +
   |                                                                       |
12 +-----------------+-----------------+-----------------+-----------------+
   |                                                                       |
.. |                                MESSAGE                                |
   |                                                                       |
.. +-----------------------------------------------------------------------+
```

### Header Fields

#### Version

8-bit integer that currently has just two valid values:

- `0x01`: Protocol version 1; `Request` frame (client -> server)
- `0x81`: Protocol version 1; `Response` frame (server -> client)

#### Opcode

8-bit integer that represents a client `Command` / `Request` or a `Response`
with (or perhaps without) an attached `Message`.

#### Length

16-bit integer that is used to declare the message length. Although the `Header`
cannot be empty, the `Message` can. There is a __maximum allowed frame length__
of 65536 bytes.

### Frame ID

The frame ID is a randomly generated value comprised of 8 bytes. The Frame ID serves two distinct
purposes:

1. To identify a particular frame and simplify the process of synchronizing responses by helping to
   keep frames in the proper order.
2. Seed a `Nonce` used for encryption. The encryption scheme is not yet implemented, but it will use
   a 192-bit nonce (24 bytes) and use the
   `XChacha20-Poly1305` authenticated encryption with associated data construct.

### Message

The message is a critical part of most frame types. The `Opcode` is largely responsible for
determining what type of payload will be included in the message body. For example, `Command`
payloads received from clients often require certain parameters. This data is encoded and sent
within the message body.

## License

Licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
