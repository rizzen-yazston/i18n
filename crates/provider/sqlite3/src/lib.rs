// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

//! Sqlite3 provider for language strings.
//! 
//! This crate implements [`LanguageStringProvider`] using Sqlite3 as the data store for language strings. As a directory path
//! is used at the time of creating a `ProviderSqlite3` object, it means that an application can have multiple data
//! stores for both application language strings, and also for data packages' language strings.
//! 
//! # Features
//! 
//! Available features for `i18n_provider_sqlite3` crate:
//! 
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//! [`RefCell`].
//! 
//! # Examples
//! 
//! ```
//! use i18n_provider_sqlite3::ProviderSqlite3;
//! use i18n_provider::LanguageStringProvider;
//! use i18n_utility::LanguageTagRegistry;
//! use std::rc::Rc;
//! use std::error::Error;
//! 
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let path = "./l10n/";
//!     let registry = Rc::new( LanguageTagRegistry::new() );
//!     let tag = registry.get_tag( "en" )?;
//!     let provider = ProviderSqlite3::try_new(
//!         path,
//!         &registry
//!     )?;
//!     let strings = provider.get(
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

#[cfg( doc )]
use std::sync::{ Arc, Mutex };

#[cfg( doc )]
use std::rc::Rc;

#[cfg( doc )]
use std::cell::RefCell;

#[cfg( doc )]
use i18n_utility::TaggedString;

#[cfg( doc )]
use i18n_provider::LanguageStringProvider;

pub mod provider;
pub use provider::*;
pub mod error;
pub use error::*;
