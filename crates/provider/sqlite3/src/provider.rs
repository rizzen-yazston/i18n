// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

#![allow(unexpected_cfgs)]

use crate::{ProviderSqlite3Error, SchemaError};
use i18n_provider::{
    ComponentDetails, IdentifierDetails, LanguageData, LocalisationProviderTrait, ProviderError,
    RepositoryDetails,
};
use i18n_utility::{LanguageTag, LanguageTagRegistry};
use rusqlite::{Connection, OpenFlags, Row};

#[cfg(feature = "logging")]
use log::{debug, error, trace};

use std::cmp::Ordering;
use std::collections::HashMap;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(not(feature = "sync"))]
use std::cell::{OnceCell as OnceMut, RefCell as MutCell};

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::{Arc as RefCount, Mutex as MutCell, OnceLock as OnceMut};

use std::path::PathBuf;

/// `LocalisationProviderSqlite3` struct is an implementation of the [`LocalisationProviderTrait`] trait, and uses
/// Sqlite3 as the data store for localisation data repository. As the directory path of the data store is embedded in
/// the `LocalisationProviderSqlite3` struct upon creation, one can have multiple `LocalisationProviderSqlite3`
/// instances representing the application itself, application plugin modules, and for various data packages that
/// supports internationalisation.
///
/// As this provider is directory based, there may be one or more Sqlite3 files present for application's
/// localisation. There may just be a single file `__all_in_one__.sqlite3` containing all the localisation strings of
/// all the components (application and libraries, or data files), or there may be multiple separate Sqlite3 for
/// each components (for application the `application.sqlite3` must be present), or even a combination of
/// `__all_in_one__.sqlite3` and separate component files.
///
/// If `__all_in_one__.sqlite3` and separate component files are present, then the `__all_in_one__.sqlite3` is accessed
/// first. If a string is not found in the `__all_in_one__.sqlite3` then the separate component file is accessed.
///
/// Any non-sqlite3 files and sub directories present will be ignored.
///
/// # Examples
///
/// ```
/// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
/// use i18n_provider::LocalisationProviderTrait;
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
///     assert_eq!( strings[ 0 ].0.as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
///     assert_eq!( strings[ 0 ].1.as_str(), "en-ZA", "Must be en-ZA." );
///     Ok( () )
/// }
/// ```
///
/// [`LocalisationProviderTrait`]: i18n_provider::LocalisationProviderTrait
pub struct LocalisationProviderSqlite3 {
    language_tag_registry: RefCount<LanguageTagRegistry>,
    queries: MutCell<HashMap<String, String>>,
    components: HashMap<String, (bool, bool)>, // 1st: in all_in_one, 2nd: as own sqlite3 file.

    #[cfg(not(feature = "sync"))]
    connections: HashMap<String, (RefCount<Connection>, String)>, // ( ..., String) is schema_version

    #[cfg(feature = "sync")]
    // Due to the Connection containing a RefCell field, instead the path to
    // the verified database file is stored.
    connections: HashMap<String, (PathBuf, String)>, // ( ..., String) is schema_version

    // Cached data (long running sql queries)
    repository_details: OnceMut<RefCount<RepositoryDetails>>,
    component_details: OnceMut<HashMap<String, RefCount<ComponentDetails>>>,
    #[allow(dead_code)]
    use_database_cache: bool, //TODO: This feature still to be implemented, once database schema is stabilised.
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
    /// Returns `ProviderSqlite3Error` when there is an error in verifying the path is a directory and it
    /// does not contain `.sqlite3` files, or Sqlite error occurred.
    pub fn try_new<T: TryInto<PathBuf>>(
        directory_path: T,
        language_tag_registry: &RefCount<LanguageTagRegistry>,
        use_database_cache: bool,
    ) -> Result<Self, ProviderSqlite3Error> {
        let Ok(directory) = directory_path.try_into() else {
            return Err(ProviderSqlite3Error::PathConversion); // If not Infallible error.
        };
        if !directory.is_dir() {
            #[cfg(feature = "logging")]
            error!("{} is not a directory.", directory.display());

            return Err(ProviderSqlite3Error::NotDirectory(directory));
        }
        let mut components = HashMap::<String, (bool, bool)>::new();

        #[cfg(not(feature = "sync"))]
        let mut connections = HashMap::<String, (RefCount<Connection>, String)>::new();

        #[cfg(feature = "sync")]
        let mut connections = HashMap::<String, (PathBuf, String)>::new();

        let iterator = directory.read_dir()?; // If IO error is returned, usually it is a permission issue.
        for entry in iterator {
            let entry_data = entry?; // If IO error is returned, usually it is a permission issue.
            if let Some(extension) = entry_data.path().extension() {
                if extension == "sqlite3" {
                    let path = entry_data.path();
                    let component = path.file_stem().unwrap().to_str().unwrap().to_string();

                    #[cfg(logging)]
                    trace!("Sqlite3 file: {}", component);

                    if component.as_str().cmp("__all_in_one__") == Ordering::Equal {
                        match Connection::open_with_flags(
                            path.clone(),
                            OpenFlags::SQLITE_OPEN_READ_ONLY
                                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                                | OpenFlags::SQLITE_OPEN_URI,
                        ) {
                            Err(_error) => {
                                #[cfg(feature = "logging")]
                                error!("Unable to connect to {}: {}.", path.display(), _error);
                            }
                            Ok(connection) => {
                                let schema_version = verify_schema(&connection)?;
                                {
                                    let mut statement = connection
                                        .prepare_cached("SELECT identifier FROM component")?;
                                    let mut rows = statement.query([])?;
                                    while let Some(row) = rows.next()? {
                                        let component: String = row.get(0)?;

                                        #[cfg(logging)]
                                        trace!(
                                            "__all_in_one__.sqlite3 has component: {}",
                                            component
                                        );

                                        if let std::collections::hash_map::Entry::Vacant(e) =
                                            components.entry(component.clone())
                                        {
                                            e.insert((true, false));
                                        } else {
                                            let value = components.get_mut(&component).unwrap();
                                            value.0 = true;
                                        }
                                    }
                                }

                                #[cfg(not(feature = "sync"))]
                                connections.insert(
                                    "__all_in_one__".to_string(),
                                    (RefCount::new(connection), schema_version),
                                );

                                #[cfg(feature = "sync")]
                                connections
                                    .insert("__all_in_one__".to_string(), (path, schema_version));
                            }
                        }
                    } else {
                        match Connection::open_with_flags(
                            path.clone(),
                            OpenFlags::SQLITE_OPEN_READ_ONLY
                                | OpenFlags::SQLITE_OPEN_NO_MUTEX
                                | OpenFlags::SQLITE_OPEN_URI,
                        ) {
                            Err(_error) => {
                                #[cfg(feature = "logging")]
                                error!("Unable to connect to {}: {}.", path.display(), _error);
                            }
                            Ok(connection) => {
                                let schema_version = verify_schema(&connection)?;

                                #[cfg(logging)]
                                trace!("Added component: {}", component);

                                if components.contains_key(&component) {
                                    let value = components.get_mut(&component).unwrap();
                                    value.1 = true;
                                } else {
                                    components.insert(component.clone(), (false, true));
                                }

                                #[cfg(not(feature = "sync"))]
                                connections
                                    .insert(component, (RefCount::new(connection), schema_version));

                                #[cfg(feature = "sync")]
                                connections.insert(component, (path, schema_version));
                            }
                        }
                    }
                }
            }
        }
        if components.is_empty() {
            #[cfg(feature = "logging")]
            error!("No sqlite3 files are found in {}.", directory.display());

            return Err(ProviderSqlite3Error::NoSqlite3Files(directory));
        }
        Ok(LocalisationProviderSqlite3 {
            language_tag_registry: RefCount::clone(language_tag_registry),
            queries: MutCell::new(HashMap::<String, String>::new()),
            components,
            connections,
            repository_details: OnceMut::new(),
            component_details: OnceMut::new(),
            use_database_cache,
        })
    }

