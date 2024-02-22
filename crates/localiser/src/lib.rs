// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_localiser-rizzen-yazston` crate.

//! The `i18n_localiser` crate contains the localiser.
//!
//! A localiser that connects to a string data store, to obtain strings for the specified language using a
//! string identifier, and formatting the string to replace any placeholders within the string with provided values.
//!
//! The localiser is capable of caching retrieved strings that are prepared for placeholder replacement, thus can be
//! reused without the need to parse the string for placeholders.
//!
//! The localiser makes use of all the other component crates that make up the `i18n` project. Ideally one only
//! needs to use the meta crate `i18n`, as it includes all the crates including this `i18n_localiser` crate.
//!
//! # Features
//!
//! Available features for `i18n_localiser` crate:
//!
//! * `compiled_data`: Allow for the internal data of the various ICU4X components.
//!
//! * `blob`: Allow for instances of `BlobDataProvider` to be used various ICU4X components that supports
//! [`BufferProvider`].
//!
//! * `fs`: Allow for instances of `FsDataProvider` to be used various ICU4X components that supports `BufferProvider`.
//!
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//! [`RefCell`].
//!
//! * `log`: To provide some logging information.
//!
//! * `extend`: Enables the [`icu_locid::Locale`] to replace the [`icu_locid::LanguageIdentifier`] to include extension subtags.
//!
//! # Examples
//!
//! ```
//! use i18n_icu::{ IcuDataProvider, DataProvider };
//! use i18n_utility::{ PlaceholderValue, LanguageTagRegistry };
//! use i18n_provider_sqlite3::LocalisationProviderSqlite3;
//! use i18n_pattern::CommandRegistry;
//! use i18n_localiser::Localiser;
//! use std::collections::HashMap;
//! use std::rc::Rc;
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
//!     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
//!     let lstring_provider = LocalisationProviderSqlite3::try_new(
//!         "./l10n/", &language_tag_registry, false
//!     )?;
//!     let command_registry = Rc::new( CommandRegistry::new() );
//!     let mut message_system = Localiser::try_new(
//!         &icu_data_provider, &language_tag_registry, Box::new( lstring_provider ), &command_registry, true, true, "en-ZA",
//!     )?;
//!     let tag = language_tag_registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
//!     let mut values = HashMap::<String, PlaceholderValue>::new();
//!     values.insert(
//!         "component".to_string(),
//!         PlaceholderValue::String( "i18n_localiser".to_string() )
//!     );
//!     values.insert(
//!         "identifier".to_string(),
//!         PlaceholderValue::String( "string_not_found".to_string() )
//!     );
//!     values.insert(
//!         "language_tag".to_string(),
//!         PlaceholderValue::String( "en-ZA".to_string() )
//!     );
//!     values.insert(
//!         "fallback".to_string(),
//!         PlaceholderValue::String( "true".to_string() )
//!     );
//!     let lstring = message_system.format(
//!         "i18n_localiser",
//!         "string_not_found",
//!         &values,
//!         &tag,
//!         None,
//!         None
//!     )?;
//!     assert_eq!(
//!         lstring.as_str(),
//!         "No string was found for the component ‘i18n_localiser’ with identifier ‘string_not_found’ for the \
//!             language tag ‘en-ZA’. Fallback was used: True.",
//!         "Check placeholder values."
//!     );
//!     Ok( () )
//! }
//! ```
//!
//! [`BufferProvider`]: https://docs.rs/icu_provider/1.2.0/icu_provider/buf/trait.BufferProvider.html

#[cfg(doc)]
use std::sync::{Arc, Mutex};

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use std::cell::RefCell;

pub mod error;
pub use error::*;
pub(crate) mod formatter;
pub(crate) use formatter::*;
pub mod localiser;
pub use localiser::*;
