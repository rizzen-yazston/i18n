// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

use icu_properties::PropertiesError;
use icu_segmenter::SegmenterError;
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug, Copy, Clone )]
#[non_exhaustive]
pub enum LexerError {
    Properties( PropertiesError ),
    Segmenter( SegmenterError ),
}

impl Display for LexerError {

    /// Simply call the display formatter of embedded error.
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match *self {
            LexerError::Properties( ref error ) => error.fmt( formatter ),
            LexerError::Segmenter( ref error ) => error.fmt( formatter ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for LexerError {}

impl From<PropertiesError> for LexerError {
    fn from( error: PropertiesError ) -> LexerError {
        LexerError::Properties( error )
    }
}

impl From<SegmenterError> for LexerError {
    fn from( error: SegmenterError ) -> LexerError {
        LexerError::Segmenter( error )
    }
}
