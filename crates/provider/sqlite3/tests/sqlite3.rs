// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

//! Testing `get()` and `default_tag()`.

use i18n_provider::LanguageStringProvider;
use i18n_provider_sqlite3::ProviderSqlite3;
use i18n_utility::LanguageTagRegistry;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

use std::error::Error;

#[test]
fn get_for_en() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let tag = registry.get_tag( "en" )?;
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let strings = provider.get(
        "i18n_provider_sqlite3",
        "path_conversion",
        &tag
    )?;
    assert_eq!( strings.len(), 1, "There should be 1 string." );
    assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    Ok( () )
}

#[test]
fn get_for_en_za_u_ca_julian() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let tag = registry.get_tag( "en-ZA-u-ca-julian" )?;
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let strings = provider.get(
        "i18n_provider_sqlite3",
        "path_conversion",
        &tag
    )?;
    assert_eq!( strings.len(), 1, "There should be 1 string." );
    assert_eq!( strings[ 0 ].as_str(), "Conversion to {`PathBuf`} error.", "Not correct string." );
    Ok( () )
}

#[test]
fn default_tag() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let tag = provider.default_language(
        "i18n_provider_sqlite3",
    )?.expect( "No default language tag found." );
    assert_eq!( tag.as_str(), "en-ZA", "Must be en-ZA." );
    Ok( () )
}

#[test]
fn identifier_languages() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let tags = provider.identifier_languages(
        "i18n_provider_sqlite3",
        "path_conversion",
    )?;//.expect( "No default language tag found." );
    assert_eq!( tags.iter().count(), 2, "Must be 2 languages." );
    Ok( () )
}

#[test]
fn component_languages() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let tags = provider.component_languages(
        "i18n_provider_sqlite3",
    )?;//.expect( "No default language tag found." );
    for tag in tags {
        if tag.language == registry.get_tag( "en-ZA" ).unwrap() {
            assert_eq!( tag.ratio, 1.0, "Ratio ust be 1.0 for en-ZA." );
        }
    }
    Ok( () )
}

#[test]
fn repository_languages() -> Result<(), Box<dyn Error>> {
    let path = "./l10n/";
    let registry = RefCount::new( LanguageTagRegistry::new() );
    let provider = ProviderSqlite3::try_new(
        path,
        &registry
    )?;
    let tags = provider.repository_languages()?;//.expect( "No default language tag found." );
    assert_eq!( tags.iter().count(), 2, "Must be 2 languages." );
    Ok( () )
}
