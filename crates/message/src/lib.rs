// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

//! TODO: Complete crate description
//! 
//! # Examples
//! 
//! ```
//! // TODO: crate example
//! ```

use i18n_lexer::{Lexer, Token, TokenType};
use std::rc::Rc;
use tree::{Tree, NodeFeatures};
use i18n_pattern::{NodeType, parse};
use std::collections::HashMap;
//use std::fmt; this is needed if implementing Display on something

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

