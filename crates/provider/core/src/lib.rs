// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

//! [`LString`] provider.
//! 
//! A trait for providing language strings in the form of [`Vec`]`<`[`LString`]`>`, and obtaining the default language
//! tag used for the crate's messages.
//! 
//! For an implementation example, see the `i18n_provider_sqlite3-rizzen-yazston` crate, which uses Sqlite3 for its
//! data store.
//! 
//! [`Vec`]: std::vec::Vec
//! [`LString`]: i18n_utility::LString

pub mod provider;
pub use provider::*;
pub mod error;
pub use error::*;
