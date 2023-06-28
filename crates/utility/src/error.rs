// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use icu_locid::ParserError;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug, PartialEq, Copy, Clone )]
#[non_exhaustive]
pub enum RegistryError {
    Locale( ParserError ),
}

impl Display for RegistryError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match *self {
            RegistryError::Locale( ref error ) => error.fmt( formatter ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for RegistryError {}

impl From<ParserError> for RegistryError {
    fn from( error: ParserError ) -> RegistryError {
        RegistryError::Locale( error )
    }
}
