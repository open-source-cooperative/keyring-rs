//! Python bindings for the Rust keyring_core crate.
use pyo3::prelude::*;

#[pymodule]
mod rust_native_keyring {
    use std::collections::HashMap;

    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;

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
            let modifiers = crate::internalize(modifiers.as_ref());
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
            let spec = crate::internalize(spec.as_ref());
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
        crate::stores::store_info()
    }

    #[pyfunction]
    #[pyo3(signature = (config = None))]
    fn use_sample_store(config: Option<HashMap<String, String>>) -> Result<(), Error> {
        let config = crate::internalize(config.as_ref());
        Ok(crate::stores::use_sample_store(&config)?)
    }

    #[pyfunction]
    #[pyo3(signature = (config = None))]
    fn use_apple_native_store(config: Option<HashMap<String, String>>) -> Result<(), Error> {
        #[allow(unused_variables)]
        let config = crate::internalize(config.as_ref());
        #[cfg(target_os = "macos")]
        {
            Ok(crate::stores::use_apple_native_store(&config)?)
        }
        #[cfg(not(target_os = "macos"))]
        {
            Err(Error(keyring_core::Error::NotSupportedByStore(
                "The macOS keychain is only available on macOS".to_string(),
            )))
        }
    }

    #[pyfunction]
    #[pyo3(signature = (config = None))]
    fn use_linux_keyutils_store(config: Option<HashMap<String, String>>) -> Result<(), Error> {
        #[allow(unused_variables)]
        let config = crate::internalize(config.as_ref());
        #[cfg(target_os = "linux")]
        {
            Ok(crate::stores::use_linux_keyutils_store(&config)?)
        }
        #[cfg(not(target_os = "linux"))]
        {
            Err(Error(keyring_core::Error::NotSupportedByStore(
                "The keyutils store is only available on Linux".to_string(),
            )))
        }
    }

    #[pyfunction]
    #[pyo3(signature = (config = None))]
    fn use_dbus_secret_service_store(config: Option<HashMap<String, String>>) -> Result<(), Error> {
        #[allow(unused_variables)]
        let config = crate::internalize(config.as_ref());
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            Ok(crate::stores::use_dbus_secret_service_store(&config)?)
        }
        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
        {
            Err(Error(keyring_core::Error::NotSupportedByStore(
                "The dbus Secret Service store is only available on Linux and FreeBSD".to_string(),
            )))
        }
    }

    #[pyfunction]
    #[pyo3(signature = (config = None))]
    fn use_zbus_secret_service_store(config: Option<HashMap<String, String>>) -> Result<(), Error> {
        #[allow(unused_variables)]
        let config = crate::internalize(config.as_ref());
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        {
            Ok(crate::stores::use_zbus_secret_service_store(&config)?)
        }
        #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
        {
            Err(Error(keyring_core::Error::NotSupportedByStore(
                "The zbus Secret Service store is only available on Linux and FreeBSD".to_string(),
            )))
        }
    }

    #[pyfunction]
    #[pyo3(signature = (config = None))]
    fn use_windows_native_store(config: Option<HashMap<String, String>>) -> Result<(), Error> {
        #[allow(unused_variables)]
        let config = crate::internalize(config.as_ref());
        #[cfg(target_os = "windows")]
        {
            Ok(crate::stores::use_windows_native_store(&config)?)
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(Error(keyring_core::Error::NotSupportedByStore(
                "The Windows credential store is only available on Windows".to_string(),
            )))
        }
    }
}
