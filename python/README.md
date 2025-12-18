## Python module for the Rust keyring

[![build](https://github.com/open-source-cooperative/keyring-rs/actions/workflows/maturin-ci.yml/badge.svg)](https://github.com/open-source-cooperative/keyring-rs/actions/workflows/maturin-ci.yml)

This Rust crate, when built using the PyO3 project's `maturin` tool, produces a Python module that can be used to access the keyring ecosystem from Python. The built module is available on PyPI in the [rust-native-keyring project](https://pypi.org/project/rust-native-keyring/); use
```shell
pip install rust-native-keyring
```
to install it and
```python
import rust_native_keyring
```
to load it into your Python REPL. Here is a sample of what you can do:
 ```python
 import rust_native_keyring as rnk

 rnk.use_named_store("sample", { 'backing-file': 'sample-test.ron' })
 rnk.store_info()

 entry = rnk.Entry('service', 'user')
 entry.set_password('test password')
 entry.info()
 entry.get_credential().info()
 if entry.get_password() == 'test password':
     print('Passwords match!')

 e2 = rnk.Entry('service', 'user2')
 e2.set_password('test password 2')
 entries = rnk.Entry.search({ 'service': 'service' })
 print(list(map(lambda e: e.info(), entries)))

 rnk.release_store()
 ```
The [crate doc](https://docs.rs/rust-native-keyring/) gives more details on the API.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
