//! Connections to available keyring stores.
//!
//! This library provides sample code for each available credential store, a convenience
//! function for choosing a default credential store by name, and a convenience function
//! for releasing the default credential store.
//!
//! For each available keyring-compatible credential store (other than mock),
//! this module defines a `use_...` function which sets that store
//! as the default credential store. It also gives that store a name
//! in the [use_named_store] convenience function.
//!
//! As developers make new credential store modules available,
//! they are encouraged to submit a pull request that adds a connection here for their module,
//! both via a `use_...` function and via [use_named store].
//! (In doing so, they will also extend both the Rust-based CLI and the Python-based REPL client
//! to support the new store.)

use std::collections::HashMap;

use keyring_core::{Error, Result, get_default_store, set_default_store, unset_default_store};

const NAMED_STORES: [&str; 9] = [
    "android",
    "keychain",
    "keyutils",
    "protected",
    "sample",
    "secret-service",
    "secret-service-async",
    "sqlite",
    "windows",
];

pub fn use_named_store(name: &str) -> Result<()> {
    if name.to_lowercase().as_str() == "sample" {
        use_sample_store(&HashMap::from([("persist", "true")]))
    } else {
        use_named_store_with_modifiers(name, &HashMap::new())
    }
}

pub fn use_named_store_with_modifiers(name: &str, modifiers: &HashMap<&str, &str>) -> Result<()> {
    match name.to_lowercase().as_str() {
        "android" => use_android_native_store(modifiers),
        "keychain" => use_apple_keychain_store(modifiers),
        "keyutils" => use_linux_keyutils_store(modifiers),
        "protected" => use_apple_protected_store(modifiers),
        "sample" => use_sample_store(modifiers),
        "secret-service" | "secret-service-sync" => use_dbus_secret_service_store(modifiers),
        "secret-service-async" => use_zbus_secret_service_store(modifiers),
        "sqlite" => use_sqlite_store(modifiers),
        "windows" => use_windows_native_store(modifiers),
        _ => {
            let ok = NAMED_STORES.join(", ");
            let err = Error::Invalid(name.to_string(), format!("must be one of: {ok}"));
            Err(err)
        }
    }
}

#[allow(unused_variables)]
pub fn use_native_store(not_keyutils: bool) -> Result<()> {
    #[cfg(target_os = "android")]
    use_named_store("android")?;
    #[cfg(target_os = "macos")]
    use_named_store("keychain")?;
    #[cfg(target_os = "windows")]
    use_named_store("windows")?;
    #[cfg(target_os = "linux")]
    if not_keyutils {
        use_named_store("secret-service")?;
    } else {
        use_named_store("keyutils")?;
    }
    #[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
    use_named_store("secret-service")?;
    #[cfg(not(any(
        target_os = "android",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "windows",
    )))]
    use_named_store("sample")?;
    Ok(())
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

#[allow(unused_variables)]
pub fn use_android_native_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(target_os = "android")]
    {
        use android_native_keyring_store::Store;
        set_default_store(Store::new_with_configuration(config)?);
        Ok(())
    }
    #[cfg(not(target_os = "android"))]
    {
        Err(Error::NotSupportedByStore(
            "The Android native store is only available on Android".to_string(),
        ))
    }
}

#[allow(unused_variables)]
pub fn use_sqlite_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(not(all(target_os = "windows", target_arch = "aarch64")))]
    {
        use db_keystore::DbKeyStore;
        set_default_store(DbKeyStore::new_with_modifiers(config)?);
        Ok(())
    }
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        Err(Error::NotSupportedByStore(
            "The sqlite store is not available on Windows AArch64".to_string(),
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

/// Given an (optional) HashMap of strings, return a HashMap of references to those strings.
///
/// This is useful when key-value pairs are collected from user input.
pub fn internalize(config: Option<&HashMap<String, String>>) -> HashMap<&str, &str> {
    if let Some(config) = config {
        config
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect()
    } else {
        HashMap::new()
    }
}
