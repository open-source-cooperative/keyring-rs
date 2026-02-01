use keyring_core::{Entry, Error, get_default_store};
use std::collections::HashMap;

type TestResult = Result<(), String>;

type TestFn = fn() -> TestResult;

pub fn run_tests() -> (i32, i32) {
    let tests: Vec<(&'static str, TestFn)> = vec![
        ("test_store_methods", test_store_methods),
        ("test_empty_service_and_user", test_empty_service_and_user),
        ("test_empty_password", test_empty_password),
        ("test_missing_entry", test_missing_entry),
        (
            "test_round_trip_ascii_password",
            test_round_trip_ascii_password,
        ),
        (
            "test_round_trip_non_ascii_password",
            test_round_trip_non_ascii_password,
        ),
        (
            "test_entries_with_same_and_different_specifiers",
            test_entries_with_same_and_different_specifiers,
        ),
        (
            "test_round_trip_random_secret",
            test_round_trip_random_secret,
        ),
        ("test_update", test_update),
        ("test_duplicate_entries", test_duplicate_entries),
        ("test_get_update_attributes", test_get_update_attributes),
        (
            "test_get_credential_and_specifiers",
            test_get_credential_and_specifiers,
        ),
        (
            "test_single_thread_create_then_move",
            test_single_thread_create_then_move,
        ),
        (
            "test_simultaneous_threads_create_then_move",
            test_simultaneous_threads_create_then_move,
        ),
        (
            "test_single_thread_create_set_then_move",
            test_single_thread_create_set_then_move,
        ),
        (
            "test_simultaneous_threads_create_set_then_move",
            test_simultaneous_threads_create_set_then_move,
        ),
        (
            "test_single_thread_move_then_create",
            test_single_thread_move_then_create,
        ),
        (
            "test_simultaneous_threads_move_then_create",
            test_simultaneous_threads_move_then_create,
        ),
        (
            "test_single_thread_multiple_create_delete",
            test_single_thread_multiple_create_delete,
        ),
        (
            "test_simultaneous_threads_multiple_create_delete",
            test_simultaneous_threads_multiple_create_delete,
        ),
        ("test_search", test_search),
    ];
    let mut successes = 0;
    let mut failures = 0;
    let mut last_failed = false;
    for (name, test) in tests {
        print_dot();
        match test() {
            Ok(()) => {
                last_failed = false;
                successes += 1;
            }
            Err(err) => {
                println!("\n    {name}: FAIL: {err}");
                last_failed = true;
                failures += 1;
            }
        }
    }
    if !last_failed {
        println!();
    }
    (successes, failures)
}

fn print_dot() {
    use std::io::Write;
    print!(". ");
    std::io::stdout().flush().unwrap();
}

fn entry_new(service: &str, user: &str) -> Result<Entry, String> {
    let s = if service.is_empty() {
        String::new()
    } else {
        format!("test-{service}")
    };
    let u = if user.is_empty() {
        String::new()
    } else {
        format!("test-{user}")
    };
    Entry::new(&s, &u).map_err(|err| {
        format!("Couldn't create entry (service: '{service}', user: '{user}'): {err:?}")
    })
}

fn generate_random_string() -> String {
    use fastrand;
    use std::iter::repeat_with;
    repeat_with(fastrand::alphanumeric).take(12).collect()
}

fn generate_random_bytes() -> Vec<u8> {
    use fastrand;
    use std::iter::repeat_with;
    repeat_with(|| fastrand::u8(..)).take(24).collect()
}

fn read_password_case(case: &str, entry: &Entry, in_pass: &str) -> TestResult {
    let out_pass = entry
        .get_password()
        .map_err(|err| format!("Can't get the password for {case}: {err:?}"))?;
    if in_pass != out_pass {
        return Err(format!(
            "Passwords don't match for {case}: set='{in_pass}', get='{out_pass}'",
        ));
    }
    Ok(())
}

fn delete_credential_case(case: &str, entry: &Entry) -> TestResult {
    entry
        .delete_credential()
        .map_err(|err| format!("Can't delete the password for {case}: {err:?}"))?;
    match entry.get_password() {
        Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Get password failure for {case}: {e}")),
        Ok(value) => Err(format!("Got a deleted password for {case}: {value}")),
    }
}

