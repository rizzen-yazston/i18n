// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

//! Welcome to the **`i18n_provider`** crate of the *Internationalisation* (i18n) project.
//!
//! This crate consists of two modules:
//!
//! * [`error`]: Contains the enum for common errors regardless of the implementation of the localisation provider trait,
//!
//! * [`provider`]: The localisation provider trait.
//!
//! # Features
//!
//! Available features for `i18n_provider` crate:
//!
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//!   [`RefCell`].
//!
//! # Modules
//!
//! ## `provider`: A localisation string provider.
//!
//! A trait for retrieving localisation strings from a data repository via a provider that implements the trait
//! methods: `strings()` and `string()`. In addition, there are other trait methods for retrieve the default
//! language for a component, and supported languages for entire data repository, component and identifier
//! respectively.
//!
//! ### Examples
//!
//! For an implementation example, see the `i18n_provider_sqlite3-rizzen-yazston` crate, which uses Sqlite3 for its
//! data store.

#[cfg(doc)]
use std::sync::{Arc, Mutex};

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use std::cell::RefCell;

#[cfg(doc)]
use i18n_utility::TaggedString;

pub mod provider;
pub use provider::*;
pub mod error;
pub use error::*;
