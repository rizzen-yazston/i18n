// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

use crate::ProviderSqlite3Error;
use i18n_registry::registry::LanguageTagRegistry;
use i18n_lstring::LString;
use i18n_provider::{ LStringProvider, ProviderError };
use rusqlite::{ Connection, OpenFlags };
use std::collections::HashMap;
use std::rc::Rc;
use std::path::PathBuf;
use core::cell::RefCell;

/// `ProviderSqlite3` struct is an implementation of the `LStringProvider` trait, and uses Sqlite3 as the data store
/// for language strings. As the directory path of the data store is embedded in the `ProviderSqlite3` struct upon
/// creating an instance of `ProviderSqlite3`, one can have multiple `ProviderSqlite3` objects representing the
/// application itself, and for data packages that supports internationalisation.
/// 
/// # Examples
/// 
/// ```
/// use i18n_provider_sqlite3::ProviderSqlite3;
/// use i18n_provider::LStringProvider;
/// use i18n_registry::LanguageTagRegistry;
/// use std::rc::Rc;
/// use std::error::Error;
/// fn main() -> Result<(), Box<dyn Error>> {
///     let path = "./i18n/";
///     let registry = Rc::new( LanguageTagRegistry::new() );
///     let tag = registry.get_language_tag( "en" )?;
///     let provider = ProviderSqlite3::try_new(
///         path,
///         &registry
///     )?;
///     let strings = provider.get(
///         "i18n_provider_sqlite3/invalid_path",
///         &tag
///     )?;
///     assert_eq!( strings.len(), 1, "There should be 1 string." );
///     assert_eq!( strings[ 0 ].as_str(), "Invalid path provided.", "Not correct string." );
///     assert_eq!( strings[ 0 ].language_tag().as_str(), "en-ZA", "Must be en-ZA." );
///     Ok( () )
/// }
/// ```
pub struct ProviderSqlite3 {
    directory: PathBuf,
    language_tag_registry: Rc<LanguageTagRegistry>,
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
        language_tag_registry: &Rc<LanguageTagRegistry>
    ) -> Result<Self, ProviderSqlite3Error> {
        let Ok( directory ) = directory_path.try_into() else {
                return Err( ProviderSqlite3Error::InvalidPath )
        };
        if !directory.is_dir() {
            return Err( ProviderSqlite3Error::NotDirectory( directory ) );
        }
        let mut found = false;
        let iterator = directory.read_dir()?;
        for entry in iterator {
            let entry_data = entry?;
            if let Some( extension ) = entry_data.path().extension() {
                if extension == "sqlite3" {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return Err( ProviderSqlite3Error::NoSqlite3Files( directory ) );
        }
        Ok( ProviderSqlite3 {
            directory,
            language_tag_registry: Rc::clone( language_tag_registry ),
            connections: RefCell::new( HashMap::<String, Connection>::new() ),
        } )
    }

    fn add_connection<T: AsRef<str>>( &self, identifier: T ) -> Result<(), ProviderSqlite3Error> {
        let mut file = identifier.as_ref().to_string();
        file.push_str( ".sqlite3" );
        let mut path = self.directory.clone();
        path.push( file );
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI
        )?;
        self.connections.borrow_mut().insert( identifier.as_ref().to_string(), connection );
        Ok( () )
    }
}

impl LStringProvider for ProviderSqlite3 {

