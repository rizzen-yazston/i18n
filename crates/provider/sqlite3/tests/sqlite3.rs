// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

//! Testing `get()` and `default_language_tag()`.

use i18n_provider::LStringProvider;
use i18n_provider_sqlite3::ProviderSqlite3;
use i18n_utility::LanguageTagRegistry;
use std::{ rc::Rc, error::Error };

#[test]
fn get_for_en() -> Result<(), Box<dyn Error>> {
    let path = "./i18n/";
    let registry = Rc::new( LanguageTagRegistry::new() );
    let tag = registry.get_language_tag( "en" )?;
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let strings = provider.get(
        "i18n_provider_sqlite3/invalid_path",
        &tag
    )?;
    assert_eq!( strings.len(), 1, "There should be 1 string." );
    assert_eq!( strings[ 0 ].as_str(), "Invalid path was provided.", "Not correct string." );
    Ok( () )
}

#[test]
fn get_for_en_za_u_ca_julian() -> Result<(), Box<dyn Error>> {
    let path = "./i18n/";
    let registry = Rc::new( LanguageTagRegistry::new() );
    let tag = registry.get_language_tag( "en-ZA-u-ca-julian" )?;
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let strings = provider.get(
        "i18n_provider_sqlite3/invalid_path",
        &tag
    )?;
    assert_eq!( strings.len(), 1, "There should be 1 string." );
    assert_eq!( strings[ 0 ].as_str(), "Invalid path was provided.", "Not correct string." );
    Ok( () )
}

#[test]
fn default_language_tag() -> Result<(), Box<dyn Error>> {
    let path = "./i18n/";
    let registry = Rc::new( LanguageTagRegistry::new() );
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let tag = provider.default_language_tag(
        "i18n_provider_sqlite3"
    )?.expect( "No default language tag found." );
    assert_eq!( tag, "en-ZA", "Must be en-ZA." );
    Ok( () )
}
