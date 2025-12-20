/*!
Test whether set or retrieved secrets are leaked into freed memory

When a client sets or retrieves a secret, their expectation is that
there are no copies of the secret data left anywhere in keyring
memory, including memory that has been released (stack or heap).

This program tests that by creating a secret, setting it into
a credential, reading it back from the credential, and
erasing all client copies. It then waits while the developer
does a core dump of the process which can then be scanned
to see if the secret (the string `super-duper-password`) is
present anywhere. If it is present, then the keyring core
or the credential manager is leaking the secret.

See the `run-...-test.bash` files in this directory for the
platform-specific shell scripts which use this program to
do leak testing.

This program expects one command line argument: the name of the
credential store to use. If none is specified, the `sample` store
is used. The `sample` store *does* leak (because it keeps all
secrets in memory), so it's a good way to test the shell scripts.
*/

use std::str::from_utf8;
use std::time::Duration;

use zeroize::Zeroize;

use keyring::{release_store, store_info, use_named_store};
use keyring_core::Entry;

fn main() {
    use_named_store(&get_arg_string(1, "sample")).unwrap();
    println!("Using store: {}", store_info());
    let master = b"s u p e r - d u p e r - p a s s w o r d";
    let mut bytes = [0u8; 20];
    for (i, b) in master.iter().enumerate() {
        if i % 2 == 1 {
            continue;
        }
        bytes[i / 2] = *b;
    }
    let entry = Entry::new("leak-test", "leak-test").unwrap();
    entry.set_password(from_utf8(&bytes).unwrap()).unwrap();
    bytes.zeroize();
    let pass_result = entry.get_password();
    let mut password = pass_result.unwrap();
    password.zeroize();
    // signal that we're ready to be scanned.
    // This leaks a string that can be scanned for to make sure scanning works
    let ready = "ok-mr-debugger-i'm-ready-for-my-close-up";
    eprintln!("Leaked string: {ready}");
    // wait while the heap is scanned
    std::thread::sleep(Duration::from_secs(get_env_int("DELAY_SECS", 10)));
    // don't leave detritus around
    entry.delete_credential().unwrap();
    release_store();
    // signal that we're done in the background
    eprintln!("Leak test program is exiting")
}

fn get_arg_string(position: usize, default: &str) -> String {
    std::env::args()
        .nth(position)
        .unwrap_or_else(|| String::from(default))
}

fn get_env_int(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .unwrap_or_else(|_| default.to_string())
        .parse()
        .unwrap_or(default)
}