    // Internal functions.

    #[cfg(not(feature = "sync"))]
    fn connection(
        &self,
        component: &str,
        all_in_one: bool,
    ) -> Result<(RefCount<Connection>, &str), ProviderError> {
        #[cfg(feature = "logging")]
        debug!("Getting database connection for component '{}'.", component,);

        #[allow(unused_variables)]
        let Some(value) = self.components.get(component) else {
            return Err(ProviderError::ComponentNotFound(component.to_string()));
        };
        if all_in_one && value.0 {
            let result = self.connections.get("__all_in_one__").unwrap();
            return Ok((RefCount::clone(&result.0), result.1.as_str()));
        }
        if value.1 {
            let result = self.connections.get(component).unwrap();
            return Ok((RefCount::clone(&result.0), result.1.as_str()));
        }
        Err(ProviderError::ComponentNotFound(component.to_string()))
    }

    #[cfg(feature = "sync")]
    fn connection_sync(
        &self,
        component: &str,
        all_in_one: bool,
    ) -> Result<(Connection, &str), ProviderError> {
        #[cfg(feature = "logging")]
        debug!("Getting database connection for component '{}'.", component,);

        #[allow(unused_variables)]
        let Some(value) = self.components.get(component) else {
            return Err(ProviderError::ComponentNotFound(component.to_string()));
        };
        if all_in_one && value.0 {
            let result = self.connections.get("__all_in_one__").unwrap();
            match Connection::open_with_flags(
                &result.0,
                OpenFlags::SQLITE_OPEN_READ_ONLY
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI,
            ) {
                Ok(value) => return Ok((value, result.1.as_str())),

                // Possible Sqlite is concurrency lock on file.
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            }
        }
        if value.1 {
            let result = self.connections.get(component).unwrap();
            match Connection::open_with_flags(
                &result.0,
                OpenFlags::SQLITE_OPEN_READ_ONLY
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX
                    | OpenFlags::SQLITE_OPEN_URI,
            ) {
                Ok(value) => return Ok((value, result.1.as_str())),

                // Possible Sqlite is concurrency lock on file.
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            }
        }
        Err(ProviderError::ComponentNotFound(component.to_string()))
    }

    // Fallback to <component>.sqlite3 is handled by caller.
    fn find_strings(
        &self,
        component: &str,
        identifier: &str,
        language_tag: &RefCount<LanguageTag>,
        all_in_one: bool,
        only_one: bool,
        exact: bool,
    ) -> Result<Vec<(String, RefCount<LanguageTag>)>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!(
            "Finding strings for identifier '{}' of component '{}' for language tag '{}' with all_in_one: {}, \
            only_one: {}, and exact: {}.",
            identifier, component, language_tag.as_str(), all_in_one, only_one, exact
        );

        // Get connection and schema_version.
        #[cfg(not(feature = "sync"))]
        let (connection, schema_version) = self.connection(component, all_in_one)?;

        #[cfg(feature = "sync")]
        let (connection, schema_version) = self.connection_sync(component, all_in_one)?;

        // Construct query identifier and get query statement.
        let mut query_identifier = "Pattern".to_string();
        if exact {
            query_identifier.push_str("Exact");
        }
        if all_in_one {
            query_identifier.push_str("Aio");
        }
        if only_one {
            query_identifier.push_str("One");
        }
        let mut _query: Option<String> = None;
        {
            #[cfg(not(feature = "sync"))]
            let borrow = self.queries.borrow();

            #[cfg(feature = "sync")]
            let borrow = self.queries.lock().unwrap();

            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(not(feature = "sync"))]
        if _query.is_none() {
            {
                self.queries.borrow_mut().insert(
                    query_identifier.clone(),
                    query_pattern(schema_version, only_one, exact),
                );
            }
            let borrow = self.queries.borrow();
            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(feature = "sync")]
        if _query.is_none() {
            {
                self.queries.lock().unwrap().insert(
                    query_identifier.clone(),
                    query_pattern(schema_version, only_one, exact),
                );
            }
            let borrow = self.queries.lock().unwrap();
            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(feature = "logging")]
        debug!("SQL query string: [{}].", _query.as_ref().unwrap());

