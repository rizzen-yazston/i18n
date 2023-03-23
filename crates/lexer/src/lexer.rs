// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

// TODO: Change return of tokenise to include a boolean to indicate if there are any Grammar tokens. This to aid in
//       skipping the costly parsing and formatter functions, where a cheap alternative function can be used.
// FUTURE: Look into storing &str instead of String, perhaps original string can live long enough for Token existence.

use crate::LexerError;
use i18n_icu::IcuDataProvider;
use icu_provider::prelude::*;
use icu_properties::{ provider::{ PatternSyntaxV1Marker, PatternWhiteSpaceV1Marker } };
use icu_segmenter::provider::GraphemeClusterBreakDataV1Marker;
use icu_plurals::provider::{ CardinalV1Marker, OrdinalV1Marker };
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_datetime::provider::calendar::{
    TimeSymbolsV1Marker,
    TimeLengthsV1Marker,
    GregorianDateLengthsV1Marker,
    BuddhistDateLengthsV1Marker,
    JapaneseDateLengthsV1Marker,
    JapaneseExtendedDateLengthsV1Marker,
    CopticDateLengthsV1Marker,
    IndianDateLengthsV1Marker,
    EthiopianDateLengthsV1Marker,
    GregorianDateSymbolsV1Marker,
    BuddhistDateSymbolsV1Marker,
    JapaneseDateSymbolsV1Marker,
    JapaneseExtendedDateSymbolsV1Marker,
    CopticDateSymbolsV1Marker,
    IndianDateSymbolsV1Marker,
    EthiopianDateSymbolsV1Marker,
};
use icu_calendar::provider::{ WeekDataV1Marker, JapaneseErasV1Marker, JapaneseExtendedErasV1Marker };
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
pub struct Lexer<'a, P>
where
    P: ?Sized + DataProvider<PatternSyntaxV1Marker> + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<GraphemeClusterBreakDataV1Marker> + DataProvider<CardinalV1Marker>
        + DataProvider<OrdinalV1Marker> + DataProvider<DecimalSymbolsV1Marker> + DataProvider<TimeSymbolsV1Marker>
        + DataProvider<TimeLengthsV1Marker> + DataProvider<WeekDataV1Marker>
        + DataProvider<GregorianDateLengthsV1Marker> + DataProvider<BuddhistDateLengthsV1Marker>
        + DataProvider<JapaneseDateLengthsV1Marker> + DataProvider<JapaneseExtendedDateLengthsV1Marker>
        + DataProvider<CopticDateLengthsV1Marker> + DataProvider<IndianDateLengthsV1Marker>
        + DataProvider<EthiopianDateLengthsV1Marker> + DataProvider<GregorianDateSymbolsV1Marker>
        + DataProvider<BuddhistDateSymbolsV1Marker> + DataProvider<JapaneseDateSymbolsV1Marker>
        + DataProvider<JapaneseExtendedDateSymbolsV1Marker> + DataProvider<CopticDateSymbolsV1Marker>
        + DataProvider<IndianDateSymbolsV1Marker> + DataProvider<EthiopianDateSymbolsV1Marker>
        + DataProvider<JapaneseErasV1Marker> + DataProvider<JapaneseExtendedErasV1Marker>,
{
    data_provider: Rc<IcuDataProvider<'a, P>>,
    token_position_byte: Option<usize>,
    token_position_character: Option<usize>,
    token_position_grapheme: Option<usize>,
    position_byte: usize,
    position_character: usize,
    position_grapheme: usize,
}

