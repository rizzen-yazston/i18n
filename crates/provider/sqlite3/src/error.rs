// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

use i18n_provider::{ ProviderError, ProviderErrorTrait };
use i18n_utility::{ LocalisationData, LocalisationTrait, PlaceholderValue, };
use rusqlite::Error as Sqlite3Error;
use std::{
    path::PathBuf,
    io::Error as IoError,
    error::Error, // Experimental in `core` crate.
    collections::HashMap,
};
use core::fmt::{ Display, Formatter, Result };

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

/// The `ProviderSqlite3Error` type consists of the follow:
/// 
/// * `Io`: Wraps the file system [`IoError`],
/// 
/// * `Sqlite3`: Wraps the Sqlite3 error [`Sqlite3Error`],
/// 
/// * `NotDirectory`: Indicates provided path is not a directory,
/// 
/// * `NoSqlite3Files`: Indicates no Sqlite3 was found in the directory,
/// 
/// * `PathConversion`: Supposed to be infallible, yet may return an error,
/// 
/// * `NotExists`: Indicates path does not exists,
/// 
/// * `SchemaInvalid`: Indicates the schema of the Sqlite3 file is invalid.
#[derive( Debug, Clone )]
#[non_exhaustive]
pub enum ProviderSqlite3Error {
    Io( RefCount<IoError> ),
    Sqlite3( RefCount<Sqlite3Error> ),
    NotDirectory( PathBuf ),
    NoSqlite3Files( PathBuf ),
    PathConversion,
    NotExists( PathBuf ),
    SchemaInvalid( SchemaError ),
}

impl ProviderErrorTrait for ProviderSqlite3Error {}

impl LocalisationTrait for ProviderSqlite3Error {
    fn localisation_data( &self ) -> LocalisationData {
        let type_string = PlaceholderValue::String( "ProviderSqlite3Error".to_string() );
        match self {
            ProviderSqlite3Error::Io( ref error ) => {
                // Currently no localisation is available for this error type: IoError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "Io".to_string() ) ); 
                values.insert( "error".to_string(), PlaceholderValue::String( error.to_string() ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some( values ),
                }
            },
            ProviderSqlite3Error::Sqlite3( ref error ) => {
                // Currently no localisation is available for this error type: Sqlite3Error.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "Sqlite3".to_string() ) ); 
                values.insert( "error".to_string(), PlaceholderValue::String( error.to_string() ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some( values ),
                }
            },
            ProviderSqlite3Error::NotDirectory( ref path ) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "path".to_string(),
                    PlaceholderValue::String( path.display().to_string() )
                );
                let message = LocalisationData {
                    component: "i18n_provider_sqlite3".to_string(),
                    identifier: "path_not_directory".to_string(),
                    values: Some( message_values ),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "NotDirectory".to_string() ) ); 
                values.insert( "message".to_string(), PlaceholderValue::LocalisationData( message ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some( values ),
                }
            },
            ProviderSqlite3Error::NoSqlite3Files( ref path ) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "path".to_string(),
                    PlaceholderValue::String( path.display().to_string() )
                );
                let message = LocalisationData {
                    component: "i18n_provider_sqlite3".to_string(),
                    identifier: "no_sqlite3".to_string(),
                    values: Some( message_values ),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "NoSqlite3Files".to_string() ) ); 
                values.insert( "message".to_string(), PlaceholderValue::LocalisationData( message ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some( values ),
                }
            },
            ProviderSqlite3Error::PathConversion => {
                let message = LocalisationData {
                    component: "i18n_provider_sqlite3".to_string(),
                    identifier: "path_conversion".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "PathConversion".to_string() ) ); 
                values.insert( "message".to_string(), PlaceholderValue::LocalisationData( message ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some( values ),
                }
            },
            ProviderSqlite3Error::NotExists( ref path ) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "path".to_string(),
                    PlaceholderValue::String( path.display().to_string() )
                );
                let message = LocalisationData {
                    component: "i18n_provider_sqlite3".to_string(),
                    identifier: "path_not_exist".to_string(),
                    values: Some( message_values ),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "NotExists".to_string() ) ); 
                values.insert( "message".to_string(), PlaceholderValue::LocalisationData( message ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some( values ),
                }
            },
            ProviderSqlite3Error::SchemaInvalid( ref error ) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "SchemaInvalid".to_string() ) ); 
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData( error.localisation_data() )
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some( values ),
                }
            },
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
            ProviderSqlite3Error::NotExists( ref path ) => write!(
                formatter, "ProviderSqlite3Error::NotExists: Provided path ‘{}’ does not exist.", path.display()
            ),
            ProviderSqlite3Error::SchemaInvalid( ref error ) => write!(
                formatter, "ProviderSqlite3Error::SchemaInvalid: [{}].", error.to_string()
            ),
        }
    }
}

