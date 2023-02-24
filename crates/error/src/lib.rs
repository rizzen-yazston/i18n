// This file is part of `i18n_error-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_error-rizzen-yazston` crate.

//! The `i18n_error` crate contains the `ErrorMessage` struct for various functions returning errors in the
//! Internationalisation (`i18n`) project. Also contains the `ErrorPlaceholderValue` enum of the various supported
//! placeholder values required when formatting for the specified language tag.
//!
//! # Examples
//!
//! ```
//! // See other `i18n` crates examples.
//! ```

use std::collections::HashMap;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Debug, Display, Formatter, Result as FmtResult };

/// An enum consists of a selection of Rust primitives for error messages.
#[derive( Debug )]
pub enum ErrorPlaceholderValue {
    String( String ), // Can also be used for date (ISO format), time (ISO format), fixed decimal.
    Integer( i128 ),
    Unsigned( u128 ),
    Float( f64 ),
}

/// Functions encountering an error, returns either `ErrorMessage`, or as an enum that uses `ErrorMessage` as values.
#[derive( Debug )]
pub struct ErrorMessage {
    pub string: String, // Preformatted string for a particular locale.
    pub identifier: String, // Message identifier.
    pub values: HashMap<String, ErrorPlaceholderValue>, // Values for formatting message for another locale.
}

impl Display for ErrorMessage {

    /// In case the error has not been handled by the application, the Rust default error handler will call this
    /// method, which simply writes the preformatted string to the formatter's buffer, ignoring the associated locale.
    /// 
    /// If wanting to have the error displayed for another locale, the error must be caught, then use message system to 
    /// produce a string for the language tag, and finally use any of the standard output macros to write out the
    /// string.
    fn fmt( &self, formatter: &mut Formatter ) -> FmtResult {
        formatter.write_str( self.string.as_str() )
    }
}

/// Does not support inner errors, thus `source()` returns `None`.
impl Error for ErrorMessage {}

