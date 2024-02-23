// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use crate::{LocalisationData, LocalisationErrorTrait, LocalisationTrait, PlaceholderValue};
use core::fmt::{Display, Formatter, Result};
use icu_locid::ParserError;
use std::{
    collections::HashMap,
    error::Error, // Experimental in `core` crate.
};

/// The `RegistryError` type consists of the follow:
///
/// * `LanguageIdentifier`: Wraps the ICU's [`ParserError`],
///
/// else if cargo feature `extend` is enabled:
///
/// * `Locale`: Wraps the ICU's `ParserError`.
#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum RegistryError {
    #[cfg(not(feature = "extend"))]
    LanguageIdentifier(ParserError),
    #[cfg(feature = "extend")]
    Locale(ParserError),
}

impl LocalisationErrorTrait for RegistryError {}

impl LocalisationTrait for RegistryError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("IcuError".to_string());
        match self {
            #[cfg(not(feature = "extend"))]
            RegistryError::LanguageIdentifier(error) => {
                // Currently no localisation is available for this error type: ParserError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("LanguageIdentifier".to_string()),
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

            #[cfg(feature = "extend")]
            RegistryError::Locale(error) => {
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
        }
    }
}

impl Display for RegistryError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            #[cfg(not(feature = "extend"))]
            RegistryError::LanguageIdentifier(ref error) => {
                write!(formatter, "RegistryError::LanguageIdentifier: [{}].", error)
            }

            #[cfg(feature = "extend")]
            RegistryError::Locale(ref error) => {
                write!(formatter, "RegistryError::Locale: [{}].", error)
            }
        }
    }
}

// Source is embedded in the enum value.
impl Error for RegistryError {}

impl From<ParserError> for RegistryError {
    #[cfg(not(feature = "extend"))]
    fn from(error: ParserError) -> RegistryError {
        RegistryError::LanguageIdentifier(error)
    }

    #[cfg(feature = "extend")]
    fn from(error: ParserError) -> RegistryError {
        RegistryError::Locale(error)
    }
}
