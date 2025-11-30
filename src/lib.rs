//! Keyring access utilities.
//!
//! The [stores] module has functions for choosing an available credential store.
//! It's used both by the Rust-based CLI and the Python-based REPL client.
//!
//! The [python] module defines a Python wrapper over keyring core functionality. This
//! module is available from PyPI via the
//! [rust-native-keyring project](https://pypi.org/project/rust-native-keyring/).
//! Imported into a REPL, it makes for a much better testing and discovery tool
//! than the Rust-based CLI.

use std::collections::HashMap;

pub mod python;
pub mod stores;

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
