// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

use std::path::PathBuf;
use i18n_utility::RegistryError;
use rusqlite::Error as Sqlite3Error;
use std::io::Error as IoError;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

/// The `LocalisationProviderSqlite3Error` type consists of the follow:
/// 
/// * `Io`: Wraps the file system [`IoError`],
/// 
/// * `Sqlite3`: Wraps the Sqlite3 error,
/// 
/// * `NotDirectory`: Indicates provided path is not a directory,
/// 
/// * `NoSqlite3Files`: Indicates no Sqlite3 was found in the directory,
/// 
/// * `LanguageTagRegistry`: Wraps the language tag registry [`RegistryError`],
/// 
/// * `PathConversion`: Supposed to be infallible, yet may return an error,
/// 
/// * `DefaultLanguage`: Expected default language was not found,
/// 
/// * `NotExists`: Indicates path does not exists,
/// 
/// * `ComponentNotFound`: Indicates requested component is not found.
#[derive( Debug )]
#[non_exhaustive]
pub enum LocalisationProviderSqlite3Error {
    Io( IoError ),
    Sqlite3( Sqlite3Error ),
    NotDirectory( PathBuf ),
    NoSqlite3Files( PathBuf ),
    LanguageTagRegistry( RegistryError ),
    PathConversion,
    DefaultLanguage( String ),
    NotExists( PathBuf ),
    ComponentNotFound( String ),
}

impl Display for LocalisationProviderSqlite3Error {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match *self {
            LocalisationProviderSqlite3Error::Io( ref error ) => error.fmt( formatter ),
            LocalisationProviderSqlite3Error::Sqlite3( ref error ) => error.fmt( formatter ),
            LocalisationProviderSqlite3Error::NotDirectory( ref path ) =>
                write!( formatter, "Provided path ‘{}’ is not a directory.", path.display() ),
            LocalisationProviderSqlite3Error::NoSqlite3Files( ref path ) =>
                write!( formatter, "No ‘.sqlite3’ files was found in ‘{}’.", path.display() ),
            LocalisationProviderSqlite3Error::LanguageTagRegistry( ref error ) => error.fmt( formatter ),
            LocalisationProviderSqlite3Error::PathConversion => write!( formatter, "Conversion to PathBuf error." ),
            LocalisationProviderSqlite3Error::DefaultLanguage( ref component ) =>
                write!( formatter, "The default language tag is missing for the component ‘{}’.", component ),
            LocalisationProviderSqlite3Error::NotExists( ref path ) =>
                write!( formatter, "Provided path ‘{}’ does not exist.", path.display() ),
            LocalisationProviderSqlite3Error::ComponentNotFound( ref component ) =>
                write!( formatter, "Component ‘{}’ is not found.", component ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for LocalisationProviderSqlite3Error {}

impl From<IoError> for LocalisationProviderSqlite3Error {
    fn from( error: IoError ) -> LocalisationProviderSqlite3Error {
        LocalisationProviderSqlite3Error::Io( error )
    }
}

impl From<Sqlite3Error> for LocalisationProviderSqlite3Error {
    fn from( error: Sqlite3Error ) -> LocalisationProviderSqlite3Error {
        LocalisationProviderSqlite3Error::Sqlite3( error )
    }
}

impl From<RegistryError> for LocalisationProviderSqlite3Error {
    fn from( error: RegistryError ) -> LocalisationProviderSqlite3Error {
        LocalisationProviderSqlite3Error::LanguageTagRegistry( error )
    }
}
