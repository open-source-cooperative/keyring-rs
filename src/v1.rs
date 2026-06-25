//! A simple, all-in-one password/secret manager.
//!
//! This module provides a simple interface for setting and getting text or binary secrets
//! on macOS, Windows, and *nix operating systems. The provided interface is a subset of
//! the functionality provided by the [keyring-core
//! library](https://crates.io/crates/keyring-core), and an appropriate platform-specific
//! credential store is automatically chosen and managed.
//!
//! * On macOS, the secure credential store is Keychain Services.
//! * On Windows, the secure credential store is the Windows Credential Manager.
//! * On *nix operating systems, the secure credential store is the Secret Service.
//!
//! Linking your application with this crate using the `v1` feature allows your app
//! to use the exported [Entry] type to set, get, and delete secrets.
//!
//! If you need more functionality than provided by this library, you need to store
//! secrets on other platforms, or you would prefer to use a different credential store,
//! then don't use this library. Instead, link your application directly with the
//! [keyring-core](https://crates.io/crates/keyring-core) library and any desired
//! credential stores. You will find lots of high-quality, maintained sample code for how
//! to do this in the [cli](super::cli) module, and lots of documentation about how to do
//! it in the [Keyring
//! wiki](https://github.com/open-source-cooperative/keyring-rs/wiki/Keyring).

use std::sync::{Mutex, Once};

pub use keyring_core::{Error, Result};

/// A named entry in a credential store.
///
/// The [Entry] objects defined here are simply wrappers for the [Entry](keyring_core::Entry) objects
/// defined by the [keyring-core library](https://crates.io/crates/keyring-core). We do this
/// to make use of the functionality of entries while overriding their constructor.
pub struct Entry {
    pub inner: keyring_core::Entry,
}

impl Entry {
    /// Create a new entry in the platform-specific credential store.
    ///
    /// For details about the service and username arguments, see the
    /// [Keyring wiki](https://github.com/open-source-cooperative/keyring-rs/wiki/Keyring).
    ///
    /// For details about possible errors, see [keyring_core::Entry::new]. If you get a
    /// [NoDefaultStore](Error::NoDefaultStore) error, it means that the platform-specific
    /// credential store could not be initialized.
    pub fn new(service: &str, username: &str) -> Result<Self> {
        SET_CREDENTIAL_STORE.call_once(set_credential_store);
        let mut guard = SET_CREDENTIAL_STORE_RESULT.lock().unwrap();
        if guard.is_none() {
            let inner = keyring_core::Entry::new(service, username)?;
            Ok(Self { inner })
        } else {
            Err(guard.take().unwrap())
        }
    }

    /// Set the password for this entry.
    ///
    /// See [keyring_core::Entry::set_password] for details.
    pub fn set_password(&self, password: &str) -> Result<()> {
        self.inner.set_password(password)
    }

    /// Set the secret for this entry.
    ///
    /// See [keyring_core::Entry::set_secret] for details.
    pub fn set_secret(&self, secret: &[u8]) -> Result<()> {
        self.inner.set_secret(secret)
    }

    /// Get the password for this entry.
    ///
    /// See [keyring_core::Entry::get_password] for details.
    pub fn get_password(&self) -> Result<String> {
        self.inner.get_password()
    }

    /// Get the secret for this entry.
    ///
    /// See [keyring_core::Entry::get_secret] for details.
    pub fn get_secret(&self) -> Result<Vec<u8>> {
        self.inner.get_secret()
    }

    /// Delete the credential associated with this entry.
    ///
    /// See [keyring_core::Entry::delete_credential] for details.
    pub fn delete_credential(&self) -> Result<()> {
        self.inner.delete_credential()
    }
}

static SET_CREDENTIAL_STORE_RESULT: Mutex<Option<Error>> = Mutex::new(None);
static SET_CREDENTIAL_STORE: Once = Once::new();

fn set_credential_store() {
    let inner = || -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            let store = apple_native_keyring_store::keychain::Store::new()?;
            keyring_core::set_default_store(store)
        }
        #[cfg(target_os = "windows")]
        {
            let store = windows_native_keyring_store::Store::new()?;
            keyring_core::set_default_store(store);
        }
        #[cfg(all(
            unix,
            not(any(target_os = "macos", target_os = "ios", target_os = "android"))
        ))]
        {
            let store = zbus_secret_service_keyring_store::Store::new()?;
            keyring_core::set_default_store(store);
        }
        Ok(())
    };
    match inner() {
        Ok(()) => {}
        Err(err) => {
            let mut result = SET_CREDENTIAL_STORE_RESULT.lock().unwrap();
            *result = Some(err);
        }
    }
}
