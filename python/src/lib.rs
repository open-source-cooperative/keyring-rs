//! Python bindings for the Rust keyring ecosystem.
//!
//! This module defines a Python module that wraps all the functionality
//! of the keyring-core `Entry`. It also provides the ability to use
//! any of the named stores provided by the keyring crate.
//!
//! The Python module defined here is available from PyPI in the
//! [rust-native-keyring project](https://pypi.org/project/rust-native-keyring/).
//! So you can install it into your Python environment with
//! ```bash
//! pip install rust-native-keyring
//! ```
//! and then import it into your Python REPL with
//! ```python
//! import rust_native_keyring
//! ```
//!
//! Here is a sample of what you can do:
//! ```python
//! import rust_native_keyring as rnk
//!
//! rnk.use_named_store("sample", { 'backing-file': 'sample-test.ron' })
//! rnk.store_info()
//!
//! entry = rnk.Entry('service', 'user')
//! entry.set_password('test password')
//! entry.info()
//! entry.get_credential().info()
//! if entry.get_password() == 'test password':
//!     print('Passwords match!')
//!
//! e2 = rnk.Entry('service', 'user2')
//! e2.set_password('test password 2')
//! entries = rnk.Entry.search({ 'service': 'service' })
//! print(list(map(lambda e: e.info(), entries)))
//!
//! rnk.release_store()
//! ```
use pyo3::prelude::*;

#[pymodule]
mod rust_native_keyring {
    use std::collections::HashMap;

    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;

    use keyring;
    use keyring_core;

    struct Error(keyring_core::Error);

    impl From<Error> for PyErr {
        fn from(value: Error) -> Self {
            PyRuntimeError::new_err(format!("{:?}", value.0))
        }
    }

    impl From<keyring_core::Error> for Error {
        fn from(value: keyring_core::Error) -> Self {
            Self(value)
        }
    }

    #[pyclass(frozen)]
    struct Entry {
        inner: keyring_core::Entry,
    }

    #[pymethods]
    impl Entry {
        #[new]
        #[pyo3(signature = (service, user, modifiers = None))]
        fn new(
            service: String,
            user: String,
            modifiers: Option<HashMap<String, String>>,
        ) -> Result<Self, Error> {
            let modifiers = keyring::internalize(modifiers.as_ref());
            Ok(Self {
                inner: keyring_core::Entry::new_with_modifiers(&service, &user, &modifiers)?,
            })
        }

        #[pyo3(signature = ())]
        fn info(&self) -> String {
            format!("{:?}", self.inner)
        }

        #[pyo3(signature = (pw))]
        fn set_password(&self, pw: String) -> Result<(), Error> {
            Ok(self.inner.set_password(&pw)?)
        }

        #[pyo3(signature = (secret))]
        fn set_secret(&self, secret: Vec<u8>) -> Result<(), Error> {
            Ok(self.inner.set_secret(&secret)?)
        }

        #[pyo3(signature = ())]
        fn get_password(&self) -> Result<String, Error> {
            Ok(self.inner.get_password()?)
        }

        #[pyo3(signature = ())]
        fn get_secret(&self) -> Result<Vec<u8>, Error> {
            Ok(self.inner.get_secret()?)
        }

        #[pyo3(signature = ())]
        fn get_attributes(&self) -> Result<HashMap<String, String>, Error> {
            Ok(self.inner.get_attributes()?)
        }

        #[pyo3(signature = (attrs))]
        fn update_attributes(&self, attrs: HashMap<String, String>) -> Result<(), Error> {
            let attrs: HashMap<&str, &str> = attrs
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();
            Ok(self.inner.update_attributes(&attrs)?)
        }

        #[pyo3(signature = ())]
        fn get_credential(&self) -> Result<Entry, Error> {
            Ok(Entry {
                inner: self.inner.get_credential()?,
            })
        }

        #[pyo3(signature = ())]
        fn get_specifiers(&self) -> Option<(String, String)> {
            self.inner.get_specifiers()
        }

        #[pyo3(signature = ())]
        fn delete_credential(&self) -> Result<(), Error> {
            Ok(self.inner.delete_credential()?)
        }

        #[staticmethod]
        #[pyo3(signature = (spec = None))]
        fn search(spec: Option<HashMap<String, String>>) -> Result<Vec<Entry>, Error> {
            let spec = keyring::internalize(spec.as_ref());
            Ok(keyring_core::Entry::search(&spec)?
                .into_iter()
                .map(|e| Entry { inner: e })
                .collect())
        }
    }

    #[pyfunction]
    fn release_store() {
        keyring_core::unset_default_store();
    }

    #[pyfunction]
    fn store_info() -> String {
        keyring::store_info()
    }

    #[pyfunction]
    #[pyo3(signature = (name, config = None))]
    fn use_named_store(name: &str, config: Option<HashMap<String, String>>) -> Result<(), Error> {
        let config = keyring::internalize(config.as_ref());
        Ok(keyring::use_named_store_with_modifiers(name, &config)?)
    }
}
