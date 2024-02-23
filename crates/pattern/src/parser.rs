// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

use crate::types::*;
use crate::ParserError;
use i18n_lexer::{Token, TokenType};
use tree::{Tree, ALLOW_CHILDREN, ALLOW_DATA};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

#[cfg(feature = "log")]
use log::{debug, trace};

use core::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::HashMap;

#[cfg(doc)]
use std::sync::Arc;

#[cfg(doc)]
use std::rc::Rc;

/// Constructs a valid syntax tree [`Tree`] from the supplied [`Vec`]`<`[`Rc`]`<`[`Token`]`>>` or
/// `Vec<`[`Arc`]`<Token>>`. Any grammar error that occurs will result in an `Err()` result being returned.
///
/// Implicit syntax tokens and optional whitespace tokens are not included in syntax trees.
///
/// Future: Add pattern types as they become available in the ICU library.
///
/// # Examples
///
/// See the examples in `i18n_localiser` crate for use of `parse()`;
pub fn parse(tokens: Vec<RefCount<Token>>) -> Result<Tree, ParserError> {
    #[cfg(feature = "log")]
    debug!("Parsing the token vector created by the lexer.");

    let mut tree = Tree::new();
    if tokens.is_empty() {
        return Ok(tree);
    }
    tree.insert(0, ALLOW_CHILDREN, Some(Box::new(NodeType::Root)), None)
        .ok();
    let mut parser = Parser {
        current: tree
            .insert(0, ALLOW_CHILDREN, Some(Box::new(NodeType::String)), None)
            .ok(),
        state: ParserStates::String,
        nested_states: Vec::<ParserStates>::new(),
    };
    let mut named_strings = HashMap::<String, usize>::new();
    let mut patterns = HashMap::<String, usize>::new();
    let mut iterator = tokens.iter();
    while let Some(token) = iterator.next() {
        match parser.state {
            ParserStates::String => {
                //  Valid tokens: PWS, `, #, {, Identifier, Syntax
                #[cfg(feature = "log")]
                trace!("ParserStates::String");

                if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some(token_next) = iterator_peeking.next() else {
                            return Err(ParserError::EndedAbruptly);
                        };
                        add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next,
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else if string == "{" {
                        let mut iterator_peeking = iterator.clone();
                        let Some(token_next) = iterator_peeking.next() else {
                            return Err(ParserError::EndedAbruptly);
                        };
                        pattern_start(&mut tree, &mut parser, token_next, &mut patterns)?;
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "#" {
                        parser.current = Some(0); // Move to root.
                        create_node(&mut tree, &mut parser, NodeType::NamedGroup, ALLOW_CHILDREN);
                        parser.state = ParserStates::NamedGroup;
                    } else {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }
                } else {
                    add_token(&mut tree, &mut parser, NodeType::Text, ALLOW_DATA, token);
                }
            }
            ParserStates::SubString => {
                //  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                #[cfg(feature = "log")]
                trace!("ParserStates::SubString");

                if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "#" {
                        let parent = tree.parent(parser.current.unwrap()).unwrap();
                        if let Ok(last) = tree.last(parent) {
                            let node_type_data =
                                tree.node_type(last).ok().unwrap().as_ref().unwrap();
                            let node_type = node_type_data.downcast_ref::<NodeType>().unwrap();
                            if *node_type == NodeType::NumberSign {
                                return Err(ParserError::MultiNumberSign(token.position_grapheme));
                            }
                        }
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            ALLOW_DATA,
                            token,
                        );
                        parser.state = ParserStates::SubString;
                    } else if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some(token_next) = iterator_peeking.next() else {
                            return Err(ParserError::EndedAbruptly);
                        };
                        add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next,
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else if string == "{" {
                        let mut iterator_peeking = iterator.clone();
                        let Some(token_next) = iterator_peeking.next() else {
                            return Err(ParserError::EndedAbruptly);
                        };
                        pattern_start(&mut tree, &mut parser, token_next, &mut patterns)?;
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "}" {
                        move_to_container(&tree, &mut parser);
                        end_nested_state(&tree, &mut parser); // Return to NamedString state
                        if tree
                            .children(*parser.current.as_ref().unwrap())
                            .ok()
                            .as_ref()
                            .unwrap()
                            .len()
                            != 2
                        {
                            return Err(ParserError::InvalidToken(
                                token.position_grapheme,
                                RefCount::clone(token),
                            ));
                        }

                        // Ends NamedString, and returns to NamedGroup
                        end_nested_state(&tree, &mut parser);
                    } else {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }
                } else {
                    add_token(&mut tree, &mut parser, NodeType::Text, ALLOW_DATA, token);
                }
            }
            ParserStates::Pattern => {
                //  Valid tokens: PWS (separator - ignore), }, Identifier
                #[cfg(feature = "log")]
                trace!("ParserStates::Pattern");

                if token.token_type == TokenType::Identifier {
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        ALLOW_DATA,
                        token,
                    );
                    move_to_container(&tree, &mut parser);
                    parser.state = ParserStates::Keyword;
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string != "}" {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }

                    // None (only identifier provide with default type of preformatted string)
                    end_nested_state(&tree, &mut parser);
                } else {
                    return Err(ParserError::InvalidToken(
                        token.position_grapheme,
                        RefCount::clone(token),
                    ));
                }
            }
            ParserStates::Keyword => {
                //  Valid tokens: PWS (separator - ignore), }, Identifier
                #[cfg(feature = "log")]
                trace!("ParserStates::Keyword");

                if token.token_type == TokenType::Identifier {
                    create_node(&mut tree, &mut parser, NodeType::Selector, ALLOW_CHILDREN);
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        ALLOW_DATA,
                        token,
                    );
                    move_to_container(&tree, &mut parser);
                    let Some(token_next) = iterator.next() else {
                        return Err(ParserError::EndedAbruptly);
                    };
                    if token_next.string.as_str() != "#" {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }
                    let Some(token_next_2nd) = iterator.next() else {
                        return Err(ParserError::EndedAbruptly);
                    };
                    if token_next_2nd.token_type != TokenType::Identifier {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }
                    add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        ALLOW_DATA,
                        token_next_2nd,
                    );
                    move_to_container(&tree, &mut parser);
                    parser.current = tree.parent(parser.current.take().unwrap()).ok();
                } else if token.token_type == TokenType::WhiteSpace {
                } else if token.token_type == TokenType::Grammar {
                    if token.string.as_str() == "}" {
                        end_nested_state(&tree, &mut parser);
                    }
                } else {
                    return Err(ParserError::InvalidToken(
                        token.position_grapheme,
                        RefCount::clone(token),
                    ));
                }
            }
            ParserStates::LiteralText => {
                //  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                #[cfg(feature = "log")]
                trace!("ParserStates::LiteralText");

                if token.token_type == TokenType::Grammar && token.string.as_str() == "`" {
                    let mut iterator_peeking = iterator.clone();
                    let Some(token_next) = iterator_peeking.next() else {
                        return Err(ParserError::EndedAbruptly);
                    };
                    if token_next.string.as_str() == "`" {
                        // Literal ` found.
                        add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next,
                        );
                        iterator = iterator_peeking; // Skip over 1st ` token.
                    } else {
                        end_nested_state(&tree, &mut parser);
                    }
                    continue;
                }
                add_token(&mut tree, &mut parser, NodeType::Text, ALLOW_DATA, token);
            }
            ParserStates::Literal => {
                //  Valid tokens: }
                #[cfg(feature = "log")]
                trace!("ParserStates::Literal");

                if token.token_type == TokenType::Grammar && token.string.as_str() == "}" {
                    end_nested_state(&tree, &mut parser);
                    continue;
                }
                return Err(ParserError::InvalidToken(
                    token.position_grapheme,
                    RefCount::clone(token),
                ));
            }
            ParserStates::Command => {
                //  Valid tokens: PWS (separator - ignore), `, }, #, Identifier
                #[cfg(feature = "log")]
                trace!("ParserStates::Command");

                if token.token_type == TokenType::Identifier {
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Identifier,
                        ALLOW_DATA,
                        token,
                    );
                    move_to_container(&tree, &mut parser);
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "}" {
                        if tree
                            .children(*parser.current.as_ref().unwrap())
                            .ok()
                            .as_ref()
                            .unwrap()
                            .is_empty()
                        {
                            return Err(ParserError::InvalidToken(
                                token.position_grapheme,
                                RefCount::clone(token),
                            ));
                        }
                        end_nested_state(&tree, &mut parser);
                    } else if string == "`" {
                        let mut iterator_peeking = iterator.clone();
                        let Some(token_next) = iterator_peeking.next() else {
                            return Err(ParserError::EndedAbruptly);
                        };
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next,
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                        parser.nested_states.push(ParserStates::Command);
                        parser.state = ParserStates::LiteralText;
                    } else if string == "#" {
                        // Only valid after command identifier.
                        if tree
                            .children(*parser.current.as_ref().unwrap())
                            .ok()
                            .as_ref()
                            .unwrap()
                            .len()
                            != 1
                        {
                            return Err(ParserError::InvalidToken(
                                token.position_grapheme,
                                RefCount::clone(token),
                            ));
                        }
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            ALLOW_DATA,
                            token,
                        );
                    } else {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }
                } else if token.token_type == TokenType::WhiteSpace {
                } else {
                    return Err(ParserError::InvalidToken(
                        token.position_grapheme,
                        RefCount::clone(token),
                    ));
                }
            }
            ParserStates::NamedString => {
                //  Valid tokens: PWS (ignored - separator), `, #, {, Identifier, Syntax
                #[cfg(feature = "log")]
                trace!("ParserStates::NamedString");

                if token.token_type == TokenType::Identifier {
                    let current = *parser.current.as_ref().unwrap();
                    let len = tree.children(current).ok().as_ref().unwrap().len();
                    if len == 0 {
                        let string = token.string.as_str().to_string();
                        if named_strings.contains_key(&string) {
                            return Err(ParserError::UniqueNamed(string));
                        }
                        named_strings.insert(string, current);
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Identifier,
                            ALLOW_DATA,
                            token,
                        );
                        move_to_container(&tree, &mut parser);

                        // Check that white space separator follows identifier
                        let mut iterator_peeking = iterator.clone();
                        if let Some(token_next) = iterator_peeking.next() {
                            if token_next.token_type == TokenType::WhiteSpace {
                                iterator = iterator_peeking; // Skip over whitespace token separator.
                                continue;
                            }
                        }
                    } else if len == 1 {
                        create_node(&mut tree, &mut parser, NodeType::String, ALLOW_CHILDREN);
                        parser.nested_states.push(ParserStates::NamedString);
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token,
                        );
                        parser.state = ParserStates::SubString;
                        continue;
                    }
                    return Err(ParserError::InvalidToken(
                        token.position_grapheme,
                        RefCount::clone(token),
                    ));
                } else if token.token_type == TokenType::Grammar {
                    let string = token.string.as_str();
                    if string == "#" {
                        // # may only appear after Identifier node indicating start of SubString.
                        if tree
                            .children(*parser.current.as_ref().unwrap())
                            .ok()
                            .as_ref()
                            .unwrap()
                            .len()
                            != 1
                        {
                            return Err(ParserError::InvalidToken(
                                token.position_grapheme,
                                RefCount::clone(token),
                            ));
                        }
                        create_node(&mut tree, &mut parser, NodeType::String, ALLOW_CHILDREN);
                        parser.nested_states.push(ParserStates::NamedString);
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::NumberSign,
                            ALLOW_DATA,
                            token,
                        );
                        parser.state = ParserStates::SubString;
                    } else if string == "{" {
                        // { may only appear after Identifier node indicating start of SubString.
                        if tree
                            .children(*parser.current.as_ref().unwrap())
                            .ok()
                            .as_ref()
                            .unwrap()
                            .len()
                            != 1
                        {
                            return Err(ParserError::InvalidToken(
                                token.position_grapheme,
                                RefCount::clone(token),
                            ));
                        }
                        let mut iterator_peeking = iterator.clone();
                        let Some(token_next) = iterator_peeking.next() else {
                            return Err(ParserError::EndedAbruptly);
                        };
                        match pattern_start(&mut tree, &mut parser, token_next, &mut patterns) {
                            Ok(_) => {}
                            Err(error) => return Err(error),
                        };
                        iterator = iterator_peeking; // Skip over { and next token.
                    } else if string == "`" {
                        // ` may only appear after Identifier node indicating start of SubString.
                        if tree
                            .children(*parser.current.as_ref().unwrap())
                            .ok()
                            .as_ref()
                            .unwrap()
                            .len()
                            != 1
                        {
                            return Err(ParserError::InvalidToken(
                                token.position_grapheme,
                                RefCount::clone(token),
                            ));
                        }
                        let mut iterator_peeking = iterator.clone();
                        let Some(token_next) = iterator_peeking.next() else {
                            return Err(ParserError::EndedAbruptly);
                        };
                        create_node(&mut tree, &mut parser, NodeType::String, ALLOW_CHILDREN);
                        parser.nested_states.push(ParserStates::NamedString);
                        create_node_add_token(
                            &mut tree,
                            &mut parser,
                            NodeType::Text,
                            ALLOW_DATA,
                            token_next,
                        );
                        iterator = iterator_peeking; // Skip over ` token.
                    } else {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }
                } else if token.token_type == TokenType::Syntax {
                    // Syntax may only appear after Identifier node indicating start of SubString.
                    if tree
                        .children(*parser.current.as_ref().unwrap())
                        .ok()
                        .as_ref()
                        .unwrap()
                        .len()
                        != 1
                    {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }
                    create_node(&mut tree, &mut parser, NodeType::String, ALLOW_CHILDREN);
                    parser.nested_states.push(ParserStates::NamedString);
                    create_node_add_token(
                        &mut tree,
                        &mut parser,
                        NodeType::Text,
                        ALLOW_DATA,
                        token,
                    );
                } else if token.token_type == TokenType::WhiteSpace {
                    // Valid WhiteSpace is only a separator between Identifier and Substring.
                    if tree
                        .children(*parser.current.as_ref().unwrap())
                        .ok()
                        .as_ref()
                        .unwrap()
                        .len()
                        != 1
                    {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }
                } else {
                    return Err(ParserError::InvalidToken(
                        token.position_grapheme,
                        RefCount::clone(token),
                    ));
                }
            }
            ParserStates::NamedGroup => {
                // Valid tokens: PWS (ignored - human readability), {
                #[cfg(feature = "log")]
                trace!("ParserStates::NamedGroup");

                if token.token_type == TokenType::Grammar {
                    if token.string.as_str() != "{" {
                        return Err(ParserError::InvalidToken(
                            token.position_grapheme,
                            RefCount::clone(token),
                        ));
                    }

                    // start of NamedString
                    create_node(
                        &mut tree,
                        &mut parser,
                        NodeType::NamedString,
                        ALLOW_CHILDREN,
                    );
                    parser.nested_states.push(ParserStates::NamedGroup);
                    parser.state = ParserStates::NamedString;
                } else if token.token_type == TokenType::WhiteSpace {
                } else {
                    return Err(ParserError::InvalidToken(
                        token.position_grapheme,
                        RefCount::clone(token),
                    ));
                }
            }
        }
    }
    if !parser.nested_states.is_empty() {
        return Err(ParserError::EndedAbruptly);
    }
    Ok(tree)
}