// A round-trip password test that doesn't delete the credential afterward
fn round_trip_case_no_delete(case: &str, entry: &Entry, in_pass: &str) -> TestResult {
    entry
        .set_password(in_pass)
        .map_err(|err| format!("Can't set password for {case}: {err:?}"))?;
    read_password_case(case, entry, in_pass)
}

// A round-trip password test that does delete the credential afterward
fn round_trip_case(case: &str, entry: &Entry, in_pass: &str) -> TestResult {
    round_trip_case_no_delete(case, entry, in_pass)?;
    delete_credential_case(case, entry)
}

// A round-trip secret test that does delete the credential afterward
fn test_round_trip_secret(case: &str, entry: &Entry, in_secret: &[u8]) -> TestResult {
    entry
        .set_secret(in_secret)
        .map_err(|err| format!("Can't set secret for {case}: {err:?}"))?;
    let out_secret = entry
        .get_secret()
        .map_err(|err| format!("Can't get secret for {case}: {err:?}"))?;
    if in_secret != out_secret {
        return Err(format!(
            "Secrets don't match for {case}: set='{in_secret:?}', get='{out_secret:?}'",
        ));
    }
    entry
        .delete_credential()
        .map_err(|err| format!("Can't delete credential for {case}: {err:?}"))?;
    match entry.get_secret() {
        Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Get secret failure: {e}")),
        Ok(value) => Err(format!("Got a deleted secret: {} bytes", value.len())),
    }
}

fn test_store_methods() -> TestResult {
    let store = match get_default_store() {
        Some(store) => store,
        None => return Err("Couldn't get default store".to_string()),
    };
    let vendor = store.vendor();
    if vendor.is_empty() {
        return Err("Store vendor is empty".to_string());
    }
    let id = store.id();
    if id.is_empty() {
        return Err("Store id is empty".to_string());
    }
    let description = format!("{store:?}");
    if !description.contains(&vendor) {
        return Err(format!(
            "Store debug description ({description} doesn't contain vendor: {vendor}"
        ));
    }
    if !description.contains(&id) {
        return Err(format!(
            "Store debug description ({description} doesn't contain id: {id}"
        ));
    }
    Ok(())
}

fn test_empty_service_and_user() -> TestResult {
    let name = generate_random_string();
    let in_pass = "it doesn't matter";
    round_trip_case("empty user", &entry_new(&name, "")?, in_pass)?;
    round_trip_case("empty service", &entry_new("", &name)?, in_pass)?;
    round_trip_case("empty service & user", &entry_new("", "")?, in_pass)?;
    Ok(())
}

fn test_empty_password() -> TestResult {
    let name = generate_random_string();
    let in_pass = "";
    round_trip_case("empty password", &entry_new(&name, &name)?, in_pass)?;
    Ok(())
}

fn test_missing_entry() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    match entry.get_password() {
        Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Get password failure: {e}")),
        Ok(value) => Err(format!("Got a deleted password: {value}")),
    }
}

fn test_round_trip_ascii_password() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    round_trip_case("ASCII password", &entry, "test ASCII password")
}

fn test_round_trip_non_ascii_password() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    round_trip_case("non-ASCII password", &entry, "このきれいな花は桜です")
}

fn test_entries_with_same_and_different_specifiers() -> TestResult {
    let name1 = generate_random_string();
    let name2 = generate_random_string();
    let entry1 = entry_new(&name1, &name2)?;
    let entry2 = entry_new(&name1, &name2)?;
    let entry3 = entry_new(&name2, &name1)?;
    round_trip_case_no_delete("entry3", &entry3, "pw 3")?;
    round_trip_case_no_delete("entry1", &entry1, "pw 1")?;
    read_password_case("entry2", &entry2, "pw 1")?;
    read_password_case("entry3", &entry3, "pw 3")?;
    delete_credential_case("entry1", &entry1)?;
    delete_credential_case("entry3", &entry3)?;
    match entry2.delete_credential() {
        Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Delete credential failure on 'entry2': {e}")),
        Ok(_) => Err("Delete credential entry2 succeeded but should fail!".to_string()),
    }
}

fn test_round_trip_random_secret() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    let secret = generate_random_bytes();
    test_round_trip_secret("non-ascii password", &entry, secret.as_slice())
}

fn test_update() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    round_trip_case_no_delete("initial ASCII password", &entry, "test ASCII password")?;
    round_trip_case(
        "updated non-ASCII password",
        &entry,
        "このきれいな花は桜です",
    )
}

