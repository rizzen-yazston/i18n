// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! Testing `tokenise()`.

use i18n_lexer::{DataProvider, IcuDataProvider, LexerIterator, TokenType};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

use std::error::Error;

#[test]
fn tokenise_single_byte_character_string() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new(IcuDataProvider::try_new(DataProvider::Internal)?);
    let mut grammar_tokens = 0;
    let mut syntax_tokens = 0;
    let mut white_space_tokens = 0;
    let mut identifier_tokens = 0;
    let mut length_bytes = 0;
    let mut length_characters = 0;
    let mut length_graphemes = 0;
    let mut length_tokens = 0;
    for token in
        LexerIterator::try_new("String contains a {placeholder}.", "{}", &icu_data_provider)
            .unwrap()
    {
        match token.token_type {
            TokenType::Grammar => grammar_tokens += 1,
            TokenType::Syntax => syntax_tokens += 1,
            TokenType::WhiteSpace => white_space_tokens += 1,
            TokenType::Identifier => identifier_tokens += 1,
        }
        length_bytes += token.length_bytes;
        length_characters += token.length_characters;
        length_graphemes += token.length_graphemes;
        length_tokens += 1;
    }
    assert_eq!(length_bytes, 32, "Supposed to be a total of 32 bytes.");
    assert_eq!(
        length_characters, 32,
        "Supposed to be a total of 32 characters."
    );
    assert_eq!(
        length_graphemes, 32,
        "Supposed to be a total of 32 graphemes."
    );
    assert_eq!(length_tokens, 10, "Supposed to be a total of 10 tokens.");
    assert_eq!(grammar_tokens, 2, "Supposed to be 2 grammar tokens.");
    assert_eq!(syntax_tokens, 1, "Supposed to be 1 syntax token.");
    assert_eq!(
        white_space_tokens, 3,
        "Supposed to be 3 white space tokens."
    );
    assert_eq!(identifier_tokens, 4, "Supposed to be 4 identifier tokens.");
    Ok(())
}

#[test]
fn tokenise_multi_byte_character_string() -> Result<(), Box<dyn Error>> {
    let icu_data_provider = RefCount::new(IcuDataProvider::try_new(DataProvider::Internal)?);
    let mut grammar_tokens = 0;
    let mut syntax_tokens = 0;
    let mut white_space_tokens = 0;
    let mut identifier_tokens = 0;
    let mut length_bytes = 0;
    let mut length_characters = 0;
    let mut length_graphemes = 0;
    let mut length_tokens = 0;
    for token in LexerIterator::try_new("Earth = \u{1F30D}. 각", "{}", &icu_data_provider).unwrap()
    {
        match token.token_type {
            TokenType::Grammar => grammar_tokens += 1,
            TokenType::Syntax => syntax_tokens += 1,
            TokenType::WhiteSpace => white_space_tokens += 1,
            TokenType::Identifier => identifier_tokens += 1,
        }
        length_bytes += token.length_bytes;
        length_characters += token.length_characters;
        length_graphemes += token.length_graphemes;
        length_tokens += 1;
    }
    assert_eq!(length_bytes, 20, "Supposed to be a total of 20 bytes.");
    assert_eq!(
        length_characters, 13,
        "Supposed to be a total of 13 characters."
    );
    assert_eq!(
        length_graphemes, 12,
        "Supposed to be a total of 12 graphemes."
    );
    assert_eq!(length_tokens, 8, "Supposed to be a total of 8 tokens.");
    assert_eq!(grammar_tokens, 0, "Supposed to be 0 grammar tokens.");
    assert_eq!(syntax_tokens, 2, "Supposed to be 2 syntax tokens.");
    assert_eq!(
        white_space_tokens, 3,
        "Supposed to be 3 white space tokens."
    );
    assert_eq!(identifier_tokens, 3, "Supposed to be 3 identifier tokens.");
    Ok(())
}