impl Error for ProviderSqlite3Error {}

impl From<IoError> for ProviderSqlite3Error {
    fn from( error: IoError ) -> ProviderSqlite3Error {
        ProviderSqlite3Error::Io( RefCount::new( error ) )
    }
}

impl From<Sqlite3Error> for ProviderSqlite3Error {
    fn from( error: Sqlite3Error ) -> ProviderSqlite3Error {
        ProviderSqlite3Error::Sqlite3( RefCount::new( error ) )
    }
}

impl From<ProviderSqlite3Error> for ProviderError {
    fn from( error: ProviderSqlite3Error ) -> ProviderError {
        ProviderError::Custom( RefCount::new( Box::new( error ) ) )
    }
}

/// The `SchemaError` type consists of the follow:
/// 
/// * `Version`: The database is using incorrect schema version,
/// 
/// * `MissingVersion`: The database is missing the schema version in the `metadata` table.
/// 
/// * `Sqlite3`: Wraps the Sqlite3 error [`Sqlite3Error`],
#[derive( Debug, Clone )]
#[non_exhaustive]
pub enum SchemaError {
    Version( String, String ), // path, expected version
    MissingVersion( String ), // path
    Sqlite3( RefCount<Sqlite3Error> ),
}

impl LocalisationTrait for SchemaError {
    fn localisation_data( &self ) -> LocalisationData {
        let type_string = PlaceholderValue::String( "SchemaError".to_string() );
        match self {
            SchemaError::Version( ref path, ref version ) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert( "path".to_string(), PlaceholderValue::String( path.clone() ) );
                message_values.insert( "version".to_string(), PlaceholderValue::String( version.to_string() ) );
                let message = LocalisationData {
                    component: "i18n_provider_sqlite3".to_string(),
                    identifier: "schema_version".to_string(),
                    values: Some( message_values ),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "Version".to_string() ) ); 
                values.insert( "message".to_string(), PlaceholderValue::LocalisationData( message ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some( values ),
                }
            },
            SchemaError::MissingVersion( ref path ) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert( "path".to_string(), PlaceholderValue::String( path.clone() ) );
                let message = LocalisationData {
                    component: "i18n_provider_sqlite3".to_string(),
                    identifier: "schema_version_missing".to_string(),
                    values: Some( message_values ),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "MissingVersion".to_string() ) ); 
                values.insert( "message".to_string(), PlaceholderValue::LocalisationData( message ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some( values ),
                }
            },
            SchemaError::Sqlite3( ref error ) => {
                // Currently no localisation is available for this error type: Sqlite3Error.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert( "type".to_string(), type_string ); 
                values.insert( "variant".to_string(), PlaceholderValue::String( "Sqlite3".to_string() ) ); 
                values.insert( "error".to_string(), PlaceholderValue::String( error.to_string() ) ); 
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some( values ),
                }
            },
        }
    }
}

impl Display for SchemaError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match *self {
            SchemaError::Version( ref path, ref version ) => write!(
                formatter,
                "SchemaError::Version: The Sqlite3 file ‘{}’ is using unsupported schema version. The schema version \
                must be ‘{}’.",
                path,
                version,
            ),
            SchemaError::MissingVersion( ref path ) => write!(
                formatter,
                "SchemaError::MissingVersion: The Sqlite3 file ‘{}’ is missing the schema version entry in the \
                ‘metadata’ table.",
                path,
            ),
            SchemaError::Sqlite3( ref error ) => write!(
                formatter, "SchemaError::Sqlite3: [{}].", error.to_string()
            ),
        }
    }
}

impl Error for SchemaError {}

impl From<Sqlite3Error> for SchemaError {
    fn from( error: Sqlite3Error ) -> SchemaError {
        SchemaError::Sqlite3( RefCount::new( error ) )
    }
}
