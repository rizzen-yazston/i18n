// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! Testing `tokenise()`.

use i18n_lexer::{ Lexer, TokenType };
use icu_testdata::buffer;
use std::error::Error;

#[test]
fn tokenise() -> Result<(), Box<dyn Error>> {
    let buffer_provider = Box::new( buffer() );
    let mut lexer = Lexer::try_new( &buffer_provider )?;
    let tokens = lexer.tokenise(
        "String contains a {placeholder}.", &vec![ '{', '}' ]
    );
    let mut grammar = 0;
    assert_eq!( tokens.iter().count(), 10, "Supposed to be a total of 10 tokens." );
    for token in tokens.iter() {
        if token.token_type == TokenType::Grammar {
            grammar += 1;
        }
    }
    assert_eq!( grammar, 2, "Supposed to be 2 grammar tokens." );
    Ok( () )
}
