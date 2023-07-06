// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

use crate::ParserError;
use crate::types::*;
use i18n_lexer::{ Token, TokenType };
use tree::{ Tree, ALLOW_CHILDREN, ALLOW_DATA };
use std::rc::Rc;
use std::collections::HashMap;
use core::fmt::{ Display, Formatter, Result as FmtResult };

/// Constructs a valid syntax tree [`Tree`] from the supplied [`Vec`]`<`[`Rc`]`<`[`Token`]`>>`. Any grammar error that
/// occurs will result in an `Err()` result being returned.
/// 
/// Implicit syntax tokens and optional whitespace tokens are not included in syntax trees.
/// 
/// Future: Add pattern types as they become available in the ICU library.
/// 
/// # Examples
/// 
/// ```
/// use i18n_icu::IcuDataProvider;
/// use i18n_lexer::{Token, TokenType, tokenise};
/// use i18n_pattern::{parse, NodeType, Formatter, FormatterError, PlaceholderValue, CommandRegistry};
/// use icu_testdata::buffer;
/// use icu_provider::serde::AsDeserializingBufferProvider;
/// use icu_locid::Locale;
/// use std::collections::HashMap;
/// use std::rc::Rc;
/// use std::error::Error;
/// 
/// fn pattern_plural() -> Result<(), Box<dyn Error>> {
///     let buffer_provider = buffer();
///     let data_provider = buffer_provider.as_deserializing();
///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( &data_provider )? );
///     let tokens = tokenise(
///         "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}",
///         &vec![ '{', '}', '`', '#' ],
///         &icu_data_provider,
///     );
///     let tree = parse( tokens.0 )?;
///     let locale: Rc<Locale> = Rc::new( "en-ZA".parse()? );
///     let language_tag = Rc::new( locale.to_string() );
///     let command_registry = Rc::new( CommandRegistry::new() );
///     let mut formatter = Formatter::try_new(
///         &icu_data_provider, &language_tag, &locale, &tree, &command_registry
///     )?;
///     let mut values = HashMap::<String, PlaceholderValue>::new();
///     values.insert(
///         "dogs_number".to_string(),
///         PlaceholderValue::Unsigned( 3 )
///     );
///     let result = formatter.format( &values )?;
///     assert_eq!( result.as_str(), "There are 3 dogs in the park.", "Strings must be the same." );
///     Ok( () )
/// }
/// ```
pub fn parse( tokens: Vec<Rc<Token>> ) -> Result<Tree, ParserError> {
    let mut tree = Tree::new();
    if tokens.len() == 0 {
        return Ok( tree );
    }
    tree.insert(
        0,
        ALLOW_CHILDREN,
        Some( Box::new( NodeType::Root ) ),
        None,
    ).ok();
    let mut parser = Parser {
        current: tree.insert(
            0,
            ALLOW_CHILDREN,
            Some( Box::new( NodeType::String ) ),
            None,
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
                            return Err( ParserError::EndedAbruptly );
                        };
                        add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else if string == "{" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::EndedAbruptly );
                        };
                        pattern_start(
                            &mut tree,
                            &mut parser,
                            token_next,
                            &mut patterns
                        )?;
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "#" {
                        parser.current = Some( 0 ); // Move to root.
                        create_node( &mut tree, &mut parser, NodeType::NamedGroup, ALLOW_CHILDREN );
                        parser.state = ParserStates::NamedGroup;
                    } else {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }
                } else {
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Text,
                        ALLOW_DATA,
                        token
                    );
                }
            },
            ParserStates::SubString => {//  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "#" {
                        let parent = tree.parent( parser.current.unwrap() ).unwrap();
                        if let Some( last ) = tree.last( parent ).ok() {
                            let node_type_data =
                                tree.node_type( last ).ok().unwrap().as_ref().unwrap();
                            let node_type = node_type_data.downcast_ref::<NodeType>().unwrap();
                            if *node_type == NodeType::NumberSign {
                                return Err( ParserError::MultiNumberSign( token.position_grapheme ) );
                            }
                        }
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            ALLOW_DATA,
                            token
                        );
                        parser.state = ParserStates::SubString;
                    } else if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::EndedAbruptly );
                        };
                        add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else if string == "{" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::EndedAbruptly );
                        };
                        pattern_start(
                            &mut tree,
                            &mut parser,
                            token_next,
                            &mut patterns
                        )?;
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "}" {
                        move_to_container( &tree, &mut parser );
                        end_nested_state( &tree, &mut parser ); // Return to NamedString state
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 2 {
                            return Err(
                                ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) )
                            );
                        }

                        // Ends NamedString, and returns to NamedGroup
                        end_nested_state( &tree, &mut parser );
                    } else {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }
                } else {
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Text,
                        ALLOW_DATA,
                        token
                    );
                }
            },
            ParserStates::Pattern => {//  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        ALLOW_DATA, 
                        token
                    );
                    move_to_container( &tree, &mut parser );
                    parser.state = ParserStates::Keyword;
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string != "}" {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }

                    // None (only identifier provide with default type of preformatted string)
                    end_nested_state( &tree, &mut parser );
                } else {
                    return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                }
            },
            ParserStates::Keyword => {//  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    create_node( &mut tree, &mut parser, NodeType::Selector, ALLOW_CHILDREN );
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        ALLOW_DATA,
                        token
                    );
                    move_to_container( &tree, &mut parser );
                    let Some( token_next ) = iterator.next() else {
                        return Err( ParserError::EndedAbruptly );
                    };
                    if token_next.string.as_str() != "#" {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }
                    let Some( token_next_2nd ) = iterator.next() else {
                        return Err( ParserError::EndedAbruptly );
                    };
                    if token_next_2nd.token_type != TokenType::Identifier {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        ALLOW_DATA,
                        token_next_2nd
                    );
                    move_to_container( &tree, &mut parser );
                    parser.current = tree.parent( parser.current.take().unwrap() ).ok();
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                    }
                } else {
                    return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                }
            },
            ParserStates::LiteralText => {//  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::EndedAbruptly );
                        };
                        if token_next.string.as_str() == "`" {// Literal ` found.
                            add_token(
                                &mut tree,
                                &mut parser,
                                NodeType::Text,
                                ALLOW_DATA,
                                token_next
                            );
                            iterator = iterator_peeking; // Skip over 1st ` token.
                        }
                        else {
                            end_nested_state( &tree, &mut parser );
                        }
                        continue;
                    }
                }
                add_token( &mut tree, &mut parser, NodeType::Text, ALLOW_DATA, token );
            },
            ParserStates::Literal => {//  Valid tokens: }
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                        continue;
                    }
                }
                return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
            },
            ParserStates::Command => {//  Valid tokens: PWS (separator - ignore), `, }, #, Identifier
                if token.token_type == TokenType::Identifier {
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        ALLOW_DATA,
                        token
                    );
                    move_to_container( &tree, &mut parser );
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "}" {
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() < 1 {
                            return Err(
                                ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) )
                            );
                        }
                        end_nested_state( &tree, &mut parser );
                    } else if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::EndedAbruptly );
                        };
                        create_node_add_token(
                            &mut tree,&mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                        parser.nested_states.push( ParserStates::Command );
                        parser.state = ParserStates::LiteralText;
                    } else if string == "#" {
                        // Only valid after command identifier.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            return Err(
                                ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) )
                            );
                        }
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            ALLOW_DATA,
                            token
                        );
                    } else {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else {
                    return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                }
            },
            ParserStates::NamedString => {//  Valid tokens: PWS (ignored - separator), `, #, {, Identifier, Syntax
                if token.token_type == TokenType::Identifier {
                    let current = *parser.current.as_ref().unwrap();
                    let len = tree.children( current ).ok().as_ref().unwrap().len();
                    if len == 0 {
                        let string = token.string.as_str().to_string();
                        if named_strings.contains_key( &string ) {
                            return Err( ParserError::UniqueNamed( string ) );
                        }
                        named_strings.insert( string, current );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Identifier,
                            ALLOW_DATA,
                            token
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
                        create_node( &mut tree, &mut parser, NodeType::String, ALLOW_CHILDREN );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token
                        );
                        parser.state = ParserStates::SubString;
                        continue;
                    }
                    return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "#" {
                        // # may only appear after Identifier node indicating start of SubString.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            return Err(
                                ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) )
                            );
                        }
                        create_node( &mut tree, &mut parser, NodeType::String, ALLOW_CHILDREN );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            ALLOW_DATA,
                            token
                        );
                        parser.state = ParserStates::SubString;
                    } else if string == "{" {
                        // { may only appear after Identifier node indicating start of SubString.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            return Err(
                                ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) )
                            );
                        }
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::EndedAbruptly );
                        };
                        match pattern_start(
                            &mut tree,
                            &mut parser,
                            token_next,
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
                            return Err(
                                ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) )
                            );
                        }
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( ParserError::EndedAbruptly );
                        };
                        create_node( &mut tree, &mut parser, NodeType::String, ALLOW_CHILDREN );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }
                } else if token.token_type == TokenType::Syntax {

                    // Syntax may only appear after Identifier node indicating start of SubString.
                    if tree.children(
                        *parser.current.as_ref().unwrap()
                    ).ok().as_ref().unwrap().len() != 1 {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }
                    create_node( &mut tree, &mut parser, NodeType::String, ALLOW_CHILDREN );
                    parser.nested_states.push( ParserStates::NamedString );
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Text,
                        ALLOW_DATA,
                        token
                    );
                } else if token.token_type == TokenType::WhiteSpace {

                    // Valid WhiteSpace is only a separator between Identifier and Substring.
                    if tree.children(
                        *parser.current.as_ref().unwrap()
                    ).ok().as_ref().unwrap().len() != 1 {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }
                } else {
                    return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                }
            },
            ParserStates::NamedGroup => {// Valid tokens: PWS (ignored - human readability), {
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() != "{" {
                        return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                    }

                    // start of NamedString
                    create_node( &mut tree, &mut parser, NodeType::NamedString, ALLOW_CHILDREN );
                    parser.nested_states.push( ParserStates::NamedGroup );
                    parser.state = ParserStates::NamedString;
                } else if token.token_type == TokenType::WhiteSpace {
                } else {
                    return Err( ParserError::InvalidToken( token.position_grapheme, Rc::clone( &token ) ) );
                }
            }
        }
    }
    if parser.nested_states.len() > 0 {
        return Err( ParserError::EndedAbruptly );
    }
    Ok( tree )
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
    Keyword, // a pattern with a keyword.
}

