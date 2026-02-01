/*!

Run unit tests on all available credential store modules.

These tests make sure that the credential store meets the API contract
expected by `keyring-core`.

*/

use keyring::{NAMED_STORES, release_store, use_named_store};
use keyring_core::Error;

mod runner;

fn main() {
    println!("Running tests on {} stores...", NAMED_STORES.len());
    for store in NAMED_STORES {
        match use_named_store(store) {
            Ok(_) => run_tests(store),
            Err(Error::NotSupportedByStore(s)) => println!("\nSkipping store '{store}': {s}"),
            Err(err) => println!("\nCouldn't instantiate store '{store}': {err:?}"),
        }
    }
    println!("\nFinished running tests.");
}

fn run_tests(store: &str) {
    println!("\nRunning tests on store: {store}...");
    let (succeeded, failed) = runner::run_tests();
    if failed == 0 {
        println!("Summary for {store}: all {succeeded} tests succeeded.");
    } else if failed == 1 {
        println!("Summary for {store}: {succeeded} tests succeeded; 1 test failed.");
    } else if succeeded == 1 {
        println!("Summary for {store}: 1 test succeeded; {failed} tests failed.");
    } else {
        println!("Summary for {store}: {succeeded} tests succeeded; {failed} tests failed.");
    }
    release_store();
}