        let mut statement = match connection.prepare_cached(_query.unwrap().as_str()) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        let mut strings = Vec::<(String, RefCount<LanguageTag>)>::new();
        let mut tag = language_tag.as_str().to_string();
        while !tag.is_empty() {
            if !exact {
                tag.push('%');
            }
            let mut rows = match statement.query([identifier, tag.as_str(), component]) {
                Ok(value) => value,
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            };
            while let Some(row) = match rows.next() {
                Ok(value) => value,
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            } {
                let string: String = match row.get(0) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(ProviderError::Custom(RefCount::new(Box::new(
                            ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                        ))))
                    }
                };
                let tag_raw: String = match row.get(1) {
                    Ok(value) => value,
                    Err(error) => {
                        return Err(ProviderError::Custom(RefCount::new(Box::new(
                            ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                        ))))
                    }
                };
                let language = self.language_tag_registry.as_ref().tag(tag_raw.as_str())?;
                strings.push((string, language));
            }
            if !strings.is_empty() {
                #[cfg(feature = "logging")]
                debug!("Found at least 1 string from '{}.sqlite3'.", component,);

                return Ok(strings);
            }
            tag = match tag.rsplit_once('-') {
                None => String::new(),
                Some(value) => value.0.to_owned(),
            };
        }
        Ok(strings)
    }

    // Fallback to <component>.sqlite3 is handled by caller.
    fn languages(
        &self,
        component: &str,
        all_in_one: bool,
    ) -> Result<Vec<RefCount<LanguageTag>>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!("Get languages for component '{}'.", component);

        // Get connection and schema_version.
        #[cfg(not(feature = "sync"))]
        let (connection, schema_version) = self.connection(component, all_in_one)?;

        #[cfg(feature = "sync")]
        let (connection, schema_version) = self.connection_sync(component, all_in_one)?;

        // Construct query identifier and get query statement.
        let mut query_identifier = "Languages".to_string();
        if all_in_one {
            query_identifier.push_str("Aio");
        }
        let mut _query: Option<String> = None;
        {
            #[cfg(not(feature = "sync"))]
            let borrow = self.queries.borrow();

            #[cfg(feature = "sync")]
            let borrow = self.queries.lock().unwrap();

            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(not(feature = "sync"))]
        if _query.is_none() {
            {
                self.queries
                    .borrow_mut()
                    .insert(query_identifier.clone(), query_languages(schema_version));
            }
            let borrow = self.queries.borrow();
            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(feature = "sync")]
        if _query.is_none() {
            {
                self.queries
                    .lock()
                    .unwrap()
                    .insert(query_identifier.clone(), query_languages(schema_version));
            }
            let borrow = self.queries.lock().unwrap();
            _query = borrow.get(&query_identifier).cloned();
        }

        let mut statement = match connection.prepare_cached(_query.unwrap().as_str()) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        let mut languages = Vec::<RefCount<LanguageTag>>::new();
        let mut rows = match statement.query([component]) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        while let Some(row) = match rows.next() {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        } {
            let tag_raw: String = match row.get(0) {
                Ok(value) => value,
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            };
            let language = self.language_tag_registry.as_ref().tag(tag_raw.as_str())?;
            languages.push(language);
        }
        Ok(languages)
    }

    // Fallback to <component>.sqlite3 is handled by caller.
    fn identifier_languages(
        &self,
        component: &str,
        identifier: &str,
        all_in_one: bool,
    ) -> Result<Vec<RefCount<LanguageTag>>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!(
            "Get languages for identifier '{}' of component '{}'.",
            identifier, component,
        );

        // Get connection and schema_version.
        #[cfg(not(feature = "sync"))]
        let (connection, schema_version) = self.connection(component, all_in_one)?;

        #[cfg(feature = "sync")]
        let (connection, schema_version) = self.connection_sync(component, all_in_one)?;

        // Construct query identifier and get query statement.
        let mut query_identifier = "Identifier".to_string();
        if all_in_one {
            query_identifier.push_str("Aio");
        }
        let mut _query: Option<String> = None;
        {
            #[cfg(not(feature = "sync"))]
            let borrow = self.queries.borrow();

            #[cfg(feature = "sync")]
            let borrow = self.queries.lock().unwrap();

            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(not(feature = "sync"))]
        if _query.is_none() {
            {
                self.queries.borrow_mut().insert(
                    query_identifier.clone(),
                    query_identifier_languages(schema_version),
                );
            }
            let borrow = self.queries.borrow();
            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(feature = "sync")]
        if _query.is_none() {
            {
                self.queries.lock().unwrap().insert(
                    query_identifier.clone(),
                    query_identifier_languages(schema_version),
                );
            }
            let borrow = self.queries.lock().unwrap();
            _query = borrow.get(&query_identifier).cloned();
        }

        let mut statement = match connection.prepare_cached(_query.unwrap().as_str()) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        let mut languages = Vec::<RefCount<LanguageTag>>::new();
        let mut rows = match statement.query([identifier, component]) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        while let Some(row) = match rows.next() {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        } {
            let tag_raw: String = match row.get(0) {
                Ok(value) => value,
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            };
            let language = self.language_tag_registry.as_ref().tag(tag_raw.as_str())?;
            languages.push(language);
        }
        Ok(languages)
    }

    // Fallback to <component>.sqlite3 is handled by caller.
    fn contributors(
        &self,
        component: &str,
        language_tag: &RefCount<LanguageTag>,
        all_in_one: bool,
    ) -> Result<Vec<String>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!(
            "Get contributors for component '{}' for language tag '{}'.",
            component,
            language_tag.as_str(),
        );

        // Get connection and schema_version.
        #[cfg(not(feature = "sync"))]
        let (connection, schema_version) = self.connection(component, all_in_one)?;

        #[cfg(feature = "sync")]
        let (connection, schema_version) = self.connection_sync(component, all_in_one)?;

        // Construct query identifier and get query statement.
        let mut query_identifier = "Contributors".to_string();
        if all_in_one {
            query_identifier.push_str("Aio");
        }
        let mut _query: Option<String> = None;
        {
            #[cfg(not(feature = "sync"))]
            let borrow = self.queries.borrow();

            #[cfg(feature = "sync")]
            let borrow = self.queries.lock().unwrap();

            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(not(feature = "sync"))]
        if _query.is_none() {
            {
                self.queries
                    .borrow_mut()
                    .insert(query_identifier.clone(), query_contributors(schema_version));
            }
            let borrow = self.queries.borrow();
            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(feature = "sync")]
        if _query.is_none() {
            {
                self.queries
                    .lock()
                    .unwrap()
                    .insert(query_identifier.clone(), query_contributors(schema_version));
            }
            let borrow = self.queries.lock().unwrap();
            _query = borrow.get(&query_identifier).cloned();
        }

        let mut statement = match connection.prepare_cached(_query.unwrap().as_str()) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        let mut contributors = Vec::<String>::new();
        let mut rows = match statement.query([language_tag.as_str(), component]) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        while let Some(row) = match rows.next() {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        } {
            let contributor: String = match row.get(0) {
                Ok(value) => value,
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            };
            contributors.push(contributor);
        }
        Ok(contributors)
    }

    // Fallback to <component>.sqlite3 is handled by caller.
    fn count(
        &self,
        component: &str,
        language_tag: &RefCount<LanguageTag>,
        all_in_one: bool,
    ) -> Result<usize, ProviderError> {
        #[cfg(feature = "logging")]
        debug!(
            "Get string count for component '{}' for language tag '{}'.",
            component,
            language_tag.as_str(),
        );

        // Get connection and schema_version.
        #[cfg(not(feature = "sync"))]
        let (connection, schema_version) = self.connection(component, all_in_one)?;

        #[cfg(feature = "sync")]
        let (connection, schema_version) = self.connection_sync(component, all_in_one)?;

        // Construct query identifier and get query statement.
        let mut query_identifier = "Count".to_string();
        if all_in_one {
            query_identifier.push_str("Aio");
        }
        let mut _query: Option<String> = None;
        {
            #[cfg(not(feature = "sync"))]
            let borrow = self.queries.borrow();

            #[cfg(feature = "sync")]
            let borrow = self.queries.lock().unwrap();

            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(not(feature = "sync"))]
        if _query.is_none() {
            {
                self.queries
                    .borrow_mut()
                    .insert(query_identifier.clone(), query_count(schema_version));
            }
            let borrow = self.queries.borrow();
            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(feature = "sync")]
        if _query.is_none() {
            {
                self.queries
                    .lock()
                    .unwrap()
                    .insert(query_identifier.clone(), query_count(schema_version));
            }
            let borrow = self.queries.lock().unwrap();
            _query = borrow.get(&query_identifier).cloned();
        }

        let mut statement = match connection.prepare_cached(_query.unwrap().as_str()) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        let mut count = 0usize;
        let mut rows = match statement.query([language_tag.as_str(), component]) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        while let Some(row) = match rows.next() {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        } {
            count = match row.get(0) {
                Ok(value) => value,
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            };
        }
        Ok(count)
    }

    // Fallback to <component>.sqlite3 is handled by caller.
    fn default_language(
        &self,
        component: &str,
        all_in_one: bool,
    ) -> Result<Option<RefCount<LanguageTag>>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!("Get default language for component '{}'", component,);

        // Get connection and schema_version.
        #[cfg(not(feature = "sync"))]
        let (connection, schema_version) = self.connection(component, all_in_one)?;

        #[cfg(feature = "sync")]
        let (connection, schema_version) = self.connection_sync(component, all_in_one)?;

        // Construct query identifier and get query statement.
        let mut query_identifier = "Default".to_string();
        if all_in_one {
            query_identifier.push_str("Aio");
        }
        let mut _query: Option<String> = None;
        {
            #[cfg(not(feature = "sync"))]
            let borrow = self.queries.borrow();

            #[cfg(feature = "sync")]
            let borrow = self.queries.lock().unwrap();

            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(not(feature = "sync"))]
        if _query.is_none() {
            {
                self.queries
                    .borrow_mut()
                    .insert(query_identifier.clone(), query_default(schema_version));
            }
            let borrow = self.queries.borrow();
            _query = borrow.get(&query_identifier).cloned();
        }

        #[cfg(feature = "sync")]
        if _query.is_none() {
            {
                self.queries
                    .lock()
                    .unwrap()
                    .insert(query_identifier.clone(), query_default(schema_version));
            }
            let borrow = self.queries.lock().unwrap();
            _query = borrow.get(&query_identifier).cloned();
        }

        let mut statement = match connection.prepare_cached(_query.unwrap().as_str()) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        let mut tag: Option<RefCount<LanguageTag>> = None;
        let mut rows = match statement.query([component]) {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        };
        while let Some(row) = match rows.next() {
            Ok(value) => value,
            Err(error) => {
                return Err(ProviderError::Custom(RefCount::new(Box::new(
                    ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                ))))
            }
        } {
            let tag_raw: String = match row.get(0) {
                Ok(value) => value,
                Err(error) => {
                    return Err(ProviderError::Custom(RefCount::new(Box::new(
                        ProviderSqlite3Error::Sqlite3(RefCount::new(error)),
                    ))))
                }
            };
            tag = Some(self.language_tag_registry.as_ref().tag(tag_raw.as_str())?);
        }
        Ok(tag)
    }

    // If __all_in_one__.sqlite3 fails, fallback to <component>.sqlite3
    fn build_cache(&self) -> Result<(), ProviderError> {
        #[cfg(feature = "logging")]
        debug!("Building details cache.");

        let mut components_details = HashMap::<String, RefCount<ComponentDetails>>::new();
        let mut repository_details = RepositoryDetails {
            languages: HashMap::<RefCount<LanguageTag>, LanguageData>::new(),
            default: None,
            total_strings: 0usize,
            components: Vec::<String>::new(),
            contributors: Vec::<String>::new(),
        };

        // Get details info per component
        let components_iterator = self.components.iter();
        for component in components_iterator {
            repository_details.components.push(component.0.to_string());
            let mut component_details = ComponentDetails {
                languages: HashMap::<RefCount<LanguageTag>, LanguageData>::new(),
                default: self.language_tag_registry.tag("und")?,
                total_strings: 0usize,
            };

            // Get languages
            let mut languages = Vec::<RefCount<LanguageTag>>::new();
            if component.1 .0 {
                // In __all_in_one__.sqlite3
                languages = self.languages(component.0, true)?;
                let languages_iterator = languages.iter();
                for language in languages_iterator {
                    component_details.languages.insert(
                        RefCount::clone(language),
                        LanguageData {
                            count: 0usize,
                            ratio: 0f32,
                            contributors: Vec::<String>::new(),
                        },
                    );
                    if !repository_details.languages.contains_key(language) {
                        repository_details.languages.insert(
                            RefCount::clone(language),
                            LanguageData {
                                count: 0usize,
                                ratio: 0f32,
                                contributors: Vec::<String>::new(),
                            },
                        );
                    }
                }
            }
            if component.1 .1 {
                // Has own <component>.sqlite3
                let languages_separate = self.languages(component.0, false)?;
                let languages_iterator = languages_separate.iter();
                for language in languages_iterator {
                    if !languages.contains(language) {
                        languages.push(RefCount::clone(language));
                        component_details.languages.insert(
                            RefCount::clone(language),
                            LanguageData {
                                count: 0usize,
                                ratio: 0f32,
                                contributors: Vec::<String>::new(),
                            },
                        );
                    }
                    if !repository_details.languages.contains_key(language) {
                        repository_details.languages.insert(
                            RefCount::clone(language),
                            LanguageData {
                                count: 0usize,
                                ratio: 0f32,
                                contributors: Vec::<String>::new(),
                            },
                        );
                    }
                }
            }

            #[cfg(feature = "logging")]
            debug!("Got languages.");

            // Get default language
            let mut language = None;
            if component.1 .0 {
                language = self.default_language(component.0, true)?;
            }
            if language.is_none() && component.1 .1 {
                language = self.default_language(component.0, false)?;
            }
            if language.is_none() {
                return Err(ProviderError::DefaultLanguage(component.0.to_string()));
            }
            if !component_details
                .languages
                .contains_key(language.as_ref().unwrap())
            {
                return Err(ProviderError::InvalidDefaultLanguage(
                    component.0.to_string(),
                ));
            }
            component_details.default = RefCount::clone(language.as_ref().unwrap());
            if component.0.as_str().cmp("application") == Ordering::Equal {
                repository_details.default = language;
            }

            #[cfg(feature = "logging")]
            debug!("Got default language.");

            // Build language data
            let languages_iterator = languages.iter();
            for language in languages_iterator {
                let language_data = component_details.languages.get_mut(language).unwrap();
                if component.1 .0 {
                    language_data.contributors = self.contributors(component.0, language, true)?;
                    let contributors_iterator = language_data.contributors.iter();
                    for contributor in contributors_iterator {
                        if !repository_details.contributors.contains(contributor) {
                            repository_details
                                .contributors
                                .push(contributor.to_string());
                        }
                    }
                }
                if component.1 .1 {
                    let contributors_separate = self.contributors(component.0, language, false)?;
                    let contributors_iterator = contributors_separate.iter();
                    for contributor in contributors_iterator {
                        if !language_data.contributors.contains(contributor) {
                            language_data.contributors.push(contributor.to_string());
                        }
                        if !repository_details.contributors.contains(contributor) {
                            repository_details
                                .contributors
                                .push(contributor.to_string());
                        }
                    }
                }

                if component.1 .0 {
                    language_data.count = self.count(component.0, language, true)?;
                }
                if component.1 .1 {
                    language_data.count += self.count(component.0, language, false)?;
                }
                let repository_language = repository_details.languages.get_mut(language).unwrap();
                repository_language.count += language_data.count;
                component_details.total_strings += language_data.count;
                repository_details.total_strings += language_data.count;
            }
            let mut _count = 0usize;
            {
                let default_language_data = component_details
                    .languages
                    .get(&component_details.default)
                    .unwrap();
                _count = default_language_data.count;
            }
            let languages_iterator = component_details.languages.iter_mut();
            for language_data in languages_iterator {
                language_data.1.ratio = language_data.1.count as f32 / _count as f32;
            }

            #[cfg(feature = "logging")]
            debug!("Got language data.");

            components_details.insert(component.0.to_string(), RefCount::new(component_details));
        }

        #[cfg(feature = "logging")]
        debug!("Components done.");

        if repository_details.default.is_some() {
            let mut _count = 0usize;
            {
                let default_language_data = repository_details
                    .languages
                    .get(repository_details.default.as_ref().unwrap())
                    .unwrap();
                _count = default_language_data.count;
            }
            let languages_iterator = repository_details.languages.iter_mut();
            for language_data in languages_iterator {
                language_data.1.ratio = language_data.1.count as f32 / _count as f32;
            }
        }
        let _ = self.component_details.set(components_details);
        let _ = self
            .repository_details
            .set(RefCount::new(repository_details));
        Ok(())
    }
}

