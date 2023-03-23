// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

use i18n_lexer::LexerError;
use i18n_pattern::{ ParserError, FormatterError };
use std::error::Error; // Experimental in `core` crate.
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug )]
#[non_exhaustive]
pub enum MessageError {
    Lexer( LexerError ),
    Parser( ParserError ),
    Formatter( FormatterError ),
}

impl Display for MessageError {

    /// Simply call the display formatter of embedded error.
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match *self {
            MessageError::Lexer( ref error ) => error.fmt( formatter ),
            MessageError::Parser( ref error ) => error.fmt( formatter ),
            MessageError::Formatter( ref error ) => error.fmt( formatter ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for MessageError {}

impl From<LexerError> for MessageError {
    fn from( error: LexerError ) -> MessageError {
        MessageError::Lexer( error )
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
