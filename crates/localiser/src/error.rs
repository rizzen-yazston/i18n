// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

use crate::{NodeType, ParserState};
use fixed_decimal::Error as FixedDecimalError;
use i18n_lexer::LexerError;
use i18n_provider::ProviderError;
use i18n_utility::{
    LocalisationData, LocalisationErrorTrait, LocalisationTrait, PlaceholderValue, RegistryError,
};
use icu_calendar::CalendarError;
use icu_datetime::DateTimeError;
use icu_decimal::Error as DecimalError;
use icu_locid::ParserError as IcuParserError;
use icu_plurals::Error as PluralError;
use std::num::ParseIntError;
use std::{
    collections::HashMap,
    error::Error, // Experimental in `core` crate.
};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

use core::fmt::{Display, Formatter, Result};

/// The `TreeError` type consists of the follow:
///
/// * `Lexer`: Wraps a custom error.
///
/// * `EndedAbruptly`: Indicates string came to an abrupt end, usually in a pattern,
///
/// * `InvalidToken`: Indicates the token is unexpected in the pattern,
///
/// * `UniquePattern`: Indicates the pattern identifier is a duplicate,
///
/// * `MultiNumberSign`: Indicates consecutive number sign characters found,
///
/// * `UniqueNamed`: Indicates there is a duplicated named string identifier.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TreeError {
    Lexer(LexerError),
    EndedAbruptly,
    InvalidToken(ParserState, usize, String),
    UniquePattern(String),
    MultiNumberSign(usize),
    UniqueNamed(String),
}

impl LocalisationErrorTrait for TreeError {}

impl LocalisationTrait for TreeError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("TreeError".to_string());
        match self {
            TreeError::Lexer(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("LexerError".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::LocalisationData(error.localisation_data()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            TreeError::EndedAbruptly => {
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "string_ended_abruptly".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("EndedAbruptly".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            TreeError::InvalidToken(state, position, token) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "state".to_string(),
                    PlaceholderValue::String(state.to_string()),
                );
                message_values.insert(
                    "position".to_string(),
                    PlaceholderValue::Unsigned(*position as u128),
                );
                message_values.insert("token".to_string(), PlaceholderValue::String(token.clone()));
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "invalid_token".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidToken".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            TreeError::UniquePattern(identifier) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.clone()),
                );
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "unique_pattern".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("UniquePattern".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            TreeError::MultiNumberSign(position) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "position".to_string(),
                    PlaceholderValue::Unsigned(*position as u128),
                );
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "multi_number_sign".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("MultiNumberSign".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            TreeError::UniqueNamed(identifier) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.clone()),
                );
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "unique_named".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("UniqueNamed".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
        }
    }
}

impl Display for TreeError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            TreeError::Lexer(ref error) => write!(
                formatter, "TreeError::LexerError: [{}].", error
            ),
            TreeError::EndedAbruptly => write!(
                formatter, "TreeError::EndedAbruptly: The string ended abruptly."
            ),
            TreeError::InvalidToken( state, position, token ) => write!(
                formatter,
                "TreeError::InvalidToken: In the parser state ‘{}’ an invalid token ‘{}’ was found at the position {} of the string.",
                state,
                token,
                position
            ),
            TreeError::UniquePattern( identifier) => write!(
                formatter,
                "TreeError::UniquePattern: Pattern identifiers must be unique. The identifier ‘{}’ already exists.",
                identifier
            ),
            TreeError::MultiNumberSign( position ) => write!(
                formatter,
                "TreeError::MultiNumberSign: Found sequential number signs at the position {} of the string.",
                position
            ),
            TreeError::UniqueNamed( identifier) => write!(
                formatter,
                "TreeError::UniqueNamed: Named substrings must have unique identifiers. The identifier ‘{}’ already \
                exists.",
                identifier
            ),
        }
    }
}

impl Error for TreeError {}

