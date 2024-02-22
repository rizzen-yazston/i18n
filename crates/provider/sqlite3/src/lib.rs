// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

//! Sqlite3 provider for localisation strings.
//!
//! This crate implements [`LocalisationProviderTrait`] using Sqlite3 as the data store for localisation strings. As a
//! directory path is used at the time of creating a `LocalisationProviderSqlite3` instance, it means that an
//! application can have multiple data stores for both application localisation strings, and also for data packages'
//! localisation strings.
//!
//! # Features
//!
//! Available features for `i18n_provider_sqlite3` crate:
//!
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//! [`RefCell`].
//!
//! * `log`: To provide some logging information, primarily debug and error.
//!
//! # Localisation Sqlite3 templates
//!
//! Can find the templates `all_in_one.sqlite3` and component `application.sqlite3` for the application in the `l10n`
//! directory.
//!
//! # Examples
//!
//! ```
//! use i18n_provider_sqlite3::LocalisationProviderSqlite3;
//! use i18n_provider::LocalisationProviderTrait;
//! use i18n_utility::LanguageTagRegistry;
//! use std::rc::Rc;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let path = "./l10n/";
//!     let registry = Rc::new( LanguageTagRegistry::new() );
//!     let tag = registry.tag( "en" )?;
//!     let provider = LocalisationProviderSqlite3::try_new(
//!         path,
//!         &registry,
//!         false
//!     )?;
//!     let strings = provider.strings(
//!         "i18n_provider_sqlite3",
//!         "path_conversion",
//!         &tag,
//!     )?;
//!     assert_eq!( strings.len(), 1, "There should be 1 string." );
//!     assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
//!     assert_eq!( strings[ 0 ].tag().as_str(), "en-ZA", "Must be en-ZA." );
//!     Ok( () )
//! }
//! ```

#[cfg(doc)]
use std::sync::{Arc, Mutex};

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use std::cell::RefCell;

#[cfg(doc)]
use i18n_utility::TaggedString;

#[cfg(doc)]
use i18n_provider::LocalisationProviderTrait;

pub mod provider;
pub use provider::*;
pub mod error;
pub use error::*;