impl LocalisationProviderTrait for LocalisationProviderSqlite3 {
    /// Obtain a localisation string as `(`[`String`]`, `[`Rc`]`<`[`LanguageTag`]`>)` from the
    /// data repository for the provided parameters, though if an exact match is not found
    /// then search using similar language tags, else [`None`] returned indicating no possible
    /// match was found. [`Rc`] can be replaced with [`Arc`] when using feature `sync`.
    ///
    /// Return of [`ProviderError`] indicates there was an error in accessing the data
    /// repository. The `ProviderError` contains the actual error [`ProviderSqlite3Error`],
    /// usually indicates there was a Sqlite3 error.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProviderTrait;
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
    ///     assert_eq!( string.0.as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    ///     assert_eq!( string.1.as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    ///
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    fn string(
        &self,
        component: &str,
        identifier: &str,
        language_tag: &RefCount<LanguageTag>,
    ) -> Result<Option<(String, RefCount<LanguageTag>)>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!(
            "Finding one string for identifier '{}' of component '{}' for language tag '{}'.",
            identifier,
            component,
            language_tag.as_str()
        );

        let Some(component_files) = self.components.get(component) else {
            return Err(ProviderError::ComponentNotFound(component.to_string()));
        };

        // Try __all_in_one__.sqlite3 first.
        if component_files.0 {
            #[cfg(feature = "logging")]
            debug!("Trying the '__all_in_one__.sqlite3' for string.");

            let mut strings =
                self.find_strings(component, identifier, language_tag, true, true, false)?;
            if !strings.is_empty() {
                return Ok(strings.pop());
            }
        }

