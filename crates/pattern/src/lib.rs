// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! TO be completed
//! 
//! # Examples
//! 
//! ```
//! Construct example once all public methods and unit tests are completed.
//! ```
//! 
//! [`BufferProvider`]: https://docs.rs/icu_provider/latest/icu_provider/buf/trait.BufferProvider.html
//! [Unicode Consortium]: https://home.unicode.org/
//! [CLDR]: https://cldr.unicode.org/
//! [ICU4X]: https://github.com/unicode-org/icu4x

use i18n_lexer::{Token, TokenType};
use std::rc::Rc;
use tree::{Tree, NodeFeatures};
use std::collections::HashMap;

/// Constructs a valid syntax tree from the supplied `Vec<Rc<Token>>`. Any grammar error that occurs will result in
/// an `Err()` result to be returned.
/// 
/// Implicit syntax tokens and optional whitespace tokens are not included in syntax trees.
/// 
/// Future: Add pattern types as they become available in the ICU library.
/// Current: Only “as is” strings is supported. Others such as decimal numbers, date and time will be added soon
/// once basic functionality of parser is confirmed.
pub fn parse( tokens: Vec<Rc<Token>> ) ->
Result<( Tree, HashMap<&str, usize>, HashMap<&str, usize> ), String> {
    if tokens.len() == 0 {
        return Err( "Empty token vector!".to_string() );
    }
    let container = NodeFeatures { allow_children: true, allow_data: false };
    let leaf = NodeFeatures { allow_children: false, allow_data: true };
    let mut tree = Tree::new();
    tree.insert( 0, container.clone(), Box::new( NodeType::Root ) ).ok();
    let root = 0;

    let mut parser = Parser {
        current: Some( 0 ),
        state: ParserStates::String,
        nested_states: Vec::<ParserStates>::new(),
    };
    let mut named_strings = HashMap::<&str, usize>::new();
    let mut patterns = HashMap::<&str, usize>::new();
    let mut iterator = tokens.iter();
//    ( tree, parser ) = self.create_node( tree, parser, NodeType::String, false );

    Err( "Temporary. Should end with Ok()".to_string() )
}

#[derive( PartialEq )]
pub enum NodeType {
    Root, // Container: The top level, which may optional contained NamedGroup container, and required String container.
    NamedGroup, // Container: Exists if at least 1 named substring (NamedString node) is detected.
    NamedString, // Container: Contains the Identifier leaf, and its substring String container
    String, // Container: represents either whole string, or a substring for a plural or select pattern.
    Text, // Leaf: Just literal text, and consist of 1 or more tokens (of any type that are treated as text).
    NumberSign, // Leaf: The number pattern `#` in text. Does not contain the token as it is always a `#`.
    Command, // Container: Contains command pattern data.
    Pattern, // Container: Usually a multilingual pattern data. 2nd node indicates pattern type.
    Identifier, // Leaf: An identifier. Always 1 token.
    Selector, // Container: contains 2 Identifier nodes. Used for `plural` and `select` patterns.
}



// Internal structures, enums, etc.

// Various ParserStates the tokens may be in.
#[derive( PartialEq, Copy, Clone )]
enum ParserStates {
    NamedGroup, // section for holding all the named strings
    NamedString, // a labelled substring for Select and Plural selection options.
    String, // indicates the outer most string.
    SubString, // indicates the string part of the NamedString.
    LiteralText, // ends with backtick, found in patterns `
    Pattern, // ends with matching }
    Literal, // a pattern only containing literal text, starts with backtick `
    Command, // a pattern that starts with number sign # and creates static text from supplied parameters.
    Complex, // a pattern of either `plural` or `select`.
    Decimal, // a decimal pattern
    DateTime, // a date_time pattern
}

struct Parser {
    current: Option<usize>,
    state: ParserStates,
    nested_states: Vec<ParserStates>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
    }
}

