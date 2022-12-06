// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! TO be completed
//! 
//! # Examples
//! 
//! ```
//! //Construct example once all public methods and unit tests are completed.
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
Result<( Tree, HashMap<String, usize>, HashMap<String, usize> ), String> {
    if tokens.len() == 0 {
        return Err( "Empty token vector!".to_string() );
    }
    let mut tree = Tree::new();
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
            ParserStates::String => {
                //  Valid tokens: PWS, `, #, {, Identifier, Syntax
                if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( "String ended abruptly at literal marker.".to_string() );
                        };
                        add_token( &mut tree, &mut parser, NodeType::Text, LEAF, token_next.clone() );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else if string == "{" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( "String ended abruptly at pattern marker.".to_string() );
                        };
                        match pattern_start( &mut tree, &mut parser, token_next.clone(), &mut patterns ) {
                            Ok( _ ) => {},
                            Err( error ) => return Err( error )
                        };
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "#" {
                        parser.current = Some( 0 ); // Move to root.
                        create_node( &mut tree, &mut parser, NodeType::NamedGroup, CONTAINER );
                        parser.state = ParserStates::NamedGroup;
                    } else {
                        return Err( "Invalid token found.".to_string() );
                    }
                } else {
                    add_token( &mut tree, &mut parser, NodeType::Text, LEAF, token.clone() );
                }
            },
            ParserStates::SubString => {
                //  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "#" {
                        // TODO: Check if previous node is not NumberSign, adjacent NumberSign not allowed
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            LEAF,
                            token.clone()
                        );
                        parser.state = ParserStates::SubString;
                    } else if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( "String ended abruptly at literal marker.".to_string() );
                        };
                        add_token( &mut tree, &mut parser, NodeType::Text, LEAF, token_next.clone() );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else if string == "{" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( "String ended abruptly at pattern marker.".to_string() );
                        };
                        match pattern_start( &mut tree, &mut parser, token_next.clone(), &mut patterns ) {
                            Ok( _ ) => {},
                            Err( error ) => return Err( error )
                        };
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "}" {
                        end_nested_state( &tree, &mut parser ); // Return to NamedString state
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 2 {
                            return Err( "Invalid token found.".to_string() );
                        }
                        // Ends NamedString, and returns to NamedGroup
                        end_nested_state( &tree, &mut parser );
                    } else {
                        return Err( "Invalid token found.".to_string() );
                    }
                } else {
                    add_token( &mut tree, &mut parser, NodeType::Text, LEAF, token.clone() );
                }
            },
            ParserStates::Pattern => {
                //  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    let string = token.string.as_str();
                    if string == "plural" || string == "select" {
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Identifier,
                            LEAF, 
                            token.clone()
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
                            token.clone()
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
                            token.clone()
                        );
                        move_to_container( &tree, &mut parser );
                        parser.state = ParserStates::DateTime;
                    }
                    // TODO: add the other types, and also to the ParserStates
                    else {
                        return Err( "Invalid token found.".to_string() );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string != "}" {
                        return Err( "Invalid token found.".to_string() );
                    }
                    // None (only identifier provide with default type of preformatted string)
                    end_nested_state( &tree, &mut parser );
                } else {
                    return Err( "Invalid token found.".to_string() );
                }
            },
            ParserStates::Decimal => {
                //  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    if pattern_option_exists( &tree, &parser, token.string.as_str() ) {
                        return Err( "Branch value already exists.".to_string() );
                    }
                    let string = token.string.as_str();
                    if string == "group" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            token.clone(),
                            token_next.clone(),
                            token_next_2nd.clone(),
                            vec!( "never", "always" , "min2" ),
                        )?;

                        /* --- keep until unit test produces correct expected AST.
                        create_node( &mut tree, &mut parser, NodeType::Selector, CONTAINER );
                        add_token( &mut tree, &mut parser, NodeType::Identifier, LEAF, token.clone() );
                        move_to_container( &tree, &mut parser );
                        let Some( token_next ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        if token_next.string.as_str() != "#" {
                            return Err( "Invalid token found.".to_string() );
                        }
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        if token_next_2nd.token_type != TokenType::Identifier {
                            return Err( "Invalid token found.".to_string() );
                        }
                        let string_2nd = token.string.as_str();
                        if string_2nd == "never" || string_2nd == "always" || string_2nd == "min2" {
                            add_token(
                                &mut tree,
                                &mut parser,
                                NodeType::Identifier,
                                LEAF,
                                token_next_2nd.clone()
                            );
                            move_to_container( &tree, &mut parser );
                            parser.current = tree.parent( parser.current.take().unwrap() ).ok();
                        } else {
                            return Err( "Invalid token found.".to_string() );
                        }
                        */
                    } else if string == "sign" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            token.clone(),
                            token_next.clone(),
                            token_next_2nd.clone(),
                            vec!( "never", "always" , "except_zero" , "negative" ),
                        )?;
                    } else {
                        return Err( "Invalid token found.".to_string() );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                    }
                } else {
                    return Err( "Invalid token found.".to_string() );
                }
            },
            ParserStates::DateTime => {
                //  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    if pattern_option_exists( &tree, &parser, token.string.as_str() ) {
                        return Err( "Branch value already exists.".to_string() );
                    }
                    let string = token.string.as_str();
                    if string == "date" || string == "time" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            token.clone(),
                            token_next.clone(),
                            token_next_2nd.clone(),
                            vec!( "full", "long", "medium", "short" ),
                        )?;
                    } else if string == "hour" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            token.clone(),
                            token_next.clone(),
                            token_next_2nd.clone(),
                            vec!( "24", "23", "12", "11" ),
                        )?;
                    } else if string == "calendar" {
                        let Some( token_next ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        let Some( token_next_2nd ) = iterator.next() else {
                            return Err( "String ended abruptly at pattern.".to_string() );
                        };
                        pattern_selector(
                            &mut tree,
                            &mut parser,
                            token.clone(),
                            token_next.clone(),
                            token_next_2nd.clone(),
                            vec!( "gregory", "buddhist", "japanese", "coptic", "indian", "ethiopic" ),
                        )?;
                    } else {
                        return Err( "Invalid token found.".to_string() );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                    }
                } else {
                    return Err( "Invalid token found.".to_string() );
                }
            },
            ParserStates::Complex => {
                //  Valid tokens: PWS (separator - ignore), }, Identifier
                if token.token_type == TokenType::Identifier {
                    if pattern_option_exists( &tree, &parser, token.string.as_str() ) {
                        return Err( "Branch value already exists.".to_string() );
                    }
                    create_node( &mut tree, &mut parser, NodeType::Selector, CONTAINER );
                    add_token( &mut tree, &mut parser, NodeType::Identifier, LEAF, token.clone() );
                    move_to_container( &tree, &mut parser );
                    let Some( token_next ) = iterator.next() else {
                        return Err( "String ended abruptly at pattern.".to_string() );
                    };
                    if token_next.string.as_str() != "#" {
                        return Err( "Invalid token found.".to_string() );
                    }
                    let Some( token_next_2nd ) = iterator.next() else {
                        return Err( "String ended abruptly at pattern.".to_string() );
                    };
                    if token_next_2nd.token_type != TokenType::Identifier {
                        return Err( "Invalid token found.".to_string() );
                    }
                    add_token( &mut tree, &mut parser, NodeType::Identifier, LEAF, token_next_2nd.clone() );
                    move_to_container( &tree, &mut parser );
                    parser.current = tree.parent( parser.current.take().unwrap() ).ok();
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                    }
                } else {
                    return Err( "Invalid token found.".to_string() );
                }
            },
            ParserStates::LiteralText => {
                //  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( "String ended abruptly at literal marker.".to_string() );
                        };
                        if token_next.string.as_str() == "`" {
                                // Literal ` found.
                            add_token( &mut tree, &mut parser, NodeType::Text, LEAF, token_next.clone() );
                            iterator = iterator_peeking; // Skip over 1st ` token.
                        }
                        else {
                            end_nested_state( &tree, &mut parser );
                        }
                        continue;
                    }
                }
                add_token( &mut tree, &mut parser, NodeType::Text, LEAF, token.clone() );
            },
            ParserStates::Literal => {
                //  Valid tokens: }
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state( &tree, &mut parser );
                        continue;
                    }
                }
                return Err( "Invalid token found.".to_string() );
            },
            ParserStates::Command => {
                //  Valid tokens: PWS (separator - ignore), `, }, Identifier
                if token.token_type == TokenType::Identifier {
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        LEAF,
                        token.clone()
                    );
                    move_to_container( &tree, &mut parser );
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "{" {
                        // None (only identifier provide with default type of preformatted string)
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            return Err( "Invalid token found.".to_string() );
                        }
                        end_nested_state( &tree, &mut parser );
                    } else if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( "String ended abruptly at literal marker.".to_string() );
                        };
                        create_node_add_token(
                            &mut tree,&mut parser,
                            NodeType::Text,
                            LEAF,
                            token_next.clone()
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                        parser.nested_states.push( ParserStates::Command );
                        parser.state = ParserStates::LiteralText;
                    } else {
                        return Err( "Invalid token found.".to_string() );
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else {
                    return Err( "Invalid token found.".to_string() );
                }
            },
            ParserStates::NamedString => {
                //  Valid tokens: PWS (ignored - separator), `, #, {, Identifier, Syntax
                if token.token_type == TokenType::Identifier {
                    let current = *parser.current.as_ref().unwrap();
                    let len = tree.children( current ).ok().as_ref().unwrap().len();
                    if len == 0 {
                        let string = token.string.as_str().to_string();
                        if named_strings.contains_key( &string ) {
                            return Err(
                                "Named substrings must have unique identifiers.".to_string()
                            );
                        }
                        named_strings.insert( string, current );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Identifier,
                            LEAF,
                            token.clone()
                        );
                        move_to_container( &tree, &mut parser );
                        // Check that white space separator follows identifier
                        let mut iterator_peeking = iterator.clone();
                        if let Some( token_next ) = iterator_peeking.next() {
                            if token_next.token_type == TokenType::WhiteSpace {
                                continue;
                            }
                        }
                    } else if len == 1 {
                        create_node( &mut tree, &mut parser, NodeType::String, CONTAINER );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token( &mut tree, &mut parser, NodeType::Text, LEAF, token.clone() );
                        continue;
                    }
                    return Err( "Invalid token found.".to_string() );
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "#" {
                        // IDEA: may capture the positions of these # to speed up substitutions
                        // # may only appear after Identifier node indicating start of SubString.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            return Err( "Invalid token found.".to_string() );
                        }
                        create_node( &mut tree, &mut parser, NodeType::String, CONTAINER );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            LEAF,
                            token.clone()
                        );
                        parser.state = ParserStates::SubString;
                    } else if string == "{" {
                        // { may only appear after Identifier node indicating start of SubString.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            return Err( "Invalid token found.".to_string() );
                        }
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( "String ended abruptly at pattern marker.".to_string() );
                        };
                        match pattern_start( &mut tree, &mut parser, token_next.clone(), &mut patterns ) {
                            Ok( _ ) => {},
                            Err( error ) => return Err( error )
                        };
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "`" {
                        // ` may only appear after Identifier node indicating start of SubString.
                        if tree.children(
                            *parser.current.as_ref().unwrap()
                        ).ok().as_ref().unwrap().len() != 1 {
                            return Err( "Invalid token found.".to_string() );
                        }
                        let mut iterator_peeking = iterator.clone();
                        let Some( token_next ) = iterator_peeking.next() else {
                            return Err( "String ended abruptly at literal marker.".to_string() );
                        };
                        create_node( &mut tree, &mut parser, NodeType::String, CONTAINER );
                        parser.nested_states.push( ParserStates::NamedString );
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            LEAF,
                            token_next.clone()
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else {
                        return Err( "Invalid token found.".to_string() );
                    }
                } else if token.token_type == TokenType::Syntax {
                    // Syntax may only appear after Identifier node indicating start of SubString.
                    if tree.children(
                        *parser.current.as_ref().unwrap()
                    ).ok().as_ref().unwrap().len() != 1 {
                        return Err( "Invalid token found.".to_string() );
                    }
                    create_node( &mut tree, &mut parser, NodeType::String, CONTAINER );
                    parser.nested_states.push( ParserStates::NamedString );
                    create_node_add_token( &mut tree, &mut parser, NodeType::Text, LEAF, token.clone() );
                } else if token.token_type == TokenType::WhiteSpace {
                    // Valid WhiteSpace is only a separator between Identifier and Substring.
                    if tree.children(
                        *parser.current.as_ref().unwrap()
                    ).ok().as_ref().unwrap().len() != 1 {
                        return Err( "Invalid token found.".to_string() );
                    }
                    // Ignore the valid WhiteSpace.
                } else {
                    return Err( "Invalid token found.".to_string() );
                }
            },
            ParserStates::NamedGroup => {
                //  Valid tokens: PWS (ignored - human readability), {
                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() != "{" {
                        return Err( "Invalid token found.".to_string() );
                    }
                    // start of NamedString
                    create_node( &mut tree, &mut parser, NodeType::NamedString, CONTAINER );
                    parser.nested_states.push( ParserStates::NamedGroup );
                    parser.state = ParserStates::NamedString;
                } else if token.token_type == TokenType::WhiteSpace {
                } else {
                    return Err( "Invalid token found.".to_string() );
                }
            }
        }
    }
    if parser.nested_states.len() > 0 {
        return Err( "String ended abruptly.".to_string() );
    }

    // Final check each select and plural that the branch exists in named string group, and that the branch `other`
    // does exist.
    for ( _, index ) in patterns.iter() {
        // First element is placeholder identifier, remainder are selectors.
        let mut pattern_iterator =
            tree.children( *index ).ok().as_ref().unwrap().iter().skip( 1 );
        if let Some( selector_keyword ) = pattern_iterator.next() {
            if let Some( keyword ) =
                tree.children( *selector_keyword ).ok().as_ref().unwrap().first()
            {
                let keyword_data = tree.data_ref( *keyword ).ok().unwrap();
                if let Some( keyword_token ) = keyword_data.first().unwrap().downcast_ref::<Token>() {
                    let keyword_string = keyword_token.string.as_str();
                    if keyword_string == "plural" || keyword_string == "select" {
                        let mut other = false;
                        while let Some( selector_branch ) = pattern_iterator.next() {
                            if let Some( branch ) =
                                tree.children( *selector_branch ).ok().as_ref().unwrap().first()
                            {
                                let branch_data =
                                    tree.data_ref( *branch ).ok().unwrap();
                                if let Some( branch_token ) =
                                    branch_data.first().unwrap().downcast_ref::<Token>()
                                {
                                    let mut found = false;
                                    for ( identifier, _ ) in named_strings.iter() {
                                        if branch_token.string == *identifier {
                                            found = true;
                                        }
                                        if branch_token.string.as_str() == "other" {
                                            other = true;
                                        }
                                    }
                                    if !found {
                                        return Err(
                                            "No named string found for select/plural branch identifier.".to_string()
                                        );
                                    }
                                }
                            }
                        }
                        if !other {
                            return Err( "Branch `other` for select/plural is missing.".to_string() );
                        }
                    }
                }
            }
        }
    }

    Ok( ( tree, named_strings, patterns ) )
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

