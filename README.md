## Keyring-rs

[![build](https://github.com/open-source-cooperative/keyring-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/open-source-cooperative/keyring-rs/actions)
[![dependencies](https://deps.rs/repo/github/open-source-cooperative/keyring-rs/status.svg)](https://deps.rs/repo/github/open-source-cooperative/keyring-rs)
[![crates.io](https://img.shields.io/crates/v/keyring.svg)](https://crates.io/crates/keyring)
[![docs.rs](https://docs.rs/keyring/badge.svg)](https://docs.rs/keyring)

This crate provides two client apps---one in Rust, one in Python---for the [Rust keyring ecosystem](https://github.com/open-source-cooperative/keyring-rs/wiki/Keyring). It also provides sample Rust code for developers who are looking to use the keyring infrastructure in their projects and an inventory of available credential store modules.

## Rust CLI

The `keyring` binary produced by building this crate is a command-line interface for issuing one keyring call at a time and examining its results. Issue the command
```shell
keyring  help
```
for usage information.

## Python Module

This crate, when built using the PyO3's project `maturin` tool, produces a Python module that can be used to access the keyring ecosystem from Python. The built module is also available on PyPI in the [rust-native-keyring project](https://pypi.org/project/rust-native-keyring/); use
```shell
pip install rust-native-keyring
```
to install it and
```python
import rust_native_keyring
```
to load it into your Python REPL. The `python` module documentation in this crate provides some sample usage information.

## Credential Stores Wanted!

If you are a credential store module developer, you are strongly encouraged to contribute a connector for your module to the library in this crate, thus making it available to both client applications. See the [module documentation](https://docs.rs/keyring/latest/keyring/) for details.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
