// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

//! Sqlite3 provider for `LString`s.
//! 
//! Provides `LString`.
//! 
//! # Examples
//! 
//! ```
//! // TODO
//! ```
//! 

use i18n_error::{ ErrorMessage, ErrorPlaceholderValue };
use i18n_utility::locale::LanguageTagRegistry;
use i18n_lstring::LString;
use i18n_provider::LStringProvider;
use rusqlite::{ Connection, OpenFlags };
use core::cell::RefCell;
use std::error::Error;
use std::collections::HashMap;
use std::rc::Rc;
use std::path::PathBuf;

/// LString provider.
/// 
/// TODO:
/// 
/// # Examples
/// 
/// ```
/// // TODO
/// ```
/// [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
/// [`icu_locid`]: https://crates.io/crates/icu_locid
pub struct ProviderSqlite3 {
    directory: PathBuf,
    language_tag_registry: Rc<RefCell<LanguageTagRegistry>>,
    connections: RefCell<HashMap<String, Connection>>,
}

impl ProviderSqlite3 {

    /// Create a `ProviderSqlite3` type for the specified directory path.
    /// 
    /// As this is directory path based provider, there may be multiple providers, where one provider would just be for
    /// the application's messages (including libraries' error messages (where supported)), while the rest of the
    /// providers would be various data packages' messages located in separate directories.
    /// 
    /// Returns a `ProviderSqlite3` struct, which indicates a valid path to directory containing `.sqlite3` files.
    /// 
    /// Returns a `ErrorMessage` when there is an error in verifying the path is a directory and contains `.sqlite3`
    /// files.
    pub fn try_new<T: TryInto<PathBuf>>(
        directory_path: T,
        language_tag_registry: &Rc<RefCell<LanguageTagRegistry>>
    ) -> Result<Self, ErrorMessage> {
        let Ok( directory ) = directory_path.try_into() else {
            return Err( ErrorMessage {
                string: String::from( "Invalid path provided." ),
                identifier: String::from( "i18n_provider_sqlite3/invalid_path" ),
                values: HashMap::<String, ErrorPlaceholderValue>::new(),
            } );
        };
        if !directory.is_dir() {
            return Err( ErrorMessage {
                string: String::from( "Provided path is not a directory." ),
                identifier: String::from( "i18n_provider_sqlite3/path_not_directory" ),
                values: HashMap::<String, ErrorPlaceholderValue>::new(),
            } );
        }
        let mut found = false;
        let Ok( iterator ) = directory.read_dir() else {
            return Err( ErrorMessage {
                string: String::from( "Failed to get directory iterator." ),
                identifier: String::from( "i18n_provider_sqlite3/directory_iterator" ),
                values: HashMap::<String, ErrorPlaceholderValue>::new(),
            } );
        };
        for entry in iterator {
            let Ok( entry_data ) = entry else {
                return Err( ErrorMessage {
                    string: String::from( "Failed to retrieve directory entry." ),
                    identifier: String::from( "i18n_provider_sqlite3/directory_entry" ),
                    values: HashMap::<String, ErrorPlaceholderValue>::new(),
                } );
            };
            if !entry_data.path().extension().is_none() {
                found = true;
                break;
            }
        }
        if !found {
            return Err( ErrorMessage {
                string: String::from( "No `.sqlite3` files was found." ),
                identifier: String::from( "i18n_provider_sqlite3/no_sqlite3" ),
                values: HashMap::<String, ErrorPlaceholderValue>::new(),
            } );
        }
        Ok( ProviderSqlite3 {
            directory,
            language_tag_registry: Rc::clone( language_tag_registry ),
            connections: RefCell::new( HashMap::<String, Connection>::new() ),
        } )
    }

    fn add_connection<T: AsRef<str>>( &self, identifier: T ) -> Result<(), ErrorMessage> {
        let mut file = identifier.as_ref().to_string();
        file.push_str( ".sqlite3" );
        let mut path = self.directory.clone();
        path.push( file );
        let connection = match Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI
        ) {
            Ok( result ) => result,
            Err( err ) => return Err( error( &err ) )
        };
        self.connections.borrow_mut().insert( identifier.as_ref().to_string(), connection );
        Ok( () )
    }
}

impl LStringProvider for ProviderSqlite3 {

