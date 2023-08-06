// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

//! [`LString`] provider.
//! 
//! A trait for retrieving localisation language strings from a data repository via a provider that implements the
//! trait methods: `get()` and `get_one()`. In addition, there are other trait methods for retrieve the default
//! language for a component, and supported languages for entire data repository, component and identifier
//! respectively.
//! 
//! # Features
//! 
//! Available features for `i18n_provider` crate:
//! 
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//! [`RefCell`].
//! 
//! For an implementation example, see the `i18n_provider_sqlite3-rizzen-yazston` crate, which uses Sqlite3 for its
//! data store.

#[cfg( doc )]
use std::sync::{ Arc, Mutex };

#[cfg( doc )]
use std::rc::Rc;

#[cfg( doc )]
use std::cell::RefCell;

#[cfg( doc )]
use i18n_utility::LString;

pub mod provider;
pub use provider::*;
pub mod error;
pub use error::*;