fn test_duplicate_entries() -> TestResult {
    let name = generate_random_string();
    let entry1 = entry_new(&name, &name)?;
    let entry2 = entry_new(&name, &name)?;
    entry1
        .set_password("password for entry1")
        .map_err(|e| format!("Set password failure: {e}"))?;
    let password = entry2
        .get_password()
        .map_err(|e| format!("Get password failure: {e}"))?;
    if password != "password for entry1" {
        return Err(format!("Got wrong password: {password}"));
    };
    entry2
        .set_password("password for entry2")
        .map_err(|e| format!("Set password failure: {e}"))?;
    let password = entry1
        .get_password()
        .map_err(|e| format!("Get password failure: {e}"))?;
    if password != "password for entry2" {
        return Err(format!("Got wrong password: {password}"));
    };
    entry1
        .delete_credential()
        .map_err(|e| format!("Delete credential failure: {e}"))?;
    match entry2.delete_credential() {
        Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Delete credential failure: {e}")),
        Ok(_) => Err("Delete credential succeeded but should fail!".to_string()),
    }
}

fn test_get_update_attributes() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    match entry.get_attributes() {
        Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Get attributes failure: {e}")),
        Ok(value) => Err(format!(
            "Got attributes before creating credential: {value:?}"
        )),
    }?;
    entry
        .set_password("password for entry")
        .map_err(|e| format!("Set password failure: {e}"))?;
    let attrs = entry
        .get_attributes()
        .map_err(|e| format!("Get attributes failure: {e}"))?;
    if attrs.is_empty() {
        return Ok(());
    }
    for (key, value) in attrs.iter() {
        match entry.update_attributes(&HashMap::from([(key.as_str(), value.as_str())])) {
            Err(Error::Invalid(_, _)) => Ok(()),
            Err(e) => Err(format!("Update attributes failure on key '{key}': {e}")),
            Ok(_) => Ok(()),
        }?;
    }
    entry
        .delete_credential()
        .map_err(|e| format!("Delete credential failure: {e}"))
}

fn test_get_credential_and_specifiers() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    match entry.get_credential() {
        Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Get credential failure: {e}")),
        Ok(value) => Err(format!(
            "Got credential before creating credential: {value:?}"
        )),
    }?;
    entry.set_password("password for entry").unwrap();
    let wrapper = entry
        .get_credential()
        .map_err(|e| format!("Get credential failure: {e}"))?;
    let (service, user) = match wrapper.get_specifiers() {
        Some((service, user)) => (service, user),
        None => return Err("Specifiers not found on get_credential result".to_string()),
    };
    if service != format!("test-{name}") || user != format!("test-{name}") {
        return Err(format!(
            "Specifiers on wrapper don't match credential: service='{service}', user='{user}'"
        ));
    };
    wrapper
        .delete_credential()
        .map_err(|e| format!("Delete credential failure: {e}"))?;
    match entry.delete_credential() {
        Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Delete credential failure: {e}")),
        Ok(_) => Err("Delete credential succeeded but should fail!".to_string()),
    }
}

fn test_single_thread_create_then_move() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    let test = move || -> TestResult { round_trip_case("single-thread", &entry, "single thread") };
    let handle = std::thread::spawn(test);
    match handle.join() {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("Err on thread: {e}")),
        Err(e) => Err(format!("Thread failure: {e:?}")),
    }
}

fn test_simultaneous_threads_create_then_move() -> TestResult {
    let mut handles = vec![];
    for i in 0..10 {
        let name = format!("{}-{}", generate_random_string(), i);
        let entry = entry_new(&name, &name)?;
        let test =
            move || -> TestResult { round_trip_case(&format!("thread {i}"), &entry, "pw {i}") };
        handles.push(std::thread::spawn(test))
    }
    for handle in handles {
        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(format!("Err on thread: {e}")),
            Err(e) => Err(format!("Thread failure: {e:?}")),
        }?
    }
    Ok(())
}

fn test_single_thread_create_set_then_move() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    let password = "pw1";
    entry
        .set_password(password)
        .map_err(|e| format!("Set password failure: {e}"))?;
    let test = move || -> TestResult {
        read_password_case("single-thread", &entry, password)?;
        delete_credential_case("single-thread", &entry)
    };
    let handle = std::thread::spawn(test);
    match handle.join() {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("Err on thread: {e}")),
        Err(e) => Err(format!("Thread failure: {e:?}")),
    }
}

