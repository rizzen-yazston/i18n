// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_utility-rizzen-yazston` crate.

//! Welcome to the **utility** crate of the *Internationalisation* (i18n) project.
//! 
//! This crate consists of two modules:
//! 
//! * [`registry`]: Registry for Language Tags and ICU4X's `Locale` instances,
//! 
//! * [`tagged_string`]: Simple tagged string type.
//! 
//! # Registry for holding ICU4X `Locale` objects.
//! 
//! This module contains the `LanguageTagRegistry` type, to provide a simple container that caches the
//! [BCP 47 Language Tag] string and the [`Locale`] type for querying language tags. The purpose of the registry is to
//! reduce the need of parsing language tags repeatedly, by storing the result `Locale` for querying language tag in
//! the registry, and uses the existing `Locale` for the querying language tag when requested.
//! 
//! The `Locale` type can be provided by either the [`icu_locid`] crate or the `icu` meta-crate. These two crates
//! are part of the [`ICU4X`] protect developed by the [Unicode Consortium].
//! 
//! This crate makes use of the `Locale` type instead of the [`LanguageIdentifier`] type due to that the `Locale`
//! type supports the entire BCP 47 Language Tag specification, where as the `LanguageIdentifier` type excludes the
//! **extension** subtags of the BCP 47 Language Tag specification.
//! 
//! ## Examples
//! 
//! ```
//! use icu_locid::Locale;
//! use std::rc::Rc;
//! use i18n_utility::registry::LanguageTagRegistry;
//! 
//! let mut registry = LanguageTagRegistry::new();
//! let result = registry.tag_and_locale( "en_ZA" ).expect( "Failed to parse language tag." );
//! let tags = registry.list().iter().count();
//! 
//! assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
//! assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
//! ```
//! 
//! # Tagged string.
//! 
//! The immutable `TaggedString` type simply associates an identifier tag ([`Rc`]`<String>` or [`Arc`]`<String>`) to a
//! text string ([`String`]).
//! 
//! In the context of the `i18n` project, the identifier tag is expected to be a [BCP 47 Language Tag] string, even
//! though any identifier could be used.
//! 
//! ## Examples
//! 
//! ```
//! use icu_locid::Locale;
//! use std::rc::Rc;
//! use i18n_utility::TaggedString;
//! 
//! let string = "This is a test string.";
//! let tag = Rc::new(
//!     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
//! );
//! let lang_string = TaggedString::new( string, &tag );
//! assert_eq!( lang_string.as_str(), string, "String failed." );
//! assert_eq!( lang_string.tag(), &tag, "Language tag failed." );
//! ```
//! 
//! # Features
//! 
//! Available features for `i18n_utility` crate:
//! 
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of `Arc` and [`Mutex`] instead `Rc` and
//! [`RefCell`].
//! 
//! [`ICU4X`]: https://github.com/unicode-org/icu4x
//! [Unicode Consortium]: https://home.unicode.org/
//! [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt

#[cfg( doc )]
use icu_locid::Locale;

#[cfg( doc )]
use icu_locid::LanguageIdentifier;

#[cfg( doc )]
use std::sync::{ Arc, Mutex };

#[cfg( doc )]
use std::rc::Rc;

#[cfg( doc )]
use std::cell::RefCell;

pub mod tagged_string;
pub use tagged_string::*;
pub mod registry;
pub use registry::*;
pub mod error;
pub use error::*;
pub mod traits;
pub use traits::*;
