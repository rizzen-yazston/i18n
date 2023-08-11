// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

//! Testing `Message`.

use i18n_icu::{ IcuDataProvider, DataProvider };
use i18n_utility::LanguageTagRegistry;
use i18n_provider_sqlite3::ProviderSqlite3;
use i18n_pattern::{ PlaceholderValue, CommandRegistry };
use i18n_message::Message;
use std::collections::HashMap;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

use std::error::Error;

#[test]
fn message() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let language_tag_registry = RefCount::new( LanguageTagRegistry::new() );
    let lstring_provider = ProviderSqlite3::try_new(
        "./l10n/", &language_tag_registry
    )?;
    let command_registry = RefCount::new( CommandRegistry::new() );
    let mut message_system = Message::try_new(
        &icu_data_provider,
        &language_tag_registry,
        lstring_provider,
        &command_registry,
        true,
        true
    )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "component".to_string(),
        PlaceholderValue::String( "i18n_message".to_string() )
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
    let lstring = message_system.format(
        "i18n_message",
        "string_not_found",
        &values,
        &language_tag_registry.get_language_tag( "en-ZA" ).unwrap(),
        None,
        None
    )?;
    assert_eq!(
        lstring.as_str(),
        "No string was found for the component ‘i18n_message’ with identifier ‘string_not_found’ for the language \
            tag ‘en-ZA’. Fallback was used: True.",
        "Check placeholder values."
    );
    Ok( () )
}
