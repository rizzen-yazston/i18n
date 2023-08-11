// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

use crate::ProviderSqlite3Error;
use i18n_utility::{ LanguageTagRegistry, LString };
use i18n_provider::{ LStringProvider, ProviderError, LanguageData };
use rusqlite::{ Connection, OpenFlags };
use std::collections::HashMap;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( not( feature = "sync" ) )]
use std::cell::RefCell as MutCell;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::{ Arc as RefCount, Mutex as MutCell };

use std::path::PathBuf;

/// `ProviderSqlite3` struct is an implementation of the [`LStringProvider`] trait, and uses Sqlite3 as the data store
/// for localisation data repository. As the directory path of the data store is embedded in the `ProviderSqlite3`
/// struct upon creation, one can have multiple `ProviderSqlite3` instances representing the application itself,
/// application plugin module, and for various data packages that supports internationalisation.
/// 
/// Supports two directory layouts, though both can be combined:
/// 
/// * A single file `all_in_one.sqlite3`, that has an additional `component` column to each of the tables for holding
/// the component name.
/// 
/// * Individual component sqlite3 files, using the component name as the filename. 
/// 
/// If both directory layouts are present, the `all_in_one.sqlite3` will be tried first, before trying the individual
/// component sqlite3 file.
/// 
/// Any non-sqlite3 files and sub directories present will be ignored.
/// 
/// # Examples
/// 
/// ```
/// use i18n_provider_sqlite3::ProviderSqlite3;
/// use i18n_provider::LStringProvider;
/// use i18n_utility::LanguageTagRegistry;
/// use std::rc::Rc;
/// use std::error::Error;
/// fn main() -> Result<(), Box<dyn Error>> {
///     let path = "./l10n/";
///     let registry = Rc::new( LanguageTagRegistry::new() );
///     let tag = registry.get_language_tag( "en" )?;
///     let provider = ProviderSqlite3::try_new(
///         path,
///         &registry
///     )?;
///     let strings = provider.get(
///         "i18n_provider_sqlite3",
///         "path_conversion",
///         &tag,
///     )?;
///     assert_eq!( strings.len(), 1, "There should be 1 string." );
///     assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
///     assert_eq!( strings[ 0 ].language_tag().as_str(), "en-ZA", "Must be en-ZA." );
///     Ok( () )
/// }
/// ```
/// 
/// [`LStringProvider`]: i18n_provider::LStringProvider
pub struct ProviderSqlite3 {
    directory: PathBuf,
    language_tag_registry: RefCount<LanguageTagRegistry>,
    connections: MutCell<HashMap<String, Connection>>,
    all_in_one: bool,
}

impl ProviderSqlite3 {

    /// Create a `ProviderSqlite3` type for the specified directory path.
    /// 
    /// As this provider is directory based, there may be multiple providers, where one provider would just be for the
    /// application's messages (including libraries' error messages (where supported)), while the rest of the
    /// providers would be various data packages' messages located in separate directories.
    /// 
    /// Parameter `directory_path` is a path to the directory containing the localisation sqlite3 files, usually named
    /// `l10n`.
    /// 
    /// Returns `ProviderSqlite3Error` when there is an error in verifying the path is a directory and it does contain
    /// `.sqlite3` files.
    pub fn try_new<T: TryInto<PathBuf>>(
        directory_path: T,
        language_tag_registry: &RefCount<LanguageTagRegistry>
    ) -> Result<Self, ProviderSqlite3Error> {
        let Ok( directory ) = directory_path.try_into() else {
            return Err( ProviderSqlite3Error::PathConversion ) // If not Infallible error.
        };
        if !directory.is_dir() {
            return Err( ProviderSqlite3Error::NotDirectory( directory ) );
        }

        // Check if all_in_one.sqlite3 exists, if so create the connection.
        let all_in_one = directory.join( "all_in_one.sqlite3" );
        if all_in_one.try_exists()? {
            let connections =
                MutCell::new( HashMap::<String, Connection>::new() );
            let connection = Connection::open_with_flags(
                all_in_one,
                OpenFlags::SQLITE_OPEN_READ_ONLY
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI
            )?;
    
            #[cfg( not( feature = "sync" ) )]
            connections.borrow_mut().insert( "all_in_one".to_string(), connection );
    
            #[cfg( feature = "sync" )]
            connections.lock().unwrap().insert( "all_in_one".to_string(), connection );

            return Ok( ProviderSqlite3 {
                directory,
                language_tag_registry: RefCount::clone( language_tag_registry ),
                connections,
                all_in_one: true,
            } )
        }

        // No all_in_one.sqlite3 present, then check for any other sqlite3 files are present, but do not create
        // connections.
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
            language_tag_registry: RefCount::clone( language_tag_registry ),
            connections: MutCell::new( HashMap::<String, Connection>::new() ),
            all_in_one: false,
        } )
    }

    fn add_connection<T: AsRef<str>>( &self, component: T ) -> Result<(), ProviderSqlite3Error> {
        let mut file = component.as_ref().to_string();
        file.push_str( ".sqlite3" );
        let path = self.directory.join( file );
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                | OpenFlags::SQLITE_OPEN_URI
        )?;

        #[cfg( not( feature = "sync" ) )]
        self.connections.borrow_mut().insert( component.as_ref().to_string(), connection );

        #[cfg( feature = "sync" )]
        self.connections.lock().unwrap().insert( component.as_ref().to_string(), connection );

        Ok( () )
    }
}

