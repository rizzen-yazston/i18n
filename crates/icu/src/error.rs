// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_icu-rizzen-yazston` crate.

#[cfg( feature = "buffer" )]
use icu_properties::PropertiesError;

#[cfg( feature = "buffer" )]
use icu_segmenter::SegmenterError;

#[cfg( feature = "buffer" )]
use icu_provider::DataError;

use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

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

impl Display for IcuError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            #[cfg( feature = "buffer" )]
            IcuError::Properties( ref error ) => return error.fmt( formatter ),

            #[cfg( feature = "buffer" )]
            IcuError::Segmenter( ref error ) => return error.fmt( formatter ),

            #[cfg( feature = "buffer" )]
            IcuError::Data( ref error ) => return error.fmt( formatter ),
            
            IcuError::Grapheme =>  write!( formatter, "No data provider is available for the ‘GraphemeClusterSegmenter’." ),
            IcuError::Syntax => write!( formatter, "No data provider is available for the ‘Pattern_Syntax’." ),
            IcuError::WhiteSpace => write!( formatter, "No data provider is available for the ‘Pattern_White_Space’." ),
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
