use std::sync::Mutex;
use tauri::Manager;

mod history;
use history::{History, HistoryEntry};

struct State {
    next_entry_id: u32,
    history: History,
}

impl Default for State {
    fn default() -> Self {
        Self {
            next_entry_id: 1,
            history: History::new(),
        }
    }
}

impl State {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn insert_entry(&mut self, entry: keyring_core::Entry) -> HistoryEntry {
        let he = HistoryEntry::new_from_entry(self.next_entry_id, entry);
        self.next_entry_id += 1;
        self.history.insert(he.id.clone(), he.clone());
        he
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_qualifications)]
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            app.manage(Mutex::new(State::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            history::use_named_store,
            history::release_store,
            history::store_info,
            history::get_entry,
            history::get_all_entries,
            history::remove_entry,
            history::entry_new,
            history::entry_set_value,
            history::entry_get_value,
            history::entry_get_attributes,
            history::entry_update_attributes,
            history::entry_delete_value,
            history::search_all,
        ])
        .run(tauri::generate_context!())
        .expect("valueError while running tauri application");
}
