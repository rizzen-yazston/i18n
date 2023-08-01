// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_message-rizzen-yazston` crate.

//! The `i18n_message` crate contains the messaging system.
//!
//! A message system that connects to a string data store, to obtain strings for the specified language using a
//! string identifier, and formatting the string to replace any placeholders within the string with provided values.
//! 
//! The message is capable of caching retrieved strings that are prepared for placeholder replacement, thus can be
//! reused without the need to parse the string for placeholders.
//! 
//! The message system makes use of all the other component crates that make up the `i18n` project. Ideally one only
//! needs to use the meta crate `i18n`, as it includes all the crates including this `i18n_message` crate.
//! 
//! # Features
//! 
//! Available features for `i18n_message` crate:
//! 
//! * `compiled_data`: Allow for the internal data of the various ICU4X components.
//! 
//! * `blob`: Allow for instances of `BlobDataProvider` to be used various ICU4X components that supports [`BufferProvider`].
//! 
//! * `fs`: Allow for instances of `FsDataProvider` to be used various ICU4X components that supports `BufferProvider`.
//! 
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//! [`RefCell`].
//! 
//! # Examples
//! 
//! ```
//! use i18n_icu::{ IcuDataProvider, DataProvider };
//! use i18n_utility::LanguageTagRegistry;
//! use i18n_provider_sqlite3::ProviderSqlite3;
//! use i18n_pattern::{ PlaceholderValue, CommandRegistry };
//! use i18n_message::Message;
//! use std::collections::HashMap;
//! use std::rc::Rc;
//! use std::error::Error;
//! 
//! fn message() -> Result<(), Box<dyn Error>> {
//!     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
//!     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
//!     let lstring_provider = ProviderSqlite3::try_new(
//!         "./i18n/", &language_tag_registry
//!     )?;
//!     let command_registry = Rc::new( CommandRegistry::new() );
//!     let mut message_system = Message::try_new(
//!         &icu_data_provider, &language_tag_registry, &lstring_provider, &command_registry, true, true
//!     )?;
//!     let mut values = HashMap::<String, PlaceholderValue>::new();
//!     values.insert(
//!         "identifier".to_string(),
//!         PlaceholderValue::String( "i18n_message/string_not_found".to_string() )
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
//!         "i18n_message/string_not_found",
//!         &values,
//!         &language_tag_registry.get_language_tag( "en-ZA" ).unwrap(),
//!         None,
//!         None
//!     )?;
//!     assert_eq!(
//!         lstring.as_str(),
//!         "No string was found for identifier ‘i18n_message/string_not_found’ and language tag ‘en-ZA’. Fallback used: True.",
//!         "Check placeholder values."
//!     );
//!     Ok( () )
//! }
//! ```
//! 
//! [`BufferProvider`]: https://docs.rs/icu_provider/1.2.0/icu_provider/buf/trait.BufferProvider.html

#[cfg( doc )]
use std::sync::{ Arc, Mutex };

#[cfg( doc )]
use std::rc::Rc;

#[cfg( doc )]
use std::cell::RefCell;

pub mod error;
pub use error::*;
pub mod message;
pub use message::*;
