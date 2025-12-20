/*!
Set or clear three credentials in the "OS-native" store.

Invoke with no arguments for usage information.
*/

use keyring::{release_store, use_native_store};
use keyring_core::Entry;
use std::env::args;
use std::iter::repeat_with;

const USAGE: &str = "\
usage: set-or-clear-three [-s] [prefix] operation
       where operation is one of: set, clear

This is a very simple utility that allows examining what keyring-created
credentials look like in your OS-native credential viewer.

If you don't specify a prefix, then the created credentials will have these specifiers:

- Service: svc1, User: usr1
- Service: svc2, User: usr2
- Service: svc3, User: usr3

If you do specify a prefix, then the prefix will be prepended to the service and user names.
The first two creds will have string passwords; the third will have a random secret.

The OS-native credential stores are:
- macOS: the login keychain
- Windows: the Credential Manager
- Linux: the kernel keyutils (unless you specify `-s` to choose the Secret Service)
- FreeBSD and OpenBSD: the Secret Service
- Other: the sample credential store with a backing file of `keyring-sample-data.ron`
";

fn main() {
    let mut args = args().skip(1).collect::<Vec<_>>();
    let mut not_keyutils = false;
    if !args.is_empty() && args[0] == "-s" {
        not_keyutils = true;
        args.remove(0);
    }
    use_native_store(not_keyutils).unwrap();
    let mut pfx = "".to_string();
    if args.len() > 1 {
        pfx = args.remove(0);
    }
    let (s1, s2, s3) = (
        pfx.clone() + "svc1",
        pfx.clone() + "svc2",
        pfx.clone() + "svc3",
    );
    let (u1, u2, u3) = (
        pfx.clone() + "usr1",
        pfx.clone() + "usr2",
        pfx.clone() + "usr3",
    );
    let entry1 = Entry::new(&s1, &u1).unwrap();
    let entry2 = Entry::new(&s2, &u2).unwrap();
    let entry3 = Entry::new(&s3, &u3).unwrap();
    if args.len() != 1 {
        args.push("--help".to_string());
    }
    match args[0].to_ascii_lowercase().as_str() {
        "s" | "set" | "store" => {
            entry1.set_password("Entry 1 password").unwrap();
            entry2.set_secret("Entry 2 password".as_bytes()).unwrap();
            let random_bytes: Vec<u8> = repeat_with(|| fastrand::u8(..)).take(16).collect();
            entry3.set_secret(&random_bytes).unwrap();
            println!("Three credentials set: {s1}/{u1}, {s2}/{u2}, {s3}/{u3}");
        }
        "c" | "clear" | "u" | "unset" => {
            entry1.delete_credential().unwrap();
            entry2.delete_credential().unwrap();
            entry3.delete_credential().unwrap();
            println!("Three credentials cleared: {s1}/{u1}, {s2}/{u2}, {s3}/{u3}")
        }
        _ => {
            print!("{USAGE}");
            std::process::exit(1);
        }
    }
    release_store();
}
