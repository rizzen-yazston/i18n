// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

use crate::{ CommandError, PlaceholderValue };
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::str::FromStr;

/// Registry for pattern commands callback functions.
/// 
/// # Examples
/// 
/// ```
/// use i18n_pattern::{CommandRegistry, file_path, PlaceholderValue};
/// use std::error::Error;
/// 
/// fn command_registry() -> Result<(), Box<dyn Error>> {
///     let registry = CommandRegistry::new();
///     registry.insert( "file_path", file_path )?;
///     let mut strings = Vec::<PlaceholderValue>::new();
///     strings.push( PlaceholderValue::String( "file_path".to_string() ) );
///     strings.push( PlaceholderValue::String( "tests/command.rs".to_string() ) );
///     let function = registry.command( "file_path" )?;
///     let string = function( strings )?;
///     assert_eq!( string.as_str(), "tests/command.rs", "Strings must be the same." );
///     Ok( () )
/// }
/// ```
pub struct CommandRegistry {
    registry: RefCell<HashMap<String, fn( Vec<PlaceholderValue> ) -> Result<String, CommandError>>>,
}

impl CommandRegistry {
    /// Creates an empty registry.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_pattern::{CommandRegistry, file_path, PlaceholderValue};
    /// use std::error::Error;
    /// 
    /// fn command_registry() -> Result<(), Box<dyn Error>> {
    ///     let registry = CommandRegistry::new();
    ///     registry.insert( "file_path", file_path )?;
    ///     let mut strings = Vec::<PlaceholderValue>::new();
    ///     strings.push( PlaceholderValue::String( "file_path".to_string() ) );
    ///     strings.push( PlaceholderValue::String( "tests/command.rs".to_string() ) );
    ///     let function = registry.command( "file_path" )?;
    ///     let string = function( strings )?;
    ///     assert_eq!( string.as_str(), "tests/command.rs", "Strings must be the same." );
    ///     Ok( () )
    /// }
    /// ```
    pub fn new() -> Self {
        CommandRegistry {
            registry: RefCell::new(
                HashMap::<String, fn( Vec<PlaceholderValue> ) -> Result<String, CommandError>>::new()
            ),
        }
    }

    /// Insert a new the command callback function to the registry.
    /// 
    /// The [`Vec`]`<`[`String`]`>` passed to callback has following index meaning:
    ///  - 0: the command identifier,
    ///  - 1-: are the parameters used by the command callback.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_pattern::{CommandRegistry, file_path, PlaceholderValue};
    /// use std::error::Error;
    /// 
    /// fn command_registry() -> Result<(), Box<dyn Error>> {
    ///     let registry = CommandRegistry::new();
    ///     registry.insert( "file_path", file_path )?;
    ///     let mut strings = Vec::<PlaceholderValue>::new();
    ///     strings.push( PlaceholderValue::String( "file_path".to_string() ) );
    ///     strings.push( PlaceholderValue::String( "tests/command.rs".to_string() ) );
    ///     let function = registry.command( "file_path" )?;
    ///     let string = function( strings )?;
    ///     assert_eq!( string.as_str(), "tests/command.rs", "Strings must be the same." );
    ///     Ok( () )
    /// }
    /// ```
    pub fn insert<T: AsRef<str>>(
        &self,
        command: T,
        function: fn( Vec<PlaceholderValue> ) -> Result<String, CommandError>,
    ) -> Result<(), CommandError> {
        {
            if self.registry.borrow().contains_key( command.as_ref() ) {
                return Err( CommandError::AlreadyExists( command.as_ref().to_string() ) );
            }
        }
        self.registry.borrow_mut().insert( command.as_ref().to_string(), function );
        Ok( () )
    }

    /// Get the callback function for the command.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_pattern::{CommandRegistry, file_path, PlaceholderValue};
    /// use std::error::Error;
    /// 
    /// fn command_registry() -> Result<(), Box<dyn Error>> {
    ///     let registry = CommandRegistry::new();
    ///     registry.insert( "file_path", file_path )?;
    ///     let mut strings = Vec::<PlaceholderValue>::new();
    ///     strings.push( PlaceholderValue::String( "file_path".to_string() ) );
    ///     strings.push( PlaceholderValue::String( "tests/command.rs".to_string() ) );
    ///     let function = registry.command( "file_path" )?;
    ///     let string = function( strings )?;
    ///     assert_eq!( string.as_str(), "tests/command.rs", "Strings must be the same." );
    ///     Ok( () )
    /// }
    /// ```
    pub fn command<T: AsRef<str>>(
        &self,
        command: T,
    ) -> Result<fn( Vec<PlaceholderValue> ) -> Result<String, CommandError>, CommandError> {
        let binding = self.registry.borrow();
        let Some( result ) = binding.get( command.as_ref() ) else {
            return Err( CommandError::NotFound( command.as_ref().to_string() ) );
        };
        Ok( result.to_owned() )
    }

    /// Returns a [`Vec`]`<`[`String`]`>` of the identifiers of all the registered commands.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_pattern::{CommandRegistry, file_path};
    /// use std::error::Error;
    /// 
    /// fn command_registry() -> Result<(), Box<dyn Error>> {
    ///     let registry = CommandRegistry::new();
    ///     registry.insert( "file_path", file_path )?;
    ///     let list = registry.list().iter().count();
    ///     assert_eq!( list, 1, "Only 1 command." );
    ///     Ok( () )
    /// }
    /// ```
    pub fn list( &self ) -> Vec<String> {
        Vec::from_iter( self.registry.borrow().keys().map( |x| x.to_string() ) )
    }
}

/// # Available commands
/// 
/// The following commands are provided by this crate.
///
/// Format a file path according to the OS being used.
/// Requires 1 parameter having type of [`PlaceholderValue`]`::String`.
/// Additional parameters are simply ignored.
pub fn file_path( parameters: Vec<PlaceholderValue> ) -> Result<String, CommandError> {
    let command = match &parameters[ 0 ] {
        PlaceholderValue::String( string ) => string,

        // When called from within the `Formatter` methods, this branch is never reached.
        _ => return Err( CommandError::InvalidType( "command identifier".to_string(), 0 ) )
    };
    if parameters.len() < 2 {
        return Err( CommandError::ParameterMissing( command.clone(), 1 ) );
    }
    let string = match &parameters[ 1 ] {
        PlaceholderValue::String( string ) => string,
        _ => return Err( CommandError::InvalidType( command.clone(), 1 ) )
    };
    let path = PathBuf::from_str( &string ).unwrap();
    Ok( path.display().to_string() )
}

/// Select a or an for english words passed as parameter.
/// Requires 1 parameter having type of [`PlaceholderValue`]`::String`.
/// Additional parameters are simply ignored.
pub fn english_a_or_an( parameters: Vec<PlaceholderValue> ) -> Result<String, CommandError> {
    let command = match &parameters[ 0 ] {
        PlaceholderValue::String( string ) => string,

        // When called from within the `Formatter` methods, this branch is never reached.
        _ => return Err( CommandError::InvalidType( "command identifier".to_string(), 0 ) )
    };
    if parameters.len() < 2 {
        return Err( CommandError::ParameterMissing( command.clone(), 1 ) );
    }
    let string = match &parameters[ 1 ] {
        PlaceholderValue::String( string ) => string,
        _ => return Err( CommandError::InvalidType( command.clone(), 1 ) )
    };
    let mut chars = string.chars();
    match chars.next().unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => return Ok( "an".to_string() ),
        _ => return Ok( "a".to_string() )
    }
}
