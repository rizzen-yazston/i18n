// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

use crate::LocalisationProviderSqlite3Error;
use i18n_utility::{ LanguageTagRegistry, TaggedString };
use i18n_provider::{
    LocalisationProvider,
    LocalisationProviderError,
    IdentifierDetails,
    ComponentDetails,
    RepositoryDetails,
    LanguageData,
};
use rusqlite::{ Connection, OpenFlags };

#[cfg( feature = "log" )]
use log::{ error, debug };

use std::collections::HashMap;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( not( feature = "sync" ) )]
use std::cell::{ RefCell as MutCell, OnceCell as OnceMut };

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::{ Arc as RefCount, Mutex as MutCell, OnceLock as OnceMut };

use std::path::PathBuf;

/// `LocalisationProviderSqlite3` struct is an implementation of the [`LocalisationProvider`] trait, and uses Sqlite3
/// as the data store for localisation data repository. As the directory path of the data store is embedded in the
/// `LocalisationProviderSqlite3` struct upon creation, one can have multiple `LocalisationProviderSqlite3` instances
/// representing the application itself, application plugin modules, and for various data packages that supports
/// internationalisation.
/// 
/// As this provider is directory based, there may be one or more Sqlite3 files present for application's
/// localisation. There may just be a single file `all_in_one.sqlite3` containing all the localisation strings of
/// all the components (application and libraries, or data files), or there may be multiple separate Sqlite3 for
/// each components (for application the `application.sqlite3` must be present), or even a combination of
/// `all_in_one.sqlite3` and separate component files.
/// 
/// If `all_in_one.sqlite3` and separate component files are present, then the `all_in_one.sqlite3` is accessed
/// first. If a string is not found in the `all_in_one.sqlite3` then the separate component file is accessed.
/// 
/// Any non-sqlite3 files and sub directories present will be ignored.
/// 
/// # Examples
/// 
/// ```
/// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
/// use i18n_provider::LocalisationProvider;
/// use i18n_utility::LanguageTagRegistry;
/// use std::rc::Rc;
/// use std::error::Error;
/// fn main() -> Result<(), Box<dyn Error>> {
///     let path = "./l10n/";
///     let registry = Rc::new( LanguageTagRegistry::new() );
///     let tag = registry.tag( "en" )?;
///     let provider = LocalisationProviderSqlite3::try_new(
///         path,
///         &registry,
///         false
///     )?;
///     let strings = provider.strings(
///         "i18n_provider_sqlite3",
///         "path_conversion",
///         &tag,
///     )?;
///     assert_eq!( strings.len(), 1, "There should be 1 string." );
///     assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
///     assert_eq!( strings[ 0 ].tag().as_str(), "en-ZA", "Must be en-ZA." );
///     Ok( () )
/// }
/// ```
/// 
/// [`LocalisationProvider`]: i18n_provider::LocalisationProvider
pub struct LocalisationProviderSqlite3 {
    directory: PathBuf,
    language_tag_registry: RefCount<LanguageTagRegistry>,
    connections: MutCell<HashMap<String, Option<RefCount<Connection>>>>, // None => already tried and failed.
    all_in_one: bool,

    // Cached data (long running sql queries)
    repository_details: OnceMut<RefCount<RepositoryDetails>>,
    component_details: OnceMut<HashMap<String, RefCount<ComponentDetails>>>,
    use_database_cache: bool,//TODO: This feature still to be implemented, once database schema is stabilised.
                             //      Leaving as a compiler warning for reminder to do.
}

impl LocalisationProviderSqlite3 {

