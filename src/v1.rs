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

use std::sync::LazyLock;

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
    /// credential store could not be initialized. (See [Entry::store_status].)
    pub fn new(service: &str, username: &str) -> Result<Self> {
        if SET_CREDENTIAL_STORE_RESULT.is_err() {
            return Err(Error::NoDefaultStore);
        }
        let inner = keyring_core::Entry::new(service, username)?;
        Ok(Self { inner })
    }

    /// Return the results of the (one-time) credential store initialization that's
    /// done on the first call to [Entry::new].
    ///
    /// If this is `Ok(())`, then the credential store is available. If this is an
    /// [Invalid](Error::Invalid) error with `platform` as the invalid parameter, then it
    /// indicates that your runtime platform is not supported by this feature. Any other
    /// error is one that came back from the attempted credential store initialization.
    ///
    /// Note that calling this function will initialize the credential store if that
    /// hasn't already been done. So if you want to do runtime checking of credential
    /// store initialization without first creating an entry, you can just call this
    /// function before [Entry::new].
    pub fn store_status() -> &'static Result<()> {
        &SET_CREDENTIAL_STORE_RESULT
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

static SET_CREDENTIAL_STORE_RESULT: LazyLock<Result<()>> = LazyLock::new(set_credential_store);

fn set_credential_store() -> Result<()> {
    #[cfg(target_os = "macos")]
    let store = apple_native_keyring_store::keychain::Store::new()?;
    #[cfg(target_os = "windows")]
    let store = windows_native_keyring_store::Store::new()?;
    #[cfg(all(
        unix,
        not(any(target_os = "macos", target_os = "ios", target_os = "android"))
    ))]
    let store = zbus_secret_service_keyring_store::Store::new()?;
    #[cfg(all(any(unix, windows), not(any(target_os = "ios", target_os = "android"))))]
    {
        keyring_core::set_default_store(store);
        Ok(())
    }
    #[cfg(not(all(any(unix, windows), not(any(target_os = "ios", target_os = "android")))))]
    Err(Error::Invalid(
        "platform".to_string(),
        "must be macOS, Windows, or a non-iOS, non-Android *nix variant".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn test_new_before_store() {
        let entry = Entry::new("svc", "usr");
        #[cfg(all(any(unix, windows), not(any(target_os = "ios", target_os = "android"))))]
        assert!(entry.is_ok());
        #[cfg(not(all(any(unix, windows), not(any(target_os = "ios", target_os = "android")))))]
        assert!(entry.is_err());
        assert_eq!(entry.is_ok(), Entry::store_status().is_ok());
    }

    #[test]
    fn test_new_after_store() {
        let store = Entry::store_status();
        #[cfg(all(any(unix, windows), not(any(target_os = "ios", target_os = "android"))))]
        assert!(store.is_ok());
        #[cfg(not(all(any(unix, windows), not(any(target_os = "ios", target_os = "android")))))]
        assert!(store.is_err());
        assert_eq!(store.is_ok(), Entry::new("svc", "usr").is_ok());
    }
}
