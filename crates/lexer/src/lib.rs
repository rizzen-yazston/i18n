// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! String lexer and resultant tokens.
//! 
//! The `Lexer` is initialised using a data provider [`BufferProvider`] to an [Unicode Consortium] [CLDR] data
//! repository, usually it is just a local copy of the CLDR in the application's data directory. Once the `Lexer` has
//! been initialised it may be used to tokenise strings, without needing to re-initialising the `Lexer` before use.
//! Consult the [ICU4X] website for instructions on generating a suitable data repository for the application, by
//! leaving out data that is not used by the application. 
//! 
//! Strings are tokenised using the method `tokenise()` taking string slice and a vector containing grammar syntax
//! characters.
//! 
//! # Examples
//! 
//! ```
//! use icu_provider::prelude::*;
//! use std::rc::Rc;
//! use i18n_lexer::{Token, TokenType, Lexer};
//! use icu_testdata::buffer;
//! 
//! let buffer_provider = Box::new( buffer() );
//! let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
//! let tokens = lexer.tokenise(
//!     "String contains a {placeholder}.", &vec![ '{', '}' ]
//! );
//! let mut grammar = 0;
//! assert_eq!( tokens.iter().count(), 10, "Supposed to be a total of 10 tokens." );
//! for token in tokens.iter() {
//!     if token.token_type == TokenType::Grammar {
//!         grammar += 1;
//!     }
//! }
//! assert_eq!( grammar, 2, "Supposed to be 2 grammar tokens." );
//! ```
//! 
//! [`BufferProvider`]: https://docs.rs/icu_provider/latest/icu_provider/buf/trait.BufferProvider.html
//! [Unicode Consortium]: https://home.unicode.org/
//! [CLDR]: https://cldr.unicode.org/
//! [ICU4X]: https://github.com/unicode-org/icu4x

use icu_provider::prelude::*;
use icu_provider::serde::AsDeserializingBufferProvider;
use icu_properties::sets::{load_pattern_white_space, load_pattern_syntax, CodePointSetData};
use icu_segmenter::GraphemeClusterBreakSegmenter;
use std::rc::Rc;

/// String lexer and token types.
/// 
/// A simple generic token struct, which contains a string slice of the original string and information pertaining to
/// the slice. The information assigned to the string slice are:
/// - a token type;
/// - the starting position of token in terms of bytes, characters and graphemes;
/// - the length of token in terms of bytes, characters and graphemes.
pub struct Token {
    pub token_type: TokenType, // Assigned token type.
    pub string: String, // Owned string slice of the original string.
    pub position_byte: usize, // Start position in the original string in terms of bytes.
    pub position_character: usize, // Start position in the original string in terms of characters.
    pub position_grapheme: usize, // Start position in the original string in terms of graphemes.
    pub length_bytes: usize, // Length of the slice in terms of bytes.
    pub length_characters: usize, // Length of the slice in terms of characters.
    pub length_graphemes: usize, // Length of the slice in terms of graphemes.
}

/// The available token types:
/// - WhiteSpace contains only white space characters;
/// - Identifier contains only characters, that are not white space or syntax;
/// - Grammar contains a single syntax character: any of the valid grammar syntax characters;
/// - Syntax contains only syntax characters, which generally consists of punctuation and control characters.
#[derive( PartialEq )]
pub enum TokenType {
    WhiteSpace, // Just Pattern_White_Space characters (UAX #31).
    Identifier, // Excludes Pattern_Syntax and Pattern_White_Space characters (UAX #31), typically words.
    Grammar, // Contains a single grammar character.
    Syntax, // All other Pattern_Syntax characters (UAX #31), excluding listed grammar syntax characters.
}

/// The `Lexer` tokenises the provided string into a vector of `Token`s.
/// 
/// No character is removed during the tokenising process, thus the original string can be reconstructed just from the
/// tokens. Editors and formatting tools can reformat source files, usually just altering the white space, in order for
/// the source file to adhere to formatting specifications.
/// 
/// Identifiers are determined according to the
/// [Unicode Standard Annex #31: Unicode Identifier and Pattern Syntax](https://unicode.org/reports/tr31/).
/// 
/// White space and syntax characters are identified according to the character properties `Pattern_Syntax` and
/// `Pattern_White_Space` as defined in the 
/// [Unicode Standard Annex #44: Unicode Character Database](https://www.unicode.org/reports/tr44/).
pub struct Lexer {
    pattern_syntax: CodePointSetData,
    pattern_white_space: CodePointSetData,
    grapheme_segmenter: GraphemeClusterBreakSegmenter,
    token_position_byte: Option<usize>,
    token_position_character: Option<usize>,
    token_position_grapheme: Option<usize>,
    position_byte: usize,
    position_character: usize,
    position_grapheme: usize,
}