    /// Create a `LocalisationProviderSqlite3` type for the specified directory path.
    /// 
    /// Any non-sqlite3 files and sub directories present will be ignored.
    /// 
    /// Parameter `directory_path` is a path to the directory containing the localisation sqlite3 files, usually named
    /// `l10n`.
    /// 
    /// Parameter `language_tag_registry` is the shared language tag registry.
    /// 
    /// Finally parameter `use_database_cache` is used to indicate the provider is to use the detail data stored in the
    /// Sqlite files. This is a future feature and currently not implemented, thus the boolean value has no effect. 
    /// 
    /// Returns `LocalisationProviderSqlite3Error` when there is an error in verifying the path is a directory and it
    /// does not contain `.sqlite3` files, or Sqlite error occurred.
    // TODO: do schema verification.
    pub fn try_new<T: TryInto<PathBuf>>(
        directory_path: T,
        language_tag_registry: &RefCount<LanguageTagRegistry>,
        use_database_cache: bool,
    ) -> Result<Self, LocalisationProviderSqlite3Error> {
        let Ok( directory ) = directory_path.try_into() else {
            return Err( LocalisationProviderSqlite3Error::PathConversion ) // If not Infallible error.
        };
        if !directory.is_dir() {
            #[cfg( feature = "log" )]
            error!( "{} is not a directory.", directory.display() );

            return Err( LocalisationProviderSqlite3Error::NotDirectory( directory ) );
        }
        let connections =
        MutCell::new( HashMap::<String, Option<RefCount<Connection>>>::new() );

        // Check if all_in_one.sqlite3 exists, if so create the connection.
        // If present, only a connection to all_in_one.sqlite3 is made. Connections to other sqlite3 files will be made
        // if needed at the time of querying.
        let all_in_one = directory.join( "all_in_one.sqlite3" );
        if all_in_one.try_exists()? {
            let connection = Connection::open_with_flags(
                all_in_one,
                OpenFlags::SQLITE_OPEN_READ_ONLY
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI
            )?;
    
            // TODO: do schema verification here.

            #[cfg( not( feature = "sync" ) )]
            connections.borrow_mut().insert( "all_in_one".to_string(), Some( RefCount::new( connection ) ) );
    
            #[cfg( feature = "sync" )]
            connections.lock().unwrap().insert( "all_in_one".to_string(), Some( RefCount::new( connection ) ) );

            return Ok( LocalisationProviderSqlite3 {
                directory,
                language_tag_registry: RefCount::clone( language_tag_registry ),
                connections,
                all_in_one: true,
                repository_details: OnceMut::new(),
                component_details: OnceMut::new(),
                use_database_cache,
            } )
        }

        // No all_in_one.sqlite3 is present, check for any other sqlite3 files are present, but do not create
        // connections. Connection to Sqlite3 file will be made at the time of accessing the component.
        #[cfg( feature = "log" )]
        debug!( "all_in_one.sqlite3 is not present in {}.", directory.display() );

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
            #[cfg( feature = "log" )]
            error!( "No sqlite3 files is found in {}.", directory.display() );

            return Err( LocalisationProviderSqlite3Error::NoSqlite3Files( directory ) );
        }
        Ok( LocalisationProviderSqlite3 {
            directory,
            language_tag_registry: RefCount::clone( language_tag_registry ),
            connections,
            all_in_one: false,
            repository_details: OnceMut::new(),
            component_details: OnceMut::new(),
        use_database_cache,
        } )
    }

    // Get database connection.
    // TODO: do schema verification.
    fn connection<T: AsRef<str>>(
        &self,
        component: T,
        all_in_one: bool,
    ) -> Result<Option<RefCount<Connection>>, LocalisationProviderSqlite3Error> {
        #[cfg( feature = "log" )]
        debug!( "Getting database connection." );

        {
            #[cfg( not( feature = "sync" ) )]
            let borrow = self.connections.borrow();
    
            #[cfg( feature = "sync" )]
            let borrow = self.connections.lock().unwrap();
    
            let option = if all_in_one {
                borrow.get( "all_in_one" )
            } else {
                borrow.get( component.as_ref() )
            };
            if option.is_some() {
                return Ok( option.unwrap().clone() );
            }
            if all_in_one {
                return Ok( None );
            }
        }

        // No entry in components.
        {
            #[cfg( not( feature = "sync" ) )]
            let mut borrow_mut = self.connections.borrow_mut();
    
            #[cfg( feature = "sync" )]
            let mut borrow_mut = self.connections.lock().unwrap();

            let mut file = component.as_ref().to_string();
            file.push_str( ".sqlite3" );
            let path = self.directory.join( file );
            if !path.try_exists()? {
                #[cfg( feature = "log" )]
                error!( "{} does not exist.", path.display() );
    
                borrow_mut.insert( component.as_ref().to_string(), None );
                return Err( LocalisationProviderSqlite3Error::NotExists( path ) );
            }
            let connection = match Connection::open_with_flags(
                path,
                OpenFlags::SQLITE_OPEN_READ_ONLY
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI
            ) {
                Err( error ) => {
                    borrow_mut.insert( component.as_ref().to_string(), None );
                    return Err( LocalisationProviderSqlite3Error::Sqlite3( error ) );
                },
                Ok( result ) => result
            };
            borrow_mut.insert( component.as_ref().to_string(), Some( RefCount::new( connection ) ) );
        }

        #[cfg( not( feature = "sync" ) )]
        let borrow = self.connections.borrow();

        #[cfg( feature = "sync" )]
        let borrow = self.connections.lock().unwrap();

        let option = borrow.get( component.as_ref() );
        Ok( option.unwrap().clone() )
    }

    fn find_strings<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
        all_in_one: bool,
        only_one: bool,
        exact: bool,
    ) -> Result<Vec<TaggedString>, LocalisationProviderSqlite3Error> {
        #[cfg( feature = "log" )]
        debug!(
            "Finding strings for identifier '{}' of component '{}' for language tag '{}' with all_in_one: {}, \
            only_one: {}, and exact: {}.",
            identifier.as_ref(), component.as_ref(), language_tag, all_in_one, only_one, exact
        );

            match self.connection( component.as_ref(), all_in_one ) {
            Err( error ) => {
                return Err( error )
            },
            Ok( result ) => match result {
                None => {
                    #[cfg( feature = "log" )]
                    debug!( "No database connection for `{}`.", component.as_ref() );
                    return Ok( Vec::<TaggedString>::new() );
                },
                Some( connection ) => {
                    let exact_equal = if exact {
                        "="
                    } else {
                        "LIKE"
                    };
                    let mut query = if all_in_one {
                        let mut string =
                        "SELECT string, languageTag FROM pattern WHERE component = ?1 AND identifier = ?2 AND \
                        languageTag "
                        .to_string();
                        string.push_str( exact_equal );
                        string.push_str( " ?3" );
                        string
                    } else {
                        let mut string =
                        "SELECT string, languageTag FROM pattern WHERE identifier = ?1 AND languageTag "
                        .to_string();
                        string.push_str( exact_equal );
                        string.push_str( " ?2" );
                        string
                    };
                    if only_one {
                        query.push_str( " LIMIT 1" );
                    }
                    let mut statement = connection.prepare( query.as_str() )?;
                    let mut strings = Vec::<TaggedString>::new();
                    let mut tag = language_tag.to_string();
                    while tag.len() > 0 {
                        if !exact {
                            tag.push( '%' );
                        }
                        let mut rows = if all_in_one {
                            statement.query( [ component.as_ref(), identifier.as_ref(), tag.as_str() ] )?
                        } else {
                            statement.query( [ identifier.as_ref(), tag.as_str() ] )?
                        };
                        while let Some( row ) = rows.next()? {
                            let string: String = row.get( 0 )?;
                            let tag_raw: String = row.get( 1 )?;
                            let language = self.language_tag_registry.as_ref().tag(
                                tag_raw
                            )?;
                            strings.push( TaggedString::new( string, &language ) );
                        }
                        if strings.len() > 0 {
                            #[cfg( feature = "log" )]
                            debug!( "Found at least 1 string from '{}.sqlite3'.", component.as_ref() );
            
                            return Ok( strings );
                        }
                        if exact {
                            return Ok( strings );
                        }
                        tag = match tag.rsplit_once( '-' ) {
                            None => String::new(),
                            Some( value ) => value.0.to_owned(),
                        };
                    }
                    Ok( strings )
                }
            }
        }
    }

    fn languages<T: AsRef<str>>(
        &self,
        component: T,
        all_in_one: bool,
    ) -> Result<Vec<RefCount<String>>, LocalisationProviderSqlite3Error> {
        #[cfg( feature = "log" )]
        debug!( "Get languages for component '{}'.", component.as_ref() );

        let option = self.connection( component.as_ref(), all_in_one )?;
        match option {
            None => {
                return Ok( Vec::<RefCount<String>>::new() );
            },
            Some( connection ) => {
                let query = if all_in_one {
                    "SELECT DISTINCT languageTag FROM contributor WHERE component = ?1"
                } else {
                    "SELECT DISTINCT languageTag FROM contributor"
                };
                let mut statement = connection.prepare( query )?;
                let mut languages = Vec::<RefCount<String>>::new();
                let mut rows = if all_in_one {
                    statement.query( [ component.as_ref() ] )?
                } else {
                    statement.query( [] )?
                };
                while let Some( row ) = rows.next()? {
                    let tag_raw: String = row.get( 0 )?;
                    let language = self.language_tag_registry.as_ref().tag( tag_raw )?;
                    languages.push( language );
                }
                Ok( languages )
            }
        }
    }

    fn identifier_languages<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        all_in_one: bool,
    ) -> Result<Vec<RefCount<String>>, LocalisationProviderSqlite3Error> {
        #[cfg( feature = "log" )]
        debug!( "Get languages for identifier '{}' of component '{}'.",
            identifier.as_ref(), component.as_ref() );

        let option = self.connection( component.as_ref(), all_in_one )?;
        match option {
            None => {
                return Ok( Vec::<RefCount<String>>::new() );
            },
            Some( connection ) => {
                let query = if all_in_one {
                    "SELECT languageTag FROM pattern WHERE identifier = ?1 AND component = ?2"
                } else {
                    "SELECT languageTag FROM pattern WHERE identifier = ?1"
                };
                let mut statement = connection.prepare( query )?;
                let mut languages = Vec::<RefCount<String>>::new();
                let mut rows = if all_in_one {
                    statement.query( [ identifier.as_ref(), component.as_ref() ] )?
                } else {
                    statement.query( [ identifier.as_ref() ] )?
                };
                while let Some( row ) = rows.next()? {
                    let tag_raw: String = row.get( 0 )?;
                    let language = self.language_tag_registry.as_ref().tag( tag_raw )?;
                    languages.push( language );
                }
                Ok( languages )
            }
        }
    }

    fn contributors<T: AsRef<str>>(
        &self,
        component: T,
        language_tag: &RefCount<String>,
        all_in_one: bool,
    ) -> Result<Vec<String>, LocalisationProviderSqlite3Error> {
        #[cfg( feature = "log" )]
        debug!( "Get contributors for component '{}' for language tag '{}'.", component.as_ref(), language_tag );

        let option = self.connection( component.as_ref(), all_in_one )?;
        match option {
            None => {
                return Ok( Vec::<String>::new() );
            },
            Some( connection ) => {
                let query = if all_in_one {
                    "SELECT DISTINCT contributor FROM contributor WHERE languageTag = ?1 AND component = ?2"
                } else {
                    "SELECT DISTINCT contributor FROM contributor WHERE languageTag = ?1"
                };
                let mut statement = connection.prepare( query )?;
                let mut contributors = Vec::<String>::new();
                let mut rows = if all_in_one {
                    statement.query( [ &language_tag, component.as_ref() ] )?
                } else {
                    statement.query( [ &language_tag ] )?
                };
                while let Some( row ) = rows.next()? {
                    let contributor: String = row.get( 0 )?;
                    contributors.push( contributor );
                }
                Ok( contributors )
            }
        }
    }

    fn count<T: AsRef<str>>(
        &self,
        component: T,
        language_tag: &RefCount<String>,
        all_in_one: bool,
    ) -> Result<usize, LocalisationProviderSqlite3Error> {
        #[cfg( feature = "log" )]
        debug!( "Get string count for component '{}' for language tag '{}'.", component.as_ref(), language_tag );

        let option = self.connection( component.as_ref(), all_in_one )?;
        match option {
            None => {
                return Ok( 0usize );
            },
            Some( connection ) => {
                let query = if all_in_one {
                    "SELECT count( * ) FROM pattern WHERE languageTag = ?1 AND component = ?2"
                } else {
                    "SELECT count( * ) FROM pattern WHERE languageTag = ?1"
                };
                let mut statement = connection.prepare( query )?;
                let mut count = 0usize;
                let mut rows = if all_in_one {
                    statement.query( [ &language_tag, component.as_ref() ] )?
                } else {
                    statement.query( [ &language_tag ] )?
                };
                while let Some( row ) = rows.next()? {
                    count = row.get( 0 )?;
                }
                Ok( count )
            }
        }
    }

    fn default_language<T: AsRef<str>>(
        &self,
        component: T,
        all_in_one: bool,
    ) -> Result<Option<RefCount<String>>, LocalisationProviderSqlite3Error> {
        #[cfg( feature = "log" )]
        debug!( "Get default language for component '{}'", component.as_ref() );

        let option = self.connection( component.as_ref(), all_in_one )?;
        match option {
            None => {
                return Ok( None );
            },
            Some( connection ) => {
                let query = if all_in_one {
                    "SELECT value FROM metadata WHERE key = 'default_language_tag' AND component = ?1"
                } else {
                    "SELECT value FROM metadata WHERE key = 'default_language_tag'"
                };
                let mut statement = connection.prepare( query )?;
                let mut tag: Option<RefCount<String>> = None;
                let mut rows = if all_in_one {
                    statement.query( [ component.as_ref() ] )?
                } else {
                    statement.query( [] )?
                };
                while let Some( row ) = rows.next()? {
                    let tag_raw: String = row.get( 0 )?;
                    tag = Some( self.language_tag_registry.as_ref().tag( tag_raw )? );
                }
                Ok( tag )
            }
        }
    }

    fn build_cache( &self ) -> Result<(), LocalisationProviderSqlite3Error> {
        #[cfg( feature = "log" )]
        debug!( "Building details cache." );

        let mut details = HashMap::<String, RefCount<ComponentDetails>>::new();
        let mut components = Vec::<String>::new();

        // Check if all_in_one has component.
        if self.all_in_one {
            let connection: RefCount<Connection> = self.connection( "all_in_one", true ).unwrap().unwrap();
            let mut statement = connection.prepare(
                "SELECT DISTINCT component FROM contributor"
            )?;
            let mut rows = statement.query( [] )?;
            while let Some( row ) = rows.next()? {
                let component: String = row.get( 0 )?;
                components.push( component );
            }
        }
        
        // Try individual component sqlite3 files.
        let iterator = self.directory.read_dir()?;
        for entry in iterator {
            let entry_data = entry?;
            if let Some( extension ) = entry_data.path().extension() {
                if extension == "sqlite3" {
                    let path = entry_data.path();
                    let component = path.file_stem().unwrap().to_str().unwrap().to_string();
                    if component.as_str() != "all_in_one" && !components.contains( &component ) {
                        match self.connection( component.clone(), false ) {
                            Err( error ) => {
                                return Err( error )
                            },
                            Ok( result ) => match result {
                                None => {
                                    #[cfg( feature = "log" )]
                                    debug!( "No database connection for `{}`.", component );
                                    // Skipping component.
                                },
                                Some( _connection ) => {
                                    components.push( component );
                                }
                            }
                        }
                    }
                }
            }
        }

        // Repository details
        let mut repository_total_strings = 0usize;
        let mut repository_contributors = Vec::<String>::new();
        let mut repository_languages = Vec::<RefCount<String>>::new();

        // Get details info per component
        let mut components_iterator = components.iter();
        while let Some( component ) = components_iterator.next() {
            let mut total_strings = 0usize;
    
            // Get languages
            let mut languages = self.languages( component, true )?;
            let mut language_iterator = languages.iter();
            while let Some( language ) = language_iterator.next() {
                if !repository_languages.contains( &language ) {
                    repository_languages.push( RefCount::clone( &language ) );
                }
            }
            let languages_separate = self.languages( component, false )?;
            let mut language_iterator = languages_separate.iter();
            while let Some( language ) = language_iterator.next() {
                if !languages.contains( &language ) {
                    languages.push( RefCount::clone( &language ) );
                }
                if !repository_languages.contains( &language ) {
                    repository_languages.push( RefCount::clone( &language ) );
                }
            }

            // Build language data
            let mut language_data_all = Vec::<LanguageData>::new();
            let mut languages_iterator = languages.iter();
            while let Some( language ) = languages_iterator.next() {
                let mut contributors = self.contributors(
                    component, language, true
                )?;
                let mut contributors_iterator = contributors.iter();
                while let Some( contributor ) = contributors_iterator.next() {
                    if !repository_contributors.contains( contributor ) {
                        repository_contributors.push( contributor.to_string() );
                    }
                }
                let contributors_separate = self.contributors(
                    component, language, false
                )?;
                let mut contributors_iterator = contributors_separate.iter();
                while let Some( contributor ) = contributors_iterator.next() {
                    if !contributors.contains( contributor ) {
                        contributors.push( contributor.to_string() );
                    }
                    if !repository_contributors.contains( contributor ) {
                        repository_contributors.push( contributor.to_string() );
                    }
                }
                let mut count = self.count( component, language, true )?;
                count += self.count( component, language, false )?;
                total_strings += count;
                repository_total_strings += count;
                let data = LanguageData {
                    language: RefCount::clone( language ),
                    count,
                    ratio: 0f32,
                    contributors,
                };
                language_data_all.push( data );
            }

            // Get default language
            let mut default = self.default_language( component, true )?;
            if default.is_none() {
                default = self.default_language( component, false )?;
            }
            if default.is_none() {
                return Err( LocalisationProviderSqlite3Error::DefaultLanguage( component.to_string() ) );
            }

            // Create ComponentDetails and add to cache
            let component_details = ComponentDetails {
                languages: language_data_all,
                default: default.unwrap(),
                total_strings,
            };
            details.insert(component.to_string(), RefCount::new( component_details ) );
        }
        let _ = self.component_details.set( details );

        // Complete repository details
        let mut default = self.default_language( "application", true )?;
        if default.is_none() {
            default = self.default_language( "application", false )?;
        }

        let repository = RefCount::new(
            RepositoryDetails {
                languages: repository_languages,
                default,
                total_strings: repository_total_strings,
                contributors: repository_contributors,
                components,
            }
        );
        let _ = self.repository_details.set( repository );
        Ok( () )
    }
}

