// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! Testing `tokenise()`.

use i18n_icu::{ IcuDataProvider, DataProvider };
use i18n_lexer::{ Lexer, TokenType };
use std::rc::Rc;
use std::error::Error;

#[test]
fn tokenise_single_byte_character_string() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}' ], &icu_data_provider );
    let ( tokens, lengths, grammar ) =
        lexer.tokenise( "String contains a {placeholder}." );
    let mut grammar_tokens = 0;
    assert_eq!( lengths.bytes, 32, "Supposed to be a total of 32 bytes." );
    assert_eq!( lengths.characters, 32, "Supposed to be a total of 32 characters." );
    assert_eq!( lengths.graphemes, 32, "Supposed to be a total of 32 graphemes." );
    assert_eq!( lengths.tokens, 10, "Supposed to be a total of 10 tokens." );
    for token in tokens.iter() {
        if token.token_type == TokenType::Grammar {
            grammar_tokens += 1;
        }
    }
    assert_eq!( grammar_tokens, 2, "Supposed to be 2 grammar tokens." );
    assert!(grammar, "There supposed to be grammar tokens." );
    Ok( () )
}

#[test]
fn tokenise_multi_byte_character_string() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    let mut lexer = Lexer::new( vec![ '{', '}' ], &icu_data_provider );
    let ( tokens, lengths, grammar ) =
        lexer.tokenise( "Earth = \u{1F30D}. 각" );
    let mut grammar_tokens = 0;
    assert_eq!( lengths.bytes, 20, "Supposed to be a total of 20 bytes." );
    assert_eq!( lengths.characters, 13, "Supposed to be a total of 13 characters." );
    assert_eq!( lengths.graphemes, 12, "Supposed to be a total of 12 graphemes." );
    assert_eq!( lengths.tokens, 8, "Supposed to be a total of 8 tokens." );
    for token in tokens.iter() {
        if token.token_type == TokenType::Grammar {
            grammar_tokens += 1;
        }
    }
    assert_eq!( grammar_tokens, 0, "Supposed to be 0 grammar tokens." );
    assert!(!grammar, "There supposed to be no grammar tokens." );
    Ok( () )
}
