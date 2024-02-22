// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_icu-rizzen-yazston` crate.

use i18n_utility::{LocalisationData, LocalisationErrorTrait, LocalisationTrait, PlaceholderValue};

#[cfg(feature = "buffer")]
use icu_properties::PropertiesError;

#[cfg(feature = "buffer")]
use icu_segmenter::SegmenterError;

#[cfg(feature = "buffer")]
use icu_provider::DataError;

#[cfg(doc)]
use icu_properties::PropertiesError;

#[cfg(doc)]
use icu_segmenter::SegmenterError;

#[cfg(doc)]
use icu_provider::DataError;

use core::fmt::{Display, Formatter, Result};
use std::{
    collections::HashMap,
    error::Error, // Experimental in `core` crate.
};

/// The `IcuError` type consists of the follow:
///
/// * `Properties`: Requires `buffer` feature. Wraps the ICU4X [`PropertiesError`],
///
/// * `Segmenter`: Requires `buffer` feature. Wraps the ICU4X [`SegmenterError`],
///
/// * `Data`: Requires `buffer` feature. Wraps the ICU4X [`DataError`],
///
/// * `Grapheme`: Indicates no ICU4X provider for `GraphemeClusterSegmenter`,
///
/// * `Syntax`: Indicates no ICU4X provider for `Pattern_Syntax`,
///
/// * `WhiteSpace`: Indicates no ICU4X provider for `Pattern_White_Space`.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum IcuError {
    #[cfg(feature = "buffer")]
    Properties(PropertiesError),

    #[cfg(feature = "buffer")]
    Segmenter(SegmenterError),

    #[cfg(feature = "buffer")]
    Data(DataError),

    Grapheme,
    Syntax,
    WhiteSpace,
}

impl LocalisationErrorTrait for IcuError {}

impl LocalisationTrait for IcuError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("IcuError".to_string());
        match self {
            #[cfg(feature = "buffer")]
            IcuError::Properties(ref error) => {
                // Currently no localisation is available for this error type: PropertiesError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Properties".to_string()),
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

            #[cfg(feature = "buffer")]
            IcuError::Segmenter(ref error) => {
                // Currently no localisation is available for this error type: SegmenterError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Segmenter".to_string()),
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

            #[cfg(feature = "buffer")]
            IcuError::Data(ref error) => {
                // Currently no localisation is available for this error type: DataError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Data".to_string()),
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

            IcuError::Grapheme => {
                let message = LocalisationData {
                    component: "i18n_icu".to_string(),
                    identifier: "grapheme".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Grapheme".to_string()),
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
            IcuError::Syntax => {
                let message = LocalisationData {
                    component: "i18n_icu".to_string(),
                    identifier: "syntax".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Grapheme".to_string()),
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
            IcuError::WhiteSpace => {
                let message = LocalisationData {
                    component: "i18n_icu".to_string(),
                    identifier: "white_space".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Grapheme".to_string()),
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

impl Display for IcuError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            #[cfg(feature = "buffer")]
            IcuError::Properties(ref error) => {
                write!(formatter, "IcuError::Properties: [{}].", error.to_string())
            }

            #[cfg(feature = "buffer")]
            IcuError::Segmenter(ref error) => {
                write!(formatter, "IcuError::Segmenter: [{}].", error.to_string())
            }

            #[cfg(feature = "buffer")]
            IcuError::Data(ref error) => {
                write!(formatter, "IcuError::Data: [{}].", error.to_string())
            }

            IcuError::Grapheme => write!(
                formatter,
                "No data provider is available for the ‘GraphemeClusterSegmenter’."
            ),
            IcuError::Syntax => write!(
                formatter,
                "No data provider is available for the ‘Pattern_Syntax’."
            ),
            IcuError::WhiteSpace => write!(
                formatter,
                "No data provider is available for the ‘Pattern_White_Space’."
            ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for IcuError {}

#[cfg(feature = "buffer")]
impl From<PropertiesError> for IcuError {
    fn from(error: PropertiesError) -> IcuError {
        IcuError::Properties(error)
    }
}

#[cfg(feature = "buffer")]
impl From<SegmenterError> for IcuError {
    fn from(error: SegmenterError) -> IcuError {
        IcuError::Segmenter(error)
    }
}

#[cfg(feature = "buffer")]
impl From<DataError> for IcuError {
    fn from(error: DataError) -> IcuError {
        IcuError::Data(error)
    }
}
