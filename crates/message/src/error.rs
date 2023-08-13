// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

use i18n_utility::RegistryError;
use i18n_pattern::{ ParserError, FormatterError };
use i18n_provider::ProviderError;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug )]
#[non_exhaustive]
pub enum MessageError {
    Registry( RegistryError ),
    Parser( ParserError ),
    Formatter( FormatterError ),
    Provider( ProviderError ),
    StringNotFound( String, String, String, bool ), // component, identifier, language_tag, fallback
    NoDefaultLanguageTag( String ), // component
    CacheEntry( String, String ),
}

impl Display for MessageError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            MessageError::Registry( ref error ) => error.fmt( formatter ),
            MessageError::Parser( ref error ) => error.fmt( formatter ),
            MessageError::Formatter( ref error ) => error.fmt( formatter ),
            MessageError::Provider( ref error ) => error.fmt( formatter ),
            MessageError::StringNotFound(
                component, identifier, language_tag, fallback
            ) => {
                let string = match fallback {
                    true => "True".to_string(),
                    false => "False".to_string()
                };
                write!(
                    formatter,
                    "No string was found for the component ‘{}’ with identifier ‘{}’ for the language tag \
                        ‘{}’. Fallback was used: {}.",
                    component,
                    identifier,
                    language_tag,
                    string,
                )
            },
            MessageError::NoDefaultLanguageTag( component ) =>
                write!( formatter, "No default language tag was found for the component ‘{}’.", component ),
            MessageError::CacheEntry( component, identifier ) =>
                write!(
                    formatter,
                    "Unable to get the string for the component ‘{}’ with the identifier ‘{}’ as the cache entry \
                        requires values for formatting.",
                    component,
                    identifier
                ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for MessageError {}

impl From<RegistryError> for MessageError {
    fn from( error: RegistryError ) -> MessageError {
        MessageError::Registry( error )
    }
}

impl From<ParserError> for MessageError {
    fn from( error: ParserError ) -> MessageError {
        MessageError::Parser( error )
    }
}

impl From<FormatterError> for MessageError {
    fn from( error: FormatterError ) -> MessageError {
        MessageError::Formatter( error )
    }
}

impl From<ProviderError> for MessageError {
    fn from( error: ProviderError ) -> MessageError {
        MessageError::Provider( error )
    }
}
