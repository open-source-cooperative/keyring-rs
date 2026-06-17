//! The Rust Keyring
//!
//! This library operates in one of two modes, depending on which of the two features
//! `v1` or `cli` are enabled. Both can be enabled if desired, but typically you use
//! one or the other.
//!
//! If you enable the `v1` feature, this library behaves essentially the same as the v1
//! version of Keyring behaved: it allows easy, platform-independent setting and reading
//! of passwords/secrets on macOS, Windows, and *nix platforms. See the [v1] module docs
//! for details.
//!
//! If you enable the `cli` feature, this library provides the glue by which the Rust CLI
//! example app, the `rust-native-keyring` Python module, and the `keyring-demo` cross-platform
//! application access all available credential stores on all platforms. See the [cli]
//! module docs for details, and the README of this crate for more information.
//!
//! Note that *neither* of these modes are either useful for or meant for use by
//! applications which want to control which credential stores they use on which
//! platforms, or which want to offer more functionality than just reading and writing
//! specific passwords/secrets. Such applications should not be linking to this library at
//! all; they should instead be linking to the
//! [keyring-core](crates.io/crates/keyring-core) library and any specific credential
//! stores they want to use. While this library provides high-quality, maintained sample
//! code for how to link to every known credential store, linking to this library using
//! the `cli` feature will saddle an application with a lot of dependent modules it won't
//! need. Instead, as explained the README for this crate, developers should just
//! copy/paste useful code from the `cli` module into their applications.
//!
//! For more about how to write an application that uses the Keyring ecosystem, see [this
//! wiki](https://github.com/open-source-cooperative/keyring-rs/wiki/Keyring).

#[cfg(not(any(feature = "v1", feature = "cli")))]
compile_error!("At least one of the features 'v1' or 'cli' must be enabled");

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "cli")]
pub use cli::*;

#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "v1")]
pub use v1::*;