// Internal structures, enums, etc.

// Various ParserStates the tokens may be in.
#[derive(PartialEq, Copy, Clone)]
enum ParserStates {
    NamedGroup,  // section for holding all the named strings
    NamedString, // a labelled substring for Select and Plural selection options.
    String,      // indicates the outer most string.
    SubString,   // indicates the string part of the NamedString.
    LiteralText, // ends with backtick, found in patterns `
    Pattern,     // ends with matching }
    Literal,     // a pattern only containing literal text, starts with backtick `
    Command, // a pattern that starts with number sign # and creates static text from supplied parameters.
    Keyword, // a pattern with a keyword.
}

impl Display for ParserStates {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ParserStates::Command => write!(f, "Command"),
            ParserStates::Keyword => write!(f, "Keyword"),
            ParserStates::Literal => write!(f, "Literal"),
            ParserStates::LiteralText => write!(f, "LiteralText"),
            ParserStates::NamedGroup => write!(f, "NamedGroup"),
            ParserStates::NamedString => write!(f, "NamedString"),
            ParserStates::Pattern => write!(f, "Pattern"),
            ParserStates::String => write!(f, "String"),
            ParserStates::SubString => write!(f, "SubString"),
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
fn move_to_container(tree: &Tree, parser: &mut Parser) {
    let node_index = *parser.current.as_ref().unwrap();
    if tree.features(node_index).unwrap() & ALLOW_CHILDREN != ALLOW_CHILDREN {
        parser.current = tree.parent(node_index).ok(); // Root node always a ALLOW_CHILDREN.
    }
}

// Create a new child node of a specified type.
// Also changes the parser current node index to the newly created child node.
fn create_node(tree: &mut Tree, parser: &mut Parser, node_type: NodeType, features: u8) {
    move_to_container(tree, parser);
    parser.current = tree
        .insert(
            parser.current.take().unwrap(),
            features,
            Some(Box::new(node_type)),
            None,
        )
        .ok();
}

// Create a new child node of a specified type, and add the Token to this new node.
// Also changes the parser current node index to the newly created node.
fn create_node_add_token(
    tree: &mut Tree,
    parser: &mut Parser,
    node_type: NodeType,
    features: u8,
    token: &RefCount<Token>,
) {
    move_to_container(tree, parser);
    parser.current = tree
        .insert(
            parser.current.take().unwrap(),
            features,
            Some(Box::new(node_type)),
            None,
        )
        .ok();
    tree.data_mut(*parser.current.as_ref().unwrap())
        .unwrap()
        .push(Box::new(RefCount::clone(token)));
}

// Add to a token to a specified node type.
// If current node is not the specified node type, a specified node type will be created, and current node is set
// to it, before adding the token to the node.
fn add_token(
    tree: &mut Tree,
    parser: &mut Parser,
    node_type: NodeType,
    features: u8,
    token: &RefCount<Token>,
) {
    let current = *parser.current.as_ref().unwrap();
    let node_type_ref = tree.node_type(current).ok().unwrap().as_ref().unwrap();
    let node_type_is = node_type_ref.downcast_ref::<NodeType>().unwrap();
    if *node_type_is == node_type {
        tree.data_mut(current)
            .ok()
            .unwrap()
            .push(Box::new(RefCount::clone(token)));
    } else {
        create_node_add_token(tree, parser, node_type, features, token);
    }
}

// End the current nested state, change back to previous state and move to parent node.
fn end_nested_state(tree: &Tree, parser: &mut Parser) {
    parser.state = match parser.nested_states.pop() {
        Some(s) => s,
        None => ParserStates::String,
    };
    parser.current = tree.parent(parser.current.take().unwrap()).ok();
}

// Check if start of pattern is valid.
fn pattern_start(
    tree: &mut Tree,
    parser: &mut Parser,
    token_next: &RefCount<Token>,
    patterns: &mut HashMap<String, usize>,
) -> Result<(), ParserError> {
    if token_next.token_type == TokenType::Identifier {
        // Multilingual pattern
        create_node(tree, parser, NodeType::Pattern, ALLOW_CHILDREN);
        if patterns.contains_key(&token_next.string) {
            return Err(ParserError::UniquePattern(
                token_next.string.as_str().to_string(),
            ));
        }
        patterns.insert(
            token_next.string.as_str().to_string(),
            *parser.current.as_ref().unwrap(),
        );
        create_node_add_token(tree, parser, NodeType::Identifier, ALLOW_DATA, token_next);
        move_to_container(tree, parser); // Move back to Pattern node.
        parser.nested_states.push(parser.state);
        parser.state = ParserStates::Pattern;
    } else if token_next.token_type == TokenType::Grammar {
        // Future: may allow empty {} to be treated as literal {}
        if token_next.string.as_str() == "`" {
            // Literal pattern.
            create_node(tree, parser, NodeType::Text, ALLOW_DATA);
            parser.nested_states.push(parser.state);
            parser.nested_states.push(ParserStates::Literal);
            parser.state = ParserStates::LiteralText;
        } else if token_next.string.as_str() == "#" {
            // Command pattern.
            create_node(tree, parser, NodeType::Command, ALLOW_CHILDREN);
            parser.nested_states.push(parser.state);
            parser.state = ParserStates::Command;
        } else {
            return Err(ParserError::InvalidToken(
                token_next.position_grapheme,
                RefCount::clone(token_next),
            ));
        }
    } else {
        return Err(ParserError::InvalidToken(
            token_next.position_grapheme,
            RefCount::clone(token_next),
        ));
    }
    Ok(())
}

// Get the node type as a string for the current node.
// May be of use in generating future detailed error messages
#[allow(dead_code)]
fn node_type_to_string(tree: &Tree, parser: &mut Parser) -> String {
    let current = *parser.current.as_ref().unwrap();
    let node_type_ref = tree.node_type(current).ok().unwrap().as_ref().unwrap();
    node_type_ref
        .downcast_ref::<NodeType>()
        .unwrap()
        .to_string()
}
