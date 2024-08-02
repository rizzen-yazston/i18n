// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_utility-rizzen-yazston` crate.

//! Welcome to the **`i18n_utility`** crate of the *Internationalisation* (i18n) project.
//!
//! This crate consists of five modules:
//!
//! * [`error`]: Contains the error enum for the language registry,
//!
//! * [`language`]: Registry of language tags with ICU4X's [`LanguageIdentifier`] or [`Locale`] instances,
//!
//! * [`tagged_string`]: Simple tagged string type,
//!
//! * [`traits`]: Traits for localisation of structs and enums,
//!
//! * [`types`]: Simple struct and enum types.
//!
//! # Features
//!
//! Available features for `i18n_utility` crate:
//!
//! * `icu_extended`: Use the more detailed ICU information structs, types, and methods.
//!
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//! [`RefCell`].
//!
//! # Modules
//!
//! ## `language`: Registry for holding ICU4X `Locale` objects.
//!
//! Registry for holding the validated [BCP 47 Language Tag] strings, and optionally holding the
//! `ICU4X` [`LanguageIdentifier`] or [`Locale`] (using feature `icu_extended`) instances.
//!
//! The purpose of the registry is to reduce the need of parsing language tags repeatedly, by
//! storing the validated language tag against the requested language tag.
//!
//! The `LanguageIdentifier` or `Locale` type can be provided by either the [`icu_locid`] crate
//! or the `icu` meta-crate. These two crates are part of the [ICU4X] project developed by the
//! [Unicode Consortium].
//!
//! ### Examples
//!
//! ```
//! use icu_locid::LanguageIdentifier;
//! use std::rc::Rc;
//! use i18n_utility::LanguageTagRegistry;
//!
//! let mut registry = LanguageTagRegistry::new();
//! let result = registry.tag("en_ZA").expect("Failed to parse language tag.");
//! let tags = registry.list().iter().count();
//!
//! assert_eq!(result.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format.");
//! assert_eq!(tags, 1, "Supposed to be 1 entries: en-ZA.")
//! ```
//!
//! ## `tagged_string`: Tagged string.
//!
//! The immutable `TaggedString` type simply associates a language tag ([`Rc`]`<`[`LanguageTag`]`>` or
//! [`Arc`]`<LanguageTag>`) to a text string ([`String`]).
//!
//! In the context of the `i18n` project, the identifier tag is expected to be a [BCP 47 Language Tag] string.
//!
//! ### Examples
//!
//! ```
//! use i18n_utility::{TaggedString, LanguageTagRegistry};
//!
//! let registry = LanguageTagRegistry::new();
//! let string = "This is a test string.";
//! let tag = registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
//! let tagged_string = TaggedString::new(string, &tag);
//! assert_eq!(tagged_string.as_str(), string, "String failed.");
//! assert_eq!(tagged_string.tag(), &tag, "Language tag failed.");
//! ```
//!
//! [ICU4X]: https://github.com/unicode-org/icu4x
//! [Unicode Consortium]: https://home.unicode.org/
//! [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt

#[cfg(doc)]
use icu_locid::Locale;

#[cfg(doc)]
use icu_locid::LanguageIdentifier;

#[cfg(doc)]
use std::sync::{Arc, Mutex};

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use std::cell::RefCell;

pub mod tagged_string;
pub use tagged_string::*;
pub mod language;
pub use language::*;
pub mod script;
pub use script::*;
pub mod types;
pub use types::*;
pub mod traits;
pub use traits::*;
pub mod error;
pub use error::*;
