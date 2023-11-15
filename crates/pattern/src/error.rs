// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

use crate::NodeType;
use i18n_lexer::Token;
use icu_locid::ParserError as IcuParserError;
use icu_calendar::CalendarError;
use icu_decimal::Error as DecimalError;
use icu_datetime::DateTimeError;
use icu_plurals::Error as PluralError;
use fixed_decimal::Error as FixedDecimalError;
use std::num::ParseIntError;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

/// The `ParserError` type consists of the follow:
/// 
/// * `EndedAbruptly`: Indicates the string ended abruptly,
/// 
/// * `UniqueNamed`: Indicates named substring identifiers are not unique,
/// 
/// * `InvalidToken`: Indicates the token is in an invalid position in the string,
/// 
/// * `MultiNumberSign`: Indicates sequential number signs was found.
/// 
/// * `UniquePattern`: Indicates pattern identifiers are not unique.
#[derive( Debug )]
#[non_exhaustive]
pub enum ParserError {
    EndedAbruptly,
    UniqueNamed( String ),
    InvalidToken( usize, RefCount<Token> ),
    MultiNumberSign( usize ),
    UniquePattern( String ),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            ParserError::EndedAbruptly => write!( formatter, "The string ended abruptly." ),
            ParserError::UniqueNamed( identifier) =>
                write!( formatter,
                    "Named substrings must have unique identifiers. The identifier ‘{}’ already exists.",
                    identifier
                ),
            ParserError::InvalidToken( position, token ) =>
                write!(
                    formatter,
                    "Invalid token ‘{}’ was found at the position {} of the string.",
                    token.string,
                    position
                ),
            ParserError::MultiNumberSign( position ) =>
                write!( formatter, "Found sequential number signs at the position {} of the string.", position ),
            ParserError::UniquePattern( identifier) =>
                write!(
                    formatter,
                    "Pattern identifiers must be unique. The identifier ‘{}’ already exists.",
                    identifier
                ),
        }
    }
}

/// The `CommandError` type consists of the follow:
/// 
/// * `AlreadyExists`: Indicates command already present in command registry,
/// 
/// * `NotFound`: Indicates command was not found in command registry,
/// 
/// * `ParameterMissing`: Indicates a parameter is missing for the command,
/// 
/// * `InvalidType`: Indicates the command's parameter has incorrect type,
/// 
/// * `Custom`: Wraps a custom error.
#[derive( Debug )]
#[non_exhaustive]
pub enum CommandError {
    AlreadyExists( String ),
    NotFound( String ),
    ParameterMissing( String, usize ),
    InvalidType( String, usize ),
    Custom( Box<dyn Error> ), // For custom commands returning errors not of this enum.
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            CommandError::AlreadyExists( command ) =>
                write!( formatter, "The command ‘{}’ already exists in the CommandRegistry.", command ),
            CommandError::NotFound( command ) =>
                write!( formatter, "The command ‘{}’ was not found in the CommandRegistry.", command ),
            CommandError::ParameterMissing( command, index ) =>
                write!( formatter, "The parameter number {} is missing for the command ‘{}’.", index, command ),
            CommandError::InvalidType( command, index ) =>
                write!( formatter, "The parameter number {} has invalid type for the command ‘{}’.", index, command ),
            CommandError::Custom( ref error ) => error.fmt( formatter ),
        }
    }
}