impl LocalisationProvider for LocalisationProviderSqlite3 {

    /// Obtain a localisation string ([`TaggedString`]) from the data repository for the provided parameters, though
    /// if an exact match is not found then search using similar language tags, else [`None`] returned indicating no
    /// possible match was found.
    /// 
    /// If no string is found for the requested tag, the provider must remove the right most subtag sequentially until
    /// either a match is found or there are no more subtags remaining, at which the result is `None` (not found).
    /// 
    /// If more than one string matches the requested tag, then only one string is returned. This trait does not
    /// specify how the string is to be selected to be returned, thus varied results may be experienced. Look at
    /// `strings()` method to obtain all the strings, that matches the requested tag.
    ///  
    /// Return of [`LocalisationProviderError`] indicates there was an error in accessing the data repository. The
    /// `LocalisationProviderError` contains the actual error [`LocalisationProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let tag = registry.tag( "en" )?;
    ///     let provider = LocalisationProviderSqlite3::try_new(
    ///         path,
    ///         &registry,
    ///         false
    ///     )?;
    ///     let string = provider.string(
    ///         "i18n_provider_sqlite3",
    ///         "path_conversion",
    ///         &tag,
    ///     )?.unwrap();
    ///     assert_eq!( string.as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    ///     assert_eq!( string.tag().as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    fn string<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
    ) -> Result<Option<TaggedString>, LocalisationProviderError> {
        #[cfg( feature = "log" )]
        debug!(
            "Finding one string for identifier '{}' of component '{}' for language tag '{}'.",
            identifier.as_ref(), component.as_ref(), language_tag
        );

        // Try all_in_one.sqlite3 first.
        if self.all_in_one {
            #[cfg( feature = "log" )]
            debug!( "Trying the 'all_in_one.sqlite3' for string." );

            let strings = match self.find_strings(
                component.as_ref(),
                identifier.as_ref(),
                language_tag,
                true,
                true,
                false,
            ) {
                Err( error ) => {
                    return Err(
                        LocalisationProviderError {
                            error_type: "LocalisationProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    )
                },
                Ok( result ) => result,
            };
            if !strings.is_empty() {
                return Ok( Some( strings[ 0 ].clone() ) );
            }
        }

        // Not found in all_in_one.sqlite3 or not present. Trying individual <component>.sqlite3 file.
        #[cfg( feature = "log" )]
        debug!( "Trying the component sqlite3 file for string." );

        let strings = match self.find_strings(
            component.as_ref(),
            identifier.as_ref(),
            language_tag,
            false,
            true,
            false,
        ) {
            Err( error ) => {
                return Err(
                    LocalisationProviderError {
                        error_type: "LocalisationProviderSqlite3Error",
                        source: Box::new( error ),
                    }
                )
            },
            Ok( result ) => result,
        };
        if !strings.is_empty() {
            return Ok( Some( strings[ 0 ].clone() ) );
        }
        Ok( None )
    }

    /// Obtain a localisation string ([`TaggedString`]) only if there is an exact match in the data repository for the
    /// provided parameters, else [`None`] returned indicating no match was found.
    /// 
    /// Return of [`LocalisationProviderError`] indicates there was an error in accessing the data repository. The
    /// `LocalisationProviderError` contains the actual error [`LocalisationProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let tag = registry.tag( "en-ZA" )?;
    ///     let provider = LocalisationProviderSqlite3::try_new(
    ///         path,
    ///         &registry,
    ///         false
    ///     )?;
    ///     let string = provider.string_exact_match(
    ///         "i18n_provider_sqlite3",
    ///         "path_conversion",
    ///         &tag,
    ///     )?.unwrap();
    ///     assert_eq!( string.as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    ///     assert_eq!( string.tag().as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    fn string_exact_match<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
    ) -> Result<Option<TaggedString>, LocalisationProviderError> {
        #[cfg( feature = "log" )]
        debug!(
            "Finding strings for identifier '{}' of component '{}' for language tag '{}'.",
            identifier.as_ref(), component.as_ref(), language_tag
        );

        // Try all_in_one.sqlite3 first.
        if self.all_in_one {
            #[cfg( feature = "log" )]
            debug!( "Trying the 'all_in_one.sqlite3' for exact match string." );

            let strings = match self.find_strings(
                component.as_ref(),
                identifier.as_ref(),
                language_tag,
                true,
                true,
                true,
            ) {
                Err( error ) => {
                    return Err(
                        LocalisationProviderError {
                            error_type: "LocalisationProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    )
                },
                Ok( result ) => result,
            };
            if !strings.is_empty() {
                return Ok( Some( strings[ 0 ].clone() ) );
            }
        }

        // Not found in all_in_one.sqlite3 or not present. Trying individual <component>.sqlite3 file.
        #[cfg( feature = "log" )]
        debug!( "Trying the component sqlite3 file for exact match string." );

        let strings = match self.find_strings(
            component.as_ref(),
            identifier.as_ref(),
            language_tag,
            false,
            true,
            true,
        ) {
            Err( error ) => {
                return Err(
                    LocalisationProviderError {
                        error_type: "LocalisationProviderSqlite3Error",
                        source: Box::new( error ),
                    }
                )
            },
            Ok( result ) => result,
        };
        if !strings.is_empty() {
            return Ok( Some( strings[ 0 ].clone() ) );
        }
        Ok( None )
    }

    /// Similar to `string()`, except all the strings are returned for the matching requested tag.
    /// 
    /// Empty [`Vec`] returned indicates no match was found.
    ///  
    /// Return of [`LocalisationProviderError`] indicates there was an error in accessing the data repository. The
    /// `LocalisationProviderError` contains the actual error [`LocalisationProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let tag = registry.tag( "en" )?;
    ///     let provider = LocalisationProviderSqlite3::try_new(
    ///         path,
    ///         &registry,
    ///         false
    ///     )?;
    ///     let strings = provider.strings(
    ///         "i18n_provider_sqlite3",
    ///         "path_conversion",
    ///         &tag,
    ///     )?;
    ///     assert_eq!( strings.len(), 1, "There should be 1 string." );
    ///     assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    ///     assert_eq!( strings[ 0 ].tag().as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    fn strings<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
    ) -> Result<Vec<TaggedString>, LocalisationProviderError> {
        #[cfg( feature = "log" )]
        debug!(
            "Finding strings for identifier '{}' of component '{}' for language tag '{}'.",
            identifier.as_ref(), component.as_ref(), language_tag
        );

        // Try all_in_one.sqlite3 first.
        if self.all_in_one {
            #[cfg( feature = "log" )]
            debug!( "Trying the 'all_in_one.sqlite3' for strings." );

            let strings = match self.find_strings(
                component.as_ref(),
                identifier.as_ref(),
                language_tag,
                true,
                false,
                false,
            ) {
                Err( error ) => {
                    return Err(
                        LocalisationProviderError {
                            error_type: "LocalisationProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    )
                },
                Ok( result ) => result,
            };
            if !strings.is_empty() {
                return Ok( strings );
            }
        }

        // Not found in all_in_one.sqlite3 or not present. Trying individual <component>.sqlite3 file.
        #[cfg( feature = "log" )]
        debug!( "Trying the component sqlite3 file for strings." );

        let strings = match self.find_strings(
            component.as_ref(),
            identifier.as_ref(),
            language_tag,
            false,
            false,
            false,
        ) {
            Err( error ) => {
                return Err(
                    LocalisationProviderError {
                        error_type: "LocalisationProviderSqlite3Error",
                        source: Box::new( error ),
                    }
                )
            },
            Ok( result ) => result,
        };
        Ok( strings )
    }

    /// Obtain the information details [`IdentifierDetails`] of an identifier within a component. 
    /// 
    /// Return of [`LocalisationProviderError`] indicates there was an error in accessing the data repository. The
    /// `LocalisationProviderError` contains the actual error [`LocalisationProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let provider = LocalisationProviderSqlite3::try_new(
    ///         path,
    ///         &registry,
    ///         false
    ///     )?;
    ///     let details = provider.identifier_details(
    ///         "application",
    ///         "example",
    ///     )?;
    ///     assert_eq!( details.default, registry.tag( "en-US" )?, "Should be en-US." );
    ///     assert_eq!( details.languages.iter().count(), 2, "Should be 2 languages" );
    ///     Ok( () )
    /// }
    /// ```
    fn identifier_details<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
    ) -> Result<IdentifierDetails, LocalisationProviderError> {
        #[cfg( feature = "log" )]
        debug!( "Getting identifier details for '{}' of component '{}'.", identifier.as_ref(), component.as_ref() );

        let components = match self.component_details.get() {
            None => {
                match self.build_cache() {
                    Ok( _ ) => {},
                    Err( error ) => return Err(
                        LocalisationProviderError {
                            error_type: "LocalisationProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    ),
                };
                self.component_details.get().unwrap()
            },
            Some( value ) => value,
        };
        match components.get( component.as_ref() ) {
            None => return Err(
                LocalisationProviderError {
                    error_type: "LocalisationProviderSqlite3Error",
                    source: Box::new(
                        LocalisationProviderSqlite3Error::ComponentNotFound( component.as_ref().to_string() )
                    ),
                }
            ),
            Some( component_details ) => {
                let mut languages = match self.identifier_languages(
                    component.as_ref(), identifier.as_ref(), true
                ) {
                    Err( error ) => return Err(
                        LocalisationProviderError {
                            error_type: "LocalisationProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    ),
                    Ok( value ) => value,
                };
                if languages.is_empty() {
                    languages = match self.identifier_languages(
                        component.as_ref(), identifier.as_ref(), false
                    ) {
                        Err( error ) => return Err(
                            LocalisationProviderError {
                                error_type: "LocalisationProviderSqlite3Error",
                                source: Box::new( error ),
                            }
                        ),
                        Ok( value ) => value,
                    };
                }
                Ok( IdentifierDetails {
                    languages,
                    default: RefCount::clone( &component_details.default )
                } )
            }
        }
    }
    
    /// Obtain the information details [`ComponentDetails`] of a component. 
    /// 
    /// Return of [`LocalisationProviderError`] indicates there was an error in accessing the data repository. The
    /// `LocalisationProviderError` contains the actual error [`LocalisationProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let provider = LocalisationProviderSqlite3::try_new(
    ///         path,
    ///         &registry,
    ///         false
    ///     )?;
    ///     let details = provider.component_details(
    ///         "i18n_provider_sqlite3",
    ///     )?;
    ///     assert_eq!( details.default, registry.tag( "en-ZA" )?, "Should be en-ZA." );
    ///     assert_eq!( details.languages.iter().count(), 2, "Should be 2 languages" );
    ///     assert_eq!( details.total_strings, 12, "Should be 12 strings for component" );
    ///     Ok( () )
    /// }
    /// ```
    fn component_details<T: AsRef<str>>(
        &self,
        component: T,
    ) -> Result<RefCount<ComponentDetails>, LocalisationProviderError> {
        #[cfg( feature = "log" )]
        debug!( "Getting component details for '{}'.", component.as_ref() );

        let components = match self.component_details.get() {
            None => {
                match self.build_cache() {
                    Ok( _ ) => {},
                    Err( error ) => return Err(
                        LocalisationProviderError {
                            error_type: "LocalisationProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    ),
                };
                self.component_details.get().unwrap()
            },
            Some( value ) => value,
        };
        match components.get( component.as_ref() ) {
            None => return Err(
                LocalisationProviderError {
                    error_type: "LocalisationProviderSqlite3Error",
                    source: Box::new(
                        LocalisationProviderSqlite3Error::ComponentNotFound( component.as_ref().to_string() )
                    ),
                }
            ),
            Some( value ) => return Ok( RefCount::clone( value ) )
        }
    }
    

    /// Obtain the information details [`RepositoryDetails`] of the provider's repository.
    /// 
    /// Return of [`LocalisationProviderError`] indicates there was an error in accessing the data repository. The
    /// `LocalisationProviderError` contains the actual error [`LocalisationProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProvider;
    /// use i18n_utility::LanguageTagRegistry;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let path = "./l10n/";
    ///     let registry = Rc::new( LanguageTagRegistry::new() );
    ///     let provider = LocalisationProviderSqlite3::try_new(
    ///         path,
    ///         &registry,
    ///         false
    ///     )?;
    ///     let details = provider.repository_details()?;
    ///     assert_eq!( details.default.as_ref().unwrap(), &registry.tag( "en-US" )?, "Should be en-US." );
    ///     assert_eq!( details.languages.iter().count(), 3, "Should be 3 languages" );
    ///     assert_eq!( details.total_strings, 16, "Should be 16 strings for repository" );
    ///     assert_eq!( details.components.iter().count(), 2, "Should be 2 components" );
    ///     assert_eq!( details.contributors.iter().count(), 2, "Should be contributors" );
    ///     Ok( () )
    /// }
    /// ```
    fn repository_details( &self ) -> Result<RefCount<RepositoryDetails>, LocalisationProviderError> {
        #[cfg( feature = "log" )]
        debug!( "Getting repository details." );

        match self.repository_details.get() {
            None => {
                match self.build_cache() {
                    Ok( _ ) => {},
                    Err( error ) => return Err(
                        LocalisationProviderError {
                            error_type: "LocalisationProviderSqlite3Error",
                            source: Box::new( error ),
                        }
                    ),
                };
                return Ok( RefCount::clone( self.repository_details.get().unwrap() ) )
            },
            Some( value ) => return Ok( RefCount::clone( value ) ),
        }
    }
}
