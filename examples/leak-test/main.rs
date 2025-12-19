use std::time::Duration;

use zeroize::Zeroize;

use keyring::{release_store, use_native_store};
use keyring_core::Entry;

fn main() {
    use_native_store(false).unwrap();
    let allocated_string = String::from("ok-mr-debugger-i'm-ready-for-my-close-up");
    let master = b"s u p e r - d u p e r - p a s s w o r d";
    let mut bytes = [0u8; 20];
    for (i, b) in master.iter().enumerate() {
        if i % 2 == 1 {
            continue;
        }
        bytes[i / 2] = *b;
    }
    let entry = Entry::new("leak-test", "leak-test").unwrap();
    entry.set_secret(&bytes).unwrap();
    bytes.zeroize();
    let pass_result = entry.get_password();
    let mut password = pass_result.unwrap();
    password.zeroize();
    // wait while the heap is scanned
    println!("Allocated string: {}", allocated_string);
    std::thread::sleep(Duration::from_secs(get_env_int("DELAY_SECS", 10)));
    entry.delete_credential().unwrap();
    release_store()
}

fn get_env_int(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .unwrap_or_else(|_| default.to_string())
        .parse()
        .unwrap_or(default)
}
