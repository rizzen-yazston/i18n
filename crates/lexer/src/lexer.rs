// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

#![allow(unexpected_cfgs)]

use crate::{IcuDataProvider, LexerError};

#[allow(unused_imports)]
use icu_properties::sets::CodePointSetData;

#[cfg(feature = "buffer")]
#[allow(unused_imports)]
use icu_properties::{
    provider::{PatternSyntaxV1Marker, PatternWhiteSpaceV1Marker},
    sets::{load_pattern_syntax, load_pattern_white_space},
};

#[cfg(feature = "buffer")]
#[allow(unused_imports)]
use icu_provider::AsDeserializingBufferProvider;

#[cfg(feature = "blob")]
use icu_provider_blob::BlobDataProvider;

#[cfg(feature = "fs")]
use icu_provider_fs::FsDataProvider;

use std::vec::IntoIter;
use std::{char, str};
//use core::fmt::{Display, Formatter, Result as FmtResult};
use core::iter::IntoIterator;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

#[cfg(doc)]
use std::sync::Arc;

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use icu_provider_blob::BlobDataProvider;

#[cfg(doc)]
use icu_provider_fs::FsDataProvider;

/// The available token types:
///
/// - WhiteSpace contains only white space (Pattern_White_Space ([UAX #31])) characters;
///
/// - Identifier contains only characters, that are not white space, or syntax (including grammar);
///
/// - Grammar contains a single syntax character: any of the valid grammar syntax characters;
///
/// - Syntax contains only syntax (Pattern_Syntax (UAX #31)) characters, which generally consists of punctuation and
///   control characters, and not including grammar syntax characters.
///
/// [UAX #31]: https://www.unicode.org/reports/tr31/
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    WhiteSpace,
    Identifier,
    Grammar,
    Syntax,
}

/// A simple generic token struct, which contains a string slice of the original string and information pertaining to
/// the slice. The information assigned to the string slice are:
///
/// - a token type;
///
/// - the starting position of token in terms of bytes, characters and graphemes;
///
/// - the length of token in terms of bytes, characters and graphemes.
#[derive(Debug, Clone)]
pub struct Token {
    pub string: String,           // Fragment of the original string.
    pub token_type: TokenType,    // Assigned token type.
    pub start_byte: usize,        // Start position in the original string in terms of bytes.
    pub start_character: usize,   // Start position in the original string in terms of characters.
    pub start_grapheme: usize,    // Start position in the original string in terms of graphemes.
    pub length_bytes: usize,      // Length of the slice in terms of bytes.
    pub length_characters: usize, // Length of the slice in terms of characters.
    pub length_graphemes: usize,  // Length of the slice in terms of graphemes.
    pub end_byte: usize,          // End position in original string in terms of bytes, after token.
}

/// An owned iterator for tokenising a provided string reference using the provided grammar syntax and ICU data
/// provider.
///
/// The `LexerIterator` appears to be a borrow iterator, though internally an owned iterator is used containing copy
/// of the provided string reference.
///
/// See module documentation for two examples.
pub struct LexerIterator {
    end_reached: bool,
    icu_data_provider: RefCount<IcuDataProvider>,
    iterator: IntoIter<(usize, char)>,
    grammar: Vec<char>,
    start_byte: usize,
    start_character: usize,
    length_bytes: usize,
    token_type: TokenType,
    token_string: String,
    token_start_byte: usize,
    token_start_character: usize,
    token_start_grapheme: usize,
}

impl LexerIterator {
    /// Creates an owned iterator for the provided string reference using the provided grammar syntax and ICU data
    /// provider.
    ///
    /// See module documentation for two examples.
    pub fn try_new(
        string: &str,
        grammar: &str,
        icu_data_provider: &RefCount<IcuDataProvider>,
    ) -> Result<LexerIterator, LexerError> {
        if grammar.is_empty() {
            return Err(LexerError::NoGrammar);
        }
        if string.is_empty() {
            return Err(LexerError::EmptyString);
        }
        let mut iterator = string.char_indices().collect::<Vec<_>>().into_iter();
        let mut token_string = String::new();
        let (_, character) = iterator.next().unwrap();
        let grammar = grammar.chars().collect::<Vec<_>>();
        let token_type = if icu_data_provider
            .white_space()
            .as_borrowed()
            .contains(character)
        {
            TokenType::WhiteSpace
        } else if icu_data_provider.syntax().as_borrowed().contains(character) {
            if grammar.contains(&character) {
                TokenType::Grammar
            } else {
                TokenType::Syntax
            }
        } else {
            TokenType::Identifier
        };
        token_string.push(character);
        Ok(LexerIterator {
            end_reached: false,
            icu_data_provider: RefCount::clone(icu_data_provider),
            iterator,
            grammar,
            length_bytes: string.len(),
            token_type,
            token_string,
            token_start_byte: 0,
            token_start_character: 0,
            token_start_grapheme: 0,
            start_byte: 0,
            start_character: 0,
        })
    }

    // Internal methods.

    fn token_type(&self, character: &char) -> TokenType {
        if self
            .icu_data_provider
            .white_space()
            .as_borrowed()
            .contains(*character)
        {
            TokenType::WhiteSpace
        } else if self
            .icu_data_provider
            .syntax()
            .as_borrowed()
            .contains(*character)
        {
            if self.grammar.contains(character) {
                TokenType::Grammar
            } else {
                TokenType::Syntax
            }
        } else {
            TokenType::Identifier
        }
    }

    fn token(&mut self, new_token_type: TokenType, character: char) -> Token {
        let len_graphemes = self
            .icu_data_provider
            .grapheme_segmenter()
            .segment_str(&self.token_string)
            .count()
            - 1;
        let token_type = self.token_type;
        self.token_type = new_token_type;
        let token = Token {
            string: self.token_string.clone(),
            token_type,
            start_byte: self.token_start_byte,
            start_character: self.token_start_character,
            start_grapheme: self.token_start_grapheme,
            length_bytes: self.start_byte - self.token_start_byte,
            length_characters: self.start_character - self.token_start_character,
            length_graphemes: len_graphemes,
            end_byte: self.start_byte,
        };
        self.token_start_byte = self.start_byte;
        self.token_start_character = self.start_character;
        self.token_start_grapheme += len_graphemes;
        self.token_string = character.to_string();
        token
    }
}

impl Iterator for LexerIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_reached {
            return None;
        }
        while let Some((position, character)) = self.iterator.next() {
            self.start_byte = position;
            self.start_character += 1;
            let token_type = self.token_type(&character);
            match token_type {
                TokenType::WhiteSpace => {
                    if self.token_type != TokenType::WhiteSpace {
                        return Some(self.token(TokenType::WhiteSpace, character));
                    }
                    self.token_string.push(character);
                }
                TokenType::Grammar => {
                    // Each grammar syntax character is its own token
                    return Some(self.token(TokenType::Grammar, character));
                }
                TokenType::Syntax => {
                    if self.token_type != TokenType::Syntax {
                        return Some(self.token(TokenType::Syntax, character));
                    }
                    self.token_string.push(character);
                }
                TokenType::Identifier => {
                    if self.token_type != TokenType::Identifier {
                        return Some(self.token(TokenType::Identifier, character));
                    }
                    self.token_string.push(character);
                }
            }
        }

        // Complete final token
        self.end_reached = true;
        self.start_byte = self.length_bytes;
        self.start_character += 1;
        Some(self.token(TokenType::Identifier, ' '))
    }
}