        // Not found in __all_in_one__.sqlite3 or not present. Trying individual <component>.sqlite3 file.
        #[cfg(feature = "logging")]
        debug!("Trying the component sqlite3 file for string.");

        if component_files.1 {
            let mut strings =
                self.find_strings(component, identifier, language_tag, false, true, false)?;
            if !strings.is_empty() {
                return Ok(strings.pop());
            }
        }
        Ok(None)
    }

    /// Obtain a localisation string as `(`[`String`]`, `[`Rc`]`<`[`LanguageTag`]`>)` only if
    /// there is an exact match in the data repository for the provided parameters, else [`None`]
    /// returned indicating no exact match was found. [`Rc`] can be replaced with [`Arc`] when
    /// using feature `sync`.
    ///
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository. The
    /// `ProviderError` contains the actual error [`ProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProviderTrait;
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
    ///     assert_eq!( string.0.as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    ///     assert_eq!( string.1.as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    ///
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    fn string_exact_match(
        &self,
        component: &str,
        identifier: &str,
        language_tag: &RefCount<LanguageTag>,
    ) -> Result<Option<(String, RefCount<LanguageTag>)>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!(
            "Finding strings for identifier '{}' of component '{}' for language tag '{}'.",
            identifier,
            component,
            language_tag.as_str()
        );

        let Some(component_files) = self.components.get(component) else {
            return Err(ProviderError::ComponentNotFound(component.to_string()));
        };

        // Try __all_in_one__.sqlite3 first.
        if component_files.0 {
            #[cfg(feature = "logging")]
            debug!("Trying the '__all_in_one__.sqlite3' for exact match string.");

            let mut strings =
                self.find_strings(component, identifier, language_tag, true, true, true)?;
            if !strings.is_empty() {
                return Ok(strings.pop());
            }
        }

        // Not found in __all_in_one__.sqlite3 or not present. Trying individual <component>.sqlite3 file.
        #[cfg(feature = "logging")]
        debug!("Trying the component sqlite3 file for exact match string.");

        if component_files.1 {
            let mut strings =
                self.find_strings(component, identifier, language_tag, false, true, true)?;
            if !strings.is_empty() {
                return Ok(strings.pop());
            }
        }
        Ok(None)
    }

    /// Similar to `string()`, except all the strings are returned for the matching requested tag.
    ///
    /// Empty [`Vec`] returned indicates no match was found.
    ///  
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository. The
    /// `ProviderError` contains the actual error [`ProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProviderTrait;
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
    ///     assert_eq!( strings[ 0 ].0.as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    ///     assert_eq!( strings[ 0 ].1.as_str(), "en-ZA", "Must be en-ZA." );
    ///     Ok( () )
    /// }
    /// ```
    ///
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    fn strings(
        &self,
        component: &str,
        identifier: &str,
        language_tag: &RefCount<LanguageTag>,
    ) -> Result<Vec<(String, RefCount<LanguageTag>)>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!(
            "Finding strings for identifier '{}' of component '{}' for language tag '{}'.",
            identifier,
            component,
            language_tag.as_str()
        );

        let Some(component_files) = self.components.get(component) else {
            return Err(ProviderError::ComponentNotFound(component.to_string()));
        };
        let mut strings = Vec::<(String, RefCount<LanguageTag>)>::new();

        // Try __all_in_one__.sqlite3 first.
        if component_files.0 {
            #[cfg(feature = "logging")]
            debug!("Trying the '__all_in_one__.sqlite3' for strings.");

            strings = self.find_strings(component, identifier, language_tag, true, false, false)?;
            if !strings.is_empty() {
                return Ok(strings);
            }
        }

        // Not found in __all_in_one__.sqlite3 or not present. Trying individual <component>.sqlite3 file.
        #[cfg(feature = "logging")]
        debug!("Trying the component sqlite3 file for strings.");

        if component_files.1 {
            strings =
                self.find_strings(component, identifier, language_tag, false, false, false)?;
        }
        Ok(strings)
    }

    /// Obtain the information details [`IdentifierDetails`] of an identifier within a component.
    ///
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository. The
    /// `ProviderError` contains the actual error [`ProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProviderTrait;
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
    fn identifier_details(
        &self,
        component: &str,
        identifier: &str,
    ) -> Result<IdentifierDetails, ProviderError> {
        #[cfg(feature = "logging")]
        debug!(
            "Getting identifier details for '{}' of component '{}'.",
            identifier, component
        );

        let components = match self.component_details.get() {
            None => {
                self.build_cache()?;
                self.component_details.get().unwrap()
            }
            Some(value) => value,
        };
        match components.get(component) {
            None => Err(ProviderError::ComponentNotFound(component.to_string())),
            Some(component_details) => {
                let mut languages = self.identifier_languages(component, identifier, true)?;
                if languages.is_empty() {
                    languages = self.identifier_languages(component, identifier, false)?;
                }
                Ok(IdentifierDetails {
                    languages,
                    default: RefCount::clone(&component_details.default),
                })
            }
        }
    }

    /// Obtain the information details [`ComponentDetails`] of a component.
    ///
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository. The
    /// `ProviderError` contains the actual error [`ProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProviderTrait;
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
    ///     assert_eq!( details.total_strings, 26, "Should be 26 strings for component" );
    ///     Ok( () )
    /// }
    /// ```
    fn component_details(
        &self,
        component: &str,
    ) -> Result<RefCount<ComponentDetails>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!("Getting component details for '{}'.", component);

        let components = match self.component_details.get() {
            None => {
                self.build_cache()?;
                self.component_details.get().unwrap()
            }
            Some(value) => value,
        };
        match components.get(component) {
            None => Err(ProviderError::ComponentNotFound(component.to_string())),
            Some(value) => Ok(RefCount::clone(value)),
        }
    }

    /// Obtain the information details [`RepositoryDetails`] of the provider's repository.
    ///
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository. The
    /// `ProviderError` contains the actual error [`ProviderSqlite3Error`], usually indicates
    /// there was a Sqlite3 error.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_provider::LocalisationProviderTrait;
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
    ///     assert_eq!( details.total_strings, 30, "Should be 30 strings for repository" );
    ///     assert_eq!( details.components.iter().count(), 2, "Should be 2 components" );
    ///     assert_eq!( details.contributors.iter().count(), 2, "Should be contributors" );
    ///     Ok( () )
    /// }
    /// ```
    fn repository_details(&self) -> Result<RefCount<RepositoryDetails>, ProviderError> {
        #[cfg(feature = "logging")]
        debug!("Getting repository details.");

        match self.repository_details.get() {
            None => {
                self.build_cache()?;
                return Ok(RefCount::clone(self.repository_details.get().unwrap()));
            }
            Some(value) => Ok(RefCount::clone(value)),
        }
    }
}

