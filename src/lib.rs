/*!

# Keyring

This is a cross-platform library that does storage and retrieval of passwords
(or other secrets) in an underlying platform-specific secure store.
A top-level introduction to the library's usage, as well as a small code sample,
may be found in [the library's entry on crates.io](https://crates.io/crates/keyring).
Currently supported platforms are
Linux,
FreeBSD,
OpenBSD,
Windows,
macOS, and iOS.

## Design

This crate implements a very simple, platform-independent concrete object called an _entry_.
Each entry is identified by a <_service name_, _user name_> pair of UTF-8 strings,
optionally augmented by a _target_ string (which can be used to distinguish two entries
that have the same _service name_ and _user name_).
Entries support setting, getting, and forgetting (aka deleting) passwords (UTF-8 strings).

Entries provide persistence for their passwords by wrapping credentials held in platform-specific
credential stores.  The implementations of these platform-specific stores are captured
in two types (with associated traits):

- a _credential builder_, represented by the [CredentialBuilder] type
(and [CredentialBuilderApi](credential::CredentialBuilderApi) trait).  Credential
builders are given the identifying information provided for an entry and maps
it to the identifying information for a matching platform-specific credential.
- a _credential_, represented by the [Credential] type
(and [CredentialApi](credential::CredentialApi) trait).  The platform-specific credential
identified by a builder for an entry is what provides the secure storage for that entry's password.

## Crate-provided Credential Stores

This crate runs on several different platforms, and it provides one
or more implementations of credential stores on each platform.
These implementations work by mapping the data used to identify an entry
to data used to identify platform-specific storage objects.
For example, on macOS, the service and user names provided for an entry
are mapped to the service and user attributes that identify an element
in the macOS keychain.

Typically, platform-specific stores have a richer model of an entry than
the one used by this crate.  They expose their specific model in the
concrete credential objects they use to implement the Credential trait.
In order to allow clients to access this richer model, the Credential trait
has an [as_any](credential::CredentialApi::as_any) method that returns a
reference to the underlying
concrete object typed as [Any](std::any::Any), so that it can be downgraded to
its concrete type.

## Client-provided Credential Stores

In addition to the platform stores implemented by this crate, clients
are free to provide their own secure stores and use those.  There are
two mechanisms provided for this:

- Clients can give their desired credential builder to the crate
for use by the [Entry::new] and [Entry::new_with_target] calls.
This is done by making a call to [set_default_credential_builder].
The major advantage of this approach is that client code remains
independent of the credential builder being used.
- Clients can construct their concrete credentials directly and
then turn them into entries by using the [Entry::new_with_credential]
call. The major advantage of this approach is that credentials
can be identified however clients want, rather than being restricted
to the simple model used by this crate.

## Mock Credential Store

In addition to the platform-specific credential stores, this crate
also provides a mock credential store that clients can use to
test their code in a platform independent way.  The mock credential
store allows for pre-setting errors as well as password values to
be returned from [Entry] method calls.

## Interoperability with Third Parties

Each of the credential stores provided by this crate uses an underlying
platform-specific store that may also be used by modules written
in other languages.  If you want to interoperate with these third party
credential writers, then you will need to understand the details of how the
target, service name, and user name of this crate's generic model
are used to identify credentials in the platform-specific store.
These details are in the implementation of this crate's secure-storage
modules, and are documented in the headers of those modules.

(_N.B._ Since the included credential store implementations are platform-specific,
you may need to use the Platform drop-down on [docs.rs](https://docs.rs/keyring) to
view the storage module documentation for your desired platform.)

## Caveats

This module manipulates passwords as UTF-8 encoded strings,
so if a third party has stored an arbitrary byte string
then retrieving that password will return a [BadEncoding](Error::BadEncoding) error.
The returned error will have the raw bytes attached,
so you can access them.

While this crate's code is thread-safe,
accessing the _same_ entry from multiple threads
in close proximity may be unreliable (especially on Windows),
in that the underlying platform
store may actually execute those calls in a different
order than they are made. As long as you access a single entry from
only one thread at a time, multi-threading should be fine.

(N.B. Creating an entry is not the same as accessing it, because
entry creation doesn't go through the platform credential manager.
It's fine to create an entry on one thread and then immediately use
it on a different thread.  This is thoroughly tested on all platforms.)
 */
use std::collections::HashMap;

pub use credential::{Credential, CredentialBuilder};
pub use error::{Error, Result};
use keyring_search::{CredentialSearchResult, Error as SearchError, Search};

// Included keystore implementations and default choice thereof.

pub mod mock;

