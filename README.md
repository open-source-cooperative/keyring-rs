## Keyring-rs

[![build](https://github.com/open-source-cooperative/keyring-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/open-source-cooperative/keyring-rs/actions)
[![dependencies](https://deps.rs/repo/github/open-source-cooperative/keyring-rs/status.svg)](https://deps.rs/repo/github/open-source-cooperative/keyring-rs)
[![crates.io](https://img.shields.io/crates/v/keyring.svg)](https://crates.io/crates/keyring)
[![docs.rs](https://docs.rs/keyring/badge.svg)](https://docs.rs/keyring)

This crate provides a simple CLI and various other clients of the [Rust keyring ecosystem](https://github.com/open-source-cooperative/keyring-rs/wiki/Keyring). It provides sample Rust code for developers who are looking to use the keyring infrastructure in their projects, a variety of tools for users who want to access their keyring-compatible credential stores, and an inventory of available credential store modules.

## Library

The `lib.rs` file in this crate provides the sample Rust code and the inventory of available credential store modules. The first part of the library contains connectors for each of the available keyring credential stores. As developers make new credential store modules available, they are encouraged to submit a pull request that adds their module to the library. The second part of the library contains sample calls to each member of the keyring `Entry` API.

## Applications

There are currently two application-level tools provided by this crate:

* The `keyring-cli` application is a simple command-line interface for issuing one keyring call at a time and examining its results. You can build this application using cargo.
* The `rust_native_keyring` Python module provides a way of accessing entries in the context of a Python application or REPL. You can download this module from [PyPI](https://pypi.org/project/rust_native_keyring/).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