impl LStringProvider for ProviderSqlite3 {

    /// Retrieve a vector of possible [`LString`] for requested identifier that matches a language tag.
    /// 
    /// Ideally a single exact match should be returned, yet may not be for the requested language tag. If no strings
    /// is found for the requested tag, the right most subtag is removed sequentially until there are no more subtags.
    /// Multiple [`LString`]s may be returned when there are multiple entries of language tags having additional
    /// subtags than the requested language tag.
    /// 
    /// Return of `ProviderSqlite3Errore` indicates there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_provider::LStringProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let tag = registry.get_language_tag( "en" )?;
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let strings = provider.get(
    ///         "i18n_provider_sqlite3",
    ///         "path_conversion",
    ///         &tag,
    ///     )?;
    ///     assert_eq!( strings.len(), 1, "There should be 1 string." );
    ///     assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    ///     assert_eq!( strings[ 0 ].language_tag().as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    /// 
    /// [`LString`]: i18n_utility::LString
    fn get<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>
    ) -> Result<Vec<LString>, ProviderError> {
        let mut result = Vec::<LString>::new();
        let mut tag = language_tag.to_string();

        // Check if all_in_one has component.
        if self.all_in_one {
            #[cfg( not( feature = "sync" ) )]
            let borrow = self.connections.borrow();
    
            #[cfg( feature = "sync" )]
            let borrow = self.connections.lock().unwrap();
    
            let connection = borrow.get( "all_in_one" ).unwrap();
            let mut stmt = match connection.prepare(
                "SELECT string, languageTag FROM pattern WHERE component = ?1 AND identifier = ?2 AND \
                    languageTag like ?3"
            ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            while tag.len() > 0 {
                tag.push( '%' );
                let mut rows = match stmt.query(
                    [ component.as_ref(), identifier.as_ref(), tag.as_str() ]
                ) {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                };
                while let Some( row ) = match rows.next() {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                } {
                    let string: String = match row.get( 0 ) {
                        Ok( result ) => result,
                        Err( error ) => return Err(
                            ProviderError {
                                error_type: "ProviderSqlite3Error",
                                source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                            }
                        )
                    };
                    let tag_raw: String = match row.get( 1 ) {
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
                tag = match tag.rsplit_once( '-' ) {
                    None => String::new(),
                    Some( value ) => value.0.to_owned(),
                };
            }
        }

        // Try individual component sqlite3 files.
        let mut have = false;
        {
            #[cfg( not( feature = "sync" ) )]
            if self.connections.borrow().contains_key( component.as_ref() ) {
                have = true;
            }

            #[cfg( feature = "sync" )]
            if self.connections.lock().unwrap().contains_key( component.as_ref() ) {
                have = true;
            }
        }
        {
            if !have {
                match self.add_connection( component.as_ref() ) {
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

        #[cfg( not( feature = "sync" ) )]
        let borrow = self.connections.borrow();

        #[cfg( feature = "sync" )]
        let borrow = self.connections.lock().unwrap();

        let connection = borrow.get( component.as_ref() ).unwrap();
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
        while tag.len() > 0 {
            tag.push( '%' );
            let mut rows = match stmt.query(
                [ identifier.as_ref(), tag.as_str() ]
            ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            while let Some( row ) = match rows.next() {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            } {
                let string: String = match row.get( 0 ) {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                };
                let tag_raw: String = match row.get( 1 ) {
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
            tag = match tag.rsplit_once( '-' ) {
                None => String::new(),
                Some( value ) => value.0.to_owned(),
            };
        }
        Ok( result )
    }

    /// Similar to `get()` method, except that `get_one()` will only return a single [`LString`] if multiple strings
    /// are available. 
    /// 
    /// `None` is returned when there is no strings available for the language tag.
    /// 
    /// Return of [`ProviderError`] indicates there was a Sqlite3 error.
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
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let tag = registry.get_language_tag( "en" )?;
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let strings = provider.get(
    ///         "i18n_provider_sqlite3",
    ///         "path_conversion",
    ///         &tag,
    ///     )?;
    ///     assert_eq!( strings.len(), 1, "There should be 1 string." );
    ///     assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    ///     assert_eq!( strings[ 0 ].language_tag().as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    /// 
    /// [`LString`]: i18n_utility::LString
    /// [`ProviderError`]: i18n_provider::ProviderError
    fn get_one<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>
    ) -> Result<Option<LString>, ProviderError> {
        let mut result = self.get( component, identifier, language_tag )?;
        //temp for now, TODO: try to return string closest to the language tag, by match language length
        Ok( result.pop() )
    }

    /// Retrieve the default language tag of the component in the data repository.
    /// 
    /// Return of `None` indicates no default language tag was found for the component.
    /// 
    /// Return of [`ProviderError`] indicates there was a Sqlite3 error.
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
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let tag = provider.default_language_tag(
    ///         "i18n_provider_sqlite3",
    ///     )?.expect( "No default language tag found." );
    ///     assert_eq!( tag.as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    /// 
    /// [`ProviderError`]: i18n_provider::ProviderError
    fn default_language_tag<T: AsRef<str>>( &self, component: T ) -> Result<Option<RefCount<String>>, ProviderError> {

        // Check if all_in_one has component.
        if self.all_in_one {
            #[cfg( not( feature = "sync" ) )]
            let borrow = self.connections.borrow();
    
            #[cfg( feature = "sync" )]
            let borrow = self.connections.lock().unwrap();
    
            let connection = borrow.get( "all_in_one" ).unwrap();
            let mut stmt = match connection.prepare(
                "SELECT value FROM metadata WHERE key = 'default_language_tag' AND component = ?1"
            ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            let rows = match stmt.query_map(
                [ component.as_ref() ],
                |row| Ok( row.get( 0 )? )
            ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            let mut string = "".to_string();
            for row in rows {
                string = match row {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                };
            }
            if !string.is_empty() {
                match self.language_tag_registry.get_language_tag( string ) {
                    Ok( result ) => return Ok( Some( result ) ),
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                        }
                    )
                };
            }
        }

        // Try individual component sqlite3 files.
        let mut have = false;
        {
            #[cfg( not( feature = "sync" ) )]
                if self.connections.borrow().contains_key( component.as_ref() ) {
                have = true;
            }

            #[cfg( feature = "sync" )]
                if self.connections.lock().unwrap().contains_key( component.as_ref() ) {
                have = true;
            }
        }
        {
            if !have {
                match self.add_connection( component.as_ref() ) {
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

        #[cfg( not( feature = "sync" ) )]
        let borrow = self.connections.borrow();

        #[cfg( feature = "sync" )]
        let borrow = self.connections.lock().unwrap();

        let connection = borrow.get( component.as_ref() ).unwrap();
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
        let rows = match stmt.query_map(
            [],
            |row| Ok( row.get( 0 )? )
        ) {
            Ok( result ) => result,
            Err( error ) => return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                }
            )
        };
        let mut string = "".to_string();
        for row in rows {
            string = match row {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
        }
        if !string.is_empty() {
            match self.language_tag_registry.get_language_tag( string ) {
                Ok( result ) => return Ok( Some( result ) ),
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                    }
                )
            };
        }
        Ok( None )
    }

    /// Obtain a list of all the supported languages for a specific identifier.
    /// 
    /// Return of [`ProviderError`] indicates there was an error, usually from within the data repository.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_provider::LStringProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let tags = provider.identifier_language_tags(
    ///         "i18n_provider_sqlite3",
    ///         "path_conversion",
    ///     )?;
    ///     assert_eq!( tags.iter().count(), 2, "Must be 2 languages." );
    ///     Ok( () )
    /// }
    /// ```
    fn identifier_language_tags<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
    ) -> Result<Vec<RefCount<String>>, ProviderError> {
        let mut list = Vec::<RefCount<String>>::new();

        // Check if all_in_one has component.
        if self.all_in_one {
            #[cfg( not( feature = "sync" ) )]
            let borrow = self.connections.borrow();
    
            #[cfg( feature = "sync" )]
            let borrow = self.connections.lock().unwrap();
    
            let connection = borrow.get( "all_in_one" ).unwrap();
            let mut stmt = match connection.prepare(
                "SELECT languageTag FROM pattern WHERE component = ?1 AND identifier = ?2"
            ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            let rows = match stmt.query_map(
                [ component.as_ref(), identifier.as_ref() ],
                |row| Ok( row.get( 0 )? )
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
                let string: String = match row {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                };
                let language = match self.language_tag_registry.get_language_tag( string ) {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                        }
                    )
                };
                list.push( language );
            }
            return Ok( list );
        }

        // Try individual component sqlite3 files.
        let mut have = false;
        {
            #[cfg( not( feature = "sync" ) )]
                if self.connections.borrow().contains_key( component.as_ref() ) {
                have = true;
            }

            #[cfg( feature = "sync" )]
                if self.connections.lock().unwrap().contains_key( component.as_ref() ) {
                have = true;
            }
        }
        {
            if !have {
                match self.add_connection( component.as_ref() ) {
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

        #[cfg( not( feature = "sync" ) )]
        let borrow = self.connections.borrow();

        #[cfg( feature = "sync" )]
        let borrow = self.connections.lock().unwrap();

        let connection = borrow.get( component.as_ref() ).unwrap();
        let mut stmt = match connection.prepare(
            "SELECT languageTag FROM pattern WHERE identifier = ?1"
        ) {
            Ok( result ) => result,
            Err( error ) => return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                }
            )
        };
        let rows = match stmt.query_map(
            [ identifier.as_ref() ],
            |row| Ok( row.get( 0 )? )
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
            let string: String = match row {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            let language = match self.language_tag_registry.get_language_tag( string ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                    }
                )
            };
            list.push( language );
        }
        Ok( list )
    }

    /// Obtain a list of all the supported languages for a specific component.
    /// 
    /// Return of [`ProviderError`] indicates there was an error, usually from within the data repository.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_provider::LStringProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let tags = provider.component_language_tags(
    ///         "i18n_provider_sqlite3",
    ///     )?;
    ///     for tag in tags {
    ///         if tag.language == registry.get_language_tag( "en-ZA" ).unwrap() {
    ///             assert_eq!( tag.ratio, 1.0, "Ratio ust be 1.0 for en-ZA." );
    ///         }
    ///     }
    ///     Ok( () )
    /// }
    /// ```
    fn component_language_tags<T: AsRef<str>>(
        &self,
        component: T,
    ) -> Result<Vec<LanguageData>, ProviderError> {
        let mut list = Vec::<LanguageData>::new();
        let mut default_count = 0usize;
        let Some( default_language ) = self.default_language_tag( component.as_ref() )? else {
            return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::DefaultLanguage( component.as_ref().to_string() ) ),
                }
            )
        };

        // Check if all_in_one has component.
        if self.all_in_one {
            #[cfg( not( feature = "sync" ) )]
            let borrow = self.connections.borrow();
    
            #[cfg( feature = "sync" )]
            let borrow = self.connections.lock().unwrap();
    
            let connection = borrow.get( "all_in_one" ).unwrap();
            let mut stmt = match connection.prepare(
                "SELECT languageTag FROM contributor WHERE component = ?1"
            ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            let rows = match stmt.query_map(
                [ component.as_ref() ],
                |row| {
                    let string: String = row.get( 0 )?;
                    Ok( string )
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
                let string = match row {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                };
                let language = match self.language_tag_registry.get_language_tag( string ) {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                        }
                    )
                };
                list.push( LanguageData {
                    language,
                    count: 0,
                    ratio: 0.0,
                } );
            }

            // Get counts for the languages, get count for default.
            let mut iterator = list.iter_mut();
            while let Some( data ) = iterator.next() {
                let mut stmt = match connection.prepare(
                    "SELECT count( * ) FROM pattern WHERE component = ?1 AND languageTag = ?2"
                ) {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                };
                let rows = match stmt.query_map(
                    [ component.as_ref(), data.language.as_ref() ],
                    |row| {
                        let count: usize = row.get( 0 )?;
                        Ok( count )
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
                let mut count = 0usize;
                for row in rows {
                    count = match row {
                        Ok( result ) => result,
                        Err( error ) => return Err(
                            ProviderError {
                                error_type: "ProviderSqlite3Error",
                                source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                            }
                        )
                    }
                }
                if data.language == default_language {
                    default_count = count.clone();
                }
                data.count = count;
            }

            // Loop through to get percentage.
            if default_count == 0 {
                return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new(
                            ProviderSqlite3Error::DefaultLanguageCount(
                                component.as_ref().to_string(), default_language.to_string()
                            )
                        ),
                    }
                )
            }
            iterator = list.iter_mut();
            while let Some( data ) = iterator.next() {
                data.ratio = data.count as f32 / default_count as f32;
            }
            return Ok( list );
        }

        // Try individual component sqlite3 files.
        let mut have = false;
        {
            #[cfg( not( feature = "sync" ) )]
                if self.connections.borrow().contains_key( component.as_ref() ) {
                have = true;
            }

            #[cfg( feature = "sync" )]
                if self.connections.lock().unwrap().contains_key( component.as_ref() ) {
                have = true;
            }
        }
        {
            if !have {
                match self.add_connection( component.as_ref() ) {
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

        #[cfg( not( feature = "sync" ) )]
        let borrow = self.connections.borrow();

        #[cfg( feature = "sync" )]
        let borrow = self.connections.lock().unwrap();

        let connection = borrow.get( component.as_ref() ).unwrap();
        let mut stmt = match connection.prepare(
            "SELECT languageTag FROM contributor"
        ) {
            Ok( result ) => result,
            Err( error ) => return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                }
            )
        };
        let rows = match stmt.query_map(
            [],
            |row| {
                let string: String = row.get( 0 )?;
                Ok( string )
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
            let string = match row {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            let language = match self.language_tag_registry.get_language_tag( string ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                    }
                )
            };
            list.push( LanguageData {
                language,
                count: 0,
                ratio: 0.0,
            } );
        }

        // Get counts for the languages, get count for default.
        let mut iterator = list.iter_mut();
        while let Some( data ) = iterator.next() {
            let mut stmt = match connection.prepare(
                "SELECT count( * ) FROM pattern WHERE languageTag = ?1"
            ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            let rows = match stmt.query_map(
                [ data.language.as_ref() ],
                |row| {
                    let count: usize = row.get( 0 )?;
                    Ok( count )
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
            let mut count = 0usize;
            for row in rows {
                count = match row {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                }
            }
            if data.language == default_language {
                default_count = count.clone();
            }
            data.count = count;
        }

        // Loop through to get percentage.
        if default_count == 0 {
            return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new(
                        ProviderSqlite3Error::DefaultLanguageCount(
                            component.as_ref().to_string(), default_language.to_string()
                        )
                    ),
                }
            )
        }
        iterator = list.iter_mut();
        while let Some( data ) = iterator.next() {
            data.ratio = data.count as f32 / default_count as f32;
        }
        Ok( list )
    } 

    /// Obtain a list of all the supported languages of the entire repository.
    /// 
    /// Return of [`ProviderError`] indicates there was an error, usually from within the data repository.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_provider::LStringProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let provider = ProviderSqlite3::try_new(
    ///         path,
    ///         &registry
    ///     )?;
    ///     let tags = provider.repository_language_tags()?;
    ///     assert_eq!( tags.iter().count(), 2, "Must be 2 languages." );
    ///     Ok( () )
    /// }
    /// ```
    fn repository_language_tags( &self ) -> Result<Vec<RefCount<String>>, ProviderError> {
        let mut list = Vec::<RefCount<String>>::new();

        // Check if all_in_one has component.
        if self.all_in_one {
            #[cfg( not( feature = "sync" ) )]
            let borrow = self.connections.borrow();
    
            #[cfg( feature = "sync" )]
            let borrow = self.connections.lock().unwrap();
    
            let connection = borrow.get( "all_in_one" ).unwrap();
            let mut stmt = match connection.prepare(
                "SELECT DISTINCT languageTag FROM contributor"
            ) {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                    }
                )
            };
            let rows = match stmt.query_map(
                [],
                |row| {
                    let string: String = row.get( 0 )?;
                    Ok( string )
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
                let string = match row {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                        }
                    )
                };
                let language = match self.language_tag_registry.get_language_tag( string ) {
                    Ok( result ) => result,
                    Err( error ) => return Err(
                        ProviderError {
                            error_type: "ProviderSqlite3Error",
                            source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                        }
                    )
                };
                list.push( language );
            }
            return Ok( list );
        }
        
        // Try individual component sqlite3 files.
        let iterator = match self.directory.read_dir() {
            Ok( result ) => result,
            Err( error ) => return Err(
                ProviderError {
                    error_type: "ProviderSqlite3Error",
                    source: Box::new( ProviderSqlite3Error::Io( error ) ),
                }
            )
        };
        for entry in iterator {
            let entry_data = match entry {
                Ok( result ) => result,
                Err( error ) => return Err(
                    ProviderError {
                        error_type: "ProviderSqlite3Error",
                        source: Box::new( ProviderSqlite3Error::Io( error ) ),
                    }
                )
            };
            if let Some( extension ) = entry_data.path().extension() {
                if extension == "sqlite3" {
                    let path = entry_data.path();
                    let component = path.file_stem().unwrap().to_str().unwrap();
                    let mut have = false;
                    {
                        #[cfg( not( feature = "sync" ) )]
                            if self.connections.borrow().contains_key( component ) {
                            have = true;
                        }
            
                        #[cfg( feature = "sync" )]
                            if self.connections.lock().unwrap().contains_key( component ) {
                            have = true;
                        }
                    }
                    {
                        if !have {
                            match self.add_connection( component ) {
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
            
                    #[cfg( not( feature = "sync" ) )]
                    let borrow = self.connections.borrow();
            
                    #[cfg( feature = "sync" )]
                    let borrow = self.connections.lock().unwrap();
            
                    let connection = borrow.get( component ).unwrap();
                    let mut stmt = match connection.prepare(
                        "SELECT DISTINCT languageTag FROM contributor"
                    ) {
                        Ok( result ) => result,
                        Err( error ) => return Err(
                            ProviderError {
                                error_type: "ProviderSqlite3Error",
                                source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                            }
                        )
                    };
                    let rows = match stmt.query_map(
                        [],
                        |row| {
                            let string: String = row.get( 0 )?;
                            Ok( string )
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
                        let string = match row {
                            Ok( result ) => result,
                            Err( error ) => return Err(
                                ProviderError {
                                    error_type: "ProviderSqlite3Error",
                                    source: Box::new( ProviderSqlite3Error::Sqlite3( error ) ),
                                }
                            )
                        };
                        let language = match self.language_tag_registry.get_language_tag( string ) {
                            Ok( result ) => result,
                            Err( error ) => return Err(
                                ProviderError {
                                    error_type: "ProviderSqlite3Error",
                                    source: Box::new( ProviderSqlite3Error::LanguageTagRegistry( error ) ),
                                }
                            )
                        };
                        //check if language is in list already, ignore if present.
                        let mut have = false;
                        let mut iterator = list.iter();
                        while let Some( entry ) = iterator.next() {
                            if *entry == language {
                                have = true;
                            }
                        }
                        if !have {
                            list.push( language );
                        }
                    }
                }
            }
        }
        Ok( list )
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