/// The `FormatterError` type consists of the follow:
/// 
/// * `InvalidRoot`: Indicates the token tree did not have a `NodeType::Root` node for the root,
/// 
/// * `RetrieveChildren`: Indicates not children was retrieved,
/// 
/// * `NodeNotFound`: Indicates the expected node was not found,
/// 
/// * `FirstChild`: Indicates the first child of the node was not found,
/// 
/// * `RetrieveNodeData`: Indicates the data was not retrieved from the node,
/// 
/// * `RetrieveNodeToken`: Indicates the token was not retrieved from the node,
/// 
/// * `LastChild`: Indicates the last child of the node was not found,
/// 
/// * `InvalidNode`: Indicates the child not is invalid,
/// 
/// * `PatternNamed`: Indicates the pattern substring was not retrieved for the named string,
/// 
/// * `PatternPart`: Indicates a part of the pattern substring was not retrieved for the named string,
/// 
/// * `InvalidOptionValue`: Indicates the value for the option is invalid,
/// 
/// * `InvalidKeyword`: Indicates an invalid keyword was found for the pattern substring,
/// 
/// * `SelectorNamed`: Indicates the named string identifier was not found for placeholder,
/// 
/// * `SelectorOther`: Indicates the required `other` selector was not found in the pattern substring,
/// 
/// * `NoChildren`: Indicates no children was found for the node,
/// 
/// * `InvalidOption`: Indicates an invalid option was found for the pattern substring,
/// 
/// * `InvalidSelector`: Indicates an invalid selector was found for the pattern substring,
/// 
/// * `Locale`: Wraps the ICU4X locale error [`IcuParserError`],
/// 
/// * `Calendar`: Wraps the ICU4X calendar error [`CalendarError`],
/// 
/// * `ParseInt`: Wraps the integer parsing error [`ParseIntError`],
/// 
/// * `NumberSignString`: Indicates the formatted string was not retrieved for the number sign index,
/// 
/// * `SelectorsIndex`: Indicates the index was not found in the collected selectors,
/// 
/// * `SelectorsIndexNamed`: Indicates the named string was not found for the selector index,
/// 
/// * `PlaceholderValue`: Indicates the placeholder value was not found for the placeholder of the pattern,
/// 
/// * `InvalidValue`: Indicates an invalid value type for the placeholder of the pattern,
/// 
/// * `Decimal`: Wraps the ICU4X decimal error [`DecimalError`],
/// 
/// * `DateTime`: Wraps the ICU4X date time error [`DateTimeError`],
/// 
/// * `PluralRules`: Wraps the ICU4X plural error [`PluralError`],
/// 
/// * `FixedDecimal`: Wraps the ICU4X fixed error [`FixedDecimalError`],
/// 
/// * `NamedStringIdentifier`: Indicates the named string identifiers must be unique,
/// 
/// * `Command`: Wraps the crate's command error [`CommandError`],
/// 
/// * `NoIcuProvider`: Indicates no ICU4X data provider was provided,
/// 
/// * `NeverReached`: Indicates this branch should never be reached. A serious bug has occurred.
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
    PlaceholderValue( String, String ),
    InvalidValue( String ),
    Decimal( DecimalError ),
    DateTime( DateTimeError ),
    PluralRules( PluralError ),
    FixedDecimal( FixedDecimalError ),
    NamedStringIdentifier( String ),
    Command( CommandError ),
    NoIcuProvider,
    NeverReached,
}

impl Error for FormatterError {}

