// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

//! Testing formatter.

/* Need to figure out how to modify these tests
use i18n_icu::{ IcuDataProvider, DataProvider };
use i18n_lexer::Lexer;
use i18n_pattern::{ parse, /*Formatter, *//*PlaceholderValue, +*/CommandRegistry, file_path, english_a_or_an };
use i18n_utility::PlaceholderValue;
use icu_locid::Locale;
use icu_calendar::{ Iso, DateTime, Date, types::Time };
use os_info;
use std::collections::HashMap;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

use std::error::Error;

#[test]
fn plain_text() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "A simple plain text string." );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let values = HashMap::<String, PlaceholderValue>::new();
    let result = formatter.format( &values )?;
    assert_eq!( result.as_str(), "A simple plain text string.", "Strings must be the same." );
    Ok( () )
}

#[test]
fn pattern_string() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "Expecting a string for placeholder: {string}" );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "string".to_string(),
        PlaceholderValue::String( "This is a string.".to_string() )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "Expecting a string for placeholder: This is a string.",
        "Strings must be the same."
    );
    Ok( () )
}

#[test]
fn pattern_plural() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}\
        {one_dog is 1 dog}" );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "dogs_number".to_string(),
        PlaceholderValue::Unsigned( 3 )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "There are 3 dogs in the park.",
        "Strings must be the same."
    );
    Ok( () )
}

#[test]
fn pattern_decimal() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "There is {amount decimal} kg of rice in the container." );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "amount".to_string(),
        PlaceholderValue::Float( 3.678 )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "There is 3.678 kg of rice in the container.",
        "Strings must be the same."
    );
    Ok( () )
}

#[test]
fn pattern_decimal_with_option() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "There is {amount decimal sign#always} kg of rice in the container." );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "amount".to_string(),
        PlaceholderValue::Float( 3.678 )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "There is +3.678 kg of rice in the container.",
        "Strings must be the same."
    );
    Ok( () )
}

#[test]
fn pattern_datetime() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "At this point in time {time date_time} the moon winked out." );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "time".to_string(),
        PlaceholderValue::DateTime( DateTime::<Iso>::new(
            Date::try_new_iso_date( 248624, 10, 6 ).ok().unwrap(),
            Time::try_new( 5, 47, 23, 254 ).ok().unwrap()
        ) )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "At this point in time 06 Oct 248624, 05:47:23 the moon winked out.",
        "Strings must be the same."
    );
    Ok( () )
}

#[test]
fn pattern_datetime_string() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "At this point in time {time date_time} the moon winked out." );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "time".to_string(),
        PlaceholderValue::String( "+248624-10-06T05:47:23.254".to_string() )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "At this point in time 06 Oct 248624, 05:47:23 the moon winked out.",
        "Strings must be the same."
    );
    Ok( () )
}

#[test]
fn command_static() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "The file ‘{#file_path `tests/formatter.rs`}’ failed to open." );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    command_registry.insert( "file_path", file_path )?;
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let values = HashMap::<String, PlaceholderValue>::new();
    let result = formatter.format( &values )?;
    let info = os_info::get();
    if info.os_type() == os_info::Type::Windows {
        assert_eq!( result.as_str(), "The file ‘tests\\formatter.rs’ failed to open.", "Should be Windows path." );
    } else {
        assert_eq!( result.as_str(), "The file ‘tests/formatter.rs’ failed to open.", "Should be non-Windows path." );
    }
    Ok( () )
}

#[test]
fn command_delayed() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "At night {#english_a_or_an# hunter} {hunter} stalked {#english_a_or_an # prey} {prey}." );
    let tree = parse( tokens )?;
    let locale: RefCount<Locale> = RefCount::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    let language_tag = RefCount::new( locale.to_string() );
    let command_registry = RefCount::new( CommandRegistry::new() );
    command_registry.insert( "english_a_or_an", english_a_or_an )?;
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "hunter".to_string(),
        PlaceholderValue::String( "owl".to_string() )
    );
    values.insert(
        "prey".to_string(),
        PlaceholderValue::String( "mouse".to_string() )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "At night an owl stalked a mouse.",
        "Strings must be the same."
    );
    Ok( () )
}
*/
