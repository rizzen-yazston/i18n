// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

use crate::{ NodeType };
use i18n_lexer::{ Token };
use icu_locid::ParserError as IcuParserError;
use icu_calendar::CalendarError;
use icu_decimal::Error as DecimalError;
use icu_datetime::DateTimeError;
use icu_plurals::Error as PluralError;
use fixed_decimal::Error as FixedDecimalError;
use std::num::ParseIntError;
use std::rc::Rc;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug )]
#[non_exhaustive]
pub enum ParserError {
    EndedAbruptly,
    UniqueNamed( String ),
    InvalidToken( usize, Rc<Token> ),
    MultiNumberSign( usize ),
    UniquePattern( String ),
}

impl Error for ParserError {}

impl Display for ParserError {

    /// Write to the formatter the default preformatted error message.
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            ParserError::EndedAbruptly => write!( formatter, "String ended abruptly." ),
            ParserError::UniqueNamed( identifier) =>
                write!( formatter, "Named substrings must have unique identifiers. ‘{}’ already exists.", identifier ),
            ParserError::InvalidToken( position, token ) =>
                write!( formatter, "Invalid token ‘{}’ found at position {} of the string.", token.string, position ),
            ParserError::MultiNumberSign( position ) =>
                write!( formatter, "Found sequential number signs at ‘{}’ of the string.", position ),
            ParserError::UniquePattern( identifier) =>
                write!( formatter, "Pattern identifiers must have unique. ‘{}’ already exists.", identifier ),
        }
    }
}

#[derive( Debug )]
#[non_exhaustive]
pub enum FormatterError {
    InvalidRoot,
    RetrieveChildren( NodeType ),
    NodeNotFound( NodeType ),
    FirstChild( NodeType ),
    RetrieveNodeData( NodeType ),
    RetrieveNodeToken( NodeType ),
    LastChild( NodeType ),
    InvalidNode( NodeType ),
    PatternNamed( String ),
    PatternPart( String, usize ),
    InvalidOptionValue( String, String, String ),
    InvalidKeyword( String, String ),
    SelectorNamed( String, String, String ),
    SelectorOther( String, String ),
    NoChildren( NodeType ),
    InvalidOption( String, String, String ),
    InvalidSelector( String, String, String ),
    Locale( IcuParserError ),
    Calendar( CalendarError ),
    ParseInt( ParseIntError ),
    NumberSignString( usize ),
    SelectorsIndex( usize ),
    SelectorsIndexNamed( String, usize ),
    PlaceholderValue( String ),
    InvalidValue( String ),
    Decimal( DecimalError ),
    DateTime( DateTimeError ),
    PluralRules( PluralError ),
    FixedDecimal( FixedDecimalError ),
    NamedStringIdentifier( String ),
}

impl Error for FormatterError {}

impl Display for FormatterError {

