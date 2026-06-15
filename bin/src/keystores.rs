

/// Use a cross-platform encrypted sqlite (Turso) database.
///
/// Fails with a `NotSupportedByStore` error on Windows/AArch64
/// until the Turso release supports it.
#[allow(unused_variables)]
pub fn use_sqlite_store(config: &HashMap<&str, &str>) -> Result<()> {
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        use db_keystore::DbKeyStore;
        set_default_store(DbKeyStore::new_with_modifiers(config)?);
        Ok(())
    }
    #[cfg(any(target_os = "ios", target_os = "android"))]
    {
        Err(Error::NotSupportedByStore(
            "The sqlite store is not available on iOS or Android".to_string(),
        ))
    }
}


/// Set the default store to the `keyring-core::Sample` store.
///
/// This is available on all platforms.
pub fn use_sample_store(config: &HashMap<&str, &str>) -> Result<()> {
    use keyring_core::sample::Store;
    set_default_store(Store::new_with_configuration(config)?);
    Ok(())
}
