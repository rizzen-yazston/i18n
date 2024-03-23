// This file is part of `i18n-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n-rizzen-yazston` crate.

//! Welcome to the **`i18n`** crate of the *Internationalisation* (i18n) project.
//!
//! This is the main meta-crate of the project.
//!
//! This convenience meta crate contains selected available crates:
//!
//! * `i18n_lexer`: A simple lexer to tokenise a string,
//!
//! * `i18n_localiser`: The multilingual messaging system,
//!
//! * `i18n_provider`: Trait for providing language strings, and error struct,
//!
//! * `i18n_provider_sqlite3`: Implementation of `i18n_provider` using Sqlite3 as its data store,
//!
//! * `i18n_utility`: Contains the Language tag registry, and the TaggedString type.
//!
//! NOTE: All these crates on `crates.io` have the names appended with the suffix `-rizzen-yazston` to distinguish them
//! from internationalisation crates created by other authors.
//!
//! # Usage
//!
//! For most use cases, just the use of `i18n-rizzen-yazston` crate will be sufficient to use the multilingual message
//! system, though the individual crates can be selected individual if the entire `i18n` project is not required.
//!
//! ## Features
//!
//! Available features for `i18n_icu` crate:
//!
//! * `icu_blob`: Allow for instances of `BlobDataProvider` to be used various ICU4X components that supports
//! [`BufferProvider`]. An alternative provider when the internal data of ICU4X components are insufficient for a
//! particular use case.
//!
//! * `icu_compiled_data` \[default\]: Allow for the internal data of the various ICU4X components.
//!
//! * `icu_extended`: Use the more detailed ICU information structs, types, and methods.
//!
//! * `icu_fs`: Allow for instances of `FsDataProvider` to be used various ICU4X components that supports
//! `BufferProvider`. An alternative provider when the internal data of ICU4X components are insufficient for a
//! particular use case.
//!
//! * `logging`: To provide some logging information.
//!
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of `Arc` and `Mutex` instead `Rc` and
//! `RefCell`.
//!
//! ## Examples
//!
//! See the various component crates for usage examples.
//!
//! [`BufferProvider`]: https://docs.rs/icu_provider/1.2.0/icu_provider/buf/trait.BufferProvider.html

#[cfg(doc)]
use std::sync::{Arc, Mutex};

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use std::cell::RefCell;

pub use i18n_lexer as lexer;
pub use i18n_localiser as localiser;
pub use i18n_provider as provider;
pub use i18n_provider_sqlite3 as provider_sqlite3;
pub use i18n_utility as utility;
