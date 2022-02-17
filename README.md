# saint-protocol

[![Crates.io](https://img.shields.io/crates/v/saint-protocol.svg)](https://crates.io/crates/saint-protocol)
[![Docs.rs](https://docs.rs/saint-protocol/badge.svg)](https://docs.rs/saint-protocol)
[![CI](https://github.com/dark-fusion/saint-protocol/workflows/CI/badge.svg)](https://github.com/dark-fusion/saint-protocol/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/saint-protocol/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/saint-protocol?branch=main)

## Installation

## Protocol

- Byte-oriented protocol
- Binary encoding
-

```text
    /       0        |         1       |         2       |        3        |
   /                 |                 |                 |                 |
   | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 |
   +-----------------+-----------------+-----------------+-----------------+
   |       MAGIC  SIGNATURE            |     VERSION     |     OPCODE      |
32 +-----------------+-----------------+-----------------+-----------------+
   |             FRAME ID              |              LENGTH               |
64 +-----------------+-----------------+-----------------+-----------------+
   |                              MESSAGE BODY                             |
.. | Message payload as determined by OPCODE. The body length is limited   |
   | by the LENGTH field value.                                            |
   |                                                                       |
.. +-----------------+-----------------+-----------------+-----------------+
```

### VERSION

Determines the protocol version and whether the Frame is a request or response

- If the MSB is set the frame is a response: 0x81
- Else, the frame is a request: 0x01

### Frame ID

24-bit integer that expands to a

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install saint-protocol`

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
