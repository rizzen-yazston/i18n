// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! Testing `decimal` pattern and `plural` pattern.

use i18n_icu::IcuDataProvider;
use i18n_lexer::tokenise;
use i18n_pattern::parse;
use icu_testdata::buffer;
use icu_provider::serde::AsDeserializingBufferProvider;
use std::rc::Rc;
use std::error::Error;

#[test]
fn decimal() -> Result<(), Box<dyn Error>> {
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        IcuDataProvider::try_new( &data_provider )?;
    let tokens = tokenise(
        "String contains a {placeholder decimal sign#negative}.",
        &vec![ '{', '}', '`', '#' ],
        &Rc::new( icu_data_provider ),
    );
    let tree = parse( tokens.0 )?;
    assert_eq!( tree.len(), 10, "Should contain 10 nodes." );
    Ok( () )
}

#[test]
fn plural() -> Result<(), Box<dyn Error>> {
    let buffer_provider = buffer();
    let data_provider = buffer_provider.as_deserializing();
    let icu_data_provider =
        IcuDataProvider::try_new( &data_provider )?;
    let tokens = tokenise(
        "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}",
        &vec![ '{', '}', '`', '#' ],
        &Rc::new( icu_data_provider ),
    );
    let tree = parse( tokens.0 )?;
    assert_eq!( tree.len(), 24, "Should contain 24 nodes." );
    Ok( () )
}