    /// Retrieve a vector of possible `LString` for requested identifier that matches a language tag.
    /// 
    /// Ideally a single exact match should be returned, yet may not be for the requested language tag. If no strings
    /// is found for the requested tag, the right most subtag is removed sequentially until there are no more subtags.
    /// Multiple `LString`s may be returned when there are multiple entries of language tags having additional subtags
    /// than the requested language tag.
    /// 
    /// Return of `ErrorMessage` indicates there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_provider::LStringProvider;
    /// use i18n_registry::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./i18n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let tag = registry.get_language_tag( "en" )?;
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let strings = provider.get(
    ///         "i18n_provider_sqlite3/invalid_path",
    ///         &tag
    ///     )?;//.expect( "No string found for language tag." );
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
    ) -> Result<Vec<LString>, ProviderError> {
        let parts: Vec<&str> = identifier.as_ref().split( '/' ).collect();
        if parts.len() != 2 {
            return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::MissingIdentifierPart( identifier.as_ref().to_string() ) ),
                }
            );
        }
        let mut have = false;
        {
            if self.connections.borrow().contains_key( parts[ 0 ] ) {
                have = true;
            }
        }
        {
            if !have {
                match self.add_connection( parts[ 0 ] ) {
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    ),
                    Ok( () ) => {}
                };
            }
        }
        let borrow = self.connections.borrow();
        let connection = borrow.get( parts[ 0 ] ).unwrap();
        let mut stmt = match connection.prepare(
            "SELECT string, languageTag FROM pattern WHERE identifier = ?1 AND languageTag like ?2"
        ) {
            Ok( result ) => result,
            Err( error ) => return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                }
            )
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
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            for row in rows {
                let ( string, tag_raw ) = match row {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                };
                let language = match
                    self.language_tag_registry.as_ref().get_language_tag( tag_raw ) {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                        }
                    )
                };
                result.push( LString::new( string, &language ) );
            }
            if result.len() > 0 {
                return Ok( result );
            }
            let Some( parts ) = tag.rsplit_once( '-' ) else {
                return Ok( result );
            };
            tag = parts.0.to_owned();
            tag.push( '%' );
        }
        Ok( result )
    }

    /// Similar to `get()` method, except that `get_one()` will only return a single `LString` if multiple strings are
    /// available. 
    /// 
    /// `None` is returned when there is no strings available for the language tag.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_provider::LStringProvider;
    /// use i18n_registry::LanguageTagRegistry;
    /// use core::cell::RefCell;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./i18n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let tag = registry.get_language_tag( "en" )?;
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let strings = provider.get(
    ///         "i18n_provider_sqlite3/invalid_path",
    ///         &tag
    ///     )?;//.expect( "No string found for language tag." );
    ///     assert_eq!( strings.len(), 1, "There should be 1 string." );
    ///     assert_eq!( strings[ 0 ].as_str(), "Invalid path provided.", "Not correct string." );
    ///     assert_eq!( strings[ 0 ].language_tag().as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    fn get_one<T: AsRef<str>>(
        &self, identifier: T,
        language_tag: &Rc<String>
    ) -> Result<Option<LString>, ProviderError> {
        let mut result = self.get( identifier, language_tag )?;
        //temp for now, TODO: try to return string closest to the language tag, by match language length
        Ok( result.pop() )
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
    /// use i18n_registry::LanguageTagRegistry;
    /// use core::cell::RefCell;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./i18n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
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
    fn default_language_tag<T: AsRef<str>>( &self, identifier: T ) -> Result<Option<String>, ProviderError> {
        let mut have = false;
        {
            if self.connections.borrow().contains_key( identifier.as_ref() ) {
                have = true;
            }
        }
        {
            if !have {
                match self.add_connection( identifier.as_ref() ) {
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    ),
                    Ok( () ) => {}
                };
            }
        }
        let borrow = self.connections.borrow();
        let connection = borrow.get( identifier.as_ref() ).unwrap();
        let mut stmt = match connection.prepare(
            "SELECT value FROM metadata WHERE key = 'default_language_tag'"
        ) {
            Ok( result ) => result,
            Err( error ) => return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                }
            )
        };
        let mut rows = match stmt.query( [] ) {
            Ok( result ) => result,
            Err( error ) => return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                }
            )
        };
        if let Some( row ) = match rows.next() {
            Ok( result ) => result,
            Err( error ) => return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                }
            )
        } {
            if let Some( string ) = match row.get( 0 ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            } {
                return Ok( Some( string ) );
            }
        };
        Ok( None )
    }
}

// The next 3 items (struct, trait, and impl) is implementing the concept of storing an `impl Trait` into a struct as
// a member, has been taken from the `icu_provider` crate, particular the serde module.
pub struct LStringProviderSqlite3<'a, P: ?Sized>( &'a P );

pub trait AsLStringProviderSqlite3 {
    fn as_lstring_provider( &self ) -> LStringProviderSqlite3<Self>;
}

impl<P> AsLStringProviderSqlite3 for P
where
    P: LStringProvider + ?Sized,
{
    fn as_lstring_provider( &self ) -> LStringProviderSqlite3<Self> {
        LStringProviderSqlite3( self )
    }
}
