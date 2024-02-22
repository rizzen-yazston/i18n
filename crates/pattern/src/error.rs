// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

use i18n_lexer::Token;
use i18n_utility::{LocalisationData, LocalisationErrorTrait, LocalisationTrait, PlaceholderValue};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

use core::fmt::{Display, Formatter, Result};
use std::{
    collections::HashMap,
    error::Error, // Experimental in `core` crate.
};

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
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ParserError {
    EndedAbruptly,
    UniqueNamed(String),
    InvalidToken(usize, RefCount<Token>),
    MultiNumberSign(usize),
    UniquePattern(String),
}

impl LocalisationErrorTrait for ParserError {}

impl LocalisationTrait for ParserError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("ParserError".to_string());
        match self {
            ParserError::EndedAbruptly => {
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
            ParserError::UniqueNamed(identifier) => {
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
            ParserError::InvalidToken(position, token) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "position".to_string(),
                    PlaceholderValue::Unsigned(*position as u128),
                );
                message_values.insert(
                    "token".to_string(),
                    PlaceholderValue::String(token.string.clone()),
                );
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
            ParserError::MultiNumberSign(position) => {
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
            ParserError::UniquePattern(identifier) => {
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
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            ParserError::EndedAbruptly => write!(
                formatter, "ParserError::EndedAbruptly: The string ended abruptly."
            ),
            ParserError::UniqueNamed( identifier) => write!(
                formatter,
                "ParserError::UniqueNamed: Named substrings must have unique identifiers. The identifier ‘{}’ already \
                exists.",
                identifier
            ),
            ParserError::InvalidToken( position, token ) => write!(
                formatter,
                "ParserError::InvalidToken: Invalid token ‘{}’ was found at the position {} of the string.",
                token.string,
                position
            ),
            ParserError::MultiNumberSign( position ) => write!(
                formatter,
                "ParserError::MultiNumberSign: Found sequential number signs at the position {} of the string.",
                position
            ),
            ParserError::UniquePattern( identifier) => write!(
                formatter,
                "ParserError::UniquePattern: Pattern identifiers must be unique. The identifier ‘{}’ already exists.",
                identifier
            ),
        }
    }
}

impl Error for ParserError {}

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
                formatter, "CommandError::Custom: [{}].", error.to_string()
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
