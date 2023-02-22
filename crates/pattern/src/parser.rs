// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! Parsing of string tokens into an Abstract Syntax Tree (AST), checking the grammar of patterns is valid.
//! The parser only does the syntactic analysis of the supplied Token vector.
//! 
//! See `pattern strings.asciidoc` in `docs` of `pattern` for the pattern formatting specification.
//! 
//! # Examples
//! 
//! ```
//! use icu_provider::prelude::*;
//! use std::rc::Rc;
//! use i18n_lexer::{ Token, TokenType, Lexer };
//! use icu_testdata::buffer;
//! use i18n_pattern::{ parse, NodeType };
//! 
//! let buffer_provider = Box::new( buffer() );
//! let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
//! let tokens = lexer.tokenise(
//!     "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}",
//!     &vec![ '{', '}', '`', '#' ]
//! );
//! let tree = match parse( tokens ) {
//!     Err( error ) => {
//!         println!( "Error: {}", error );
//!         std::process::exit( 1 )
//!     },
//!     Ok( result ) => result
//! };
//! let len = tree.len();
//! let mut index = 0;
//! while index < len {
//!     println!( "Index: {}", index );
//!     let node_type_data = tree.node_type( index ).ok().unwrap();
//!     let node_type = node_type_data.downcast_ref::<NodeType>().unwrap();
//!     println!( "Type: {}", node_type );
//!     let mut string = String::new();
//!     match tree.children( index ).ok() {
//!         None => string.push_str( "None" ),
//!         Some( children ) => {
//!             for child in children.iter() {
//!                 string.push_str( &child.to_string() );
//!                 string.push( ' ' );
//!             }
//!         }
//!     }
//!     println!( "Children: {}", string );
//!     let mut string = String::new();
//!     match tree.data_ref( index ).ok() {
//!         None => string.push_str( "None" ),
//!         Some( tokens ) => {
//!             for token_ref in tokens.iter() {
//!                 let token = token_ref.downcast_ref::<Rc<Token>>().unwrap();
//!                 string.push( '`' );
//!                 string.push_str( token.string.as_str() );
//!                 string.push_str( "`, " );
//!             }
//!         }
//!     }
//!     println!( "Tokens: {}", string );
//!     index += 1;
//! }
//! ```

use i18n_error::{ ErrorMessage, ErrorPlaceholderValue };
use crate::types::*;
use i18n_lexer::{ Token, TokenType };
use tree::{ Tree, NodeFeatures };
use std::error::Error; // Experimental in `core` crate.
use std::rc::Rc;
use std::collections::HashMap;
use core::fmt::{ Debug, Display, Formatter, Result as FmtResult };