struct Parser {
//    container: NodeFeatures,
//    leaf: NodeFeatures,
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
    token: Rc<Token>
) {
    move_to_container( tree, parser );
    parser.current = tree.insert(
        parser.current.take().unwrap(),
        features,
        Box::new( node_type )
    ).ok();
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
    token: Rc<Token>
) {
    let current = *parser.current.as_ref().unwrap();
    let node_type_ref = tree.node_type( current ).ok().unwrap().as_ref();
    let node_type_is = node_type_ref.downcast_ref::<NodeType>().unwrap();
    if *node_type_is == node_type {
        tree.data_mut( current ).ok().unwrap().push( Box::new( token.clone() ) );
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

/// Check if start of pattern is valid.
fn pattern_start(
    tree: &mut Tree, parser: &mut Parser,
    token_next: Rc<Token>,
    patterns: &mut HashMap<String, usize>,
) -> Result<(), String> {
    if token_next.token_type == TokenType::Identifier {
        // Multilingual pattern
        create_node( tree, parser, NodeType::Pattern, CONTAINER );
        if patterns.contains_key( &token_next.string ) {
            return Err(
                "Pattern identifiers must have unique.".to_string()
            );
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
            return Err( "Invalid token found.".to_string() );
        }
    } else {
        return Err( "Invalid token found.".to_string() );
    }
    Ok( () )
}

fn pattern_option_exists( tree: &Tree, parser: &Parser, identifier: &str ) -> bool {
    let children = *tree.children( *parser.current.as_ref().unwrap() ).ok().as_ref().unwrap();
    let mut iterator = children.iter().skip( 2 ); // Skip over identifier and pattern type keyword.
    while let Some( index ) = iterator.next() {
        let data = tree.data_ref( *index ).ok().unwrap();
        if let Some( token ) = data.first().unwrap().downcast_ref::<Token>() {
            if token.string.as_str() == identifier {
                return true;
            }
        }
    }
    false
}

fn pattern_selector(
    tree: &mut Tree,
    parser: &mut Parser,
    token: Rc<Token>,
    token_next: Rc<Token>,
    token_next_2nd: Rc<Token>,
    keywords: Vec<&str>,
) -> Result<(), String> {
    create_node( tree, parser, NodeType::Selector, CONTAINER );
    add_token( tree, parser, NodeType::Identifier, LEAF, token.clone() );
    move_to_container( &tree, parser );
    if token_next.string.as_str() != "#" {
        return Err( "Invalid token found.".to_string() );
    }
    if token_next_2nd.token_type != TokenType::Identifier {
        return Err( "Invalid token found.".to_string() );
    }
//    let string_2nd = token.string.as_str();
    if keywords.contains( &token_next_2nd.string.as_str() ) {
        add_token(
            tree,
            parser,
            NodeType::Identifier,
            LEAF,
            token_next_2nd.clone()
        );
        move_to_container( &tree, parser );
        parser.current = tree.parent( parser.current.take().unwrap() ).ok();
    } else {
        return Err( "Invalid token found.".to_string() );
    }
    Ok( () )
}

#[cfg(test)]
mod tests {
//    use super::*;

    #[test]
    fn check() {
    }
}

