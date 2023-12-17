// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

use std::path::PathBuf;
use i18n_utility::{ RegistryError, LocalisationTrait, LocalisationErrorTrait };
use rusqlite::Error as Sqlite3Error;
use std::io::Error as IoError;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

/// The `ProviderSqlite3Error` type consists of the follow:
/// 
/// * `Io`: Wraps the file system [`IoError`],
/// 
/// * `Sqlite3`: Wraps the Sqlite3 error [`Sqlite3Error`],
/// 
/// * `LanguageTagRegistry`: Wraps the language tag registry [`RegistryError`],
/// 
/// * `NotDirectory`: Indicates provided path is not a directory,
/// 
/// * `NoSqlite3Files`: Indicates no Sqlite3 was found in the directory,
/// 
/// * `PathConversion`: Supposed to be infallible, yet may return an error,
/// 
/// * `DefaultLanguage`: Expected default language was not found,
/// 
/// * `InvalidDefaultLanguage`: Default language is nit in component's language.
/// 
/// * `NotExists`: Indicates path does not exists,
/// 
/// * `ComponentNotFound`: Indicates requested component is not found,
/// 
/// * `SchemaInvalid`: Indicates the schema of the Sqlite3 file is invalid.
#[derive( Debug )]
#[non_exhaustive]
pub enum ProviderSqlite3Error {
    Io( IoError ),
    Sqlite3( Sqlite3Error ),
    LanguageTagRegistry( RegistryError ),
    NotDirectory( PathBuf ),
    NoSqlite3Files( PathBuf ),
    PathConversion,
    DefaultLanguage( String ),
    InvalidDefaultLanguage( String ),
    NotExists( PathBuf ),
    ComponentNotFound( String ),
    SchemaInvalid( String ),
}

impl LocalisationTrait for ProviderSqlite3Error {
    fn identifier( &self ) -> &str {
        match *self {
            ProviderSqlite3Error::NotDirectory( _ ) => "path_not_directory",
            ProviderSqlite3Error::NoSqlite3Files( _ ) => "no_sqlite3",
            ProviderSqlite3Error::PathConversion => "path_conversion",
            ProviderSqlite3Error::DefaultLanguage( _ ) => "default_language",
            ProviderSqlite3Error::InvalidDefaultLanguage( _ ) => "invalid_default_language",
            ProviderSqlite3Error::NotExists( _ ) => "path_not_exist",
            ProviderSqlite3Error::ComponentNotFound( _ ) => "component_not_found",
            ProviderSqlite3Error::SchemaInvalid( _ ) => "schema_invalid",
            _ => "",
        }
    }

    fn component( &self ) -> &str {
        "i18n_provider_sqlite3"
    }
}

impl LocalisationErrorTrait for ProviderSqlite3Error {
    fn error_type( &self ) -> &str {
        "ProviderSqlite3Error"
    }

    fn error_variant( &self ) -> &str {
        match *self {
            ProviderSqlite3Error::Io( _ ) => "Io",
            ProviderSqlite3Error::Sqlite3( _ ) => "Sqlite3",
            ProviderSqlite3Error::LanguageTagRegistry( _ ) => "LanguageTagRegistry",
            ProviderSqlite3Error::NotDirectory( _ ) => "NotDirectory",
            ProviderSqlite3Error::NoSqlite3Files( _ ) => "NoSqlite3Files",
            ProviderSqlite3Error::PathConversion => "PathConversion",
            ProviderSqlite3Error::DefaultLanguage( _ ) => "DefaultLanguage",
            ProviderSqlite3Error::InvalidDefaultLanguage( _ ) => "InvalidDefaultLanguage",
            ProviderSqlite3Error::NotExists( _ ) => "NotExists",
            ProviderSqlite3Error::ComponentNotFound( _ ) => "ComponentNotFound",
            ProviderSqlite3Error::SchemaInvalid( _ ) => "SchemaInvalid",
        }
    }    
}

impl Display for ProviderSqlite3Error {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match *self {
            ProviderSqlite3Error::Io( ref error ) => write!(
                formatter, "ProviderSqlite3Error::Io: [{}].", error.to_string()
            ),
            ProviderSqlite3Error::Sqlite3( ref error ) => write!(
                formatter, "ProviderSqlite3Error::Sqlite3: [{}].", error.to_string()
            ),
            ProviderSqlite3Error::LanguageTagRegistry( ref error ) => write!(
                formatter, "ProviderSqlite3Error::LanguageTagRegistry: [{}].", error.to_string()
            ),
            ProviderSqlite3Error::NotDirectory( ref path ) => write!(
                formatter,
                "ProviderSqlite3Error::NotDirectory: Provided path ‘{}’ is not a directory.",
                path.display()
            ),
            ProviderSqlite3Error::NoSqlite3Files( ref path ) => write!(
                formatter,
                "ProviderSqlite3Error::NoSqlite3Files: No ‘.sqlite3’ files was found in ‘{}’.",
                path.display()
            ),
            ProviderSqlite3Error::PathConversion => write!(
                formatter, "ProviderSqlite3Error::PathConversion: Conversion to PathBuf error."
            ),
            ProviderSqlite3Error::DefaultLanguage( ref component ) => write!(
                formatter,
                "ProviderSqlite3Error::DefaultLanguage: The default language tag is missing for the component ‘{}’.",
                component
            ),
            ProviderSqlite3Error::InvalidDefaultLanguage( ref component ) => write!(
                formatter,
                "ProviderSqlite3Error::InvalidDefaultLanguage: The default language tag is invalid for the component \
                ‘{}’.",
                component
            ),
            ProviderSqlite3Error::NotExists( ref path ) => write!(
                formatter, "ProviderSqlite3Error::NotExists: Provided path ‘{}’ does not exist.", path.display()
            ),
            ProviderSqlite3Error::ComponentNotFound( ref component ) => write!(
                formatter, "ProviderSqlite3Error::ComponentNotFound: The component ‘{}’ could not found.", component ),
            ProviderSqlite3Error::SchemaInvalid( ref component ) => write!(
                formatter,
                "ProviderSqlite3Error::SchemaInvalid: The Sqlite3 file schema is invalid for the component ‘{}’.",
                component
            ),
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
