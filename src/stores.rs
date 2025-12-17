//! Connections to available keyring stores.
//!
//! The connectors in this module serve as sample code and are also used both by the
//! Rust-based CLI and the Python-based REPL client.
//!
//! For each available keyring-compatible credential store (other than mock),
//! this module defines a `use_...` function which sets that store
//! as the default credential store. It also associates that store with a name
//! that can be used in the [use_named_store] function to use that store as the default.
//!
//! As developers make new credential store modules available,
//! they are encouraged to submit a pull request that adds a connection here for their module,
//! both via a `use_...` function and via [use_named store].
//! (In doing so, they will also extend both the Rust-based CLI and the Python-based REPL client
//! to support the new store.)

use std::collections::HashMap;

use keyring_core::{Error, Result, get_default_store, set_default_store, unset_default_store};

pub fn use_named_store(name: &str) -> Result<()> {
    use_named_store_with_modifiers(name, &HashMap::new())
}

pub fn use_named_store_with_modifiers(name: &str, modifiers: &HashMap<&str, &str>) -> Result<()> {
    match name.to_lowercase().as_str() {
        "sample" => use_sample_store(modifiers),
        "keychain" => use_apple_keychain_store(modifiers),
        "protected" => use_apple_protected_store(modifiers),
        "keyutils" => use_linux_keyutils_store(modifiers),
        "secret-service" | "secret-service-sync" => use_dbus_secret_service_store(modifiers),
        "secret-service-async" => use_zbus_secret_service_store(modifiers),
        "windows" => use_windows_native_store(modifiers),
        _ => {
            let names = [
                "sample",
                "keychain",
                "protected",
                "keyutils",
                "secret-service",
                "secret-service-async",
                "windows",
            ];
            let ok = names.join(", ");
            let err = Error::Invalid(name.to_string(), format!("must be one of: {ok}"));
            Err(err)
        }
    }
}

pub fn use_sample_store(config: &HashMap<&str, &str>) -> Result<()> {
    use keyring_core::sample::Store;
    set_default_store(Store::new_with_configuration(config)?);
    Ok(())
}

#[allow(unused_variables)]
pub fn use_apple_keychain_store(config: &HashMap<&str, &str>) -> Result<()> {
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
pub fn use_apple_protected_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        use apple_native_keyring_store::protected::Store;
        set_default_store(Store::new_with_configuration(config)?);
        Ok(())
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
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