/// Constructs a valid syntax tree from the supplied `Vec<Rc<Token>>`. Any grammar error that occurs will result in     
/// an `Err()` result to be returned.
/// 
/// Implicit syntax tokens and optional whitespace tokens are not included in syntax trees.
/// 
/// Future: Add pattern types as they become available in the ICU library.
/// 
/// # Examples
/// 
/// ```
/// use icu_provider::prelude::*;
/// use std::rc::Rc;
/// use i18n_lexer::{Token, TokenType, Lexer};
/// use icu_testdata::buffer;
/// use i18n_pattern::{parse, NodeType};
/// 
/// let buffer_provider = Box::new( buffer() );
/// let mut lexer = match Lexer::try_new( &buffer_provider ) {
///     Err( error ) => {
///         println!( "{}", error );
///         std::process::exit( 1 )
///     },
///     Ok( result ) => result
/// };
/// let tokens = lexer.tokenise(
///     "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}",
///     &vec![ '{', '}', '`', '#' ]
/// );
/// let tree = match parse( tokens ) {
///     Err( error ) => {
///         println!( "Error: {}", error );
///         assert!( false );
///         std::process::exit( 1 )
///     },
///     Ok( result ) => result
/// };
/// assert_eq!( tree.len(), 24, "Should contain 24 nodes." );
/// ```
pub fn parse( tokens: Vec<Rc<Token>> ) -> Result<Tree, ParserError> {
    let mut tree = Tree::new();
    if tokens.len() == 0 {
        return Ok( tree );
    }
    tree.insert( 0, CONTAINER, Box::new( NodeType::Root ) ).ok();
    let mut parser = Parser {
        current: tree.insert(
            0,
            CONTAINER,
            Box::new( NodeType::String )
        ).ok(),
        state: ParserStates::String,
        nested_states: Vec::<ParserStates>::new(),
    };
    let mut named_strings = HashMap::<String, usize>::new();
    let mut patterns = HashMap::<String, usize>::new();
    let mut iterator = tokens.iter();
    while let Some( token ) = iterator.next() {
        match parser.state {
            ParserStates::String => {//  Valid tokens: PWS, `, #, {, Identifier, Syntax
                if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly at literal marker." ),
                                identifier: String::from( "i18n_pattern/incomplete_literal" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            LEAF,
                            Rc::clone( &token_next )
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else if string == "{" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly at pattern marker." ),
                                identifier: String::from( "i18n_pattern/incomplete_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        pattern_start(
                            &mut tree,
                            &mut parser,
                            Rc::clone( &token_next ),
                            &mut patterns
                        )?;
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "#" {
                        parser.current = Some( 0 ); // Move to root.
                        create_node( &mut tree, &mut parser, NodeType::NamedGroup, CONTAINER );
                        parser.state = ParserStates::NamedGroup;
                    } else {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                } else {
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Text,
                        LEAF,
                        Rc::clone( &token )
                    );
                }
            },
            ParserStates::SubString => {//  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "#" {
                        // TODO: Check if previous node is not NumberSign, adjacent NumberSign not allowed
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            LEAF,
                            Rc::clone( &token )
                        );
                        parser.state = ParserStates::SubString;
                    } else if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly at literal marker." ),
                                identifier: String::from( "i18n_pattern/incomplete_literal" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            LEAF,
                            Rc::clone( &token_next )
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else if string == "{" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly at pattern marker." ),
                                identifier: String::from( "i18n_pattern/incomplete_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        match pattern_start(
                            &mut tree,
                            &mut parser,
                            Rc::clone( &token_next ),
                            &mut patterns
                        ) {
                            Ok( _ ) => {},
                            Err( error ) => return Err( error )
                        };
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "}" {
                        move_to_container( &tree, &mut parser );
                        end_nested_state( &tree, &mut parser ); // Return to NamedString state
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 2 {
                            // Fix values with values.
                            return Err( ParserError::InvalidToken( ErrorMessage {
                                string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                                identifier: String::from( "i18n_pattern/invalid_token" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        }

                        // Ends NamedString, and returns to NamedGroup
                        end_nested_state( &tree, &mut parser );
                    } else {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                } else {
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Text,
                        LEAF,
                        Rc::clone( &token )
                    );
                }
            },
            ParserStates::Pattern => {//  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    let string = token.string.as_str();
                    if string == "plural" || string == "select" {
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Identifier,
                            LEAF, 
                            Rc::clone( &token )
                        );
                        move_to_container( &tree, &mut parser );
                        parser.state = ParserStates::Complex;
                    }
                    else if string == "decimal" {
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Identifier,
                            LEAF, 
                            Rc::clone( &token )
                        );
                        move_to_container( &tree, &mut parser );
                        parser.state = ParserStates::Decimal;
                    }
                    else if string == "date_time" {
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Identifier,
                            LEAF, 
                            Rc::clone( &token )
                        );
                        move_to_container( &tree, &mut parser );
                        parser.state = ParserStates::DateTime;
                    }
                    // TODO: add the other types, and also to the ParserStates
                    else {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string != "}" {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                    // None (only identifier provide with default type of preformatted string)
                    end_nested_state( &tree, &mut parser );
                } else {
                    // Fix values with values.
                    return Err( ParserError::InvalidToken( ErrorMessage {
                        string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                        identifier: String::from( "i18n_pattern/invalid_token" ),
                        values: HashMap::<String, ErrorPlaceholderValue>::new(),
                    } ) );
                }
            },
            ParserStates::Decimal => {//  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    /*
                    if pattern_option_exists( &tree, &parser, token.string.as_str() ) {
                        return Err( ParserError::BranchExists( ErrorMessage {
                            string: String::from( "Branch value already exists." ),
                            identifier: String::from( "i18n_pattern/branch_exists" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                    */
                    let string = token.string.as_str();
                    if string == "group" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            Rc::clone( &token ),
                            Rc::clone( &token_next ),
                            Rc::clone( &token_next_2nd ),
                            vec!( "never", "always" , "min2" ),
                        )?;
                    } else if string == "sign" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            Rc::clone( &token ),
                            Rc::clone( &token_next ),
                            Rc::clone( &token_next_2nd ),
                            vec!( "never", "always" , "except_zero" , "negative" ),
                        )?;
                    } else {
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                    }
                } else {
                    return Err( ParserError::InvalidToken( ErrorMessage {
                        string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                        identifier: String::from( "i18n_pattern/invalid_token" ),
                        values: HashMap::<String, ErrorPlaceholderValue>::new(),
                    } ) );
                }
            },
            ParserStates::DateTime => {//  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    /*
                    if pattern_option_exists( &tree, &parser, token.string.as_str() ) {
                        return Err( ParserError::BranchExists( ErrorMessage {
                            string: String::from( "Branch value already exists." ),
                            identifier: String::from( "i18n_pattern/branch_exists" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                    */
                    let string = token.string.as_str();
                    if string == "date" || string == "time" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            Rc::clone( &token ),
                            Rc::clone( &token_next ),
                            Rc::clone( &token_next_2nd ),
                            vec!( "full", "long", "medium", "short" ),
                        )?;
                    } else if string == "hour" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            Rc::clone( &token ),
                            Rc::clone( &token_next ),
                            Rc::clone( &token_next_2nd ),
                            vec!( "24", "23", "12", "11" ),
                        )?;
                    } else if string == "calendar" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            Rc::clone( &token ),
                            Rc::clone( &token_next ),
                            Rc::clone( &token_next_2nd ),
                            vec!( "gregory", "buddhist", "japanese", "coptic", "indian", "ethiopic" ),
                        )?;
                    } else {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                    }
                } else {
                    // Fix values with values.
                    return Err( ParserError::InvalidToken( ErrorMessage {
                        string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                        identifier: String::from( "i18n_pattern/invalid_token" ),
                        values: HashMap::<String, ErrorPlaceholderValue>::new(),
                    } ) );
                }
            },
            ParserStates::Complex => {//  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    /*
                    if pattern_option_exists( &tree, &parser, token.string.as_str() ) {
                        return Err( ParserError::BranchExists( ErrorMessage {
                            string: String::from( "Branch value already exists." ),
                            identifier: String::from( "i18n_pattern/branch_exists" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                    */
                    create_node( &mut tree, &mut parser, NodeType::Selector, CONTAINER );
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        LEAF,
                        Rc::clone( &token )
                    );
                    move_to_container( &tree, &mut parser );
                    let Some( token_next ) = iterator.next() else {
                        return Err( ParserError::Incomplete( ErrorMessage {
                            string: String::from( "String ended abruptly within pattern." ),
                            identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                };
                    if token_next.string.as_str() != "#" {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                    let Some( token_next_2nd ) = iterator.next() else {
                        return Err( ParserError::Incomplete( ErrorMessage {
                            string: String::from( "String ended abruptly within pattern." ),
                            identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                };
                    if token_next_2nd.token_type != TokenType::Identifier {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        LEAF,
                        Rc::clone( &token_next_2nd )
                    );
                    move_to_container( &tree, &mut parser );
                    parser.current = tree.parent( parser.current.take().unwrap() ).ok();
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                    }
                } else {
                    // Fix values with values.
                    return Err( ParserError::InvalidToken( ErrorMessage {
                        string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                        identifier: String::from( "i18n_pattern/invalid_token" ),
                        values: HashMap::<String, ErrorPlaceholderValue>::new(),
                    } ) );
                }
            },
            ParserStates::LiteralText => {//  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly at literal marker." ),
                                identifier: String::from( "i18n_pattern/incomplete_literal" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        if token_next.string.as_str() == "`" {// Literal ` found.
                            add_token(
                                &mut tree,
                                &mut parser,
                                NodeType::Text,
                                LEAF,
                                Rc::clone( &token_next )
                            );
                            iterator = iterator_peeking; // Skip over 1st ` token.
                        }
                        else {
                            end_nested_state( &tree, &mut parser );
                        }
                        continue;
                    }
                }
                add_token( &mut tree, &mut parser, NodeType::Text, LEAF, Rc::clone( &token ) );
            },
            ParserStates::Literal => {//  Valid tokens: }
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                        continue;
                    }
                }
                // Fix values with values.
                return Err( ParserError::InvalidToken( ErrorMessage {
                    string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                    identifier: String::from( "i18n_pattern/invalid_token" ),
                    values: HashMap::<String, ErrorPlaceholderValue>::new(),
                } ) );
            },
            ParserStates::Command => {//  Valid tokens: PWS (separator - ignore), `, }, Identifier
                if token.token_type == TokenType::Identifier {
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        LEAF,
                        Rc::clone( &token )
                    );
                    move_to_container( &tree, &mut parser );
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "{" {
                        // None (only identifier provide with default type of preformatted string)
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            // Fix values with values.
                            return Err( ParserError::InvalidToken( ErrorMessage {
                                string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                                identifier: String::from( "i18n_pattern/invalid_token" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        }
                        end_nested_state( &tree, &mut parser );
                    } else if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly at literal marker." ),
                                identifier: String::from( "i18n_pattern/incomplete_literal" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        create_node_add_token(
                            &mut tree,&mut parser,
                            NodeType::Text,
                            LEAF,
                            Rc::clone( &token_next )
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                        parser.nested_states.push( ParserStates::Command );
                        parser.state = ParserStates::LiteralText;
                    } else {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else {
                    // Fix values with values.
                    return Err( ParserError::InvalidToken( ErrorMessage {
                        string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                        identifier: String::from( "i18n_pattern/invalid_token" ),
                        values: HashMap::<String, ErrorPlaceholderValue>::new(),
                    } ) );
                }
            },
            ParserStates::NamedString => {//  Valid tokens: PWS (ignored - separator), `, #, {, Identifier, Syntax
                if token.token_type == TokenType::Identifier {
                    let current = *parser.current.as_ref().unwrap();
                    let len = tree.children( current ).ok().as_ref().unwrap().len();
                    if len == 0 {
                        let string = token.string.as_str().to_string();
                        if named_strings.contains_key( &string ) {
                            return Err( ParserError::Unique( ErrorMessage {
                                string: String::from( "Named substrings must have unique identifiers." ),
                                identifier: String::from( "i18n_pattern/unique_named" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        }
                        named_strings.insert( string, current );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Identifier,
                            LEAF,
                            Rc::clone( &token )
                        );
                        move_to_container( &tree, &mut parser );

                        // Check that white space separator follows identifier
                        let mut iterator_peeking = iterator.clone();
                        if let Some( token_next ) = iterator_peeking.next() {
                            if token_next.token_type == TokenType::WhiteSpace {
                                iterator = iterator_peeking; // Skip over whitespace token separator.
                                continue;
                            }
                        }
                    } else if len == 1 {
                        create_node( &mut tree, &mut parser, NodeType::String, CONTAINER );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            LEAF,
                            Rc::clone( &token )
                        );
                        parser.state = ParserStates::SubString;
                        continue;
                    }
                    // Fix values with values.
                    return Err( ParserError::InvalidToken( ErrorMessage {
                        string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                        identifier: String::from( "i18n_pattern/invalid_token" ),
                        values: HashMap::<String, ErrorPlaceholderValue>::new(),
                    } ) );
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "#" {
                        // # may only appear after Identifier node indicating start of SubString.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            // Fix values with values.
                            return Err( ParserError::InvalidToken( ErrorMessage {
                                string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                                identifier: String::from( "i18n_pattern/invalid_token" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        }
                        create_node( &mut tree, &mut parser, NodeType::String, CONTAINER );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            LEAF,
                            Rc::clone( &token )
                        );
                        parser.state = ParserStates::SubString;
                    } else if string == "{" {
                        // { may only appear after Identifier node indicating start of SubString.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            // Fix values with values.
                            return Err( ParserError::InvalidToken( ErrorMessage {
                                string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                                identifier: String::from( "i18n_pattern/invalid_token" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        }
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly within pattern." ),
                                identifier: String::from( "i18n_pattern/incomplete_with_pattern" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        match pattern_start(
                            &mut tree,
                            &mut parser,
                            Rc::clone( &token_next ),
                            &mut patterns
                        ) {
                            Ok( _ ) => {},
                            Err( error ) => return Err( error )
                        };
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "`" {
                        // ` may only appear after Identifier node indicating start of SubString.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            // Fix values with values.
                            return Err( ParserError::InvalidToken( ErrorMessage {
                                string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                                identifier: String::from( "i18n_pattern/invalid_token" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        }
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::Incomplete( ErrorMessage {
                                string: String::from( "String ended abruptly at literal marker." ),
                                identifier: String::from( "i18n_pattern/incomplete_literal" ),
                                values: HashMap::<String, ErrorPlaceholderValue>::new(),
                            } ) );
                        };
                        create_node( &mut tree, &mut parser, NodeType::String, CONTAINER );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            LEAF,
                            Rc::clone( &token_next )
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                } else if token.token_type == TokenType::Syntax {

                    // Syntax may only appear after Identifier node indicating start of SubString.
                    if tree.children(
                        *parser.current.as_ref().unwrap()
                    ).ok().as_ref().unwrap().len() != 1 {
                        // Fix values with values.
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                    create_node( &mut tree, &mut parser, NodeType::String, CONTAINER );
                    parser.nested_states.push( ParserStates::NamedString );
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Text,
                        LEAF,
                        Rc::clone( &token )
                    );
                } else if token.token_type == TokenType::WhiteSpace {

                    // Valid WhiteSpace is only a separator between Identifier and Substring.
                    if tree.children(
                        *parser.current.as_ref().unwrap()
                    ).ok().as_ref().unwrap().len() != 1 {
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }
                } else {
                    return Err( ParserError::InvalidToken( ErrorMessage {
                        string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                        identifier: String::from( "i18n_pattern/invalid_token" ),
                        values: HashMap::<String, ErrorPlaceholderValue>::new(),
                    } ) );
                }
            },
            ParserStates::NamedGroup => {// Valid tokens: PWS (ignored - human readability), {
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() != "{" {
                        return Err( ParserError::InvalidToken( ErrorMessage {
                            string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                            identifier: String::from( "i18n_pattern/invalid_token" ),
                            values: HashMap::<String, ErrorPlaceholderValue>::new(),
                        } ) );
                    }

                    // start of NamedString
                    create_node( &mut tree, &mut parser, NodeType::NamedString, CONTAINER );
                    parser.nested_states.push( ParserStates::NamedGroup );
                    parser.state = ParserStates::NamedString;
                } else if token.token_type == TokenType::WhiteSpace {
                } else {
                    return Err( ParserError::InvalidToken( ErrorMessage {
                        string: error_invalid_token( &mut parser, Rc::clone( &token ) ),
                        identifier: String::from( "i18n_pattern/invalid_token" ),
                        values: HashMap::<String, ErrorPlaceholderValue>::new(),
                    } ) );
                }
            }
        }
    }
    if parser.nested_states.len() > 0 {
        return Err( ParserError::Incomplete( ErrorMessage {
            string: String::from( "String ended abruptly." ),
            identifier: String::from( "i18n_pattern/incomplete_string" ),
            values: HashMap::<String, ErrorPlaceholderValue>::new(),
        } ) );
    }
    Ok( tree )
}

#[derive( Debug )]
pub enum ParserError {
    Incomplete( ErrorMessage ),
    Unique( ErrorMessage ),
    InvalidToken( ErrorMessage ),
    BranchExists( ErrorMessage ),
}

impl Error for ParserError {}

impl Display for ParserError {

    /// Write to the formatter the default preformatted error message.
    fn fmt( &self, formatter: &mut Formatter ) -> FmtResult {
        match self {
            ParserError::Incomplete( error ) => formatter.write_str( error.string.as_str() ),
            ParserError::Unique( error) => formatter.write_str( error.string.as_str() ),
            ParserError::InvalidToken( error ) => formatter.write_str( error.string.as_str() ),
            ParserError::BranchExists( error ) => formatter.write_str( error.string.as_str() ),
        }
    }
}

// Internal structures, enums, etc.

const CONTAINER: NodeFeatures = NodeFeatures { allow_children: true, allow_data: false };
const LEAF: NodeFeatures = NodeFeatures { allow_children: false, allow_data: true };

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

impl Display for ParserStates {
    fn fmt( &self, f: &mut Formatter ) -> FmtResult {
        match *self {
            ParserStates::Command => write!( f, "Command" ),
            ParserStates::Complex => write!( f, "Complex" ),
            ParserStates::DateTime => write!( f, "DateTime" ),
            ParserStates::Decimal => write!( f, "Decimal" ),
            ParserStates::Literal => write!( f, "Literal" ),
            ParserStates::LiteralText => write!( f, "LiteralText" ),
            ParserStates::NamedGroup => write!( f, "NamedGroup" ),
            ParserStates::NamedString => write!( f, "NamedString" ),
            ParserStates::Pattern => write!( f, "Pattern" ),
            ParserStates::String => write!( f, "String" ),
            ParserStates::SubString => write!( f, "SubString" )
       }
    }
}

// A struct for tracking the parser states.
struct Parser {
    current: Option<usize>,
    state: ParserStates,
    nested_states: Vec<ParserStates>,
}

// Move `current` to its parent node only if `current` is a leaf node.
// Usually this signals the leaf has all its tokens.
fn move_to_container( tree: &Tree, parser: &mut Parser, ) {
    let node_index = *parser.current.as_ref().unwrap();
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
    token: Rc<Token>
) {
    move_to_container( tree, parser );
    parser.current = tree.insert(
        parser.current.take().unwrap(),
        features,
        Box::new( node_type )
    ).ok();
    tree.data_mut(
        *parser.current.as_ref().unwrap() ).unwrap().push( Box::new( Rc::clone( &token ) )
    );
}

// Add to a token to a specified node type.
// If current node is not the specified node type, a specified node type will be created, and current node is set
// to it, before adding the token to the node.
fn add_token(
    tree: &mut Tree,
    parser: &mut Parser,
    node_type: NodeType,
    features: NodeFeatures,
    token: Rc<Token>
) {
    let current = *parser.current.as_ref().unwrap();
    let node_type_ref = tree.node_type( current ).ok().unwrap().as_ref();
    let node_type_is = node_type_ref.downcast_ref::<NodeType>().unwrap();
    if *node_type_is == node_type {
        tree.data_mut( current ).ok().unwrap().push( Box::new( Rc::clone( &token ) ) );
    } else {
        create_node_add_token( tree, parser, node_type, features, token );
    }
}

// End the current nested state, change back to previous state and move to parent node.
fn end_nested_state( tree: &Tree, parser: &mut Parser ) {
    parser.state = match parser.nested_states.pop() {
        Some( s ) => s,
        None => ParserStates::String
    };
    parser.current = tree.parent( parser.current.take().unwrap() ).ok();
}

// Check if start of pattern is valid.
fn pattern_start(
    tree: &mut Tree,
    parser: &mut Parser,
    token_next: Rc<Token>,
    patterns: &mut HashMap<String, usize>,
) -> Result<(), ParserError> {
    if token_next.token_type == TokenType::Identifier {
        // Multilingual pattern
        create_node( tree, parser, NodeType::Pattern, CONTAINER );
        if patterns.contains_key( &token_next.string ) {
            return Err( ParserError::Unique( ErrorMessage {
                string: String::from( "Pattern identifiers must have unique." ),
                identifier: String::from( "i18n_pattern/unique_identifiers" ),
                values: HashMap::<String, ErrorPlaceholderValue>::new(),
            } ) );
        }
        patterns.insert( token_next.string.as_str().to_string(), *parser.current.as_ref().unwrap() );
        create_node_add_token( tree, parser, NodeType::Identifier, LEAF, token_next );
        move_to_container( tree, parser ); // Move back to Pattern node.
        parser.nested_states.push( parser.state );
        parser.state = ParserStates::Pattern;
    } else if token_next.token_type == TokenType::Grammar {
        // Future: may allow empty {} to be treated as literal {}
        if token_next.string.as_str() == "`" {
            // Literal pattern.
            create_node( tree, parser, NodeType::Text, LEAF );
            parser.nested_states.push( parser.state );
            parser.nested_states.push( ParserStates::Literal );
            parser.state = ParserStates::LiteralText;
        } else if token_next.string.as_str() == "#" {
            // Command pattern.
            create_node( tree, parser, NodeType::Command, CONTAINER );
            parser.nested_states.push( parser.state );
            parser.state = ParserStates::Command;
        } else {
            // Fix values with values.
            return Err( ParserError::InvalidToken( ErrorMessage {
                string: error_invalid_token( parser, Rc::clone( &token_next ) ),
                identifier: String::from( "i18n_pattern/invalid_token" ),
                values: HashMap::<String, ErrorPlaceholderValue>::new(),
            } ) );
        }
    } else {
        // Fix values with values.
        return Err( ParserError::InvalidToken( ErrorMessage {
            string: error_invalid_token( parser, Rc::clone( &token_next ) ),
            identifier: String::from( "i18n_pattern/invalid_token" ),
            values: HashMap::<String, ErrorPlaceholderValue>::new(),
        } ) );
    }
    Ok( () )
}

/*
// Checks if an option selector already exists.
fn pattern_option_exists( tree: &Tree, parser: &Parser, identifier: &str ) -> bool {
    let mut iterator =
        tree.children( *parser.current.as_ref().unwrap() ).ok().as_ref().unwrap().iter().skip( 2 );
    while let Some( selector ) = iterator.next() {
        if let Some( option ) =
            tree.children( *selector ).ok().as_ref().unwrap().first()
        {
            let option_data = tree.data_ref( *option ).ok().unwrap();
            if let Some( option_token ) =
                option_data.first().unwrap().downcast_ref::<Rc<Token>>()
            {
                if option_token.string.as_str() == identifier {
                    return true;
                }
            }
        }
    }
    false
}
*/

// Check a selector is correctly composed and create the selector node.
// Selector is always in format "identifier#identifier".
fn pattern_selector(
    tree: &mut Tree,
    parser: &mut Parser,
    token: Rc<Token>,
    token_next: Rc<Token>,
    token_next_2nd: Rc<Token>,
    keywords: Vec<&str>,
) -> Result<(), ParserError> {
    create_node( tree, parser, NodeType::Selector, CONTAINER );
    add_token( tree, parser, NodeType::Identifier, LEAF, Rc::clone( &token ) );
    move_to_container( &tree, parser );
    if token_next.string.as_str() != "#" {
        return Err( ParserError::InvalidToken( ErrorMessage {
            string: error_invalid_token( parser, Rc::clone( &token_next ) ),
            identifier: String::from( "i18n_pattern/invalid_token" ),
            values: HashMap::<String, ErrorPlaceholderValue>::new(),
        } ) );
    }
    if token_next_2nd.token_type != TokenType::Identifier {
        return Err( ParserError::InvalidToken( ErrorMessage {
            string: error_invalid_token( parser, Rc::clone( &token_next_2nd ) ),
            identifier: String::from( "i18n_pattern/invalid_token" ),
            values: HashMap::<String, ErrorPlaceholderValue>::new(),
        } ) );
    }
    if keywords.contains( &token_next_2nd.string.as_str() ) {
        add_token(
            tree,
            parser,
            NodeType::Identifier,
            LEAF,
            Rc::clone( &token_next_2nd )
        );
        move_to_container( &tree, parser );
        parser.current = tree.parent( parser.current.take().unwrap() ).ok();
    } else {
        return Err( ParserError::InvalidToken( ErrorMessage {
            string: error_invalid_token( parser, Rc::clone( &token_next_2nd ) ),
            identifier: String::from( "i18n_pattern/invalid_token" ),
            values: HashMap::<String, ErrorPlaceholderValue>::new(),
        } ) );
    }
    Ok( () )
}

// Creates an error String for invalid token, providing the position in the original string.
fn error_invalid_token( parser: &mut Parser, token: Rc<Token> ) -> String {
    let mut string = String::new();
    string.push_str( "Parser state [" );
    string.push_str( parser.state.to_string().as_str() );
    string.push_str( "] Invalid token `" );
    string.push_str( token.string.as_str() );
    string.push_str( "` at position " );
    string.push_str( token.position_grapheme.to_string().as_str() );
    string
}

// Get the node type as a string for the current node.
// May be of use in generating future detailed error messages
#[allow( dead_code )]
fn node_type_to_string( tree: &Tree, parser: &mut Parser, ) -> String {
    let current = *parser.current.as_ref().unwrap();
    let node_type_ref = tree.node_type( current ).ok().unwrap().as_ref();
    node_type_ref.downcast_ref::<NodeType>().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use i18n_lexer::Lexer;
    use icu_testdata::buffer;

    #[test]
    fn decimal() {
        let buffer_provider = Box::new( buffer() );
        let mut lexer = match Lexer::try_new( &buffer_provider ) {
            Err( error ) => {
                println!( "{}", error );
                std::process::exit( 1 )
            },
            Ok( result ) => result
        };
        let tokens = lexer.tokenise(
            "String contains a {placeholder decimal sign#negative}.", &vec![ '{', '}', '`', '#' ]
        );
        let tree = match parse( tokens ) {
            Err( error ) => {
                println!( "Error: {}", error );
                assert!( false );
                std::process::exit( 1 )
            },
            Ok( result ) => result
        };
        assert_eq!( tree.len(), 10, "Should contain 10 nodes." );
    }

    #[test]
    fn plural() {
        let buffer_provider = Box::new( buffer() );
        let mut lexer = match Lexer::try_new( &buffer_provider ) {
            Err( error ) => {
                println!( "{}", error );
                std::process::exit( 1 )
            },
            Ok( result ) => result
        };
        let tokens = lexer.tokenise(
            "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}",
            &vec![ '{', '}', '`', '#' ]
        );
        let tree = match parse( tokens ) {
            Err( error ) => {
                println!( "Error: {}", error );
                assert!( false );
                std::process::exit( 1 )
            },
            Ok( result ) => result
        };
        assert_eq!( tree.len(), 24, "Should contain 24 nodes." );
    }
}

