//! Keyring access utilities.

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