impl Lexer {

    /// Attempts to initialise the `Lexer` for tokenising a string using an ICU provider for character data.
    /// 
    /// For the parameter `buffer_provider`, a [`Box`] reference to a [`FsDataProvider`] type is usually used,
    /// though other providers are available to use.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::rc::Rc;
    /// use i18n_lexer::{Token, TokenType, Lexer};
    /// use icu_testdata::buffer;
    /// 
    /// let buffer_provider = Box::new( buffer() );
    /// let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
    /// let tokens = lexer.tokenise(
    ///     // World Map (U+1F5FA) is encoded in four bytes in UTF-8.
    ///     "String contains a {placeholder}.", &vec![ '{', '}' ]
    /// );
    /// let mut grammar = 0;
    /// assert_eq!( tokens.iter().count(), 10, "Supposed to be a total of 10 tokens." );
    /// for token in tokens.iter() {
    ///     if token.token_type == TokenType::Grammar {
    ///         grammar += 1;
    ///     }
    /// }
    /// assert_eq!( grammar, 2, "Supposed to be 2 grammar tokens." );
    /// ```
    /// [`Box`]: https://doc.rust-lang.org/nightly/std/boxed/struct.Box.html
    /// [`FsDataProvider`]: https://docs.rs/icu_provider_fs/latest/icu_provider_fs/struct.FsDataProvider.html
    pub fn try_new( buffer_provider: &Box<impl BufferProvider + ?Sized> ) -> Result<Self, String> {
        let syntax = match load_pattern_syntax(
            &buffer_provider.as_deserializing()
        ) {
            Err( _ ) => return Err( "Failed to load Pattern_Syntax.".to_string() ),
            Ok( response ) => response
        };
        let white_space = match load_pattern_white_space(
            &buffer_provider.as_deserializing()
        ) {
            Err( _ ) => return Err( "Failed to load Pattern_White_Space.".to_string() ),
            Ok( response ) => response
        };
        let grapheme_segmenter = match GraphemeClusterBreakSegmenter::try_new_unstable(
            &buffer_provider.as_deserializing()
        ) {
            Err(_) => return Err( "Failed to get grapheme segmenter.".to_string() ),
            Ok( segmenter ) => segmenter
        };
        Ok( Lexer {
            pattern_syntax: syntax,
            pattern_white_space: white_space,
            grapheme_segmenter: grapheme_segmenter,
            token_position_byte: None,
            token_position_character: None,
            token_position_grapheme: None,
            position_byte: 0,
            position_character: 0,
            position_grapheme: 0,
        } )
    }

    /// Tokenise a string (as [`&str`]) into a vector of tokens ([`Vec`]`<`[`Rc`]`<Token>>`).
    /// 
    /// Non-grammar syntax characters are simply made into `Syntax` tokens for the parser to handle.
    /// 
    /// No characters are discarded, thus every character belongs to a token. Allowing for the full reconstruction of
    /// the original string, that was tokenised.
    /// 
    /// The `grammar` parameter contain of a simple vector of [`char`]s containing all the characters that are used as
    /// grammar syntax characters within a parser. Each grammar syntax character is placed in its own `Token` of type
    /// `Grammar`.
    /// 
    /// Note: Only single character graphemes are supported for grammar syntax characters. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::rc::Rc;
    /// use i18n_lexer::{Token, TokenType, Lexer};
    /// use icu_testdata::buffer;
    /// 
    /// let buffer_provider = Box::new( buffer() );
    /// let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
    /// let tokens = lexer.tokenise(
    ///     "String contains a {placeholder}.", &vec![ '{', '}' ]
    /// );
    /// let mut grammar = 0;
    /// assert_eq!( tokens.iter().count(), 10, "Supposed to be a total of 10 tokens." );
    /// for token in tokens.iter() {
    ///     if token.token_type == TokenType::Grammar {
    ///         grammar += 1;
    ///     }
    /// }
    /// assert_eq!( grammar, 2, "Supposed to be 2 grammar tokens." );
    /// ```
    /// 
    /// [`&str`]: https://doc.rust-lang.org/nightly/std/primitive.str.html
    /// [`Vec`]: https://doc.rust-lang.org/nightly/std/vec/index.html
    /// [`Rc`]: https://doc.rust-lang.org/nightly/std/rc/struct.Rc.html
    /// [`char`]: https://doc.rust-lang.org/nightly/std/primitive.char.html
    pub fn tokenise<'a>( &'a mut self, string: &'a str, grammar: &Vec<char> ) -> Vec<Rc<Token>> {
        let mut tokens = Vec::<Rc<Token>>::new();
        if string.len() == 0 {
            return tokens;
        }

        // Resets the Lexer
        self.position_byte = 0;
        self.position_character = 0;
        self.position_grapheme = 0;
        self.token_position_byte = Some( self.position_byte );
        self.token_position_character = Some( self.position_character );
        self.token_position_grapheme = Some( self.position_grapheme );

