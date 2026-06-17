## Keyring-rs

[![build](https://github.com/open-source-cooperative/keyring-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/open-source-cooperative/keyring-rs/actions)
[![dependencies](https://deps.rs/repo/github/open-source-cooperative/keyring-rs/status.svg)](https://deps.rs/repo/github/open-source-cooperative/keyring-rs)
[![crates.io](https://img.shields.io/crates/v/keyring.svg)](https://crates.io/crates/keyring)
[![docs.rs](https://docs.rs/keyring/badge.svg)](https://docs.rs/keyring)

This crate provides a simple wrapper library for the [Rust keyring ecosystem](https://github.com/open-source-cooperative/keyring-rs/wiki/Keyring). If you link to this library with the default (`v1`) feature, it gives your application the ability to set, get, and delete both plain-text and binary secrets in the native secure stores on Mac, Windows, and \*nix operating systems, using exactly the same API as the original “v1” version of this crate did.

This crate also provides well-maintained and comprehensive sample Rust code for developers who are looking to use the full power of the Keyring ecosystem in their projects. This sample code is kept in the `cli` module wrapped by the `cli` feature and is used in a number of demonstration applications, described below.

Developers who build applications that use the full power of the [Keyring ecosystem](https://github.com/open-source-cooperative/keyring-rs/wiki/Keyring) should not link directly against this crate using the `cli` feature, because that will bring with it a host of credential stores they don’t need. Instead, developers should take dependencies on the [keyring-core crate](https://crates.io/crates/keyring-core) and the specific credential stores they want to use. They can then copy any of the code in the `cli` module of this crate that they want to instantiate and access the desired credential stores.

## History

This crate has a long history. It was first written by [Walther Chen](https://github.com/hwchen) as an "API library plus credential store" combination. Currently maintained by [Dan Brotsky](https://github.com/brotskydotcom), it can still be used in the original way, but the library/API parts have been moved into the [keyring-core crate](https://crates.io/crates/keyring-core) and the credential stores all in [separate crates](https://crates.io/search?q=keyring%20credential%20store) of their own. This allows developers of keyring-using applications a lot more control over exactly what credential stores to use on which platforms. The [Contributors file](Contributors.md) lists the many, many people who have contributed to various generations of this crate.

## Demonstration Applications

### Rust CLI

The `keyring-cli` binary produced by building the `keyring-cli` example app in this crate is a command-line interface for issuing one keyring call at a time and examining its results. Issue the command
```shell
keyring --help
```
for usage information.

### Python Module

The CLI provided by this crate is neither efficient nor convenient for scripting, because each invocation loads a credential store, issues just one command against it, and then outputs the results in a format that is hard to parse. If you are looking to do scripting of keyring commands, you are better off using the Python wrapper for this crate available on PyPI in the [rust-native-keyring project](https://pypi.org/project/rust-native-keyring/). Use the shell command
```shell
pip install rust-native-keyring
```
to install it and
```python
import rust_native_keyring
```
to load it into your Python REPL. The sources for this Python module are built using [PyO3](https://github.com/PyO3/pyo3) and can be found in [this repository](https://github.com/open-source-cooperative/keyring-for-python).

### Cross-platform GUI

There is a [Tauri 2.0](https://tauri.app/) cross-platform GUI for Keyring in [this repository](https://github.com/open-source-cooperative/keyring-demo). This GUI allows you to poke around in any of the keyring-compatible credential stores available on your platform. This GUI is currently in public beta testing on iOS, macOS, and Android (instructions [here for iOS/macOS](https://github.com/open-source-cooperative/keyring-demo/issues/2) and [here for Android](https://github.com/open-source-cooperative/keyring-demo/issues/1)), and it’s available for MacOS (not sandboxed), Linux, and Windows on [CrabNebula](https://web.crabnebula.cloud/brotskydotcom/keyring-demo/releases).

## Example Applications

In addition to the `keyring-cli` example described above, there are some other useful example applications in this crate.

### Unit Test

This app, which is available on all platforms that can run command-line apps, will run unit tests against all the credential stores available on that platform. It’s invoked with no arguments. Developers that contribute credential store modules to this crate (see [below](#credential-stores-wanted)) should be sure to run these unit tests against their store once they’ve integrated it.

### Leak Test

This is another example that developers should use to test their credential store implementations. It makes sure that none of the secret data that is being transferred to and from the credential store leaks into heap in dropped data. Read the shell scripts for info about how to use it, and try it with the `keyring-core::sample` store to see what it looks like when secrets do leak.

### Set or Clear Three

Almost all the OS platforms that provide native secure stores provide native viewers for seeing the credentials in those stores. The little `set-or-clear-three` example creates or deletes three sample applications using the keyring. Users can then look at the native viewers to see how those credentials appear (and disappear), and how they can be manipulated in the native tools. This is a great way to understand how the conventions used by the keyring compare to those used by other tools. Invoke it with no arguments to get usage information.

## Credential Stores Wanted!

If you are a credential store module developer, you are strongly encouraged to contribute a connector for your module to the `cli` module in this crate, thus making it available to users (in the test apps) and application developers (via sample code). See the [module documentation](https://docs.rs/keyring/latest/keyring/) for details.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
