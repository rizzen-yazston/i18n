// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

use crate::LexerError;
use icu_provider::prelude::*;
use icu_provider::serde::AsDeserializingBufferProvider;
use icu_properties::sets::{ load_pattern_white_space, load_pattern_syntax, CodePointSetData };
use icu_segmenter::GraphemeClusterSegmenter;
use std::rc::Rc;

/// String lexer and token types.
/// 
/// A simple generic token struct, which contains a string slice of the original string and information pertaining to
/// the slice. The information assigned to the string slice are:
/// - a token type;
/// - the starting position of token in terms of bytes, characters and graphemes;
/// - the length of token in terms of bytes, characters and graphemes.
#[derive( Debug )]
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
#[derive( Debug, PartialEq )]
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
    grapheme_segmenter: GraphemeClusterSegmenter,
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
    /// let mut lexer = match Lexer::try_new( &buffer_provider ) {
    ///     Err( error ) => {
    ///         println!( "{}", error );
    ///         std::process::exit( 1 )
    ///     },
    ///     Ok( result ) => result
    /// };
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
    /// [`Box`]: https://doc.rust-lang.org/std/boxed/index.html
    /// [`FsDataProvider`]: https://docs.rs/icu_provider_fs/latest/icu_provider_fs/struct.FsDataProvider.html
    pub fn try_new( buffer_provider: &Box<impl BufferProvider + ?Sized> ) -> Result<Self, LexerError> {
        let syntax = load_pattern_syntax(
            &buffer_provider.as_deserializing()
        )?;
        let white_space = load_pattern_white_space(
            &buffer_provider.as_deserializing()
        )?;
        let grapheme_segmenter = GraphemeClusterSegmenter::try_new_unstable(
            &buffer_provider.as_deserializing()
        )?;
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
    /// let mut lexer = match Lexer::try_new( &buffer_provider ) {
    ///     Err( error ) => {
    ///         println!( "{}", error );
    ///         std::process::exit( 1 )
    ///     },
    ///     Ok( result ) => result
    /// };
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
    /// [`&str`]: https://doc.rust-lang.org/core/primitive.str.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/index.html
    /// [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
    /// [`char`]: https://doc.rust-lang.org/core/primitive.char.html
    pub fn tokenise<T: AsRef<str>>( &mut self, string: T, grammar: &Vec<char> ) -> Vec<Rc<Token>> {
        let mut tokens = Vec::<Rc<Token>>::new();
        if string.as_ref().len() == 0 {
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

        let mut iterator = string.as_ref().char_indices();

        while let Some( ( position, character ) ) = iterator.next() {
            self.position_byte = position;
            if self.pattern_white_space.as_borrowed().contains( character ) {
                if state == LexerStates::Identifier {
                    self.add_previous_characters( &mut tokens, TokenType::Identifier, string.as_ref() );
                }
                else if state == LexerStates::Grammar {
                    self.add_previous_characters( &mut tokens, TokenType::Grammar, string.as_ref() );
                }
                else if state == LexerStates::Syntax {
                    self.add_previous_characters( &mut tokens, TokenType::Syntax, string.as_ref() );
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
                    self.add_previous_characters( &mut tokens, TokenType::Identifier, string.as_ref() );
                }
                else if state_previous == LexerStates::WhiteSpace {
                    self.add_previous_characters( &mut tokens, TokenType::WhiteSpace, string.as_ref() );
                }
                else {
                    if state_previous == LexerStates::Grammar {
                        self.add_previous_characters( &mut tokens, TokenType::Grammar, string.as_ref() );
                    }
                    else {
                        if state == LexerStates::Grammar {
                            self.add_previous_characters( &mut tokens, TokenType::Syntax, string.as_ref() );
                        }
                    }
                }
            }
            else {
                if state == LexerStates::WhiteSpace {
                    self.add_previous_characters( &mut tokens, TokenType::WhiteSpace, string.as_ref() );
                }
                else if state == LexerStates::Grammar {
                    self.add_previous_characters( &mut tokens, TokenType::Grammar, string.as_ref() );
                }
                else if state == LexerStates::Syntax {
                    self.add_previous_characters( &mut tokens, TokenType::Syntax, string.as_ref() );
                }
                state = LexerStates::Identifier;
            }
            self.position_character += 1;
        }

        // Complete final token
        if !self.token_position_byte.is_none() {
            self.position_byte = string.as_ref().len();
            match state {
                LexerStates::Grammar => {
                    self.add_previous_characters( &mut tokens, TokenType::Grammar, string.as_ref() );
                },
                LexerStates::Syntax => {
                    self.add_previous_characters( &mut tokens, TokenType::Syntax, string.as_ref() );
                },
                LexerStates::Identifier => {
                    self.add_previous_characters( &mut tokens, TokenType::Identifier, string.as_ref() );
                },
                LexerStates::WhiteSpace => {
                    self.add_previous_characters( &mut tokens, TokenType::WhiteSpace, string.as_ref() );
                }
            }
        }
        tokens
    }

    // Create a token for slice starting at the byte position after the previous token until current byte position.
    fn add_previous_characters<T: AsRef<str>>(
        &mut self,
        tokens: &mut Vec::<Rc<Token>>,
        token: TokenType,
        string: T,
    ) {
        if self.token_position_byte != Some( self.position_byte ) {
            let start_byte = self.token_position_byte.unwrap();
            let start_character = self.token_position_character.unwrap();
            let start_grapheme = self.token_position_grapheme.unwrap();
            let slice = &string.as_ref()[ start_byte .. self.position_byte ];
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