#[cfg(all(target_os = "linux", feature = "linux-keyutils"))]
pub mod keyutils;
#[cfg(all(
    target_os = "linux",
    feature = "secret-service",
    not(feature = "linux-no-secret-service")
))]
pub mod secret_service;
#[cfg(all(
    target_os = "linux",
    feature = "secret-service",
    not(feature = "linux-default-keyutils")
))]
use crate::secret_service as default;
#[cfg(all(
    target_os = "linux",
    feature = "linux-keyutils",
    any(feature = "linux-default-keyutils", not(feature = "secret-service"))
))]
use keyutils as default;
#[cfg(all(
    target_os = "linux",
    not(feature = "secret-service"),
    not(feature = "linux-keyutils")
))]
use mock as default;

#[cfg(all(target_os = "freebsd", feature = "secret-service"))]
pub mod secret_service;
#[cfg(all(target_os = "freebsd", feature = "secret-service"))]
use crate::secret_service as default;
#[cfg(all(target_os = "freebsd", not(feature = "secret-service")))]
use mock as default;

#[cfg(all(target_os = "openbsd", feature = "secret-service"))]
pub mod secret_service;
#[cfg(all(target_os = "openbsd", feature = "secret-service"))]
use crate::secret_service as default;
#[cfg(all(target_os = "openbsd", not(feature = "secret-service")))]
use mock as default;

#[cfg(all(target_os = "macos", feature = "platform-macos"))]
pub mod macos;
#[cfg(all(target_os = "macos", feature = "platform-macos"))]
use macos as default;
#[cfg(all(target_os = "macos", not(feature = "platform-macos")))]
use mock as default;

#[cfg(all(target_os = "windows", feature = "platform-windows"))]
pub mod windows;
#[cfg(all(target_os = "windows", not(feature = "platform-windows")))]
use mock as default;
#[cfg(all(target_os = "windows", feature = "platform-windows"))]
use windows as default;

#[cfg(all(target_os = "ios", feature = "platform-ios"))]
pub mod ios;
#[cfg(all(target_os = "ios", feature = "platform-ios"))]
use ios as default;
#[cfg(all(target_os = "ios", not(feature = "platform-ios")))]
use mock as default;

#[cfg(not(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "macos",
    target_os = "ios",
    target_os = "windows",
)))]
use mock as default;

pub mod credential;
pub mod error;

#[derive(Default, Debug)]
struct EntryBuilder {
    inner: Option<Box<CredentialBuilder>>,
}

static DEFAULT_BUILDER: std::sync::RwLock<EntryBuilder> =
    std::sync::RwLock::new(EntryBuilder { inner: None });

/// Set the credential builder used by default to create entries.
///
/// This is really meant for use by clients who bring their own credential
/// store and want to use it everywhere.  If you are using multiple credential
/// stores and want precise control over which credential is in which store,
/// then use [new_with_credential](Entry::new_with_credential).
///
/// This will block waiting for all other threads currently creating entries
/// to complete what they are doing. It's really meant to be called
/// at app startup before you start creating entries.
pub fn set_default_credential_builder(new: Box<CredentialBuilder>) {
    let mut guard = DEFAULT_BUILDER
        .write()
        .expect("Poisoned RwLock in keyring-rs: please report a bug!");
    guard.inner = Some(new);
}

fn build_default_credential(target: Option<&str>, service: &str, user: &str) -> Result<Entry> {
    lazy_static::lazy_static! {
        static ref DEFAULT: Box<CredentialBuilder> = default::default_credential_builder();
    }
    let guard = DEFAULT_BUILDER
        .read()
        .expect("Poisoned RwLock in keyring-rs: please report a bug!");
    let builder = match guard.inner.as_ref() {
        Some(builder) => builder,
        None => &DEFAULT,
    };
    let credential = builder.build(target, service, user)?;
    Ok(Entry { inner: credential })
}

fn default_search(service: bool, target: bool, user: bool, query: &str) -> CredentialSearchResult {
    let search = match Search::new() {
        Ok(search) => search,
        Err(err) => return Err(SearchError::SearchError(err.to_string())),
    };

    if service {
        search.by_service(query)
    } else if target {
        search.by_target(query)
    } else if user {
        search.by_user(query)
    } else {
        let mut results = vec![];
        if let Ok(service_result) = search.by_service(query) {
            results.push(service_result)
        }
        if let Ok(target_result) = search.by_target(query) {
            results.push(target_result)
        }
        if let Ok(user_result) = search.by_user(query) {
            results.push(user_result)
        }
        // More than 1 result, check for duplicates
        if results.len() > 1 {
            for index in 0..results.len() - 1 {
                if results[0] == results[index] {
                    results.remove(index);
                }
            }
        }

        let mut final_result: HashMap<String, HashMap<String, String>> = HashMap::new();
        for result in results {
            final_result.extend(result);
        }

        Ok(final_result)
    }
}

