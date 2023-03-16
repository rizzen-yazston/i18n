// This file is part of `i18n_registry-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_registry-rizzen-yazston` crate.

//! # Registry for holding [`ICU4X`] [`Locale`] objects.
//! 
//! This module contains the `LanguageTagRegistry` type, to provide a simple container that caches the
//! [BCP 47 Language Tag] string and the [`Locale`] type for querying language tags. The purpose of the registry is to
//! reduce the need of parsing language tags repeatedly, by storing the result `Locale` for querying language tag in
//! the registry, and uses the existing `Locale` for the querying language tag when requested.
//! 
//! The `Locale` type can be provided by either the [`icu_locid`] crate or the [`icu`] meta-crate. These two crates
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
//! use i18n_registry::registry::LanguageTagRegistry;
//! 
//! let mut registry = LanguageTagRegistry::new();
//! let result = registry.get( "en_ZA" ).expect( "Failed to parse language tag." );
//! let tags = registry.list().iter().count();
//! 
//! assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
//! assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
//! ```
//! 
//! [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
//! [`icu_locid`]: https://crates.io/crates/icu_locid
//! [`icu`]: https://crates.io/crates/icu
//! [`ICU4X`]: https://github.com/unicode-org/icu4x
//! [Unicode Consortium]: https://home.unicode.org/
//! [`LanguageIdentifier`]: https://docs.rs/icu/latest/icu/locid/struct.LanguageIdentifier.html
//! [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt

pub mod registry;
pub use registry::*;
pub mod error;
pub use error::*;