impl From<LexerError> for TreeError {
    fn from(error: LexerError) -> TreeError {
        TreeError::Lexer(error)
    }
}

/// The `CommandError` type consists of the follow:
///
/// * `Custom`: Wraps a custom error.
///
/// * `AlreadyExists`: Indicates command already present in command registry,
///
/// * `NotFound`: Indicates command was not found in command registry,
///
/// * `ParameterMissing`: Indicates a parameter is missing for the command,
///
/// * `InvalidType`: Indicates the command's parameter has incorrect type,
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum CommandError {
    Custom(RefCount<Box<dyn LocalisationErrorTrait>>), // For custom commands returning errors not of this enum.
    AlreadyExists(String),
    NotFound(String),
    ParameterMissing(String, usize),
    InvalidType(String, usize),
}

impl LocalisationErrorTrait for CommandError {}

impl LocalisationTrait for CommandError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("CommandError".to_string());
        match self {
            CommandError::Custom(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Registry".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::LocalisationData(error.localisation_data()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            CommandError::AlreadyExists(command) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "command".to_string(),
                    PlaceholderValue::String(command.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "already_exists".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("AlreadyExists".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            CommandError::NotFound(command) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "command".to_string(),
                    PlaceholderValue::String(command.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "command_not_found".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NotFound".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            CommandError::ParameterMissing(command, index) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "command".to_string(),
                    PlaceholderValue::String(command.to_string()),
                );
                message_values.insert(
                    "index".to_string(),
                    PlaceholderValue::Unsigned(*index as u128),
                );
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "parameter_missing".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("ParameterMissing".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            CommandError::InvalidType(command, index) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "command".to_string(),
                    PlaceholderValue::String(command.to_string()),
                );
                message_values.insert(
                    "index".to_string(),
                    PlaceholderValue::Unsigned(*index as u128),
                );
                let message = LocalisationData {
                    component: "i18n_pattern".to_string(),
                    identifier: "invalid_type".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidType".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            CommandError::Custom( ref error ) => write!(
                formatter, "CommandError::Custom: [{}].", error
            ),
            CommandError::AlreadyExists( command ) => write!(
                formatter,
                "CommandError::AlreadyExists: The command ‘{}’ already exists in the CommandRegistry.",
                command
            ),
            CommandError::NotFound( command ) => write!(
                formatter,
                "CommandError::NotFound: The command ‘{}’ was not found in the CommandRegistry.",
                command
            ),
            CommandError::ParameterMissing( command, index ) => write!(
                formatter,
                "CommandError::ParameterMissing: The parameter number {} is missing for the command ‘{}’.",
                index,
                command
            ),
            CommandError::InvalidType( command, index ) => write!(
                formatter,
                "CommandError::InvalidType: The parameter number {} has invalid type for the command ‘{}’.",
                index,
                command
            ),
        }
    }
}

impl Error for CommandError {}

/// The `LocaliserError` type consists of the follow:
///
/// * `Registry`: Wraps the `LanguageTagRegistry` [`RegistryError`],
///
/// * `Formatter`: Wraps the pattern `Formatter`'s [`FormatterError`],
///
/// * `Provider`: Wraps the `LocalisationProviderTrait`'s [`ProviderError`],
///
/// * `StringNotFound`: Indicates the pattern string was not found in localisation repository,
///
/// * `CacheEntry`: Indicates error occurred when accessing internal cache.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum LocaliserError {
    Registry(RegistryError),
    Formatter(FormatterError),
    Provider(ProviderError),
    StringNotFound(String, String, String, bool), // component, identifier, language_tag, fallback
    CacheEntry(String, String),
}

impl LocalisationErrorTrait for LocaliserError {}

