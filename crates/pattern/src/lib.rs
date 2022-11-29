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
pub fn parse( tokens: Vec<Rc<Token<'static>>> ) ->
Result<( Tree, HashMap<&str, usize>, HashMap<&str, usize> ), String> {
    if tokens.len() == 0 {
        return Err( "Empty token vector!".to_string() );
    }
    let container = NodeFeatures { allow_children: true, allow_data: false };
    let leaf = NodeFeatures { allow_children: false, allow_data: true };
    let mut tree = Tree::new();
    tree.insert( 0, container.clone(), Box::new( NodeType::Root ) ).ok();
    let current = tree.insert(
        0,
        container.clone(),
        Box::new( NodeType::String )
    ).ok();
    let mut parser = Parser {
        current,
        state: ParserStates::String,
        nested_states: Vec::<ParserStates>::new(),
    };
    let mut named_strings = HashMap::<&str, usize>::new();
    let mut patterns = HashMap::<&str, usize>::new();
    let mut iterator = tokens.iter();
    while let Some( token ) = iterator.next() {
        match parser.state {
            ParserStates::String => {
                //  Valid tokens: PWS, `, #, {, Identifier, Syntax
                match token.token_type {
                    TokenType::Identifier | TokenType::WhiteSpace | TokenType::Syntax => {
                        add_token( &mut tree, &mut parser, NodeType::Text, leaf.clone(), token.clone() );
                    },
                    TokenType::Grammar => {
                        match 

                        parser.current = Some( 0usize );
                        create_node( &mut tree, &mut parser, NodeType::NamedGroup, container.clone() );
                        parser.state = ParserStates::NamedGroup;
                    },
                    /*
                    TokenType::CurlyBracketLeft => {
                        let mut iterator_peeking = iterator.clone();
                        // TODO: redo with "if let else" feature as there is actually two error types.
                        match iterator_peeking.next() {
                            None => return Err( "Pattern ended abruptly.".to_string() ),
                            Some( token_next ) => {
                                ( tree, patterns ) = match self.pattern_start(
                                    tree, token_next.clone(), patterns
                                ) {
                                    Ok( result ) => result,
                                    Err( error ) => return Err( error )
                                };
                                iterator = iterator_peeking; // Skip over { and next token.
                            }
                        }
                    },
                    TokenType::GraveAccent => {
                        let mut iterator_peeking = iterator.clone();
                        // TODO: redo with "if let else" feature as there is actually two error types.
                        if let Some( token_next ) = iterator_peeking.next() {
                            tree = self.add_token( tree, NodeType::Text, token_next.clone() );
                            iterator = iterator_peeking; // Skip over ` token.
                        }
                    },
                    */
                    _ => {
                        return Err( "Invalid token found.".to_string() );
                    }
                }
            },
            _ => {} // Temporary until all states have been added.
        }


//    create_node( &mut tree, &mut parser, NodeType::String, container.clone() );



    }
    if parser.nested_states.len() > 0 {
        return Err( "String ended abruptly.".to_string() );
    }
    // Final check each select and plural that the branch exists in named string group, and that the branch `other`
    // does exist.
    // Get code from old parser.rs and adapt.

    // once done change to: Ok( tree )
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

// Move `current` to its parent node only if `current` is a leaf node.
// Usually this signals the leaf has all its tokens.
fn move_to_container( tree: &Tree, parser: &mut Parser, ) {
    let node_index = parser.current.take().unwrap();
    if !tree.features( node_index ).ok().unwrap().allow_children {
        parser.current = tree.parent( node_index ).ok(); // Root node always a container.
    }
}

// Create a new child node of a specified type.
// Also changes the parser current node index to the newly created child node.
fn create_node(
    tree: &mut Tree,
    parser: &mut Parser,
    node_type: NodeType,
    features: NodeFeatures,
) {
    move_to_container( tree, parser );
    parser.current = tree.insert(
        parser.current.take().unwrap(),
        features,
        Box::new( node_type )
    ).ok();
}

// Create a new child node of a specified type, and add the Token to this new node.
// Also changes the parser current node index to the newly created node.
fn create_node_add_token(
    tree: &mut Tree,
    parser: &mut Parser,
    node_type: NodeType,
    features: NodeFeatures,
    token: Rc<Token<'static>>
) {
    move_to_container( tree, parser );
    parser.current = tree.insert(
        parser.current.take().unwrap(),
        features,
        Box::new( node_type )
    ).ok();
    tree.data_mut( *parser.current.as_ref().unwrap() ).unwrap().push( Box::new( token.clone() ) );
}

// Add a token to current leaf node.
// Use this if the node type is known to be correct for current node, as no checks are done.
fn add_token_at_current( tree: &mut Tree, parser: &Parser, token: Rc<Token<'static>> ) {
    tree.data_mut( *parser.current.as_ref().unwrap() ).unwrap().push( Box::new( token.clone() ) );
}

// Add to a token to a specified node type.
// If current node is not the specified node type, a specified node type will be created, and current node is set
// to it, before adding the token to the node.
fn add_token(
    tree: &mut Tree,
    parser: &mut Parser,
    node_type: NodeType,
    features: NodeFeatures,
    token: Rc<Token<'static>>
) {
    let current = *parser.current.as_ref().unwrap();
    let node_type_ref = tree.node_type( current ).ok().unwrap().as_ref();
    let node_type_is = node_type_ref.downcast_ref::<NodeType>().unwrap();
    if *node_type_is == node_type {
        tree.data_mut( current ).unwrap().push( Box::new( token.clone() ) );
    } else {
        create_node_add_token( tree, parser, node_type, features, token );
    }
}

/*
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 514, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 5usize ) ).ok();
    /// let type_any_ref = tree.node_type( 0 ).ok().unwrap().as_ref();
    /// let type_usize = type_any_ref.downcast_ref::<usize>().unwrap();
    /// assert_eq!( *type_usize, 5 );
    /// ```
*/


// End the current nested state, change back to previous state and move to parent node.
fn end_nested_state( tree: &Tree, parser: &mut Parser ) {
    parser.state = match parser.nested_states.pop() {
        Some( s ) => s,
        None => ParserStates::String
    };
    parser.current = tree.parent( parser.current.take().unwrap() ).ok();
}


    /*
    /// ```
    /// use tree::{NodeFeatures, Tree};
    /// 
    /// let mut tree = Tree::new();
    /// tree.insert( 974, NodeFeatures { allow_children: true, allow_data: true }, Box::new( 0usize ) ).ok();
    /// tree.data_mut( 0 ).unwrap().push( Box::new( "String data".to_string() ) );
    /// let data_vec_mut = tree.data_mut( 0 ).ok().unwrap();
    /// let data = data_vec_mut.get_mut( 0 ).unwrap().downcast_mut::<String>().unwrap();
    /// 
    /// // mutate the data
    /// *data = "Mutated data".to_string();
    /// 
    /// // Take node to check if data did mutate.
    /// let mut data_vec = tree.take( 0 ).ok().unwrap().unwrap(); // Deleting root node, and take data.
    /// let data_taken = data_vec.pop().unwrap().downcast::<String>().ok().unwrap();
    /// assert_eq!( tree.count(), 0, "0 nodes are present." );
    /// assert_eq!( *data_taken, "Mutated data".to_string(), "Data of node is a mutated string" );
    /// ```
    */


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
    }
}