#[derive(Debug)]
pub struct Entry {
    inner: Box<Credential>,
}

impl Entry {
    /// Create an entry for the given service and user.
    ///
    /// The default credential builder is used.
    pub fn new(service: &str, user: &str) -> Result<Entry> {
        build_default_credential(None, service, user)
    }

    /// Create an entry for the given target, service, and user.
    ///
    /// The default credential builder is used.
    pub fn new_with_target(target: &str, service: &str, user: &str) -> Result<Entry> {
        build_default_credential(Some(target), service, user)
    }

    /// Create an entry that uses the given platform credential for storage.
    pub fn new_with_credential(credential: Box<Credential>) -> Entry {
        Entry { inner: credential }
    }

    /// Set the password for this entry.
    ///
    /// Can return an [Ambiguous](Error::Ambiguous) error
    /// if there is more than one platform credential
    /// that matches this entry.  This can only happen
    /// on some platforms, and then only if a third-party
    /// application wrote the ambiguous credential.
    pub fn set_password(&self, password: &str) -> Result<()> {
        self.inner.set_password(password)
    }

    /// Retrieve the password saved for this entry.
    ///
    /// Returns a [NoEntry](Error::NoEntry) error if there isn't one.
    ///
    /// Can return an [Ambiguous](Error::Ambiguous) error
    /// if there is more than one platform credential
    /// that matches this entry.  This can only happen
    /// on some platforms, and then only if a third-party
    /// application wrote the ambiguous credential.
    pub fn get_password(&self) -> Result<String> {
        self.inner.get_password()
    }

    /// Delete the password for this entry.
    ///
    /// Returns a [NoEntry](Error::NoEntry) error if there isn't one.
    ///
    /// Can return an [Ambiguous](Error::Ambiguous) error
    /// if there is more than one platform credential
    /// that matches this entry.  This can only happen
    /// on some platforms, and then only if a third-party
    /// application wrote the ambiguous credential.
    pub fn delete_password(&self) -> Result<()> {
        self.inner.delete_password()
    }

    /// Return a reference to this entry's wrapped credential.
    ///
    /// The reference is of the [Any](std::any::Any) type so it can be
    /// downgraded to a concrete credential object.  The client must know
    /// what type of concrete object to cast to.
    pub fn get_credential(&self) -> &dyn std::any::Any {
        self.inner.as_any()
    }

    /// Default search method.
    ///
    /// Takes in a query and searches all possible options,
    /// filtering out duplicate results and performing the most
    /// broad search.
    pub fn search(query: &str) -> CredentialSearchResult {
        default_search(false, false, false, query)
    }

    /// Search credential services.
    ///
    /// Only searches based on the service a credential was
    /// created under.
    pub fn search_services(query: &str) -> CredentialSearchResult {
        default_search(true, false, false, query)
    }

    /// Search credential targets.
    ///
    /// Only searches based on the target a credential was
    /// created under.
    pub fn search_targets(query: &str) -> CredentialSearchResult {
        default_search(false, true, false, query)
    }

    /// Search credential users.
    ///
    /// Only searches based on the username a credential was
    /// created under.
    pub fn search_users(query: &str) -> CredentialSearchResult {
        default_search(false, false, true, query)
    }

    /// Create entry from search results.
    ///
    /// Pass a `&CredentialSearchResult` and the ID to the credential.
    /// `CredentialSearchResult` is a bilevel hashmap: `HashMap<String, HashMap<String, String>>`,
    /// The outer map's key corresponds to the ID of the result from 1 to the length of the map.
    /// The inner map contains the keys and values of the metadata of the result, i.e.
    /// target, service, user, last modified/date written, etc. In the case of keyutils, the Linux
    /// Kernel keystore provides IDs for all credentials, the user must know the ID of the credential 
    /// to manipulate and pass this value to `from_search_results`. Since keyutils only returns one
    /// result, this is the only valid parameter. 
    /// # Example
    /// First result:
    /// ```rust
    /// use keyring::Entry;
    ///
    /// let result = &Entry::search("Foo");
    /// let entry = Entry::from_search_results(result, 1);
    /// ```
    /// All results:
    /// ```rust
    /// use keyring::Entry;
    ///
    /// let result = Entry::search("Foo");
    /// let size = result.as_ref().expect("No results").keys().len();
    /// let entries: Vec<Entry> = vec![];
    /// for index in 0..=size {
    ///     let entry = Entry::from_search_results(&result, index);
    /// }
    /// ```
    pub fn from_search_results(result: &CredentialSearchResult, id: usize) -> Result<Entry> {
        let result = match result {
            Ok(result) => result,
            Err(err) => {
                return Err(Error::Invalid(
                    "from search results".to_string(),
                    err.to_string(),
                ))
            }
        };

        let credential = match result.get_key_value(&id.to_string()) {
            Some(credential) => credential.1,
            None => return Err(Error::NoEntry),
        };
        // values[0] = target
        // values[1] = service
        // values[2] = user
        let values: [&String; 3] = default::get_entry_values(credential)?;

        Self::new_with_target(values[0], values[1], values[2])
    }
}