impl Display for FormatterError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            FormatterError::InvalidRoot => write!( formatter, "The tree root must be a ‘Root’ node." ),
            FormatterError::RetrieveChildren( node_type ) =>
                write!( formatter, "Failed to retrieve the children for the ‘{}’ node.", node_type ),
            FormatterError::NodeNotFound( node_type ) =>
                write!( formatter, "The expected node ‘{}’ was not found.", node_type ),
            FormatterError::FirstChild( node_type ) =>
                write!( formatter, "The first child of the ‘{}’ node was not found.", node_type ),
            FormatterError::RetrieveNodeData( node_type ) =>
                write!( formatter, "Failed to retrieve the data for the ‘{}’ node.", node_type ),
            FormatterError::RetrieveNodeToken( node_type ) =>
                write!( formatter, "Failed to retrieve the token for the ‘{}’ node.", node_type ),
            FormatterError::LastChild( node_type ) =>
                write!( formatter, "The last child of the ‘{}’ node was not found.", node_type ),
            FormatterError::InvalidNode( node_type ) =>
                write!( formatter, "Invalid child node found in the ‘{}’ node.", node_type ),
            FormatterError::PatternNamed( identifier ) =>
                write!( formatter, "Failed to retrieve the pattern for the named string ‘{}’.", identifier ),
            FormatterError::PatternPart( identifier, index ) =>
                write!(
                    formatter,
                    "Failed to retrieve the part ‘{}’ of the pattern for the named string ‘{}’.",
                    identifier, index
                ),
            FormatterError::InvalidOptionValue( value, option, keyword ) =>
                write!(
                    formatter,
                    "The value ‘{}’ is invalid for the option ‘{}’ for the keyword ‘{}’.",
                    value, option, keyword
                ),
            FormatterError::InvalidKeyword( keyword, placeholder ) =>
                write!( formatter, "Invalid keyword ‘{}’ for the placeholder ‘{}’.", keyword, placeholder ),
            FormatterError::SelectorNamed( named, selector, identifier ) =>
                write!(
                    formatter,
                    "The named string identifier ‘{}’ was not found for the selector ‘{}’ of the placeholder ‘{}’.",
                    named, selector, identifier
                ),
            FormatterError::SelectorOther( keyword, placeholder ) =>
                write!(
                    formatter,
                    "The required ‘other’ selector was not found for the keyword ‘{}’ of the placeholder ‘{}’.",
                    keyword, placeholder
                ),
            FormatterError::NoChildren( node_type) =>
                write!( formatter, "No children nodes was found for ‘{}’ node.", node_type ),
            FormatterError::InvalidOption( option, keyword, placeholder ) =>
                write!(
                    formatter,
                    "Invalid option ‘{}’ for the keyword ‘{}’ of the placeholder ‘{}’.",
                    option, keyword, placeholder
                ),
            FormatterError::InvalidSelector( option, keyword, placeholder ) =>
                write!(
                    formatter,
                    "Invalid selector ‘{}’ for the keyword ‘{}’ of the placeholder ‘{}’.",
                    option, keyword, placeholder
                ),
            FormatterError::Locale( ref error ) => error.fmt( formatter ),
            FormatterError::Calendar( ref error ) => error.fmt( formatter ),
            FormatterError::ParseInt( ref error ) => error.fmt( formatter ),
            FormatterError::NumberSignString( index ) =>
                write!( formatter, "Unable to retrieve the formatted string for the NumberSign index {}.", index ),
            FormatterError::SelectorsIndex( index ) =>
                write!( formatter, "The index {} is not found in the collected selectors.", index ),
            FormatterError::SelectorsIndexNamed( identifier, index ) =>
                write!(
                    formatter,
                    "Failed to retrieve the string for the named string ‘{}’ of the selectors index {}.",
                    identifier, index
                ),
            FormatterError::PlaceholderValue( part, placeholder ) =>
                write!(
                    formatter,
                    "The placeholder value was not found for the pattern part ‘{}’ of the placeholder ‘{}’.",
                    part,
                    placeholder,
                ),
            FormatterError::InvalidValue( part ) =>
                write!( formatter, "Invalid value type was provided for the pattern part ‘{}’.", part ),
            FormatterError::Decimal( ref error ) => error.fmt( formatter ),
            FormatterError::DateTime( ref error ) => error.fmt( formatter ),
            FormatterError::PluralRules( ref error ) => error.fmt( formatter ),
            FormatterError::FixedDecimal( ref error ) => error.fmt( formatter ),
            FormatterError::NamedStringIdentifier( identifier ) =>
                write!(
                    formatter,
                    "The named string identifier ‘{}’ already exists. The identifiers must be unique and not ‘_’.",
                    identifier
                ),
            FormatterError::Command( ref error ) => error.fmt( formatter ),
            FormatterError::NoIcuProvider => write!(
                formatter,
                "Build error: At least one ICU4X data provider must be specified for the crate ‘i18n_icu’ using the \
                    crate's features."
            ),
            FormatterError::NeverReached =>
                write!( formatter, "Build error: Should never have reached this match branch." ),
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

impl From<CommandError> for FormatterError {
    fn from( error: CommandError ) -> FormatterError {
        FormatterError::Command( error )
    }
}
