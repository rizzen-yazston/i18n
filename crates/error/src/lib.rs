// This file is part of `i18n_error-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_error-rizzen-yazston` crate.

//! The `i18n_error` crate contains the `ErrorMessage` struct for various functions returning errors in the
//! Internationalisation (`i18n`) project. Also contains the `PlaceholderValue` enum of the various supported
//! placeholder values required when formatting for the specified locale.
//!
//! # Examples
//!
//! ```
//! //TODO: perhaps use unit test.
//! ```

use fixed_decimal::FixedDecimal;
use i18n_lstring::LString;
use icu_calendar::{ types::Time, AsCalendar, Date, DateTime };
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{ Debug, Display, Formatter, Result as FmtResult };

/// An enum of all the data types that the Internationalisation (`i18n`) project supports in the message system.
/// Consists of a selection of Rust primitives, available ICU4X types, and `LString` type of the `i18n` project.
#[derive( Debug )]
pub enum PlaceholderValue<A: AsCalendar> {
    // Add new ICU4X types as they are made available.
    String( String ),
    LString( LString ),
    Integer( i128 ),
    Unsigned( u128 ),
    Float( f64 ),
    FixedDecimal( FixedDecimal ),
    DateTime( DateTime<A> ),
    Date( Date<A> ),
    Time( Time ),
}

/// Functions encountering an error, returns either `ErrorMessage`, or as an enum that uses `ErrorMessage` as values.
#[derive(Debug)]
pub struct ErrorMessage<A: AsCalendar> {
    string: LString,
    identifier: String,
    values: HashMap<String, PlaceholderValue<A>>,
}

impl<A: AsCalendar> ErrorMessage<A> {

    /// Returns a reference to the `LString` containing a preformatted string using the default locale of the
    /// project. The locale `en-US` is often the project's default locale, though other English variants may be used.
    pub fn string( &self ) -> &LString {
        &self.string
    }

    /// Returns a reference to the message identifier for retrieve the localised pattern string for formatting.
    pub fn identifier( &self ) -> &str {
        self.identifier.as_str()
    }

    /// Returns a reference to the values to be used during the formatting of the localised pattern string.
    pub fn values( &self ) -> &HashMap<String, PlaceholderValue<A>> {
        &self.values
    }
}

impl<A: AsCalendar> Display for ErrorMessage<A> {

    /// In case the error has not been handled by the application, the Rust default error handler will call this
    /// method, which simply writes the preformatted string to the formatter's buffer, ignoring the associated locale.
    /// 
    /// If wanting to have the error displayed for another locale, the error must be caught, then use message system to 
    /// produce a string for the locale, and finally use any of the standard output macros to write out the string.
    fn fmt( &self, formatter: &mut Formatter ) -> FmtResult {
        formatter.write_str( self.string.as_str() )
    }
}

/// Does not support inner errors.
impl<A: AsCalendar + Debug> Error for ErrorMessage<A> {}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {

    }
}
*/
