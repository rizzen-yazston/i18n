// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

use std::path::PathBuf;
use i18n_registry::RegistryError;
use rusqlite::Error as Sqlite3Error;
use std::io::Error as IoError;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug )]
#[non_exhaustive]
pub enum ProviderSqlite3Error {
    Io( IoError ),
    Sqlite3( Sqlite3Error ),
    NotDirectory( PathBuf ),
    NoSqlite3Files( PathBuf ),
    MissingIdentifierPart( String ),
    LanguageTagRegistry( RegistryError ),
    InvalidPath,
}

impl Display for ProviderSqlite3Error {

    /// Simply call the display formatter of embedded error.
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match *self {
            ProviderSqlite3Error::Io( ref error ) => error.fmt( formatter ),
            ProviderSqlite3Error::Sqlite3( ref error ) => error.fmt( formatter ),
            ProviderSqlite3Error::NotDirectory( ref path ) =>
                write!( formatter, "Provided path ‘{}’ is not a directory.", path.display() ),
            ProviderSqlite3Error::NoSqlite3Files( ref path ) =>
                write!( formatter, "No ‘.sqlite3’ files was found in ‘{}’.", path.display() ),
            ProviderSqlite3Error::MissingIdentifierPart( ref string ) =>
                write!( formatter, "Missing either string or crate identifier part for ‘{}’.", string ),
            ProviderSqlite3Error::LanguageTagRegistry( ref error ) => error.fmt( formatter ),
            ProviderSqlite3Error::InvalidPath =>
                write!( formatter, "Invalid path provided." ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for ProviderSqlite3Error {}

impl From<IoError> for ProviderSqlite3Error {
    fn from( error: IoError ) -> ProviderSqlite3Error {
        ProviderSqlite3Error::Io( error )
    }
}

impl From<Sqlite3Error> for ProviderSqlite3Error {
    fn from( error: Sqlite3Error ) -> ProviderSqlite3Error {
        ProviderSqlite3Error::Sqlite3( error )
    }
}

impl From<RegistryError> for ProviderSqlite3Error {
    fn from( error: RegistryError ) -> ProviderSqlite3Error {
        ProviderSqlite3Error::LanguageTagRegistry( error )
    }
}
