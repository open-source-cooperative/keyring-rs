//! Connections to available keyring stores.
//!
//! The connectors in this module serve as sample code and are also used both by the
//! Rust-based CLI and the Python-based REPL client.
//!
//! For each available keyring-compatible credential store (other than mock),
//! this module defines a `use_...` function which sets that store
//! as the default credential store. As developers make new credential store modules available,
//! they are encouraged to submit a pull request that adds a connection here for their module.
//! (When doing so, they should also extend both the Rust-base CLI and the Python-based REPL client
//! to support the new store.)

use std::collections::HashMap;

use keyring_core::{Error, Result, get_default_store, set_default_store, unset_default_store};

pub fn use_sample_store(config: &HashMap<&str, &str>) -> Result<()> {
    use keyring_core::sample::Store;
    set_default_store(Store::new_with_configuration(config)?);
    Ok(())
}

#[allow(unused_variables)]
pub fn use_apple_native_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        use apple_native_keyring_store::keychain::Store;
        set_default_store(Store::new_with_configuration(config)?);
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        Err(Error::NotSupportedByStore(
            "The macOS keychain is only available on macOS".to_string(),
        ))
    }
}

#[allow(unused_variables)]
pub fn use_linux_keyutils_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        use linux_keyutils_keyring_store::Store;
        set_default_store(Store::new_with_configuration(config)?);
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(Error::NotSupportedByStore(
            "The keyutils store is only available on Linux".to_string(),
        ))
    }
}

#[allow(unused_variables)]
pub fn use_dbus_secret_service_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        use dbus_secret_service_keyring_store::Store;
        set_default_store(Store::new_with_configuration(config)?);
        Ok(())
    }
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        Err(Error::NotSupportedByStore(
            "The dbus Secret Service store is only available on Linux and FreeBSD".to_string(),
        ))
    }
}

#[allow(unused_variables)]
pub fn use_zbus_secret_service_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        use zbus_secret_service_keyring_store::Store;
        set_default_store(Store::new_with_configuration(config)?);
        Ok(())
    }
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        Err(Error::NotSupportedByStore(
            "The zbus Secret Service store is only available on Linux and FreeBSD".to_string(),
        ))
    }
}

#[allow(unused_variables)]
pub fn use_windows_native_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use windows_native_keyring_store::Store;
        set_default_store(Store::new_with_configuration(config)?);
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err(Error::NotSupportedByStore(
            "The Windows credential store is only available on Windows".to_string(),
        ))
    }
}

pub fn release_store() {
    unset_default_store();
}

pub fn store_info() -> String {
    if let Some(store) = get_default_store() {
        format!("{store:?}")
    } else {
        "None".to_string()
    }
}