impl Display for ParserStates {
    fn fmt( &self, f: &mut Formatter ) -> FmtResult {
        match *self {
            ParserStates::Command => write!( f, "Command" ),
            ParserStates::Keyword => write!( f, "Keyword" ),
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

// Move `current` to its parent node only if `current` is a ALLOW_DATA node.
// Usually this signals the ALLOW_DATA has all its tokens.
fn move_to_container( tree: &Tree, parser: &mut Parser, ) {
    let node_index = *parser.current.as_ref().unwrap();
    if tree.features( node_index ).unwrap() & ALLOW_CHILDREN != ALLOW_CHILDREN {
        parser.current = tree.parent( node_index ).ok(); // Root node always a ALLOW_CHILDREN.
    }
}

// Create a new child node of a specified type.
// Also changes the parser current node index to the newly created child node.
fn create_node(
    tree: &mut Tree,
    parser: &mut Parser,
    node_type: NodeType,
    features: u8,
) {
    move_to_container( tree, parser );
    parser.current = tree.insert(
        parser.current.take().unwrap(),
        features,
        Some( Box::new( node_type ) ),
        None,
    ).ok();
}

// Create a new child node of a specified type, and add the Token to this new node.
// Also changes the parser current node index to the newly created node.
fn create_node_add_token(
    tree: &mut Tree,
    parser: &mut Parser,
    node_type: NodeType,
    features: u8,
    token: &Rc<Token>
) {
    move_to_container( tree, parser );
    parser.current = tree.insert(
        parser.current.take().unwrap(),
        features,
        Some( Box::new( node_type ) ),
        None,
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
    features: u8,
    token: &Rc<Token>
) {
    let current = *parser.current.as_ref().unwrap();
    let node_type_ref = tree.node_type( current ).ok().unwrap().as_ref().unwrap();
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
    token_next: &Rc<Token>,
    patterns: &mut HashMap<String, usize>,
) -> Result<(), ParserError> {
    if token_next.token_type == TokenType::Identifier {
        // Multilingual pattern
        create_node( tree, parser, NodeType::Pattern, ALLOW_CHILDREN );
        if patterns.contains_key( &token_next.string ) {
            return Err( ParserError::UniquePattern( token_next.string.as_str().to_string() ) );
        }
        patterns.insert( token_next.string.as_str().to_string(), *parser.current.as_ref().unwrap() );
        create_node_add_token( tree, parser, NodeType::Identifier, ALLOW_DATA, token_next );
        move_to_container( tree, parser ); // Move back to Pattern node.
        parser.nested_states.push( parser.state );
        parser.state = ParserStates::Pattern;
    } else if token_next.token_type == TokenType::Grammar {
        // Future: may allow empty {} to be treated as literal {}
        if token_next.string.as_str() == "`" {
            // Literal pattern.
            create_node( tree, parser, NodeType::Text, ALLOW_DATA );
            parser.nested_states.push( parser.state );
            parser.nested_states.push( ParserStates::Literal );
            parser.state = ParserStates::LiteralText;
        } else if token_next.string.as_str() == "#" {
            // Command pattern.
            create_node( tree, parser, NodeType::Command, ALLOW_CHILDREN );
            parser.nested_states.push( parser.state );
            parser.state = ParserStates::Command;
        } else {
            return Err( ParserError::InvalidToken( token_next.position_grapheme, Rc::clone( &token_next ) ) );
        }
    } else {
        return Err( ParserError::InvalidToken( token_next.position_grapheme, Rc::clone( &token_next ) ) );
    }
    Ok( () )
}

// Get the node type as a string for the current node.
// May be of use in generating future detailed error messages
#[allow( dead_code )]
fn node_type_to_string( tree: &Tree, parser: &mut Parser, ) -> String {
    let current = *parser.current.as_ref().unwrap();
    let node_type_ref = tree.node_type( current ).ok().unwrap().as_ref().unwrap();
    node_type_ref.downcast_ref::<NodeType>().unwrap().to_string()
}
