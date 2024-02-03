// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! Testing command functions.

use i18n_pattern::{ CommandRegistry, file_path/*, PlaceholderValue*/ };
use i18n_utility::PlaceholderValue;
use os_info;
use std::error::Error;

#[test]
fn test_file_path() -> Result<(), Box<dyn Error>> {
    let mut strings = Vec::<PlaceholderValue>::new();
    strings.push( PlaceholderValue::String( "file_path".to_string() ) );
    strings.push( PlaceholderValue::String( "tests/command.rs".to_string() ) );
    let string = file_path( strings )?;
    let info = os_info::get();
    if info.os_type() == os_info::Type::Windows {
        assert_eq!( string.as_str(), "tests\\command.rs", "Should be Windows path." );
    } else {
        assert_eq!( string.as_str(), "tests/command.rs", "Should be non-Windows path." );
    }
    Ok( () )
}

#[test]
fn command_registry() -> Result<(), Box<dyn Error>> {
    let registry = CommandRegistry::new();
    registry.insert( "file_path", file_path )?;
    let mut strings = Vec::<PlaceholderValue>::new();
    strings.push( PlaceholderValue::String( "file_path".to_string() ) );
    strings.push( PlaceholderValue::String( "tests/command.rs".to_string() ) );
    let function = registry.command( "file_path" )?;
    let string = function( strings )?;
    assert_eq!( string.as_str(), "tests/command.rs", "Strings must be the same." );
    Ok( () )
}
