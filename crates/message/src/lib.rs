// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_message-rizzen-yazston` crate.

//! The `i18n_message` crate contains the messaging system.
//!
//! Contains the follow modules:
//!
//! * `formatter`: Takes an AST with supplied values to create a language string of the specified `Locale`.

pub mod formatter;
pub use formatter::*;
