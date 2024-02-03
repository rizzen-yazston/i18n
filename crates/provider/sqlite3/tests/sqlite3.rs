// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

//! Testing `strings()` and `default_tag()`.

use i18n_provider::LocalisationProviderTrait;
use i18n_provider_sqlite3::LocalisationProviderSqlite3;
use i18n_utility::LanguageTagRegistry;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

use std::error::Error;

#[test]// positive
fn strings_for_en() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let tag = registry.tag( "en" )?;
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let strings = provider.strings(
        "i18n_provider_sqlite3",
        "path_conversion",
        &tag
    )?;
    assert_eq!( strings.len(), 1, "There should be 1 string." );
    assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    Ok( () )
}

#[test]// positive
fn strings_for_en_za_u_ca_julian() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let tag = registry.tag( "en-ZA-u-ca-julian" )?;
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let strings = provider.strings(
        "i18n_provider_sqlite3",
        "path_conversion",
        &tag
    )?;
    assert_eq!( strings.len(), 1, "There should be 1 string." );
    assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    Ok( () )
}

#[test]// negative (private use subtag)
fn strings_for_qz() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let tag = registry.tag( "qz" )?;
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let strings = provider.strings(
        "i18n_provider_sqlite3",
        "path_conversion",
        &tag
    )?;
    assert_eq!( strings.len(), 0, "There should be 0 string." );
    Ok( () )
}

#[test]//positive
fn one_string() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let tag = registry.tag( "en" )?;
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let string = provider.string(
        "i18n_provider_sqlite3",
        "path_conversion",
        &tag
    )?;
    assert_eq!( string.unwrap().as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    Ok( () )
}

#[test]//positive
fn exact_string() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let tag = registry.tag( "en-ZA" )?;
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let string = provider.string_exact_match(
        "i18n_provider_sqlite3",
        "path_conversion",
        &tag
    )?;
    assert_eq!( string.unwrap().as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    Ok( () )
}

#[test]//negative
fn exact_string_fail() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let tag = registry.tag( "en" )?;
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let string = provider.string_exact_match(
        "i18n_provider_sqlite3",
        "path_conversion",
        &tag
    )?;
    assert!( string.is_none(), "Should be None." );
    Ok( () )
}

#[test]
fn identifier_details() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let details = provider.identifier_details(
        "application",
        "example",
    )?;
    assert_eq!( details.default, registry.tag( "en-US" )?, "Should be en-US." );
    assert_eq!( details.languages.iter().count(), 2, "Should be 2 languages" );
    Ok( () )
}

#[test]
fn component_details() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let details = provider.component_details(
        "i18n_provider_sqlite3",
    )?;
    assert_eq!( details.default, registry.tag( "en-ZA" )?, "Should be en-ZA." );
    assert_eq!( details.languages.iter().count(), 2, "Should be 2 languages" );
    assert_eq!( details.total_strings, 24, "Should be 24 strings for component" );
    Ok( () )
}

#[test]
fn repository_details() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let provider = LocalisationProviderSqlite3::try_new(
        path,
        &registry,
        false,
    )?;
    let details = provider.repository_details()?;
    assert_eq!( details.default.as_ref().unwrap(), &registry.tag( "en-US" )?, "Should be en-US." );
    assert_eq!( details.languages.iter().count(), 3, "Should be 3 languages" );
    assert_eq!( details.total_strings, 28, "Should be 28 strings for repository" );
    assert_eq!( details.components.iter().count(), 2, "Should be 2 components" );
    assert_eq!( details.contributors.iter().count(), 2, "Should be contributors" );
    Ok( () )
}
