// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

// FUTURE: Look into storing &str instead of String, perhaps original string can live long enough for Token existence.

use i18n_icu::IcuDataProvider;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

#[cfg( doc )]
use std::sync::Arc;

#[cfg( doc )]
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
/// - WhiteSpace contains only white space (Pattern_White_Space ([UAX #31])) characters;
/// - Identifier contains only characters, that are not white space or grammar;
/// - Grammar contains a single syntax character: any of the valid grammar syntax characters;
/// - Syntax contains only syntax (Pattern_Syntax (UAX #31)) characters, which generally consists of punctuation and
/// control characters, and not including grammar syntax characters.
/// 
/// [UAX #31]: https://www.unicode.org/reports/tr31/
#[derive( Debug, PartialEq )]
pub enum TokenType {
    WhiteSpace, // Just Pattern_White_Space characters (UAX #31).
    Identifier, // Excludes Pattern_Syntax and Pattern_White_Space characters (UAX #31), typically words.
    Grammar, // Contains a single grammar character.
    Syntax, // All other Pattern_Syntax characters (UAX #31), excluding listed grammar syntax characters.
}

/// Just a simple struct containing the string length in terms of bytes, characters, graphemes, and tokens.
pub struct Length {
    pub bytes: usize,
    pub characters: usize,
    pub graphemes: usize,
    pub tokens: usize,
}

/// A reusable lexer that tracks the current position in the string being tokenised, besides holding the length of
/// the string in terms of bytes, characters, graphemes, and tokens. Also holds the data provider, that contains 
/// locale independent data such as Pattern_Syntax and Pattern_White_Space properties, and the Grapheme Cluster
/// Segmenter.
pub struct Lexer{
    data_provider: RefCount<IcuDataProvider>,
    grammar: Vec<char>,
    length_bytes: usize,
    length_characters: usize,
    length_graphemes: usize,
    length_tokens: usize,
    token_position_byte: usize,
    token_position_character: usize,
    token_position_grapheme: usize,
    position_byte: usize,
    position_character: usize,
}

impl Lexer {

    /// Create a `Lexer` instance initiated with a `&`[`Rc`]`<`[`IcuDataProvider`]`>` or `&`[`Arc`]`<IcuDataProvider>`
    /// and grammar syntax vector.
    /// 
    /// The `grammar` parameter contain of a simple [`Vec`]`<`[`char`]`>` containing all the characters that are used
    /// as grammar syntax characters within a parser. Each grammar syntax character is placed in its own `Token` of
    /// type `Grammar`.
    /// 
    /// Note: Only single character graphemes are supported for grammar syntax characters. 
    /// 
    pub fn new( grammar: Vec<char>, data_provider: &RefCount<IcuDataProvider>, ) -> Self {
        Lexer {
            data_provider: RefCount::clone( data_provider ),
            grammar,
            length_bytes: 0,
            length_characters: 0,
            length_graphemes: 0,
            length_tokens: 0,
            token_position_byte: 0,
            token_position_character: 0,
            token_position_grapheme: 0,
            position_byte: 0,
            position_character: 0,
        }       
    }

    /// Tokenise a string (as [`&str`]) into a vector of tokens ([`Vec`]`<`[`Rc`]`<Token>>` or
    /// `Vec<`[`Arc`]`<Token>>`). The lexer is reset before tokenising the string.
    /// 
    /// Non-grammar syntax characters are simply made into `Syntax` tokens for the parser to handle.
    /// 
    /// No characters are discarded, thus every character belongs to a token. Allowing for the full reconstruction of
    /// the original string, that was tokenised.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_icu::{ IcuDataProvider, DataProvider };
    /// use i18n_lexer::{Token, TokenType, Lexer};
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn test_tokenise() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let mut lexer = Lexer::new( vec![ '{', '}' ], &icu_data_provider );
    ///     let ( tokens, lengths, grammar ) =
    ///         lexer.tokenise( "String contains a {placeholder}." );
    ///     let mut grammar_tokens = 0;
    ///     assert_eq!( lengths.bytes, 32, "Supposed to be a total of 32 bytes." );
    ///     assert_eq!( lengths.characters, 32, "Supposed to be a total of 32 characters." );
    ///     assert_eq!( lengths.graphemes, 32, "Supposed to be a total of 32 graphemes." );
    ///     assert_eq!( lengths.tokens, 10, "Supposed to be a total of 10 tokens." );
    ///     for token in tokens.iter() {
    ///     if token.token_type == TokenType::Grammar {
    ///          grammar_tokens += 1;
    ///         }
    ///     }
    ///     assert_eq!( grammar_tokens, 2, "Supposed to be 2 grammar tokens." );
    ///     assert!(grammar, "There supposed to be grammar tokens." );
    ///     Ok( () )
    /// }
    /// ```
    /// [`&str`]: core::str
    pub fn tokenise<T: AsRef<str>>( &mut self, string: T ) -> ( Vec<RefCount<Token>>, Length, bool ) {
        let mut tokens = Vec::<RefCount<Token>>::new();
        let mut has_grammar = false;
        self.length_bytes = 0;
        self.length_characters = 0;
        self.length_graphemes = 0;
        self.length_tokens = 0;
        self.token_position_byte = 0;
        self.token_position_character = 0;
        self.token_position_grapheme = 0;
        self.position_byte = 0;
        self.position_character = 0;

        if string.as_ref().len() == 0 {
            return ( tokens, Length {
                bytes: self.length_bytes,
                characters: self.length_characters,
                graphemes: self.length_graphemes,
                tokens: self.length_tokens,
            }, has_grammar );
        }
        let mut state = LexerStates::Identifier; // Most strings would begin with an alphabet letter.
        let mut iterator = string.as_ref().char_indices();
        while let Some( ( position, character ) ) = iterator.next() {
            self.position_byte = position;
            if self.data_provider.white_space().as_borrowed().contains( character ) {
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
            else if self.data_provider.syntax().as_borrowed().contains( character ) {
                let state_previous = state;
                if self.grammar.contains( &character ) {
                    state = LexerStates::Grammar;
                    has_grammar = true;
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
        ( tokens, Length {
            bytes: self.length_bytes,
            characters: self.length_characters,
            graphemes: self.length_graphemes,
            tokens: self.length_tokens,
        }, has_grammar )
    }

    // Create a token for slice starting at the byte position after the previous token until current byte position.
    fn add_previous_characters<T: AsRef<str>>(
        &mut self,
        tokens: &mut Vec::<RefCount<Token>>,
        token: TokenType,
        string: T,
    ) {
        if self.token_position_byte != self.position_byte {
            let slice = &string.as_ref()[ self.token_position_byte .. self.position_byte ];
            let len_bytes = self.position_byte - self.token_position_byte;
            let len_characters = self.position_character - self.token_position_character;
            let len_graphemes = self.data_provider.grapheme_segmenter().segment_str( slice ).count() - 1;
            tokens.push( RefCount::new(
                Token {
                    token_type: token,
                    string: slice.to_string(),
                    position_byte: self.token_position_byte,
                    position_character: self.token_position_character,
                    position_grapheme: self.token_position_grapheme,
                    length_bytes: len_bytes,
                    length_characters: len_characters,
                    length_graphemes: len_graphemes,
                }
            ) );
            self.token_position_byte += len_bytes;
            self.token_position_character += len_characters;
            self.token_position_grapheme += len_graphemes;
            self.length_bytes += len_bytes;
            self.length_characters += len_characters;
            self.length_graphemes += len_graphemes;
            self.length_tokens += 1;
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
