// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_icu-rizzen-yazston` crate.

use i18n_utility::{ LocalisationTrait, LocalisationErrorTrait };

#[cfg( feature = "buffer" )]
use icu_properties::PropertiesError;

#[cfg( feature = "buffer" )]
use icu_segmenter::SegmenterError;

#[cfg( feature = "buffer" )]
use icu_provider::DataError;

#[cfg( doc )]
use icu_properties::PropertiesError;

#[cfg( doc )]
use icu_segmenter::SegmenterError;

#[cfg( doc )]
use icu_provider::DataError;

use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

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
#[derive( Debug )]
#[non_exhaustive]
pub enum IcuError {
    #[cfg( feature = "buffer" )]
    Properties( PropertiesError ),

    #[cfg( feature = "buffer" )]
    Segmenter( SegmenterError ),

    #[cfg( feature = "buffer" )]
    Data( DataError ),

    Grapheme,
    Syntax,
    WhiteSpace,
}

impl LocalisationTrait for IcuError {
    fn identifier( &self ) -> &str {
        match *self {
            IcuError::Grapheme => "Grapheme",
            IcuError::Syntax => "Syntax",
            IcuError::WhiteSpace => "WhiteSpace",

            #[allow( unreachable_patterns )]
            _ => "",
        }
    }

    fn component( &self ) -> &str {
        "i18n_icu"
    }
}

impl LocalisationErrorTrait for IcuError {
    fn error_type( &self ) -> &str {
        "IcuError"
    }

    fn error_variant( &self ) -> &str {
        match *self {
            #[cfg( feature = "buffer" )]
            IcuError::Properties( _ ) => "Properties",

            #[cfg( feature = "buffer" )]
            IcuError::Segmenter( _ ) => "Segmenter",

            #[cfg( feature = "buffer" )]
            IcuError::Data( _ ) => "Data",
            
            IcuError::Grapheme => "Grapheme",
            IcuError::Syntax => "Syntax",
            IcuError::WhiteSpace => "WhiteSpace",
        }
    }    
}

impl Display for IcuError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            #[cfg( feature = "buffer" )]
            IcuError::Properties( ref error ) => write!(
                formatter, "IcuError::Properties: [{}].", error.to_string()
            ),

            #[cfg( feature = "buffer" )]
            IcuError::Segmenter( ref error ) => write!(
                formatter, "IcuError::Segmenter: [{}].", error.to_string()
            ),

            #[cfg( feature = "buffer" )]
            IcuError::Data( ref error ) => write!(
                formatter, "IcuError::Data: [{}].", error.to_string()
            ),
            
            IcuError::Grapheme =>  write!(
                formatter, "No data provider is available for the ‘GraphemeClusterSegmenter’."
            ),
            IcuError::Syntax => write!( formatter, "No data provider is available for the ‘Pattern_Syntax’." ),
            IcuError::WhiteSpace => write!(
                formatter, "No data provider is available for the ‘Pattern_White_Space’."
            ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for IcuError {}

#[cfg( feature = "buffer" )]
impl From<PropertiesError> for IcuError {
    fn from( error: PropertiesError ) -> IcuError {
        IcuError::Properties( error )
    }
}

#[cfg( feature = "buffer" )]
impl From<SegmenterError> for IcuError {
    fn from( error: SegmenterError ) -> IcuError {
        IcuError::Segmenter( error )
    }
}

#[cfg( feature = "buffer" )]
impl From<DataError> for IcuError {
    fn from( error: DataError ) -> IcuError {
        IcuError::Data( error )
    }
}
