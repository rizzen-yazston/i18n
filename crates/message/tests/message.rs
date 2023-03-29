// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

//! Testing `Message`.

use i18n_icu::IcuDataProvider;
use i18n_registry::LanguageTagRegistry;
use i18n_provider_sqlite3::ProviderSqlite3;
use i18n_pattern::PlaceholderValue;
use i18n_message::Message;
use icu_testdata::buffer;
use icu_provider::serde::AsDeserializingBufferProvider;
use std::collections::HashMap;
use std::rc::Rc;
use std::error::Error;

#[test]
fn message() -> Result<(), Box<dyn Error>> {
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider = Rc::new(
        IcuDataProvider::try_new( &data_provider )?
    );
    let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    let lstring_provider = ProviderSqlite3::try_new(
        "./i18n/", &language_tag_registry
    )?;
    let message_system = Message::try_new(
        &icu_data_provider, &language_tag_registry, &lstring_provider, true, true
    )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "identifier".to_string(),
        PlaceholderValue::String( "i18n_message/string_not_found".to_string() )
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
        "i18n_message/string_not_found",
        &values,
        &language_tag_registry.get_language_tag( "en-ZA" ).unwrap(),
        None,
        None
    )?;
    assert_eq!(
        lstring.as_str(),
        "No string was found for identifier ‘i18n_message/string_not_found’ and language tag ‘en-ZA’. Fallback used: True.",
        "Check placeholder values."
    );
    Ok( () )
}
