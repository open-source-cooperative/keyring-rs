use super::State;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize)]
pub struct HistoryEntry {
    pub id: String,
    #[serde(skip)]
    entry: Arc<keyring_core::Entry>,
    pub is_specifier: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl HistoryEntry {
    fn new(id: u32, entry: keyring_core::Entry, service: &str, user: &str) -> Self {
        HistoryEntry {
            id: format!("#{id}"),
            entry: Arc::new(entry),
            is_specifier: true,
            service: Some(service.to_owned()),
            user: Some(user.to_owned()),
        }
    }

    pub(crate) fn new_from_entry(id: u32, entry: keyring_core::Entry) -> Self {
        if let Some(specifiers) = entry.get_specifiers() {
            HistoryEntry::new(id, entry, &specifiers.0, &specifiers.1)
        } else {
            HistoryEntry {
                id: format!("#{id}"),
                entry: Arc::new(entry),
                is_specifier: false,
                service: None,
                user: None,
            }
        }
    }
}

pub type History = indexmap::IndexMap<String, HistoryEntry>;

#[tauri::command]
pub fn use_named_store(state: tauri::State<Mutex<State>>, name: String) -> Result<(), String> {
    if name.eq("sample") {
        let mods = &HashMap::from([("backing-file", "/tmp/keyring-sample.ron")]);
        keyring::use_named_store_with_modifiers(&name, mods).map_err(|e| e.to_string())?;
    } else {
        keyring::use_named_store(&name).map_err(|e| e.to_string())?;
    }
    let mut state = state.lock().unwrap();
    state.reset();
    Ok(())
}

#[tauri::command]
pub fn release_store(state: tauri::State<Mutex<State>>) {
    keyring::release_store();
    let mut state = state.lock().unwrap();
    state.reset();
}

#[tauri::command]
pub fn get_entry(state: tauri::State<Mutex<State>>, id: String) -> Result<HistoryEntry, String> {
    let state = state.lock().unwrap();
    state
        .history
        .get(&id)
        .cloned()
        .ok_or_else(|| format!("No history entry with id '{id}'"))
}

#[tauri::command]
pub fn get_all_entries(state: tauri::State<Mutex<State>>) -> Result<Vec<HistoryEntry>, String> {
    let state = state.lock().unwrap();
    Ok(state.history.values().cloned().collect())
}

#[tauri::command]
pub fn remove_entry(state: tauri::State<Mutex<State>>, id: String) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    state.history.shift_remove(&id);
    Ok(())
}

#[tauri::command]
pub fn entry_new(
    state: tauri::State<Mutex<State>>,
    service: String,
    user: String,
) -> Result<HistoryEntry, String> {
    let entry = keyring_core::Entry::new(&service, &user).map_err(|e| e.to_string())?;
    let mut state = state.lock().unwrap();
    let he = state.insert_entry(entry);
    Ok(he)
}

#[tauri::command]
pub fn entry_get_value(state: tauri::State<Mutex<State>>, id: String) -> Result<String, String> {
    let state = state.lock().unwrap();
    let he = state
        .history
        .get(&id)
        .cloned()
        .ok_or_else(|| format!("No history entry with id '{id}'"))?;
    let secret = he.entry.get_secret().map_err(|e| e.to_string())?;
    if let Ok(value) = String::from_utf8(secret.clone()) {
        Ok(format!("UTF8:{value}"))
    } else {
        Ok(format!("HEX:{}", hex::encode(&secret)))
    }
}

#[tauri::command]
pub fn entry_set_value(
    state: tauri::State<Mutex<State>>,
    id: String,
    value: String,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    let he = state
        .history
        .get(&id)
        .cloned()
        .ok_or_else(|| format!("No history entry with id '{id}'"))?;
    if let Some(value) = value.strip_prefix("HEX:") {
        let secret = hex::decode(value.as_bytes())
            .map_err(|_| "The secret value is not a valid hex string".to_string())?;
        return he.entry.set_secret(&secret).map_err(|e| e.to_string());
    }
    if let Some(value) = value.strip_prefix("UTF8:") {
        return he.entry.set_password(value).map_err(|e| e.to_string());
    }
    Err("Invalid value format. Expected 'HEX:...' or 'UTF8:...'.".to_string())
}

#[tauri::command]
pub fn entry_get_attributes(
    state: tauri::State<Mutex<State>>,
    id: String,
) -> Result<HashMap<String, String>, String> {
    let state = state.lock().unwrap();
    let he = state
        .history
        .get(&id)
        .cloned()
        .ok_or_else(|| format!("No history entry with id '{id}'"))?;
    he.entry.get_attributes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn entry_update_attributes(
    state: tauri::State<Mutex<State>>,
    id: String,
    attributes: HashMap<String, String>,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    let he = state
        .history
        .get(&id)
        .cloned()
        .ok_or_else(|| format!("No history entry with id '{id}'"))?;
    he.entry
        .update_attributes(&keyring::internalize(Some(&attributes)))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn entry_delete_value(state: tauri::State<Mutex<State>>, id: String) -> Result<(), String> {
    let state = state.lock().unwrap();
    let he = state
        .history
        .get(&id)
        .cloned()
        .ok_or_else(|| format!("No history entry with id '{id}'"))?;
    match he.entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring_core::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn search_all(state: tauri::State<Mutex<State>>) -> Result<u32, String> {
    let entries = keyring_core::Entry::search(&HashMap::new()).map_err(|e| e.to_string())?;
    let mut state = state.lock().unwrap();
    let count = entries.len() as u32;
    for entry in entries {
        state.insert_entry(entry);
    }
    Ok(count)
}