    /// Retrieve a vector of possible `LString` for requested identifier that matches a language tag.
    /// 
    /// Ideally a single exact match should be returned, yet may not be for the requested language tag. If no strings
    /// is found for the requested tag, the right most subtag is removed sequentially until either at least 1 `LString`
    /// is found, or `None returned when there are no more subtags to be removed. Multiple `LString` may be returned
    /// when there are multiple entries of language tags having additional subtags than the requested language tag. 
    /// 
    /// Return of `None` indicates no strings was found matching the requested language tag, or its more general form.
    /// 
    /// Return of `ErrorMessage` indicates there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_provider::LStringProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use core::cell::RefCell;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./i18n/";
    ///     let registry = Rc::new( RefCell::new( LanguageTagRegistry::new() ) );
    ///     let tag = registry.borrow_mut().get_language_tag( "en" )?;
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let strings = provider.get(
    ///         "i18n_provider_sqlite3/invalid_path",
    ///         &tag
    ///     )?.expect( "No string found for language tag." );
    ///     assert_eq!( strings.len(), 1, "There should be 1 string." );
    ///     assert_eq!( strings[ 0 ].as_str(), "Invalid path provided.", "Not correct string." );
    ///     assert_eq!( strings[ 0 ].language_tag().as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    fn get<T: AsRef<str>>(
        &self,
        identifier: T,
        language_tag: &Rc<String>
    ) -> Result<Option<Vec<LString>>, ErrorMessage> {
        let parts: Vec<&str> = identifier.as_ref().split( '/' ).collect();
        if parts.len() != 2 {
            return Err( ErrorMessage {
                string: String::from(
                    "Missing string or crate identifier part. Expects: 'crate_identifier/string_identifier'."
                ),
                identifier: String::from( "i18n_provider_sqlite3/missing_identifier_part" ),
                values: HashMap::<String, ErrorPlaceholderValue>::new(),
            } );
        }
        let mut have = false;
        {
            if self.connections.borrow().contains_key( parts[ 0 ] ) {
                have = true;
            }
        }
        {
            if !have {
                self.add_connection( parts[ 0 ] )?;
            }
        }
        let borrow = self.connections.borrow();
        let connection = borrow.get( parts[ 0 ] ).unwrap();
        let mut stmt = match connection.prepare(
            "SELECT string, languageTag FROM pattern WHERE identifier = ?1 AND languageTag like ?2"
        ) {
            Ok( result ) => result,
            Err( err ) => return Err( error( &err ) )
        };
        let mut result = Vec::<LString>::new();
        let mut tag = language_tag.to_string();
        tag.push( '%' );
        while tag.len() > 0 {
            let rows = match stmt.query_map(
                [ parts[ 1 ], tag.as_str() ],
                |row| {
                    let string: String = row.get( 0 )?;
                    let tag: String = row.get( 1 )?;
                    Ok( ( string, tag ) )
                }
            ) {
                Ok( result ) => result,
                Err( err ) => return Err( error( &err ) )
            };
            for row in rows {
                let ( string, tag_raw ) = match row {
                    Ok( result ) => result,
                    Err( err ) => return Err( error( &err ) )
                };
                let language = self.language_tag_registry.borrow().get_language_tag( tag_raw )?;
                result.push( LString::new( string, &language ) );
            }
            if result.len() > 0 {
                return Ok( Some( result ) );
            }
            let Some( parts ) = tag.rsplit_once( '-' ) else {
                return Ok( None );
            };
            tag = parts.0.to_owned();
            tag.push( '%' );
        }
        Ok( None )
    }

    /// Retrieve the default language of the crate.
    /// 
    /// Return of `None` indicates no default language tag was found.
    /// 
    /// Return of `ErrorMessage` indicates there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_provider::LStringProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use core::cell::RefCell;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./i18n/";
    ///     let registry = Rc::new( RefCell::new( LanguageTagRegistry::new() ) );
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let tag = provider.default_language_tag(
    ///         "i18n_provider_sqlite3"
    ///     )?.expect( "No default language tag found." );
    ///     assert_eq!( tag, "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    fn default_language_tag<T: AsRef<str>>( &self, identifier: T ) -> Result<Option<String>, ErrorMessage> {
        let mut have = false;
        {
            if self.connections.borrow().contains_key( identifier.as_ref() ) {
                have = true;
            }
        }
        {
            if !have {
                self.add_connection( identifier.as_ref() )?;
            }
        }
        let borrow = self.connections.borrow();
        let connection = borrow.get( identifier.as_ref() ).unwrap();
        let mut stmt = match connection.prepare(
            "SELECT value FROM metadata WHERE key = 'default_language_tag'"
        ) {
            Ok( result ) => result,
            Err( err ) => return Err( error( &err ) )
        };
        let mut rows = match stmt.query( [] ) {
            Ok( result ) => result,
            Err( err ) => return Err( error( &err ) )
        };
        if let Some( row ) = match rows.next() {
            Ok( result ) => result,
            Err( err ) => return Err( error( &err ) )
        } {
            if let Some( string ) = match row.get( 0 ) {
                Ok( result ) => result,
                Err( err ) => return Err( error( &err ) )
            } {
                return Ok( Some( string ) );
            }
        };
        Ok( None )
    }
}

/// Simply function for converting Sqlite errors into `ErrorMessage`.
pub fn error( err: &dyn Error ) -> ErrorMessage {
    let error = err.to_string();
    let mut string = "Sqlite error: ".to_string();
    string.push_str( error.as_str() );
    let mut values = 
        HashMap::<String, ErrorPlaceholderValue>::new();
    values.insert( "error".to_string(), ErrorPlaceholderValue::String( error ) );
    ErrorMessage {
        string,
        identifier: String::from( "i18n_provider_sqlite3/sqlite_error" ),
        values,
    }
}
