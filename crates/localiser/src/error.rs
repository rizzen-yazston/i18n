// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

use i18n_utility::RegistryError;
use i18n_pattern::{ ParserError, FormatterError };
use i18n_provider:: LocalisationProviderError;
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
/// * `Provider`: Wraps the `LocalisationProvider`'s [`LocalisationProviderError`],
/// 
/// * `StringNotFound`: Indicates the pattern string was not found in localisation repository,
/// 
/// * `NoDefaultLanguageTag`: Indicates no default language tag for component,
/// 
/// * `CacheEntry`: Indicates error occurred when accessing internal cache.
#[derive( Debug )]
#[non_exhaustive]
pub enum LocaliserError {
    Registry( RegistryError ),
    Parser( ParserError ),
    Formatter( FormatterError ),
    Provider( LocalisationProviderError ),
    StringNotFound( String, String, String, bool ), // component, identifier, language_tag, fallback
    NoDefaultLanguageTag( String ), // component
    CacheEntry( String, String ),
}

impl Display for LocaliserError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            LocaliserError::Registry( ref error ) => error.fmt( formatter ),
            LocaliserError::Parser( ref error ) => error.fmt( formatter ),
            LocaliserError::Formatter( ref error ) => error.fmt( formatter ),
            LocaliserError::Provider( ref error ) => error.fmt( formatter ),
            LocaliserError::StringNotFound(
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
            LocaliserError::NoDefaultLanguageTag( component ) =>
                write!( formatter, "No default language tag was found for the component ‘{}’.", component ),
            LocaliserError::CacheEntry( component, identifier ) =>
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

impl From< LocalisationProviderError> for LocaliserError {
    fn from( error:  LocalisationProviderError ) -> LocaliserError {
        LocaliserError::Provider( error )
    }
}