impl LocalisationTrait for LocaliserError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("LocaliserError".to_string());
        match self {
            LocaliserError::Registry(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Registry".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::LocalisationData(error.localisation_data()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            LocaliserError::Formatter(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Formatter".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::LocalisationData(error.localisation_data()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            LocaliserError::Provider(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Provider".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::LocalisationData(error.localisation_data()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            LocaliserError::StringNotFound(component, identifier, language_tag, fallback) => {
                let string = match fallback {
                    true => "true".to_string(),
                    false => "false".to_string(),
                };
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "component".to_string(),
                    PlaceholderValue::String(component.clone()),
                );
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.clone()),
                );
                message_values.insert(
                    "language_tag".to_string(),
                    PlaceholderValue::String(language_tag.clone()),
                );
                message_values.insert("fallback".to_string(), PlaceholderValue::String(string));
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "string_not_found".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("StringNotFound".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            LocaliserError::CacheEntry(component, identifier) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "component".to_string(),
                    PlaceholderValue::String(component.clone()),
                );
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.clone()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "cache_entry".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("CacheEntry".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
        }
    }
}

impl Display for LocaliserError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            LocaliserError::Registry( ref error ) => write!(
                formatter, "LocaliserError::Registry: [{}].", error
            ),
            LocaliserError::Formatter( ref error ) => write!(
                formatter, "LocaliserError::Formatter: [{}].", error
            ),
            LocaliserError::Provider( ref error ) => write!(
                formatter, "LocaliserError::Provider: [{}].", error
            ),
            LocaliserError::StringNotFound(
                component, identifier, language_tag, fallback
            ) => {
                let string = match fallback {
                    true => "True".to_string(),
                    false => "False".to_string()
                };
                write!(
                    formatter,
                    "LocaliserError::StringNotFound: No string was found for the component ‘{}’ with identifier ‘{}’ \
                    for the language tag ‘{}’. Fallback was used: {}.",
                    component,
                    identifier,
                    language_tag,
                    string,
                )
            },
            LocaliserError::CacheEntry( component, identifier ) => write!(
                formatter,
                "LocaliserError::CacheEntry: Unable to get the string for the component ‘{}’ with the identifier
                ‘{}’ as the cache entry requires values for formatting.",
                component,
                identifier
            ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for LocaliserError {}

impl From<RegistryError> for LocaliserError {
    fn from(error: RegistryError) -> LocaliserError {
        LocaliserError::Registry(error)
    }
}

impl From<FormatterError> for LocaliserError {
    fn from(error: FormatterError) -> LocaliserError {
        LocaliserError::Formatter(error)
    }
}

impl From<ProviderError> for LocaliserError {
    fn from(error: ProviderError) -> LocaliserError {
        LocaliserError::Provider(error)
    }
}

/// The `FormatterError` type consists of the follow:
///
/// * `Localiser`: Wraps the crate's localiser error [`LocaliserError`].
///
/// * `Locale`: Wraps the ICU4X locale error [`IcuParserError`],
///
/// * `Calendar`: Wraps the ICU4X calendar error [`CalendarError`],
///
/// * `ParseInt`: Wraps the integer parsing error [`ParseIntError`],
///
/// * `Decimal`: Wraps the ICU4X decimal error [`DecimalError`],
///
/// * `DateTime`: Wraps the ICU4X date time error [`DateTimeError`],
///
/// * `PluralRules`: Wraps the ICU4X plural error [`PluralError`],
///
/// * `FixedDecimal`: Wraps the ICU4X fixed error [`FixedDecimalError`],
///
/// * `Command`: Wraps the `i18n_pattern`'s command error [`CommandError`],
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
/// * `NamedStringIdentifier`: Indicates the named string identifiers must be unique,
///
/// * `NoIcuProvider`: Indicates no ICU4X data provider was provided,
///
/// * `NeverReached`: Indicates this branch should never be reached. A serious bug has occurred.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum FormatterError {
    Localiser(Box<LocaliserError>),
    Tree(TreeError),
    Locale(IcuParserError),
    Calendar(CalendarError),
    ParseInt(ParseIntError),
    Decimal(DecimalError),
    DateTime(DateTimeError),
    PluralRules(PluralError),
    FixedDecimal(FixedDecimalError),
    Command(CommandError),
    NoGrammar,
    InvalidRoot,
    RetrieveChildren(NodeType),
    NodeNotFound(NodeType),
    FirstChild(NodeType),
    RetrieveNodeData(NodeType),
    RetrieveNodeToken(NodeType),
    LastChild(NodeType),
    InvalidNode(NodeType),
    PatternNamed(String),
    PatternPart(String, usize),
    InvalidOptionValue(String, String, String),
    InvalidKeyword(String, String),
    SelectorNamed(String, String, String),
    SelectorOther(String, String),
    NoChildren(NodeType),
    InvalidOption(String, String, String),
    InvalidSelector(String, String, String),
    NumberSignString(usize),
    SelectorsIndex(usize),
    SelectorsIndexNamed(String, usize),
    PlaceholderValue(String, String),
    InvalidValue(String),
    NamedStringIdentifier(String),
    NoIcuProvider,
    NeverReached,
}

