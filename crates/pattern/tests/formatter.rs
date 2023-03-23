// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! Testing formatter.

use i18n_icu::IcuDataProvider;
use i18n_lexer::Lexer;
use i18n_pattern::{ parse, Formatter, PlaceholderValue };
use icu_testdata::buffer;
use icu_provider::serde::AsDeserializingBufferProvider;
use icu_locid::Locale;
use icu_calendar::{ Iso, DateTime, Date, types::Time };
use std::collections::HashMap;
use std::{ rc::Rc, error::Error };

#[test]
fn plain_text() -> Result<(), Box<dyn Error>> {
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        Rc::new( IcuDataProvider::try_new( &data_provider )? );
    let mut lexer = Lexer::try_new( &icu_data_provider )?;
    let tokens = lexer.tokenise(
        "A simple plain text string.", &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens.0 )?;
    let locale: Rc<Locale> = Rc::new( "en-ZA".parse()? );
    let language_tag = Rc::new( locale.to_string() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree )?;
    let values = HashMap::<String, PlaceholderValue>::new();
    let result = formatter.format( &values )?;
    assert_eq!( result.as_str(), "A simple plain text string.", "Strings must be the same." );
    Ok( () )
}

#[test]
fn pattern_string() -> Result<(), Box<dyn Error>> {
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        Rc::new( IcuDataProvider::try_new( &data_provider )? );
    let mut lexer = Lexer::try_new( &icu_data_provider )?;
    let tokens = lexer.tokenise(
        "Expecting a string for placeholder: {string}", &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens.0 )?;
    let locale: Rc<Locale> = Rc::new( "en-ZA".parse()? );
    let language_tag = Rc::new( locale.to_string() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree )?;
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
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        Rc::new( IcuDataProvider::try_new( &data_provider )? );
    let mut lexer = Lexer::try_new( &icu_data_provider )?;
    let tokens = lexer.tokenise(
        "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}",
        &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens.0 )?;
    let locale: Rc<Locale> = Rc::new( "en-ZA".parse()? );
    let language_tag = Rc::new( locale.to_string() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree )?;
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
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        Rc::new( IcuDataProvider::try_new( &data_provider )? );
    let mut lexer = Lexer::try_new( &icu_data_provider )?;
    let tokens = lexer.tokenise(
        "There is {amount decimal} kg of rice in the container.",
        &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens.0 )?;
    let locale: Rc<Locale> = Rc::new( "en-ZA".parse()? );
    let language_tag = Rc::new( locale.to_string() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "amount".to_string(),
        PlaceholderValue::Float( 3.678 )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "There is 3,678 kg of rice in the container.",
        "Strings must be the same."
    );
    Ok( () )
}

#[test]
fn pattern_decimal_with_option() -> Result<(), Box<dyn Error>> {
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        Rc::new( IcuDataProvider::try_new( &data_provider )? );
    let mut lexer = Lexer::try_new( &icu_data_provider )?;
    let tokens = lexer.tokenise(
        "There is {amount decimal sign#always} kg of rice in the container.",
        &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens.0 )?;
    let locale: Rc<Locale> = Rc::new( "en-ZA".parse()? );
    let language_tag = Rc::new( locale.to_string() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree )?;
    let mut values = HashMap::<String, PlaceholderValue>::new();
    values.insert(
        "amount".to_string(),
        PlaceholderValue::Float( 3.678 )
    );
    let result = formatter.format( &values )?;
    assert_eq!(
        result.as_str(),
        "There is +3,678 kg of rice in the container.",
        "Strings must be the same."
    );
    Ok( () )
}

#[test]
fn pattern_dateime() -> Result<(), Box<dyn Error>> {
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        Rc::new( IcuDataProvider::try_new( &data_provider )? );
    let mut lexer = Lexer::try_new( &icu_data_provider )?;
    let tokens = lexer.tokenise(
        "At this point in time {time date_time} the moon winked out.",
        &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens.0 )?;
    let locale: Rc<Locale> = Rc::new( "en-ZA".parse()? );
    let language_tag = Rc::new( locale.to_string() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree )?;
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
fn pattern_dateime_string() -> Result<(), Box<dyn Error>> {
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        Rc::new( IcuDataProvider::try_new( &data_provider )? );
    let mut lexer = Lexer::try_new( &icu_data_provider )?;
    let tokens = lexer.tokenise(
        "At this point in time {time date_time} the moon winked out.",
        &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens.0 )?;
    let locale: Rc<Locale> = Rc::new( "en-ZA".parse()? );
    let language_tag = Rc::new( locale.to_string() );
    let mut formatter =
        Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree )?;
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