        let mut state = LexerStates::Identifier;

        let mut iterator = string.char_indices();

        while let Some( ( position, character ) ) = iterator.next() {
            self.position_byte = position;
            if self.pattern_white_space.as_borrowed().contains( character ) {
                if state == LexerStates::Identifier {
                    self.add_previous_characters( &mut tokens, TokenType::Identifier, string );
                }
                else if state == LexerStates::Grammar {
                    self.add_previous_characters( &mut tokens, TokenType::Grammar, string );
                }
                else if state == LexerStates::Syntax {
                    self.add_previous_characters( &mut tokens, TokenType::Syntax, string );
                }
                state = LexerStates::WhiteSpace;
            }
            else if self.pattern_syntax.as_borrowed().contains( character ) {
                let state_previous = state;
                if grammar.contains( &character ) {
                    state = LexerStates::Grammar;
                }
                else {
                    state = LexerStates::Syntax;
                }
                if state_previous == LexerStates::Identifier {
                    self.add_previous_characters( &mut tokens, TokenType::Identifier, string );
                }
                else if state_previous == LexerStates::WhiteSpace {
                    self.add_previous_characters( &mut tokens, TokenType::WhiteSpace, string );
                }
                else {
                    if state_previous == LexerStates::Grammar {
                        self.add_previous_characters( &mut tokens, TokenType::Grammar, string );
                    }
                    else {
                        if state == LexerStates::Grammar {
                            self.add_previous_characters( &mut tokens, TokenType::Syntax, string );
                        }
                    }
                }
            }
            else {
                if state == LexerStates::WhiteSpace {
                    self.add_previous_characters( &mut tokens, TokenType::WhiteSpace, string );
                }
                else if state == LexerStates::Grammar {
                    self.add_previous_characters( &mut tokens, TokenType::Grammar, string );
                }
                else if state == LexerStates::Syntax {
                    self.add_previous_characters( &mut tokens, TokenType::Syntax, string );
                }
                state = LexerStates::Identifier;
            }
            self.position_character += 1;
        }

        // Complete final token
        if !self.token_position_byte.is_none() {
            self.position_byte = string.len();
            match state {
                LexerStates::Grammar => {
                    self.add_previous_characters( &mut tokens, TokenType::Grammar, string );
                },
                LexerStates::Syntax => {
                    self.add_previous_characters( &mut tokens, TokenType::Syntax, string );
                },
                LexerStates::Identifier => {
                    self.add_previous_characters( &mut tokens, TokenType::Identifier, string );
                },
                LexerStates::WhiteSpace => {
                    self.add_previous_characters( &mut tokens, TokenType::WhiteSpace, string );
                }
            }
        }
        tokens
    }

    // Create a token for slice starting at the byte position after the previous token until current byte position.
    fn add_previous_characters<'t, 'a>(
        &mut self,
        tokens: &mut Vec::<Rc<Token>>,
        token: TokenType,
        string: &'t str,
    ) {
        if self.token_position_byte != Some( self.position_byte ) {
            let start_byte = self.token_position_byte.unwrap();
            let start_character = self.token_position_character.unwrap();
            let start_grapheme = self.token_position_grapheme.unwrap();
            let slice = &string[ start_byte .. self.position_byte ];
            let len_byte = self.position_character - start_character;
            let len_character = self.position_character - start_character;
            let len_grapheme = self.grapheme_segmenter.segment_str( slice ).count() - 1;
            self.position_grapheme += len_grapheme;
            tokens.push( Rc::new(
                Token {
                    token_type: token,
                    string: slice.to_string(),
                    position_byte: start_byte,
                    position_character: start_character,
                    position_grapheme: start_grapheme,
                    length_bytes: len_byte,
                    length_characters: len_character,
                    length_graphemes: len_grapheme,
                }
            ) );
            self.token_position_byte = Some( start_byte + len_byte );
            self.token_position_character = Some( start_character + len_character );
            self.token_position_grapheme = Some( start_grapheme + len_grapheme );
        }
    }
}

// Internal structures, enums, etc.

// Various states the `Lexer` may be in.
#[derive( PartialEq )]
enum LexerStates {
    Identifier, // Ends with either white space or syntax character.
    WhiteSpace, // Ends with non-white space.
    Grammar, // A grammar syntax character.
    Syntax, // Any other syntax character.
}


#[cfg(test)]
mod tests {
    use super::*;
    use icu_testdata::buffer;

    #[test]
    fn tokenise() {
        let buffer_provider = Box::new( buffer() );
        let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
        let tokens = lexer.tokenise(
            // World Map (U+1F5FA) is encoded in four bytes in UTF-8.
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
    }
}

