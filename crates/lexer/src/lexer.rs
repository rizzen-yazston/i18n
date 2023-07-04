// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

// FUTURE: Look into storing &str instead of String, perhaps original string can live long enough for Token existence.

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
/// use i18n_lexer::{Token, TokenType, tokenise};
/// use icu_testdata::buffer;
/// use icu_provider::serde::AsDeserializingBufferProvider;
/// use std::rc::Rc;
/// use std::error::Error;
/// 
/// fn test_tokenise() -> Result<(), Box<dyn Error>> {
///     let buffer_provider = buffer();
///     let data_provider = buffer_provider.as_deserializing();
///     let icu_data_provider = IcuDataProvider::try_new( &data_provider )?;
///     let tokens = tokenise(
///         "String contains a {placeholder}.",
///         &vec![ '{', '}' ],
///         &Rc::new( icu_data_provider ),
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
/// [`&str`]: core::str
/// [`Vec`]: std::vec::Vec
/// [`Rc`]: std::rc::Rc
/// [`char`]: core::char
pub fn tokenise<'a,
    T: AsRef<str>,
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
        + DataProvider<JapaneseErasV1Marker> + DataProvider<JapaneseExtendedErasV1Marker>
>( string: T, grammar: &Vec<char>, data_provider: &Rc<IcuDataProvider<'a, P>>, ) -> ( Vec<Rc<Token>>, bool ) {
    let mut tokens = Vec::<Rc<Token>>::new();
    let mut has_grammar = false;
    if string.as_ref().len() == 0 {
        return ( tokens, has_grammar );
    }
    let mut lexer = Lexer {
        data_provider: Rc::clone( data_provider ),
        position_byte: 0,
        position_character: 0,
        token_position_byte: 0,
        token_position_character: 0,
        token_position_grapheme: 0,
    };
    let mut state = LexerStates::Identifier; // Most strings would begin with an alphabet letter.
    let mut iterator = string.as_ref().char_indices();
    while let Some( ( position, character ) ) = iterator.next() {
        lexer.position_byte = position;
        if lexer.data_provider.pattern_white_space().as_borrowed().contains( character ) {
            if state == LexerStates::Identifier {
                add_previous_characters( &mut lexer, &mut tokens, TokenType::Identifier, string.as_ref() );
            }
            else if state == LexerStates::Grammar {
                add_previous_characters( &mut lexer, &mut tokens, TokenType::Grammar, string.as_ref() );
            }
            else if state == LexerStates::Syntax {
                add_previous_characters( &mut lexer, &mut tokens, TokenType::Syntax, string.as_ref() );
            }
            state = LexerStates::WhiteSpace;
        }
        else if lexer.data_provider.pattern_syntax().as_borrowed().contains( character ) {
            let state_previous = state;
            if grammar.contains( &character ) {
                state = LexerStates::Grammar;
                has_grammar = true;
            }
            else {
                state = LexerStates::Syntax;
            }
            if state_previous == LexerStates::Identifier {
                add_previous_characters( &mut lexer, &mut tokens, TokenType::Identifier, string.as_ref() );
            }
            else if state_previous == LexerStates::WhiteSpace {
                add_previous_characters( &mut lexer, &mut tokens, TokenType::WhiteSpace, string.as_ref() );
            }
            else {
                if state_previous == LexerStates::Grammar {
                    add_previous_characters( &mut lexer, &mut tokens, TokenType::Grammar, string.as_ref() );
                }
                else {
                    if state == LexerStates::Grammar {
                        add_previous_characters( &mut lexer, &mut tokens, TokenType::Syntax, string.as_ref() );
                    }
                }
            }
        }
        else {
            if state == LexerStates::WhiteSpace {
                add_previous_characters( &mut lexer, &mut tokens, TokenType::WhiteSpace, string.as_ref() );
            }
            else if state == LexerStates::Grammar {
                add_previous_characters( &mut lexer, &mut tokens, TokenType::Grammar, string.as_ref() );
            }
            else if state == LexerStates::Syntax {
                add_previous_characters( &mut lexer, &mut tokens, TokenType::Syntax, string.as_ref() );
            }
            state = LexerStates::Identifier;
        }
        lexer.position_character += 1;
    }

    // Complete final token
    lexer.position_byte = string.as_ref().len();
    match state {
        LexerStates::Grammar => {
            add_previous_characters( &mut lexer, &mut tokens, TokenType::Grammar, string.as_ref() );
        },
        LexerStates::Syntax => {
            add_previous_characters( &mut lexer, &mut tokens, TokenType::Syntax, string.as_ref() );
        },
        LexerStates::Identifier => {
            add_previous_characters( &mut lexer, &mut tokens, TokenType::Identifier, string.as_ref() );
        },
        LexerStates::WhiteSpace => {
            add_previous_characters( &mut lexer, &mut tokens, TokenType::WhiteSpace, string.as_ref() );
        }
    }
    ( tokens, has_grammar )
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

struct Lexer<'a, P>
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
    token_position_byte: usize,
    token_position_character: usize,
    token_position_grapheme: usize,
    position_byte: usize,
    position_character: usize,
}

// Create a token for slice starting at the byte position after the previous token until current byte position.
fn add_previous_characters<'a,
T: AsRef<str>,
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
    + DataProvider<JapaneseErasV1Marker> + DataProvider<JapaneseExtendedErasV1Marker>
>(
    lexer: &mut Lexer<'a, P>,
    tokens: &mut Vec::<Rc<Token>>,
    token: TokenType,
    string: T,
) {
    if lexer.token_position_byte != lexer.position_byte {
        let slice = &string.as_ref()[ lexer.token_position_byte .. lexer.position_byte ];
        let len_byte = lexer.position_byte - lexer.token_position_byte;
        let len_character = lexer.position_character - lexer.token_position_character;
        let len_grapheme = lexer.data_provider.grapheme_segmenter().segment_str( slice ).count() - 1;
        tokens.push( Rc::new(
            Token {
                token_type: token,
                string: slice.to_string(),
                position_byte: lexer.token_position_byte,
                position_character: lexer.token_position_character,
                position_grapheme: lexer.token_position_grapheme,
                length_bytes: len_byte,
                length_characters: len_character,
                length_graphemes: len_grapheme,
            }
        ) );
        lexer.token_position_byte += len_byte;
        lexer.token_position_character += len_character;
        lexer.token_position_grapheme += len_grapheme;
    }
}
