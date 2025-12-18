## Keyring-rs

[![build](https://github.com/open-source-cooperative/keyring-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/open-source-cooperative/keyring-rs/actions)
[![dependencies](https://deps.rs/repo/github/open-source-cooperative/keyring-rs/status.svg)](https://deps.rs/repo/github/open-source-cooperative/keyring-rs)
[![crates.io](https://img.shields.io/crates/v/keyring.svg)](https://crates.io/crates/keyring)
[![docs.rs](https://docs.rs/keyring/badge.svg)](https://docs.rs/keyring)

This crate provides a simple CLI for the [Rust keyring ecosystem](https://github.com/open-source-cooperative/keyring-rs/wiki/Keyring). It also provides sample Rust code for developers who are looking to use the keyring infrastructure in their projects and an inventory of available credential store modules.

## Rust CLI

The `keyring` binary produced by building this crate is a command-line interface for issuing one keyring call at a time and examining its results. Issue the command
```shell
keyring  help
```
for usage information.

## Python interface for scripting

The CLI provided by this crate is neither efficient nor convenient for scripting, because each invocation loads a credential store, issues just one command against it, and then outputs the results in a format that is hard to parse. If you are looking to do scripting of keyring commands, you are better off using the Python wrapper for this crate available on PyPI in the [rust-native-keyring project](https://pypi.org/project/rust-native-keyring/). Use the shell command
```shell
pip install rust-native-keyring
```
to install it and
```python
import rust_native_keyring
```
to load it into your Python REPL.

## Credential Stores Wanted!

If you are a credential store module developer, you are strongly encouraged to contribute a connector for your module to the library in this crate, thus making it available to both client applications. See the [module documentation](https://docs.rs/keyring/latest/keyring/) for details.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
