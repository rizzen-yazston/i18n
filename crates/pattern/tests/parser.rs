// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! Testing `decimal` pattern and `plural` pattern.

use i18n_icu::{ IcuDataProvider, DataProvider };
use i18n_lexer::Lexer;
use i18n_pattern::parse;
use std::rc::Rc;
use std::error::Error;

#[test]
fn decimal() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "String contains a {placeholder decimal sign#negative}." );
    let tree = parse( tokens )?;
    assert_eq!( tree.len(), 10, "Should contain 10 nodes." );
    Ok( () )
}

#[test]
fn plural() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    let ( tokens, _lengths, _grammar ) =
        lexer.tokenise( "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}" );
    let tree = parse( tokens )?;
    assert_eq!( tree.len(), 24, "Should contain 24 nodes." );
    Ok( () )
}
