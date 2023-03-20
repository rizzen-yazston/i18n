// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_icu-rizzen-yazston` crate.

use icu_properties::PropertiesError;
use icu_segmenter::SegmenterError;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug )]
#[non_exhaustive]
pub enum IcuError {
    Properties( PropertiesError ),
    Segmenter( SegmenterError ),
}

impl Display for IcuError {

    /// Simply call the display formatter of embedded error.
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match *self {
            IcuError::Properties( ref error ) => error.fmt( formatter ),
            IcuError::Segmenter( ref error ) => error.fmt( formatter ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for IcuError {}

impl From<PropertiesError> for IcuError {
    fn from( error: PropertiesError ) -> IcuError {
        IcuError::Properties( error )
    }
}

impl From<SegmenterError> for IcuError {
    fn from( error: SegmenterError ) -> IcuError {
        IcuError::Segmenter( error )
    }
}
