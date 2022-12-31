// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

//! TODO: Complete crate description
//! 
//! # Examples
//! 
//! ```
//! // TODO: crate example
//! ```

use icu_locid::Locale;
use std::rc::Rc;
use i18n_lstring::LString;
use i18n_lexer::{Lexer, Token, TokenType};
use tree::{Tree, NodeFeatures};
use i18n_pattern::{ParserResult, NodeType};
use std::collections::HashMap;
//use std::fmt; this is needed if implementing Display on something

pub struct System {
    lexer: Lexer,
}

impl System {
    pub fn try_new( lexer: Lexer ) -> Result<Self, String> {
        Err( "Temporary error until implemented.".to_string() )
    }
}

pub struct Formatter {
    locale: Rc<Locale>,
    parser_result: ParserResult,
}

impl Formatter {

    /// Creates a Formatter for the language string using parsing results.
    pub fn try_new( locale: Rc<Locale>, parser_result: ParserResult ) -> Self {
        Formatter { locale, parser_result }
    }
    
    /// Format the language string with supplied values.
    pub fn format( self ) -> Result<LString, String> {
        Err( "Temporary error until implemented.".to_string() )
    }


}







#[cfg(test)]
mod tests {
    use super::*;
    use icu_testdata::buffer;

    #[test]
    fn test1() {
    }

    #[test]
    fn test2() {
    }
}