impl LocalisationErrorTrait for FormatterError {}

impl LocalisationTrait for FormatterError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("FormatterError".to_string());
        match self {
            FormatterError::Localiser(ref error) => {
                // Currently no localisation is available for this error type: ParserError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Localiser".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::Tree(ref error) => {
                // Currently no localisation is available for this error type: ParserError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Tree".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::Locale(ref error) => {
                // Currently no localisation is available for this error type: ParserError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Locale".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::Calendar(ref error) => {
                // Currently no localisation is available for this error type: CalendarError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Calendar".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::ParseInt(ref error) => {
                // Currently no localisation is available for this error type: ParseIntError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("ParseInt".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::Decimal(ref error) => {
                // Currently no localisation is available for this error type: DecimalError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Decimal".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::DateTime(ref error) => {
                // Currently no localisation is available for this error type: DateTimeError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("DateTime".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::PluralRules(ref error) => {
                // Currently no localisation is available for this error type: PluralsError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("PluralRules".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::FixedDecimal(ref error) => {
                // Currently no localisation is available for this error type: FixedDecimalError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("FixedDecimal".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::Command(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Command".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::LocalisationData(error.localisation_data()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::InvalidRoot => {
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "invalid_root".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidRoot".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::NoGrammar => {
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "no_grammar".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NoGrammar".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::RetrieveChildren(node_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "node_type".to_string(),
                    PlaceholderValue::String(node_type.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "retrieve_children".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("RetrieveChildren".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::NodeNotFound(node_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "node_type".to_string(),
                    PlaceholderValue::String(node_type.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "node_not_found".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NodeNotFound".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::FirstChild(node_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "node_type".to_string(),
                    PlaceholderValue::String(node_type.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "first_child".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("FirstChild".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::RetrieveNodeData(node_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "node_type".to_string(),
                    PlaceholderValue::String(node_type.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "retrieve_node_data".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("RetrieveNodeData".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::RetrieveNodeToken(node_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "node_type".to_string(),
                    PlaceholderValue::String(node_type.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "retrieve_node_token".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("RetrieveNodeToken".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::LastChild(node_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "node_type".to_string(),
                    PlaceholderValue::String(node_type.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "last_child".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("LastChild".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::InvalidNode(node_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "node_type".to_string(),
                    PlaceholderValue::String(node_type.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "invalid_node".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidNode".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::PatternNamed(identifier) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "pattern_named".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("PatternNamed".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::PatternPart(identifier, index) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.to_string()),
                );
                message_values.insert(
                    "index".to_string(),
                    PlaceholderValue::Unsigned(*index as u128),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "pattern_part".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("PatternPart".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::InvalidOptionValue(value, option, keyword) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "value".to_string(),
                    PlaceholderValue::String(value.to_string()),
                );
                message_values.insert(
                    "option".to_string(),
                    PlaceholderValue::String(option.to_string()),
                );
                message_values.insert(
                    "keyword".to_string(),
                    PlaceholderValue::String(keyword.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "invalid_option_value".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidOptionValue".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::InvalidKeyword(keyword, placeholder) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "keyword".to_string(),
                    PlaceholderValue::String(keyword.to_string()),
                );
                message_values.insert(
                    "placeholder".to_string(),
                    PlaceholderValue::String(placeholder.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "invalid_keyword".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidKeyword".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::SelectorNamed(named, selector, identifier) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "named".to_string(),
                    PlaceholderValue::String(named.to_string()),
                );
                message_values.insert(
                    "selector".to_string(),
                    PlaceholderValue::String(selector.to_string()),
                );
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "selector_named".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("SelectorNamed".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::SelectorOther(keyword, placeholder) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "keyword".to_string(),
                    PlaceholderValue::String(keyword.to_string()),
                );
                message_values.insert(
                    "placeholder".to_string(),
                    PlaceholderValue::String(placeholder.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "selector_other".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("SelectorOther".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::NoChildren(node_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "node_type".to_string(),
                    PlaceholderValue::String(node_type.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "no_children".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NoChildren".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::InvalidOption(option, keyword, placeholder) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "option".to_string(),
                    PlaceholderValue::String(option.to_string()),
                );
                message_values.insert(
                    "keyword".to_string(),
                    PlaceholderValue::String(keyword.to_string()),
                );
                message_values.insert(
                    "placeholder".to_string(),
                    PlaceholderValue::String(placeholder.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "invalid_option".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidOption".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::InvalidSelector(option, keyword, placeholder) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "option".to_string(),
                    PlaceholderValue::String(option.to_string()),
                );
                message_values.insert(
                    "keyword".to_string(),
                    PlaceholderValue::String(keyword.to_string()),
                );
                message_values.insert(
                    "placeholder".to_string(),
                    PlaceholderValue::String(placeholder.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "invalid_selector".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidSelector".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::NumberSignString(index) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "index".to_string(),
                    PlaceholderValue::Unsigned(*index as u128),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "number_sign_string".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NumberSignString".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::SelectorsIndex(index) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "index".to_string(),
                    PlaceholderValue::Unsigned(*index as u128),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "selectors_index".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("SelectorsIndex".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::SelectorsIndexNamed(identifier, index) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.to_string()),
                );
                message_values.insert(
                    "index".to_string(),
                    PlaceholderValue::Unsigned(*index as u128),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "selectors_index_named".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("SelectorsIndexNamed".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::PlaceholderValue(part, placeholder) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "part".to_string(),
                    PlaceholderValue::String(part.to_string()),
                );
                message_values.insert(
                    "placeholder".to_string(),
                    PlaceholderValue::String(placeholder.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "placeholder_value".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("PlaceholderValue".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::InvalidValue(part) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "part".to_string(),
                    PlaceholderValue::String(part.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "invalid_value".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidValue".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::NamedStringIdentifier(identifier) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "identifier".to_string(),
                    PlaceholderValue::String(identifier.to_string()),
                );
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "named_string_identifier".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NamedStringIdentifier".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::NoIcuProvider => {
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "no_icu_provider".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NoIcuProvider".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            FormatterError::NeverReached => {
                let message = LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "never_reach".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NeverReached".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
        }
    }
}

impl Display for FormatterError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            FormatterError::Localiser( ref error ) => write!(
                formatter, "FormatterError::Localiser: [{}].", error
            ),
            FormatterError::Tree( ref error ) => write!(
                formatter, "FormatterError::Tree: [{}].", error
            ),
            FormatterError::Locale( ref error ) => write!(
                formatter, "FormatterError::Locale: [{}].", error
            ),
            FormatterError::Calendar( ref error ) => write!(
                formatter, "FormatterError::Calendar: [{}].", error
            ),
            FormatterError::ParseInt( ref error ) => write!(
                formatter, "FormatterError::ParseInt: [{}].", error
            ),
            FormatterError::Decimal( ref error ) => write!(
                formatter, "FormatterError::Decimal: [{}].", error
            ),
            FormatterError::DateTime( ref error ) => write!(
                formatter, "FormatterError::DateTime: [{}].", error
            ),
            FormatterError::PluralRules( ref error ) => write!(
                formatter, "FormatterError::PluralRules: [{}].", error
            ),
            FormatterError::FixedDecimal( ref error ) => write!(
                formatter, "FormatterError::FixedDecimal: [{}].", error
            ),
            FormatterError::Command( ref error ) => write!(
                formatter, "FormatterError::Command: [{}].", error
            ),
            FormatterError::InvalidRoot => write!( formatter, "FormatterError::InvalidRoot: The tree root must be a ‘Root’ node." ),
            FormatterError::NoGrammar => write!( formatter, "FormatterError::NoGrammar: No grammar syntax characters was found in the provided localisation string." ),
            FormatterError::RetrieveChildren( node_type ) =>
                write!( formatter, "FormatterError::RetrieveChildren: Failed to retrieve the children for the ‘{}’ node.", node_type ),
            FormatterError::NodeNotFound( node_type ) =>
                write!( formatter, "FormatterError::NodeNotFound: The expected node ‘{}’ was not found.", node_type ),
            FormatterError::FirstChild( node_type ) =>
                write!( formatter, "FormatterError::FirstChild: The first child of the ‘{}’ node was not found.", node_type ),
            FormatterError::RetrieveNodeData( node_type ) =>
                write!( formatter, "FormatterError::RetrieveNodeData: Failed to retrieve the data for the ‘{}’ node.", node_type ),
            FormatterError::RetrieveNodeToken( node_type ) =>
                write!( formatter, "FormatterError::RetrieveNodeToken: Failed to retrieve the token for the ‘{}’ node.", node_type ),
            FormatterError::LastChild( node_type ) =>
                write!( formatter, "FormatterError::LastChild: The last child of the ‘{}’ node was not found.", node_type ),
            FormatterError::InvalidNode( node_type ) =>
                write!( formatter, "FormatterError::InvalidNode: Invalid child node found in the ‘{}’ node.", node_type ),
            FormatterError::PatternNamed( identifier ) =>
                write!( formatter, "FormatterError::PatternNamed: Failed to retrieve the pattern for the named string ‘{}’.", identifier ),
            FormatterError::PatternPart( identifier, index ) =>
                write!(
                    formatter,
                    "FormatterError::PatternPart: Failed to retrieve the part ‘{}’ of the pattern for the named string ‘{}’.",
                    identifier, index
                ),
            FormatterError::InvalidOptionValue( value, option, keyword ) =>
                write!(
                    formatter,
                    "FormatterError::InvalidOptionValue: The value ‘{}’ is invalid for the option ‘{}’ for the keyword ‘{}’.",
                    value, option, keyword
                ),
            FormatterError::InvalidKeyword( keyword, placeholder ) =>
                write!( formatter, "FormatterError::InvalidKeyword: Invalid keyword ‘{}’ for the placeholder ‘{}’.", keyword, placeholder ),
            FormatterError::SelectorNamed( named, selector, identifier ) =>
                write!(
                    formatter,
                    "FormatterError::SelectorNamed: The named string identifier ‘{}’ was not found for the selector ‘{}’ of the placeholder ‘{}’.",
                    named, selector, identifier
                ),
            FormatterError::SelectorOther( keyword, placeholder ) =>
                write!(
                    formatter,
                    "FormatterError::SelectorOther: The required ‘other’ selector was not found for the keyword ‘{}’ of the placeholder ‘{}’.",
                    keyword, placeholder
                ),
            FormatterError::NoChildren( node_type) =>
                write!( formatter, "FormatterError::NoChildren: No children nodes was found for ‘{}’ node.", node_type ),
            FormatterError::InvalidOption( option, keyword, placeholder ) =>
                write!(
                    formatter,
                    "FormatterError::InvalidOption: Invalid option ‘{}’ for the keyword ‘{}’ of the placeholder ‘{}’.",
                    option, keyword, placeholder
                ),
            FormatterError::InvalidSelector( option, keyword, placeholder ) =>
                write!(
                    formatter,
                    "FormatterError::InvalidSelector: Invalid selector ‘{}’ for the keyword ‘{}’ of the placeholder ‘{}’.",
                    option, keyword, placeholder
                ),
            FormatterError::NumberSignString( index ) =>
                write!( formatter, "FormatterError::NumberSignString: Unable to retrieve the formatted string for the NumberSign index {}.", index ),
            FormatterError::SelectorsIndex( index ) =>
                write!( formatter, "FormatterError::SelectorsIndex: The index {} is not found in the collected selectors.", index ),
            FormatterError::SelectorsIndexNamed( identifier, index ) =>
                write!(
                    formatter,
                    "FormatterError::SelectorsIndexNamed: Failed to retrieve the string for the named string ‘{}’ of the selectors index {}.",
                    identifier, index
                ),
            FormatterError::PlaceholderValue( part, placeholder ) =>
                write!(
                    formatter,
                    "FormatterError::PlaceholderValue: The placeholder value was not found for the pattern part ‘{}’ of the placeholder ‘{}’.",
                    part,
                    placeholder,
                ),
            FormatterError::InvalidValue( part ) =>
                write!( formatter, "FormatterError::InvalidValue: Invalid value type was provided for the pattern part ‘{}’.", part ),
            FormatterError::NamedStringIdentifier( identifier ) =>
                write!(
                    formatter,
                    "FormatterError::NamedStringIdentifier: The named string identifier ‘{}’ already exists. The identifiers must be unique and not ‘_’.",
                    identifier
                ),
            FormatterError::NoIcuProvider => write!(
                formatter,
                "FormatterError::NoIcuProvider: Build error: At least one ICU4X data provider must be specified for the crate ‘i18n_icu’ using the \
                    crate's features."
            ),
            FormatterError::NeverReached =>
                write!( formatter, "FormatterError::NeverReached: Build error: Should never have reached this match branch." ),
        }
    }
}

impl Error for FormatterError {}

impl From<LocaliserError> for FormatterError {
    fn from(error: LocaliserError) -> FormatterError {
        FormatterError::Localiser(Box::new(error))
    }
}

impl From<IcuParserError> for FormatterError {
    fn from(error: IcuParserError) -> FormatterError {
        FormatterError::Locale(error)
    }
}

impl From<CalendarError> for FormatterError {
    fn from(error: CalendarError) -> FormatterError {
        FormatterError::Calendar(error)
    }
}

impl From<ParseIntError> for FormatterError {
    fn from(error: ParseIntError) -> FormatterError {
        FormatterError::ParseInt(error)
    }
}

impl From<DecimalError> for FormatterError {
    fn from(error: DecimalError) -> FormatterError {
        FormatterError::Decimal(error)
    }
}

impl From<DateTimeError> for FormatterError {
    fn from(error: DateTimeError) -> FormatterError {
        FormatterError::DateTime(error)
    }
}

impl From<PluralError> for FormatterError {
    fn from(error: PluralError) -> FormatterError {
        FormatterError::PluralRules(error)
    }
}

impl From<FixedDecimalError> for FormatterError {
    fn from(error: FixedDecimalError) -> FormatterError {
        FormatterError::FixedDecimal(error)
    }
}

impl From<CommandError> for FormatterError {
    fn from(error: CommandError) -> FormatterError {
        FormatterError::Command(error)
    }
}

impl From<TreeError> for FormatterError {
    fn from(error: TreeError) -> FormatterError {
        FormatterError::Tree(error)
    }
}