impl<'a, P> Lexer<'a, P>
where
    P: ?Sized + DataProvider<PatternSyntaxV1Marker> + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<GraphemeClusterBreakDataV1Marker> + DataProvider<CardinalV1Marker>
        + DataProvider<OrdinalV1Marker> + DataProvider<DecimalSymbolsV1Marker> + DataProvider<TimeSymbolsV1Marker>
        + DataProvider<TimeLengthsV1Marker> + DataProvider<WeekDataV1Marker>
        + DataProvider<GregorianDateLengthsV1Marker> + DataProvider<BuddhistDateLengthsV1Marker>
        + DataProvider<JapaneseDateLengthsV1Marker> + DataProvider<JapaneseExtendedDateLengthsV1Marker>
        + DataProvider<CopticDateLengthsV1Marker> + DataProvider<IndianDateLengthsV1Marker>
        + DataProvider<EthiopianDateLengthsV1Marker> + DataProvider<GregorianDateSymbolsV1Marker>
        + DataProvider<BuddhistDateSymbolsV1Marker> + DataProvider<JapaneseDateSymbolsV1Marker>
        + DataProvider<JapaneseExtendedDateSymbolsV1Marker> + DataProvider<CopticDateSymbolsV1Marker>
        + DataProvider<IndianDateSymbolsV1Marker> + DataProvider<EthiopianDateSymbolsV1Marker>
        + DataProvider<JapaneseErasV1Marker> + DataProvider<JapaneseExtendedErasV1Marker>,
{

    /// Attempts to initialise the `Lexer` for tokenising a string using an ICU provider for character data.
    /// 
    /// For the parameter `data_provider`, the [`FsDataProvider`] type is usually used in creating the
    /// `IcuDataProvider`, though other providers are available to use.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_icu::IcuDataProvider;
    /// use i18n_lexer::{Token, TokenType, Lexer};
    /// use icu_testdata::buffer;
    /// use icu_provider::serde::AsDeserializingBufferProvider;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn tokenise() -> Result<(), Box<dyn Error>> {
    ///     let buffer_provider = buffer();
    ///     let data_provider = buffer_provider.as_deserializing();
    ///     let icu_data_provider = IcuDataProvider::try_new( &data_provider )?;
    ///     let mut lexer = Lexer::try_new( &Rc::new( icu_data_provider ) )?;
    ///     let tokens = lexer.tokenise(
    ///         "String contains a {placeholder}.", &vec![ '{', '}' ]
    ///     );
    ///     let mut grammar = 0;
    ///     assert_eq!( tokens.0.iter().count(), 10, "Supposed to be a total of 10 tokens." );
    ///     for token in tokens.0.iter() {
    ///         if token.token_type == TokenType::Grammar {
    ///             grammar += 1;
    ///         }
    ///     }
    ///     assert_eq!( grammar, 2, "Supposed to be 2 grammar tokens." );
    ///     Ok( () )
    /// }
    /// ```
    /// [`FsDataProvider`]: https://docs.rs/icu_provider_fs/latest/icu_provider_fs/struct.FsDataProvider.html
    pub fn try_new( data_provider: &Rc<IcuDataProvider<'a, P>> ) -> Result<Lexer<'a, P>, LexerError> {
        Ok( Lexer {
            data_provider: Rc::clone( data_provider ),
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
    /// use i18n_icu::IcuDataProvider;
    /// use i18n_lexer::{Token, TokenType, Lexer};
    /// use icu_testdata::buffer;
    /// use icu_provider::serde::AsDeserializingBufferProvider;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn tokenise() -> Result<(), Box<dyn Error>> {
    ///     let buffer_provider = buffer();
    ///     let data_provider = buffer_provider.as_deserializing();
    ///     let icu_data_provider = IcuDataProvider::try_new( &data_provider )?;
    ///     let mut lexer = Lexer::try_new( &Rc::new( icu_data_provider ) )?;
    ///     let tokens = lexer.tokenise(
    ///         "String contains a {placeholder}.", &vec![ '{', '}' ]
    ///     );
    ///     let mut grammar = 0;
    ///     assert_eq!( tokens.0.iter().count(), 10, "Supposed to be a total of 10 tokens." );
    ///     for token in tokens.0.iter() {
    ///         if token.token_type == TokenType::Grammar {
    ///             grammar += 1;
    ///         }
    ///     }
    ///     assert_eq!( grammar, 2, "Supposed to be 2 grammar tokens." );
    ///     Ok( () )
    /// }
    /// ```
    /// 
    /// [`&str`]: https://doc.rust-lang.org/core/primitive.str.html
    /// [`Vec`]: https://doc.rust-lang.org/std/vec/index.html
    /// [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
    /// [`char`]: https://doc.rust-lang.org/core/primitive.char.html
    pub fn tokenise<T: AsRef<str>>( &mut self, string: T, grammar: &Vec<char> ) -> ( Vec<Rc<Token>>, bool ) {
        let mut tokens = Vec::<Rc<Token>>::new();
        let mut has_grammar = false;
        if string.as_ref().len() == 0 {
            return ( tokens, has_grammar );
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
            if self.data_provider.pattern_white_space().as_borrowed().contains( character ) {
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
            else if self.data_provider.pattern_syntax().as_borrowed().contains( character ) {
                let state_previous = state;
                if grammar.contains( &character ) {
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
        ( tokens, has_grammar )
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
            let len_grapheme = self.data_provider.grapheme_segmenter().segment_str( slice ).count() - 1;
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
