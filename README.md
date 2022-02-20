# saint-protocol

[![Crates.io](https://img.shields.io/crates/v/saint-protocol.svg)](https://crates.io/crates/saint-protocol)
[![Docs.rs](https://docs.rs/saint-protocol/badge.svg)](https://docs.rs/saint-protocol)
[![CI](https://github.com/dark-fusion/saint-protocol/workflows/CI/badge.svg)](https://github.com/dark-fusion/saint-protocol/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/saint-protocol/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/saint-protocol?branch=main)

## Project Setup

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* This project uses [just](https://github.com/casey/just) for running commonly used commands.
* Run `just compile` to make sure the project compiles

## Description

### Protocol Specification

The protocol is a byte-oriented protocol that uses binary encoding (as opposed to text encoding).

The protocol uses a codec to parse raw bytes into frames that can be understood by client and
server.

```text
    /       0        |         1       |         2       |        3        |
   /                 |                 |                 |                 |
   | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 | 0 1 2 3 4 5 6 7 |
 0 +-----------------+-----------------+-----------------+-----------------+
   |      VERSION    |     OPCODE      |               LENGTH              |
 4 +-----------------+-----------------+-----------------+-----------------+
   |     BODY (containing the NONCE and the MESSAGE) NONCE and MESSAGE     |
.. +-----------------------------------------------------------------------+
```

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