/// Database schema verification. The schema version is return if successfully verified.
pub fn verify_schema(connection: &Connection) -> Result<String, SchemaError> {
    let mut schema_version = String::new();
    let mut table_list = Vec::new();
    let mut metadata = false;
    {
        let mut statement = connection.prepare("SELECT name FROM pragma_table_list;")?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            let name: String = row.get(0)?;
            if name.as_str() == "metadata" {
                metadata = true;
            }
            table_list.push(name);
        }
    }
    if !metadata {
        return Err(SchemaError::MissingTable("metadata".to_string()));
    }
    {
        let mut statement =
            connection.prepare("SELECT value FROM metadata WHERE key = 'schema_version';")?;
        let mut rows = statement.query([])?;
        while let Some(row) = rows.next()? {
            schema_version = row.get(0)?;
        }

        #[cfg(feature = "logging")]
        trace!("schema_version {:?}", schema_version);
    }
    match schema_version.as_str() {
        "1.0" => {
            // Required tables exists
            table_exists(&table_list, "contributor")?;
            table_exists(&table_list, "language")?;
            table_exists(&table_list, "pattern")?;
            table_exists(&table_list, "component")?;
            table_exists(&table_list, "languageData")?;

            // Check required columns exists
            // Table: contributor
            let mut statement =
                connection.prepare("SELECT * FROM pragma_table_info('contributor');")?;
            let mut rows = statement.query([])?;
            while let Some(row) = rows.next()? {
                let cid: usize = row.get(0)?;
                match cid {
                    0 => {
                        columns_check(
                            "contributor",
                            row,
                            "rowID",
                            "INTEGER",
                            1,
                            1,
                            DefaultValue::Null,
                        )?;
                    }
                    1 => {
                        columns_check(
                            "contributor",
                            row,
                            "component",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    2 => {
                        columns_check(
                            "contributor",
                            row,
                            "languageTag",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    3 => {
                        columns_check(
                            "contributor",
                            row,
                            "contributor",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    4 => {
                        columns_check(
                            "contributor",
                            row,
                            "substituteFor",
                            "TEXT",
                            0,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    5 => {
                        columns_check(
                            "contributor",
                            row,
                            "comment",
                            "TEXT",
                            0,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    6 => {
                        columns_check(
                            "contributor",
                            row,
                            "verified",
                            "DATE",
                            0,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    _ => {}
                }
            }

            // Table: language
            let mut statement =
                connection.prepare("SELECT * FROM pragma_table_info('language');")?;
            let mut rows = statement.query([])?;
            while let Some(row) = rows.next()? {
                let cid: usize = row.get(0)?;
                match cid {
                    0 => {
                        columns_check(
                            "language",
                            row,
                            "rowID",
                            "INTEGER",
                            1,
                            1,
                            DefaultValue::Null,
                        )?;
                    }
                    1 => {
                        columns_check("language", row, "tag", "TEXT", 1, 0, DefaultValue::Null)?;
                    }
                    2 => {
                        columns_check(
                            "language",
                            row,
                            "englishName",
                            "TEXT",
                            0,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    3 => {
                        columns_check("language", row, "added", "DATE", 1, 0, DefaultValue::Null)?;
                    }
                    _ => {}
                }
            }

            // Table: pattern
            let mut statement =
                connection.prepare("SELECT * FROM pragma_table_info('pattern');")?;
            let mut rows = statement.query([])?;
            while let Some(row) = rows.next()? {
                let cid: usize = row.get(0)?;
                match cid {
                    0 => {
                        columns_check(
                            "pattern",
                            row,
                            "rowID",
                            "INTEGER",
                            1,
                            1,
                            DefaultValue::Null,
                        )?;
                    }
                    1 => {
                        columns_check(
                            "pattern",
                            row,
                            "component",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    2 => {
                        columns_check(
                            "pattern",
                            row,
                            "identifier",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    3 => {
                        columns_check(
                            "pattern",
                            row,
                            "languageTag",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    4 => {
                        columns_check("pattern", row, "string", "TEXT", 1, 0, DefaultValue::Null)?;
                    }
                    5 => {
                        columns_check("pattern", row, "comment", "TEXT", 0, 0, DefaultValue::Null)?;
                    }
                    6 => {
                        columns_check(
                            "pattern",
                            row,
                            "verified",
                            "DATE",
                            0,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    _ => {}
                }
            }

            // Table: component
            let mut statement =
                connection.prepare("SELECT * FROM pragma_table_info('component');")?;
            let mut rows = statement.query([])?;
            while let Some(row) = rows.next()? {
                let cid: usize = row.get(0)?;
                match cid {
                    0 => {
                        columns_check(
                            "component",
                            row,
                            "rowID",
                            "INTEGER",
                            1,
                            1,
                            DefaultValue::Null,
                        )?;
                    }
                    1 => {
                        columns_check(
                            "component",
                            row,
                            "identifier",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    2 => {
                        columns_check(
                            "component",
                            row,
                            "languageTag",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    3 => {
                        columns_check(
                            "component",
                            row,
                            "comment",
                            "TEXT",
                            0,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    4 => {
                        columns_check("component", row, "added", "DATE", 1, 0, DefaultValue::Null)?;
                    }
                    _ => {}
                }
            }

            // Table: languageData
            let mut statement =
                connection.prepare("SELECT * FROM pragma_table_info('languageData');")?;
            let mut rows = statement.query([])?;
            while let Some(row) = rows.next()? {
                let cid: usize = row.get(0)?;
                match cid {
                    0 => {
                        columns_check(
                            "languageData",
                            row,
                            "rowID",
                            "INTEGER",
                            1,
                            1,
                            DefaultValue::Null,
                        )?;
                    }
                    1 => {
                        columns_check(
                            "languageData",
                            row,
                            "component",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    2 => {
                        columns_check(
                            "languageData",
                            row,
                            "languageTag",
                            "TEXT",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    3 => {
                        columns_check(
                            "languageData",
                            row,
                            "count",
                            "INTEGER",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    4 => {
                        columns_check(
                            "languageData",
                            row,
                            "ratio",
                            "REAL",
                            1,
                            0,
                            DefaultValue::Null,
                        )?;
                    }
                    _ => {}
                }
            }
        }
        _ => {
            return Err(SchemaError::Version(
                connection.path().unwrap().to_string(),
                "1.0".to_string(),
            ))
        }
    }
    Ok(schema_version)
}

// Internal structs, enum, and functions

/// Check table exists.
fn table_exists(table_list: &Vec<String>, name: &str) -> Result<(), SchemaError> {
    let mut found = false;
    for table in table_list {
        if table.as_str() == name {
            found = true;
        }
    }
    if !found {
        return Err(SchemaError::MissingTable(name.to_string()));
    }
    Ok(())
}

/// Holds the default value of the column.
#[allow(dead_code)]
enum DefaultValue {
    Null,

    // Included even though currently no default values are used.
    Integer(usize),
    Real(f64),
    Text(String),
    Date(String),
}

/// Check schema of the table's columns.
fn columns_check(
    table: &str,
    row: &Row,
    name: &str,
    type_: &str,
    notnull: usize,
    pk: usize,
    default: DefaultValue,
) -> Result<(), SchemaError> {
    let name_: String = row.get(1)?;
    if name_.as_str() != name {
        return Err(SchemaError::MissingColumn(
            table.to_string(),
            name.to_string(),
        ));
    }
    let type__: String = row.get(2)?;
    if type__.as_str() != type_ {
        return Err(SchemaError::ColumnProperty(
            table.to_string(),
            name.to_string(),
            type_.to_string(),
        ));
    }
    let notnull_: usize = row.get(3)?;
    if notnull_ != notnull {
        return Err(SchemaError::ColumnProperty(
            table.to_string(),
            name.to_string(),
            match notnull {
                1 => "NOT NULL",
                _ => "NULL",
            }
            .to_string(),
        ));
    }
    let pk_: usize = row.get(5)?;
    if pk_ != pk {
        return Err(SchemaError::ColumnProperty(
            table.to_string(),
            name.to_string(),
            match notnull {
                1 => "PRIMARY KEY",
                _ => "",
            }
            .to_string(),
        ));
    }

    #[cfg(feature = "logging")]
    trace!(
        "column details: name {:?}, type {:?}, notnull {:?}, pk {:?}",
        name,
        type_,
        notnull,
        pk
    );
    match type_ {
        "INTEGER" => {
            let dflt_value: Option<usize> = row.get(4)?;
            match default {
                DefaultValue::Null => {
                    if dflt_value.is_some() {
                        return Err(SchemaError::ColumnDefault(
                            table.to_string(),
                            name.to_string(),
                            "NULL".to_string(),
                        ));
                    }

                    #[cfg(feature = "logging")]
                    trace!("column details: dflt_value {:?}", "NULL");
                }
                // Included even though currently no default values are used.
                DefaultValue::Integer(value) => {
                    if dflt_value.is_none() || dflt_value != Some(value) {
                        return Err(SchemaError::ColumnDefault(
                            table.to_string(),
                            name.to_string(),
                            value.to_string(),
                        ));
                    }

                    #[cfg(feature = "logging")]
                    trace!("column details: dflt_value {:?}", value);
                }
                _ => {
                    return Err(SchemaError::ColumnMismatch(
                        table.to_string(),
                        name.to_string(),
                    ));
                }
            }
        }
        "REAL" => {
            let dflt_value: Option<f64> = row.get(4)?;
            match default {
                DefaultValue::Null => {
                    if dflt_value.is_some() {
                        return Err(SchemaError::ColumnDefault(
                            table.to_string(),
                            name.to_string(),
                            "NULL".to_string(),
                        ));
                    }

                    #[cfg(feature = "logging")]
                    trace!("column details: dflt_value {:?}", "NULL");
                }
                // Included even though currently no default values are used.
                DefaultValue::Real(value) => {
                    if dflt_value.is_none() || dflt_value != Some(value) {
                        return Err(SchemaError::ColumnDefault(
                            table.to_string(),
                            name.to_string(),
                            value.to_string(),
                        ));
                    }

                    #[cfg(feature = "logging")]
                    trace!("column details: dflt_value {:?}", value);
                }
                _ => {
                    return Err(SchemaError::ColumnMismatch(
                        table.to_string(),
                        name.to_string(),
                    ));
                }
            }
        }
        "TEXT" => {
            let dflt_value: Option<String> = row.get(4)?;
            match default {
                DefaultValue::Null => {
                    if dflt_value.is_some() {
                        return Err(SchemaError::ColumnDefault(
                            table.to_string(),
                            name.to_string(),
                            "NULL".to_string(),
                        ));
                    }

                    #[cfg(feature = "logging")]
                    trace!("column details: dflt_value {:?}", "NULL");
                }
                // Included even though currently no default values are used.
                DefaultValue::Text(ref value) => {
                    if dflt_value.is_none() || dflt_value != Some(value.clone()) {
                        return Err(SchemaError::ColumnDefault(
                            table.to_string(),
                            name.to_string(),
                            value.to_string(),
                        ));
                    }

                    #[cfg(feature = "logging")]
                    trace!("column details: dflt_value {:?}", value);
                }
                _ => {
                    return Err(SchemaError::ColumnMismatch(
                        table.to_string(),
                        name.to_string(),
                    ));
                }
            }
        }
        "DATE" => {
            let dflt_value: Option<String> = row.get(4)?;
            match default {
                DefaultValue::Null => {
                    if dflt_value.is_some() {
                        return Err(SchemaError::ColumnDefault(
                            table.to_string(),
                            name.to_string(),
                            "NULL".to_string(),
                        ));
                    }

                    #[cfg(feature = "logging")]
                    trace!("column details: dflt_value {:?}", "NULL");
                }
                // Included even though currently no default values are used.
                DefaultValue::Date(ref value) => {
                    if dflt_value.is_none() || dflt_value != Some(value.clone()) {
                        return Err(SchemaError::ColumnDefault(
                            table.to_string(),
                            name.to_string(),
                            value.to_string(),
                        ));
                    }

                    #[cfg(feature = "logging")]
                    trace!("column details: dflt_value {:?}", value);
                }
                _ => {
                    return Err(SchemaError::ColumnMismatch(
                        table.to_string(),
                        name.to_string(),
                    ));
                }
            }
        }
        _ => {} // Unreachable branch
    }
    Ok(())
}

// Created these functions to avoid code duplicates of getting around a known
// incorrect rust parsing error where preceding statements with attribute
// "#[cfg(not(feature="sync"))]" inside `if` branch. These Github issues are
// still open relating to this incorrect parsing error.

fn query_pattern(schema_version: &str, only_one: bool, exact: bool) -> String {
    match schema_version {
        "1.0" => {
            let mut query = "SELECT string, languageTag FROM pattern WHERE identifier = ?1 AND \
            languageTag "
                .to_string();
            if exact {
                query.push_str("= ?2")
            } else {
                query.push_str("LIKE ?2")
            }
            query.push_str(" AND component = ?3");
            if only_one {
                query.push_str(" LIMIT 1")
            }
            query
        }
        &_ => todo!(),
    }
}

fn query_languages(schema_version: &str) -> String {
    match schema_version {
        "1.0" => "SELECT DISTINCT languageTag FROM contributor WHERE component = ?1".to_string(),
        &_ => todo!(),
    }
}

fn query_identifier_languages(schema_version: &str) -> String {
    match schema_version {
        "1.0" => {
            "SELECT languageTag FROM pattern WHERE identifier = ?1 AND component = ?2".to_string()
        }
        &_ => todo!(),
    }
}

fn query_contributors(schema_version: &str) -> String {
    match schema_version {
        "1.0" => {
            "SELECT DISTINCT contributor FROM contributor WHERE languageTag = ?1 AND component = ?2"
                .to_string()
        }
        &_ => todo!(),
    }
}

fn query_count(schema_version: &str) -> String {
    match schema_version {
        "1.0" => {
            "SELECT count( * ) FROM pattern WHERE languageTag = ?1 AND component = ?2".to_string()
        }
        &_ => todo!(),
    }
}

fn query_default(schema_version: &str) -> String {
    match schema_version {
        "1.0" => "SELECT languageTag FROM component WHERE identifier = ?1".to_string(),
        &_ => todo!(),
    }
}
