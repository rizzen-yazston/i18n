// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

use core::fmt::{Display, Formatter, Result};
use i18n_utility::{
    LocalisationData, LocalisationErrorTrait, LocalisationTrait, PlaceholderValue, RegistryError,
};
use std::{
    collections::HashMap,
    error::Error, // Experimental in `core` crate.
};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

pub trait ProviderErrorTrait: LocalisationTrait + Error + Display {}

/// Contains the possible errors that may occur within the provider.
///
/// The `ProviderError` type consists of the follow:
///
/// * `ComponentNotFound`: Indicates requested component is not found,
///
/// * `LanguageTagRegistry`: Wraps the language tag registry [`RegistryError`],
///
/// * `DefaultLanguage`: Expected default language was not found,
///
/// * `InvalidDefaultLanguage`: Default language is not in component's language list,
///
/// * `DefaultLanguageCount`: No strings for component's default language,
///
/// * `Custom`: Holds provider specific errors such as IO, Sqlite, etc.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ProviderError {
    ComponentNotFound(String), // component
    LanguageTagRegistry(RegistryError),
    DefaultLanguage(String), // component
    InvalidDefaultLanguage(String), // component
    DefaultLanguageCount(String, String), // component, language
    Custom(RefCount<Box<dyn ProviderErrorTrait>>),
}

impl LocalisationErrorTrait for ProviderError {}

impl LocalisationTrait for ProviderError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("ProviderError".to_string());
        match self {
            ProviderError::ComponentNotFound(ref component) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "component".to_string(),
                    PlaceholderValue::String(component.clone()),
                );
                let message = LocalisationData {
                    component: "i18n_provider".to_string(),
                    identifier: "component_not_found".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("ComponentNotFound".to_string()),
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
            ProviderError::LanguageTagRegistry(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("LanguageTagRegistry".to_string()),
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
            ProviderError::DefaultLanguage(ref component) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "component".to_string(),
                    PlaceholderValue::String(component.clone()),
                );
                let message = LocalisationData {
                    component: "i18n_provider".to_string(),
                    identifier: "default_language".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("DefaultLanguage".to_string()),
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
            ProviderError::InvalidDefaultLanguage(ref component) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "component".to_string(),
                    PlaceholderValue::String(component.clone()),
                );
                let message = LocalisationData {
                    component: "i18n_provider".to_string(),
                    identifier: "invalid_default_language".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidDefaultLanguage".to_string()),
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
            ProviderError::DefaultLanguageCount(ref component, ref language) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "component".to_string(),
                    PlaceholderValue::String(component.clone()),
                );
                message_values.insert(
                    "language".to_string(),
                    PlaceholderValue::String(language.clone()),
                );
                let message = LocalisationData {
                    component: "i18n_provider".to_string(),
                    identifier: "default_language_count".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("DefaultLanguageCount".to_string()),
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
            ProviderError::Custom(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Custom".to_string()),
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
        }
    }
}

impl Display for ProviderError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            ProviderError::ComponentNotFound( ref component ) => write!(
                formatter, "ProviderError::ComponentNotFound: The component ‘{}’ could not found.", component
            ),
            ProviderError::LanguageTagRegistry( ref error ) => write!(
                formatter, "ProviderError::LanguageTagRegistry: [{}].", error.to_string()
            ),
            ProviderError::DefaultLanguage( ref component ) => write!(
                formatter,
                "ProviderError::DefaultLanguage: The default language tag is missing for the component ‘{}’.",
                component
            ),
            ProviderError::InvalidDefaultLanguage( ref component ) => write!(
                formatter,
                "ProviderError::InvalidDefaultLanguage: The default language tag is invalid for the component ‘{}’.",
                component
            ),
            ProviderError::DefaultLanguageCount( ref component, ref language ) => write!(
                formatter,
                "ProviderError::DefaultLanguageCount: There are no localisation strings in the component ‘{}’ for the \
                default language tag ‘{}’.",
                component,
                language
            ),
            ProviderError::Custom( ref error ) => write!(
                formatter, "ProviderError::Custom: [{}].", error.to_string()
            ),
        }
    }
}

impl Error for ProviderError {}

impl From<RegistryError> for ProviderError {
    fn from(error: RegistryError) -> ProviderError {
        ProviderError::LanguageTagRegistry(error)
    }
}
