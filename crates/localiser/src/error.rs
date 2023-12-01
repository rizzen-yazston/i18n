// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

use i18n_utility::{ RegistryError, LocalisationTrait, LocalisationErrorTrait };
use i18n_pattern::{ ParserError, FormatterError };
use i18n_provider:: ProviderError;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

/// The `LocaliserError` type consists of the follow:
/// 
/// * `Registry`: Wraps the `LanguageTagRegistry` [`RegistryError`],
/// 
/// * `Parser`: Wraps the pattern `Parser`'s [`ParserError`],
///
/// * `Formatter`: Wraps the pattern `Formatter`'s [`FormatterError`],
/// 
/// * `Provider`: Wraps the `LocalisationProvider`'s [`ProviderError`],
/// 
/// * `StringNotFound`: Indicates the pattern string was not found in localisation repository,
/// 
/// * `CacheEntry`: Indicates error occurred when accessing internal cache.
#[derive( Debug )]
#[non_exhaustive]
pub enum LocaliserError {
    Registry( RegistryError ),
    Parser( ParserError ),
    Formatter( FormatterError ),
    Provider( ProviderError ),
    StringNotFound( String, String, String, bool ), // component, identifier, language_tag, fallback
    CacheEntry( String, String ),
}

impl LocalisationTrait for LocaliserError {
    fn identifier( &self ) -> &str {
        match *self {
            LocaliserError::StringNotFound( _, _, _, _ ) => "string_not_found",
            LocaliserError::CacheEntry( _, _ ) => "cache_entry",
            _ => "",
        }
    }

    fn component( &self ) -> &str {
        "i18n_localiser"
    }
}

impl LocalisationErrorTrait for LocaliserError {
    fn error_type( &self ) -> &str {
        "LocaliserError"
    }

    fn error_variant( &self ) -> &str {
        match *self {
            LocaliserError::Registry( _ ) => "Registry",
            LocaliserError::Parser( _ ) => "Parser",
            LocaliserError::Formatter( _ ) => "Formatter",
            LocaliserError::Provider( _ ) => "Provider",
            LocaliserError::StringNotFound( _, _, _, _ ) => "StringNotFound",
            LocaliserError::CacheEntry( _, _ ) => "CacheEntry",
        }
    }    
}

impl Display for LocaliserError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            LocaliserError::Registry( ref error ) => write!(
                formatter, "LocaliserError::Registry: [{}].", error.to_string()
            ),
            LocaliserError::Parser( ref error ) => write!(
                formatter, "LocaliserError::Parser: [{}].", error.to_string()
            ),
            LocaliserError::Formatter( ref error ) => write!(
                formatter, "LocaliserError::Formatter: [{}].", error.to_string()
            ),
            LocaliserError::Provider( ref error ) => write!(
                formatter, "LocaliserError::Provider: [{}].", error.to_string()
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
    fn from( error: RegistryError ) -> LocaliserError {
        LocaliserError::Registry( error )
    }
}

impl From<ParserError> for LocaliserError {
    fn from( error: ParserError ) -> LocaliserError {
        LocaliserError::Parser( error )
    }
}

impl From<FormatterError> for LocaliserError {
    fn from( error: FormatterError ) -> LocaliserError {
        LocaliserError::Formatter( error )
    }
}

impl From< ProviderError> for LocaliserError {
    fn from( error: ProviderError ) -> LocaliserError {
        LocaliserError::Provider( error )
    }
}
