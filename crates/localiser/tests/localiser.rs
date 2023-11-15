// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

//! Testing `Localiser`.

use i18n_icu::{ IcuDataProvider, DataProvider };
use i18n_utility::LanguageTagRegistry;
use i18n_provider_sqlite3::LocalisationProviderSqlite3;
use i18n_pattern::{ PlaceholderValue, CommandRegistry };
use i18n_localiser::Localiser;
use std::collections::HashMap;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

use std::error::Error;

#[test]
fn format() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let language_tag_registry = RefCount::new( LanguageTagRegistry::new() );
    let lstring_provider = LocalisationProviderSqlite3::try_new(
        "./l10n/", &language_tag_registry, false
    )?;
    let command_registry = RefCount::new( CommandRegistry::new() );
    let localiser = Localiser::try_new(
        &icu_data_provider,
        &language_tag_registry,
        lstring_provider,
        &command_registry,
        true,
        true,
        "en-ZA",
    )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "component".to_string(),
        PlaceholderValue::String( "i18n_localiser".to_string() )
    );
    values.insert(
        "identifier".to_string(),
        PlaceholderValue::String( "string_not_found".to_string() )
    );
    values.insert(
        "language_tag".to_string(),
        PlaceholderValue::String( "en-ZA".to_string() )
    );
    values.insert(
        "fallback".to_string(),
        PlaceholderValue::String( "true".to_string() )
    );
    let lstring = localiser.format(
        "i18n_localiser",
        "string_not_found",
        &values,
        &language_tag_registry.tag( "en-ZA" ).unwrap(),
        None,
        None
    )?;
    assert_eq!(
        lstring.as_str(),
        "No string was found for the component ‘i18n_localiser’ with identifier ‘string_not_found’ for the language \
            tag ‘en-ZA’. Fallback was used: True.",
        "Check placeholder values."
    );
    Ok( () )
}


#[test]
fn format_with_defaults() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let language_tag_registry = RefCount::new( LanguageTagRegistry::new() );
    let lstring_provider = LocalisationProviderSqlite3::try_new(
        "./l10n/", &language_tag_registry, false
    )?;
    let command_registry = RefCount::new( CommandRegistry::new() );
    let localiser = Localiser::try_new(
        &icu_data_provider,
        &language_tag_registry,
        lstring_provider,
        &command_registry,
        true,
        true,
        "en-ZA",
    )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "component".to_string(),
        PlaceholderValue::String( "i18n_localiser".to_string() )
    );
    let lstring = localiser.format_with_defaults(
        "i18n_localiser",
        "no_default_language_tag",
        &values,
    )?;
    assert_eq!(
        lstring.as_str(),
        "No default language tag was found for the component ‘i18n_localiser’.",
        "Check placeholder values."
    );
    Ok( () )
}

#[test]
fn literal() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let language_tag_registry = RefCount::new( LanguageTagRegistry::new() );
    let lstring_provider = LocalisationProviderSqlite3::try_new(
        "./l10n/", &language_tag_registry, false
    )?;
    let command_registry = RefCount::new( CommandRegistry::new() );
    let localiser = Localiser::try_new(
        &icu_data_provider,
        &language_tag_registry,
        lstring_provider,
        &command_registry,
        true,
        true,
        "en-ZA",
    )?;
    let lstring = localiser.literal(
        "i18n_localiser",
        "no_default_language_tag",
        "en-ZA",
        None,
        None
    )?;
    assert_eq!(
        lstring.as_str(),
        "No default language tag was found for the component ‘{component}’.",
        "Check placeholder values."
    );
    Ok( () )
}

#[test]
fn literal_with_defaults() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let language_tag_registry = RefCount::new( LanguageTagRegistry::new() );
    let lstring_provider = LocalisationProviderSqlite3::try_new(
        "./l10n/", &language_tag_registry, false
    )?;
    let command_registry = RefCount::new( CommandRegistry::new() );
    let localiser = Localiser::try_new(
        &icu_data_provider,
        &language_tag_registry,
        lstring_provider,
        &command_registry,
        true,
        true,
        "en-ZA",
    )?;
    let lstring = localiser.literal_with_defaults(
        "i18n_localiser",
        "no_default_language_tag",
    )?;
    assert_eq!(
        lstring.as_str(),
        "No default language tag was found for the component ‘{component}’.",
        "Check placeholder values."
    );
    Ok( () )
}