fn test_simultaneous_threads_create_set_then_move() -> TestResult {
    let mut handles = vec![];
    for i in 0..10 {
        let name = format!("{}-{}", generate_random_string(), i);
        let entry = entry_new(&name, &name)?;
        let password = format!("pw {i}");
        entry
            .set_password(&password)
            .map_err(|e| format!("Set password failure on thread {i}: {e}"))?;
        let test = move || -> TestResult {
            read_password_case(&format!("thread {i}"), &entry, &password)?;
            delete_credential_case(&format!("thread {i}"), &entry)
        };
        handles.push(std::thread::spawn(test))
    }
    for handle in handles {
        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(format!("Err on thread: {e}")),
            Err(e) => Err(format!("Thread failure: {e:?}")),
        }?
    }
    Ok(())
}

fn test_single_thread_move_then_create() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    let test = move || -> TestResult { round_trip_case("single-thread", &entry, "single thread") };
    let handle = std::thread::spawn(test);
    match handle.join() {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("Err on thread: {e}")),
        Err(e) => Err(format!("Thread failure: {e:?}")),
    }
}

fn test_simultaneous_threads_move_then_create() -> TestResult {
    let mut handles = vec![];
    for i in 0..10 {
        let name = format!("{}-{}", generate_random_string(), i);
        let test = move || -> TestResult {
            let entry = entry_new(&name, &name)?;
            round_trip_case(&format!("thread {i}"), &entry, "pw {i}")
        };
        handles.push(std::thread::spawn(test))
    }
    for handle in handles {
        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(format!("Err on thread: {e}")),
            Err(e) => Err(format!("Thread failure: {e:?}")),
        }?
    }
    Ok(())
}

fn test_single_thread_multiple_create_delete() -> TestResult {
    let name = generate_random_string();
    let entry = entry_new(&name, &name)?;
    let repeats = 10;
    let test = move || -> TestResult {
        for j in 0..repeats {
            round_trip_case(&format!("pass {j}"), &entry, &format!("pw {j}"))?
        }
        Ok(())
    };
    let handle = std::thread::spawn(test);
    match handle.join() {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(format!("Err on thread: {e}")),
        Err(e) => Err(format!("Thread failure: {e:?}")),
    }
}

fn test_simultaneous_threads_multiple_create_delete() -> TestResult {
    let mut handles = vec![];
    for i in 0..10 {
        let name = format!("{}-{i}", generate_random_string());
        let test = move || -> TestResult {
            let name = format!("{name}-{i}");
            let entry = entry_new(&name, &name)?;
            let repeats = 10;
            for j in 0..repeats {
                let pw = format!("pw {i}-{j}");
                round_trip_case(&format!("thread {i} pass {j}"), &entry, &pw)?;
            }
            Ok(())
        };
        handles.push(std::thread::spawn(test))
    }
    for handle in handles {
        match handle.join() {
            Ok(Ok(_)) => Ok(()),
            Ok(Err(e)) => Err(format!("Err on thread: {e}")),
            Err(e) => Err(format!("Thread failure: {e:?}")),
        }?
    }
    Ok(())
}

fn test_search() -> TestResult {
    let store = match get_default_store() {
        Some(store) => store,
        None => return Err("Couldn't get default store".to_string()),
    };
    let all = match store.search(&HashMap::new()) {
        Ok(all) => all,
        Err(Error::NotSupportedByStore(_)) => return Ok(()),
        Err(e) => return Err(format!("Search failure: {e}")),
    };
    let count = all.len();
    let name1 = format!("{}-1", generate_random_string());
    let entry = entry_new(&name1, &name1)?;
    entry
        .set_password("pw 1")
        .map_err(|e| format!("Set password failure: {e}"))?;
    let all = match store.search(&HashMap::new()) {
        Ok(all) => all,
        Err(Error::NotSupportedByStore(_)) => return Ok(()),
        Err(e) => return Err(format!("Search failure: {e}")),
    };
    if all.len() != count + 1 {
        return Err(format!("Expected {} entries, got {}", count + 1, all.len()));
    }
    entry
        .delete_credential()
        .map_err(|e| format!("Delete credential failure: {e}"))?;
    let all = match store.search(&HashMap::new()) {
        Ok(all) => all,
        Err(Error::NotSupportedByStore(_)) => return Ok(()),
        Err(e) => return Err(format!("Search failure: {e}")),
    };
    if all.len() != count {
        return Err(format!("Expected {} entries, got {}", count + 1, all.len()));
    }
    Ok(())
}