    /// Write to the formatter the default preformatted error message.
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            FormatterError::InvalidRoot => write!( formatter, "Tree root must be a ‘Root’ node." ),
            FormatterError::RetrieveChildren( node_type ) =>
                write!( formatter, "Failed to retrieve children for ‘{}’ node.", node_type ),
            FormatterError::NodeNotFound( node_type ) =>
                write!( formatter, "Expected node ‘{}’ was not found.", node_type ),
            FormatterError::FirstChild( node_type ) =>
                write!( formatter, "First child of ‘{}’ node not found.", node_type ),
            FormatterError::RetrieveNodeData( node_type ) =>
                write!( formatter, "Failed to retrieve data for ‘{}’ node.", node_type ),
            FormatterError::RetrieveNodeToken( node_type ) =>
                write!( formatter, "Failed to retrieve token for ‘{}’ node.", node_type ),
            FormatterError::LastChild( node_type ) =>
                write!( formatter, "Last child of ‘{}’ node not found.", node_type ),
            FormatterError::InvalidNode( node_type ) =>
                write!( formatter, "Invalid child node found in ‘{}’ node.", node_type ),
            FormatterError::PatternNamed( identifier ) =>
                write!( formatter, "Failed to retrieve pattern for named string ‘{}’.", identifier ),
            FormatterError::PatternPart( identifier, index ) =>
                write!(
                    formatter,
                    "Failed to retrieve part ‘{}’ of pattern for named string ‘{}’.",
                    identifier, index
                ),
            FormatterError::InvalidOptionValue( value, option, keyword ) =>
                write!(
                    formatter,
                    "Value ‘{}’ is invalid for option ‘{}’ for keyword ‘{}’.",
                    value, option, keyword
                ),
            FormatterError::InvalidKeyword( keyword, placeholder ) =>
                write!( formatter, "Invalid keyword ‘{}’ for placeholder ‘{}’.", keyword, placeholder ),
            FormatterError::SelectorNamed( named, selector, identifier ) =>
                write!(
                    formatter,
                    "Named string identifier ‘{}’ is not found for selector ‘{}’ of placeholder ‘{}’.",
                    named, selector, identifier
                ),
            FormatterError::SelectorOther( keyword, placeholder ) =>
                write!(
                    formatter,
                    "The required ‘other’ selector was not found for keyword ‘{}’ of placeholder ‘{}’.",
                    keyword, placeholder
                ),
            FormatterError::NoChildren( node_type) =>
                write!( formatter, "No children nodes was not found for ‘{}’ node.", node_type ),
            FormatterError::InvalidOption( option, keyword, placeholder ) =>
                write!(
                    formatter,
                    "Invalid for option ‘{}’ for keyword ‘{}’ of placeholder ‘{}’.",
                    option, keyword, placeholder
                ),
            FormatterError::InvalidSelector( option, keyword, placeholder ) =>
                write!(
                    formatter,
                    "Invalid for selector ‘{}’ for keyword ‘{}’ of placeholder ‘{}’.",
                    option, keyword, placeholder
                ),
            FormatterError::Locale( ref error ) => error.fmt( formatter ),
            FormatterError::Calendar( ref error ) => error.fmt( formatter ),
            FormatterError::ParseInt( ref error ) => error.fmt( formatter ),
            FormatterError::NumberSignString( index ) =>
                write!( formatter, "Unable to retrieve formatted string for NumberSign index {}.", index ),
            FormatterError::SelectorsIndex( index ) =>
                write!( formatter, "Index {} is not found in collected selectors.", index ),
            FormatterError::SelectorsIndexNamed( identifier, index ) =>
                write!(
                    formatter,
                    "Failed to retrieve string for named string ‘{}’ of the selectors index {}.",
                    identifier, index
                ),
            FormatterError::PlaceholderValue( part ) =>
                write!( formatter, "Placeholder value is not found for pattern part ‘{}’.", part ),
            FormatterError::InvalidValue( part ) =>
                write!( formatter, "Invalid value type provided for pattern part ‘{}’.", part ),
            FormatterError::Decimal( ref error ) => error.fmt( formatter ),
            FormatterError::DateTime( ref error ) => error.fmt( formatter ),
            FormatterError::PluralRules( ref error ) => error.fmt( formatter ),
            FormatterError::FixedDecimal( ref error ) => error.fmt( formatter ),
            FormatterError::NamedStringIdentifier( identifier ) =>
                write!(
                    formatter,
                    "Named string identifier ‘{}’ already exists. Identifiers must be unique and not ‘_’.",
                    identifier
                ),
        }
    }
}

impl From<IcuParserError> for FormatterError {
    fn from( error: IcuParserError ) -> FormatterError {
        FormatterError::Locale( error )
    }
}

impl From<CalendarError> for FormatterError {
    fn from( error: CalendarError ) -> FormatterError {
        FormatterError::Calendar( error )
    }
}

impl From<ParseIntError> for FormatterError {
    fn from( error: ParseIntError ) -> FormatterError {
        FormatterError::ParseInt( error )
    }
}

impl From<DecimalError> for FormatterError {
    fn from( error: DecimalError ) -> FormatterError {
        FormatterError::Decimal( error )
    }
}

impl From<DateTimeError> for FormatterError {
    fn from( error: DateTimeError ) -> FormatterError {
        FormatterError::DateTime( error )
    }
}

impl From<PluralError> for FormatterError {
    fn from( error: PluralError ) -> FormatterError {
        FormatterError::PluralRules( error )
    }
}

impl From<FixedDecimalError> for FormatterError {
    fn from( error: FixedDecimalError ) -> FormatterError {
        FormatterError::FixedDecimal( error )
    }
}