#[cfg(doctest)]
doc_comment::doctest!("../README.md", readme);

#[cfg(test)]
/// There are no actual tests in this module.
/// Instead, it contains generics that each keystore invokes in their tests,
/// passing their store-specific parameters for the generic ones.
//
// Since iOS doesn't use any of these generics, we allow dead code.
#[allow(dead_code)]
mod tests {
    use std::collections::HashSet;

    use super::{credential::CredentialApi, Entry, Error, Result};

    /// Create a platform-specific credential given the constructor, service, and user
    pub fn entry_from_constructor<F, T>(f: F, service: &str, user: &str) -> Entry
    where
        F: FnOnce(Option<&str>, &str, &str) -> Result<T>,
        T: 'static + CredentialApi + Send + Sync,
    {
        match f(None, service, user) {
            Ok(credential) => Entry::new_with_credential(Box::new(credential)),
            Err(err) => {
                panic!("Couldn't create entry (service: {service}, user: {user}): {err:?}")
            }
        }
    }

    /// A basic round-trip unit test given an entry and a password.
    pub fn test_round_trip(case: &str, entry: &Entry, in_pass: &str) {
        entry
            .set_password(in_pass)
            .unwrap_or_else(|err| panic!("Can't set password for {case}: {err:?}"));
        let out_pass = entry
            .get_password()
            .unwrap_or_else(|err| panic!("Can't get password for {case}: {err:?}"));
        assert_eq!(
            in_pass, out_pass,
            "Passwords don't match for {case}: set='{in_pass}', get='{out_pass}'",
        );
        entry
            .delete_password()
            .unwrap_or_else(|err| panic!("Can't delete password for {case}: {err:?}"));
        let password = entry.get_password();
        assert!(
            matches!(password, Err(Error::NoEntry)),
            "Read deleted password for {case}",
        );
    }

