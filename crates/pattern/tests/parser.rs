// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! Testing `decimal` pattern and `plural` pattern.

use i18n_lexer::Lexer;
use i18n_pattern::parse;
use icu_testdata::buffer;
use std::error::Error;

#[test]
fn decimal() -> Result<(), Box<dyn Error>> {
    let buffer_provider = Box::new( buffer() );
    let mut lexer = Lexer::try_new( &buffer_provider )?;
    let tokens = lexer.tokenise(
        "String contains a {placeholder decimal sign#negative}.", &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens )?;
    assert_eq!( tree.len(), 10, "Should contain 10 nodes." );
    Ok( () )
}

#[test]
fn plural() -> Result<(), Box<dyn Error>> {
    let buffer_provider = Box::new( buffer() );
    let mut lexer = Lexer::try_new( &buffer_provider )?;
    let tokens = lexer.tokenise(
        "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}",
        &vec![ '{', '}', '`', '#' ]
    );
    let tree = parse( tokens )?;
    assert_eq!( tree.len(), 24, "Should contain 24 nodes." );
    Ok( () )
}