    /// When tests fail, they leave keys behind, and those keys
    /// have to be cleaned up before the tests can be run again
    /// in order to avoid bad results.  So it's a lot easier just
    /// to have tests use a random string for key names to avoid
    /// the conflicts, and then do any needed cleanup once everything
    /// is working correctly.  So we export this function for tests to use.
    pub fn generate_random_string_of_len(len: usize) -> String {
        // from the Rust Cookbook:
        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
        use rand::{distributions::Alphanumeric, thread_rng, Rng};
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub fn generate_random_string() -> String {
        generate_random_string_of_len(30)
    }

    pub fn test_empty_service_and_user<F>(f: F)
    where
        F: Fn(&str, &str) -> Entry,
    {
        let name = generate_random_string();
        let in_pass = "doesn't matter";
        test_round_trip("empty user", &f(&name, ""), in_pass);
        test_round_trip("empty service", &f("", &name), in_pass);
        test_round_trip("empty service & user", &f("", ""), in_pass);
    }

    pub fn test_missing_entry<F>(f: F)
    where
        F: FnOnce(&str, &str) -> Entry,
    {
        let name = generate_random_string();
        let entry = f(&name, &name);
        assert!(
            matches!(entry.get_password(), Err(Error::NoEntry)),
            "Missing entry has password"
        )
    }

    pub fn test_empty_password<F>(f: F)
    where
        F: FnOnce(&str, &str) -> Entry,
    {
        let name = generate_random_string();
        let entry = f(&name, &name);
        test_round_trip("empty password", &entry, "");
    }

    pub fn test_round_trip_ascii_password<F>(f: F)
    where
        F: FnOnce(&str, &str) -> Entry,
    {
        let name = generate_random_string();
        let entry = f(&name, &name);
        test_round_trip("ascii password", &entry, "test ascii password");
    }

    pub fn test_round_trip_non_ascii_password<F>(f: F)
    where
        F: FnOnce(&str, &str) -> Entry,
    {
        let name = generate_random_string();
        let entry = f(&name, &name);
        test_round_trip("non-ascii password", &entry, "このきれいな花は桜です");
    }

    pub fn test_update<F>(f: F)
    where
        F: FnOnce(&str, &str) -> Entry,
    {
        let name = generate_random_string();
        let entry = f(&name, &name);
        test_round_trip("initial ascii password", &entry, "test ascii password");
        test_round_trip(
            "updated non-ascii password",
            &entry,
            "このきれいな花は桜です",
        );
    }

    #[test]
    fn test_search_no_duplicates() {
        let names = vec![
            generate_random_string(),
            generate_random_string(),
            generate_random_string(),
        ];

        let mut entries = Vec::new();

        for name in &names {
            entries.push(
                Entry::new_with_target(&name, "test-service", "test-user")
                    .expect("Error constructing entry"),
            )
        }

        for entry in &entries {
            entry
                .set_password("test-password")
                .expect("Error setting password")
        }

        let search_users = keyring_search::List::list_credentials(
            &Entry::search_users("test"),
            keyring_search::Limit::All,
        );
        let users_set: HashSet<&str> = search_users.lines().collect();
        let search_services = keyring_search::List::list_credentials(
            &Entry::search_services("test"),
            keyring_search::Limit::All,
        );
        let services_set: HashSet<&str> = search_services.lines().collect();
        let search_default = keyring_search::List::list_credentials(
            &Entry::search("test"),
            keyring_search::Limit::All,
        );
        let search_set: HashSet<&str> = search_default.lines().collect();

        assert_eq!(users_set, services_set);
        assert_eq!(users_set, search_set);
        assert_eq!(services_set, search_set);

        for entry in &entries {
            entry.delete_password().expect("error deleting entry")
        }
    }

    #[test]
    fn test_entry_from_search() {
        let name = generate_random_string();
        let password1 = "password1";
        let password2 = "password2";
        let entry = Entry::new_with_target(&name, "test-service", "test-user")
            .expect("Error creating new entry");
        entry
            .set_password(password1)
            .expect("error setting password1");

        let old_password = entry
            .get_password()
            .expect("failed to get password from old entry");
        let results = &Entry::search(&name);

        let result_entry =
            Entry::from_search_results(results, 1).expect("Failed to create entry from results");
        result_entry
            .set_password(password2)
            .expect("error setting password2");

        let new_password = result_entry
            .get_password()
            .expect("Failed to get password from new entry");

        assert_eq!(password1, old_password);
        assert_eq!(password2, new_password);

        result_entry
            .delete_password()
            .expect("Failed to delete new entry");
        let e = entry.delete_password().unwrap_err();

        assert!(matches!(e, Error::NoEntry));
    }

    #[test]
    fn test_entries_from_search() {
        let names = [
            generate_random_string(),
            generate_random_string(),
            generate_random_string(),
        ];

        let mut entries: Vec<Entry> = vec![];
        let password1 = "password1";
        let password2 = "password2";
        for name in names {
            let entry = Entry::new_with_target(&name, "test-service", "test-user")
                .expect("Error creating new entry");
            entry
                .set_password(password1)
                .expect("error setting password1");
            entries.push(entry);
        }

        let mut old_passwords = vec![];

        for entry in &entries {
            let old_password = entry
                .get_password()
                .expect("failed to get password from old entry");
            old_passwords.push(old_password);
        }

        let result = &Entry::search("test");

        let size = result
            .as_ref()
            .expect("Error getting size of outer map")
            .keys()
            .len(); 
        let mut result_entries: Vec<Entry> = vec![];
        for index in 1..=size {
            let msg = format!("Failed to create entry at index: {index}");
            let entry = Entry::from_search_results(result, index).expect(&msg);
            entry
                .set_password(&password2)
                .expect("Error setting new password");
            result_entries.push(entry);
        }

        let mut new_passwords = vec![];

        for entry in &result_entries {
            let new_password = entry.get_password().expect("error getting new password");
            new_passwords.push(new_password);
        }

        for i in 0..new_passwords.len() {
            assert_eq!(password1, old_passwords[i]);
            assert_eq!(password2, new_passwords[i]);
        }

        for entry in &result_entries {
            entry.delete_password().expect("Error deleting password");
        }

        for entry in &entries {
            let e = entry.delete_password().unwrap_err();
            assert!(matches!(e, Error::NoEntry));
        }
    }
}
