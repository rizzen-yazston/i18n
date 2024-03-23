// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

use crate::TreeError;
use core::fmt::{Display, Formatter, Result as FmtResult};
use i18n_lexer::{IcuDataProvider, LexerIterator, Token, TokenType};
use std::collections::HashMap;
use std::str;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

//use std::str::CharIndices;
#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

#[cfg(feature = "logging")]
use log::trace;

#[cfg(doc)]
use std::sync::Arc;

#[cfg(doc)]
use std::rc::Rc;

/// Enum of the node types that can exist in the generate AST.
/// The following node types are available:
///
/// * Root: \[Container\] The top level, which may optional contained NamedGroup container, and required String
/// container,
///
/// * NamedGroup: \[Container\] Container: Exists if at least 1 named substring (NamedString node) is detected,
///
/// * NamedString: \[Container\] Contains the Identifier leaf, and its substring String container,
///
/// * String: \[Container\] represents either whole string, or a substring for a plural or select pattern,
///
/// * Text: \[Leaf\] Just literal text, and consist of 1 or more tokens (of any type that are treated as text),
///
/// * NumberSign: \[Leaf\] The number pattern `#` in text,
///
/// * Command: \[Container\] Contains command data,
///
/// * Pattern: \[Container\] Usually a multilingual pattern data. 2nd node indicates pattern type,
///
/// * Identifier: \[Leaf\] Always 1 identifier token,
///
/// * Selector: \[Container\] Contains 2 Identifier nodes. Used for `plural` and `select` patterns.
#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Root,
    NamedGroup,
    NamedString,
    String,
    Text,
    NumberSign,
    Command,
    Pattern,
    Identifier,
    Selector,
}

impl NodeType {
    /// Indicates whether the node can have children nodes.
    fn allow_children(&self) -> bool {
        matches!(
            self,
            NodeType::Root
                | NodeType::String
                | NodeType::NamedGroup
                | NodeType::NamedString
                | NodeType::Selector
                | NodeType::Pattern
                | NodeType::Command
        )
    }

    /// Indicates whether the node can have [`Token`]s.
    fn allow_tokens(&self) -> bool {
        matches!(
            self,
            NodeType::Text | NodeType::NumberSign | NodeType::Identifier
        )
    }
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            NodeType::Root => write!(f, "Root"),
            NodeType::NamedGroup => write!(f, "NamedGroup"),
            NodeType::NamedString => write!(f, "NamedString"),
            NodeType::String => write!(f, "String"),
            NodeType::Text => write!(f, "Text"),
            NodeType::NumberSign => write!(f, "NumberSign"),
            NodeType::Command => write!(f, "Command"),
            NodeType::Pattern => write!(f, "Pattern"),
            NodeType::Identifier => write!(f, "Identifier"),
            NodeType::Selector => write!(f, "Selector"),
        }
    }
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    parent: Option<usize>,
    children: Option<Vec<usize>>,
    tokens: Option<Vec<usize>>,
}

/// Just a simple struct containing the string length in terms of bytes, characters, graphemes, and tokens.
#[derive(Debug)]
pub struct Length {
    pub bytes: usize,
    pub characters: usize,
    pub graphemes: usize,
    pub tokens: usize,
}

#[derive(Debug)]
pub(crate) struct Tree {
    tokens: Vec<Token>,
    has_grammar: bool,
    lengths: Length,
    nodes: Vec<Node>,
}

impl Tree {
    pub(crate) fn try_new(
        string: &str,
        grammar: &str,
        icu_data_provider: &RefCount<IcuDataProvider>,
    ) -> Result<Self, TreeError> {
        // Initialise fields
        let mut tokens = Vec::<Token>::new();
        let mut has_grammar = false;
        let mut lengths = Length {
            bytes: 0,
            characters: 0,
            graphemes: 0,
            tokens: 0,
        };
        let mut nodes = Vec::<Node>::new();

        // Initialise tree
        let node = Node {
            node_type: NodeType::Root,
            parent: None,
            children: Some(Vec::<usize>::new()),
            tokens: None,
        };
        nodes.push(node);
        Tree::insert(&mut nodes, &0, NodeType::String);

        // Initialise parser
        let mut parser = ParserStates {
            current: Some(1),
            state: ParserState::String,
            nested_states: Vec::<ParserState>::new(),
        };
        let mut named_strings = HashMap::<String, usize>::new();
        let mut patterns = HashMap::<String, usize>::new();

        // Process the tokens
        let mut lexer = LexerIterator::try_new(string, grammar, icu_data_provider)?.enumerate();
        while let Some((current, token)) = lexer.next() {
            if token.token_type == TokenType::Grammar {
                has_grammar = true;
            }

            #[cfg(feature = "logging")]
            trace!(
                "ParserState: {}; Number: {}; Position: {}; Token: {:?}",
                parser.state,
                lengths.tokens,
                token.start_character,
                token.string
            );

            Tree::push_token(&mut tokens, &mut lengths, token);
            match parser.state {
                ParserState::String => {
                    // Valid tokens: PWS, `, #, {, Identifier, Syntax
                    #[cfg(feature = "logging")]
                    trace!("ParserState::String");

                    if tokens[current].token_type == TokenType::Grammar {
                        let token_str = tokens[current].string.as_str();
                        if token_str == "`" {
                            // Skip over ` token as next token is a literal.
                            let Some((next, token)) = lexer.next() else {
                                return Err(TreeError::EndedAbruptly);
                            };
                            Tree::push_token(&mut tokens, &mut lengths, token);
                            Tree::add_token(&mut nodes, &mut parser, NodeType::Text, &next);
                        } else if token_str == "{" {
                            // Skip over { token as it marks start of pattern.
                            let Some((next, token)) = lexer.next() else {
                                return Err(TreeError::EndedAbruptly);
                            };
                            Tree::push_token(&mut tokens, &mut lengths, token);
                            Tree::pattern_start(
                                &mut nodes,
                                &mut parser,
                                &mut tokens,
                                &next,
                                &mut patterns,
                            )?;
                        } else if token_str == "#" {
                            // Skip over # token as end of string reached, move to the root
                            parser.current = Some(0);
                            Tree::create_node(&mut nodes, &mut parser, NodeType::NamedGroup);
                            parser.state = ParserState::NamedGroup;
                        } else {
                            // Any other grammar is invalid for string.
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }
                    } else {
                        Tree::add_token(&mut nodes, &mut parser, NodeType::Text, &current);
                    }
                }
                ParserState::SubString => {
                    // Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                    #[cfg(feature = "logging")]
                    trace!("ParserState::SubString");

                    if tokens[current].token_type == TokenType::Grammar {
                        let token_str = tokens[current].string.as_str();
                        if token_str == "#" {
                            // Check that parent's last child is not a #
                            let parent = nodes[parser.current.unwrap()].parent.unwrap();
                            let children = nodes[parent].children.as_ref().unwrap();
                            if let Some(last) = children.last() {
                                if nodes[*last].node_type == NodeType::NumberSign {
                                    return Err(TreeError::MultiNumberSign(
                                        tokens[current].start_grapheme,
                                    ));
                                }
                            }
                            Tree::create_node_add_token(
                                &mut nodes,
                                &mut parser,
                                NodeType::NumberSign,
                                &current,
                            );
                        } else if token_str == "`" {
                            // Skip over ` token as next token is a literal.
                            let Some((next, token)) = lexer.next() else {
                                return Err(TreeError::EndedAbruptly);
                            };
                            Tree::push_token(&mut tokens, &mut lengths, token);
                            Tree::add_token(&mut nodes, &mut parser, NodeType::Text, &next);
                        } else if token_str == "{" {
                            // Skip over { token as it marks start of pattern.
                            let Some((next, token)) = lexer.next() else {
                                return Err(TreeError::EndedAbruptly);
                            };
                            Tree::push_token(&mut tokens, &mut lengths, token);
                            Tree::pattern_start(
                                &mut nodes,
                                &mut parser,
                                &mut tokens,
                                &next,
                                &mut patterns,
                            )?;
                        } else if token_str == "}" {
                            // Ends NamedString, and returns to NamedGroup
                            Tree::move_to_container(&mut nodes, &mut parser);
                            Tree::end_nested_state(&mut nodes, &mut parser);
                            let children =
                                nodes[parser.current.unwrap()].children.as_ref().unwrap();
                            if children.len() != 2 {
                                return Err(TreeError::InvalidToken(
                                    parser.state,
                                    tokens[current].start_grapheme,
                                    tokens[current].string.to_string(),
                                ));
                            }
                            Tree::end_nested_state(&mut nodes, &mut parser);
                        } else {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }
                    } else {
                        Tree::add_token(&mut nodes, &mut parser, NodeType::Text, &current);
                    }
                }
                ParserState::Pattern => {
                    // Valid tokens: PWS (separator - ignore), }, Identifier
                    #[cfg(feature = "logging")]
                    trace!("ParserState::Pattern");

                    let token_type = tokens[current].token_type;
                    if token_type == TokenType::Identifier {
                        Tree::create_node_add_token(
                            &mut nodes,
                            &mut parser,
                            NodeType::Identifier,
                            &current,
                        );
                        Tree::move_to_container(&mut nodes, &mut parser);
                        parser.state = ParserState::Keyword;
                    } else if tokens[current].token_type == TokenType::WhiteSpace {
                        // Optional whitespace ignored.
                    } else if tokens[current].token_type == TokenType::Grammar {
                        if tokens[current].string.as_str() != "}" {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }

                        // None (only identifier provide with default type of preformatted string)
                        Tree::end_nested_state(&mut nodes, &mut parser);
                    } else {
                        return Err(TreeError::InvalidToken(
                            parser.state,
                            tokens[current].start_grapheme,
                            tokens[current].string.to_string(),
                        ));
                    }
                }
                ParserState::Keyword => {
                    // Valid tokens: PWS (separator - ignore), }, Identifier
                    #[cfg(feature = "logging")]
                    trace!("ParserState::Keyword");

                    if tokens[current].token_type == TokenType::Identifier {
                        Tree::create_node(&mut nodes, &mut parser, NodeType::Selector);
                        Tree::add_token(&mut nodes, &mut parser, NodeType::Identifier, &current);
                        Tree::move_to_container(&mut nodes, &mut parser);
                        let Some((current, token)) = lexer.next() else {
                            return Err(TreeError::EndedAbruptly);
                        };
                        Tree::push_token(&mut tokens, &mut lengths, token);
                        if tokens[current].string.as_str() != "#" {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }
                        let Some((current, token)) = lexer.next() else {
                            return Err(TreeError::EndedAbruptly);
                        };
                        Tree::push_token(&mut tokens, &mut lengths, token);
                        if tokens[current].token_type != TokenType::Identifier {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }
                        Tree::add_token(&mut nodes, &mut parser, NodeType::Identifier, &current);
                        Tree::move_to_container(&mut nodes, &mut parser);
                        parser.current = nodes.get(parser.current.take().unwrap()).unwrap().parent;
                    } else if tokens[current].token_type == TokenType::WhiteSpace {
                    } else if tokens[current].token_type == TokenType::Grammar {
                        if tokens[current].string.as_str() == "}" {
                            Tree::end_nested_state(&mut nodes, &mut parser);
                        }
                    } else {
                        return Err(TreeError::InvalidToken(
                            parser.state,
                            tokens[current].start_grapheme,
                            tokens[current].string.to_string(),
                        ));
                    }
                }
                ParserState::LiteralText => {
                    //  Valid tokens: PWS, `, #, {, }, Identifier, Syntax
                    #[cfg(feature = "logging")]
                    trace!("ParserState::LiteralText");

                    if tokens[current].token_type == TokenType::Grammar
                        && tokens[current].string.as_str() == "`"
                    {
                        // Skip over 1st ` token.
                        let Some((next, token)) = lexer.next() else {
                            return Err(TreeError::EndedAbruptly);
                        };
                        Tree::push_token(&mut tokens, &mut lengths, token);
                        if tokens[current].string.as_str() == "`" {
                            // Literal 2nd ` found.
                            Tree::add_token(&mut nodes, &mut parser, NodeType::Text, &next);
                        } else {
                            Tree::end_nested_state(&mut nodes, &mut parser);
                        }
                        continue;
                    }
                    Tree::add_token(&mut nodes, &mut parser, NodeType::Text, &current);
                }
                ParserState::Literal => {
                    //  Valid tokens: }
                    #[cfg(feature = "logging")]
                    trace!("ParserState::Literal");

                    if tokens[current].token_type == TokenType::Grammar
                        && tokens[current].string.as_str() == "}"
                    {
                        Tree::end_nested_state(&mut nodes, &mut parser);
                        continue;
                    }
                    return Err(TreeError::InvalidToken(
                        parser.state,
                        tokens[current].start_grapheme,
                        tokens[current].string.to_string(),
                    ));
                }
                ParserState::Command => {
                    //  Valid tokens: PWS (separator - ignore), `, }, #, Identifier
                    #[cfg(feature = "logging")]
                    trace!("ParserState::Command");

                    if tokens[current].token_type == TokenType::Identifier {
                        Tree::create_node_add_token(
                            &mut nodes,
                            &mut parser,
                            NodeType::Identifier,
                            &current,
                        );
                        Tree::move_to_container(&mut nodes, &mut parser);
                    } else if tokens[current].token_type == TokenType::Grammar {
                        let token_str = tokens[current].string.as_str();
                        if token_str == "}" {
                            let children =
                                nodes[parser.current.unwrap()].children.as_ref().unwrap();
                            if children.is_empty() {
                                return Err(TreeError::InvalidToken(
                                    parser.state,
                                    tokens[current].start_grapheme,
                                    tokens[current].string.to_string(),
                                ));
                            }
                            Tree::end_nested_state(&mut nodes, &mut parser);
                        } else if token_str == "`" {
                            // Skip over ` token as next token is a literal.
                            let Some((next, token)) = lexer.next() else {
                                return Err(TreeError::EndedAbruptly);
                            };
                            Tree::push_token(&mut tokens, &mut lengths, token);
                            Tree::create_node_add_token(
                                &mut nodes,
                                &mut parser,
                                NodeType::Text,
                                &next,
                            );
                            parser.nested_states.push(ParserState::Command);
                            parser.state = ParserState::LiteralText;
                        } else if token_str == "#" {
                            // Only valid after command identifier.
                            let children =
                                nodes[parser.current.unwrap()].children.as_ref().unwrap();
                            if children.len() != 1 {
                                return Err(TreeError::InvalidToken(
                                    parser.state,
                                    tokens[current].start_grapheme,
                                    tokens[current].string.to_string(),
                                ));
                            }
                            Tree::create_node_add_token(
                                &mut nodes,
                                &mut parser,
                                NodeType::NumberSign,
                                &current,
                            );
                        } else {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }
                    } else if tokens[current].token_type == TokenType::WhiteSpace {
                    } else {
                        return Err(TreeError::InvalidToken(
                            parser.state,
                            tokens[current].start_grapheme,
                            tokens[current].string.to_string(),
                        ));
                    }
                }
                ParserState::NamedString => {
                    // Valid tokens: PWS (ignored - separator), `, #, {, Identifier, Syntax
                    #[cfg(feature = "logging")]
                    trace!("ParserState::NamedString");

                    #[cfg(feature = "logging")]
                    trace!("Nodes: {:?}", nodes);

                    if tokens[current].token_type == TokenType::Identifier {
                        let node = *parser.current.as_ref().unwrap();
                        let len = nodes[node].children.as_ref().unwrap().len();
                        if len == 0 {
                            let string = tokens[current].string.as_str().to_string();
                            if named_strings.contains_key(&string) {
                                return Err(TreeError::UniqueNamed(string));
                            }
                            named_strings.insert(string, current);
                            Tree::create_node_add_token(
                                &mut nodes,
                                &mut parser,
                                NodeType::Identifier,
                                &current,
                            );
                            Tree::move_to_container(&mut nodes, &mut parser);

                            // Check that white space separator follows identifier
                            let Some((current, token)) = lexer.next() else {
                                return Err(TreeError::EndedAbruptly);
                            };
                            Tree::push_token(&mut tokens, &mut lengths, token);
                            if tokens[current].token_type == TokenType::WhiteSpace {
                                // Skip over whitespace token separator.
                                continue;
                            }
                        } else if len == 1 {
                            Tree::create_node(&mut nodes, &mut parser, NodeType::String);
                            parser.nested_states.push(ParserState::NamedString);
                            Tree::create_node_add_token(
                                &mut nodes,
                                &mut parser,
                                NodeType::Text,
                                &current,
                            );
                            parser.state = ParserState::SubString;
                            continue;
                        }
                        return Err(TreeError::InvalidToken(
                            parser.state,
                            tokens[current].start_grapheme,
                            tokens[current].string.to_string(),
                        ));
                    } else if tokens[current].token_type == TokenType::Grammar {
                        let token_str = tokens[current].string.as_str();
                        if token_str == "#" {
                            // # may only appear after Identifier node indicating start of SubString.
                            let children =
                                nodes[parser.current.unwrap()].children.as_ref().unwrap();
                            if children.len() != 1 {
                                return Err(TreeError::InvalidToken(
                                    parser.state,
                                    tokens[current].start_grapheme,
                                    tokens[current].string.to_string(),
                                ));
                            }
                            Tree::create_node(&mut nodes, &mut parser, NodeType::String);
                            parser.nested_states.push(ParserState::NamedString);
                            Tree::create_node_add_token(
                                &mut nodes,
                                &mut parser,
                                NodeType::NumberSign,
                                &current,
                            );
                            parser.state = ParserState::SubString;
                        } else if token_str == "{" {
                            // { may only appear after Identifier node indicating start of SubString.
                            let children =
                                nodes[parser.current.unwrap()].children.as_ref().unwrap();
                            if children.len() != 1 {
                                return Err(TreeError::InvalidToken(
                                    parser.state,
                                    tokens[current].start_grapheme,
                                    tokens[current].string.to_string(),
                                ));
                            }
                            let Some((_currrent, token)) = lexer.next() else {
                                // Skip the next token
                                return Err(TreeError::EndedAbruptly);
                            };
                            Tree::push_token(&mut tokens, &mut lengths, token);
                            match Tree::pattern_start(
                                &mut nodes,
                                &mut parser,
                                &mut tokens,
                                &current,
                                &mut patterns,
                            ) {
                                Ok(_) => {}
                                Err(error) => return Err(error),
                            };
                        } else if token_str == "`" {
                            // ` may only appear after Identifier node indicating start of SubString.
                            let children =
                                nodes[parser.current.unwrap()].children.as_ref().unwrap();
                            if children.len() != 1 {
                                return Err(TreeError::InvalidToken(
                                    parser.state,
                                    tokens[current].start_grapheme,
                                    tokens[current].string.to_string(),
                                ));
                            }
                            let Some((current, token)) = lexer.next() else {
                                return Err(TreeError::EndedAbruptly);
                            };
                            Tree::push_token(&mut tokens, &mut lengths, token);
                            Tree::create_node(&mut nodes, &mut parser, NodeType::String);
                            parser.nested_states.push(ParserState::NamedString);
                            Tree::create_node_add_token(
                                &mut nodes,
                                &mut parser,
                                NodeType::Text,
                                &current,
                            );
                        } else {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }
                    } else if tokens[current].token_type == TokenType::Syntax {
                        // Syntax may only appear after Identifier node indicating start of SubString.
                        let children = nodes[parser.current.unwrap()].children.as_ref().unwrap();
                        if children.len() != 1 {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }
                        Tree::create_node(&mut nodes, &mut parser, NodeType::String);
                        parser.nested_states.push(ParserState::NamedString);
                        Tree::create_node_add_token(
                            &mut nodes,
                            &mut parser,
                            NodeType::Text,
                            &current,
                        );
                    } else if tokens[current].token_type == TokenType::WhiteSpace {
                        // Valid WhiteSpace is only a separator between Identifier and Substring.
                        let children = nodes[parser.current.unwrap()].children.as_ref().unwrap();
                        if children.len() != 1 {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }
                    } else {
                        return Err(TreeError::InvalidToken(
                            parser.state,
                            tokens[current].start_grapheme,
                            tokens[current].string.to_string(),
                        ));
                    }
                }
                ParserState::NamedGroup => {
                    // Valid tokens: PWS (ignored - human readability), {
                    #[cfg(feature = "logging")]
                    trace!("ParserState::NamedGroup");

                    if tokens[current].token_type == TokenType::Grammar {
                        if tokens[current].string.as_str() != "{" {
                            return Err(TreeError::InvalidToken(
                                parser.state,
                                tokens[current].start_grapheme,
                                tokens[current].string.to_string(),
                            ));
                        }

                        // start of NamedString
                        Tree::create_node(&mut nodes, &mut parser, NodeType::NamedString);
                        parser.nested_states.push(ParserState::NamedGroup);
                        parser.state = ParserState::NamedString;
                    } else if tokens[current].token_type == TokenType::WhiteSpace {
                    } else {
                        return Err(TreeError::InvalidToken(
                            parser.state,
                            tokens[current].start_grapheme,
                            tokens[current].string.to_string(),
                        ));
                    }
                }
            }
        }
        if !parser.nested_states.is_empty() {
            return Err(TreeError::EndedAbruptly);
        }
        Ok(Tree {
            tokens,
            has_grammar,
            lengths,
            nodes,
        })
    }

    // -- information methods --

    /// Obtain the lengths of the string in terms of bytes, characters, graphemes and tokens.
    #[allow(dead_code)]
    pub(crate) fn length(&self) -> &Length {
        &self.lengths
    }

    /// Indicates if the string contained any grammar syntax characters.
    pub(crate) fn has_grammar(&self) -> bool {
        self.has_grammar
    }

    /// Obtain reference to the node type for the specified node `node_index`.
    pub(crate) fn node_type(&self, node_index: &usize) -> &NodeType {
        &self.nodes[*node_index].node_type
    }

    /// Obtain reference to the node children for the specified node `node_index`.
    pub(crate) fn children(&self, node_index: &usize) -> &Vec<usize> {
        self.nodes[*node_index].children.as_ref().unwrap()
    }

    /// Convenience method to obtain the first child of the node `node_index`.
    pub(crate) fn first(&self, node_index: &usize) -> Option<&usize> {
        self.children(node_index).first()
    }

    /// Convenience method to obtain the last child of the node `node_index`.
    pub(crate) fn last(&self, node_index: &usize) -> Option<&usize> {
        self.children(node_index).last()
    }

    /// Obtain an immutable reference to the node's data for the specified node `node_index`.
    pub(crate) fn tokens(&self, node_index: &usize) -> Option<&Vec<usize>> {
        self.nodes[*node_index].tokens.as_ref() // )
    }

    pub(crate) fn token(&self, token_index: &usize) -> &Token {
        &self.tokens[*token_index]
    }

    // Internal methods

    fn push_token(tokens: &mut Vec<Token>, lengths: &mut Length, token: Token) {
        lengths.bytes += token.length_bytes;
        lengths.characters += token.length_characters;
        lengths.graphemes += token.length_graphemes;
        lengths.tokens += 1;
        tokens.push(token);
    }

    fn insert(
        nodes: &mut Vec<Node>,
        node_index: &usize, // Node to add to.
        node_type: NodeType,
    ) -> usize {
        // no external influence, thus errors should not be produced
        let parent = Some(*node_index);
        let children: Option<Vec<usize>> = if node_type.allow_children() {
            Some(Vec::<usize>::new())
        } else {
            None
        };
        let tokens: Option<Vec<usize>> = if node_type.allow_tokens() {
            Some(Vec::<usize>::new())
        } else {
            None
        };
        let node = Node {
            node_type,
            parent,
            children,
            tokens,
        };
        let index = nodes.len();
        nodes.push(node);
        let index_node = nodes.get_mut(*node_index).unwrap();
        index_node.children.as_mut().unwrap().push(index);
        index
    }

    // Move `current` to its parent node only if `current` is a ALLOW_CHILDREN node.
    // Usually this signals the ALLOW_CHILDREN has all its tokens.
    fn move_to_container(nodes: &mut [Node], parser: &mut ParserStates) {
        let node_index = *parser.current.as_ref().unwrap();
        let node = nodes.get(node_index).unwrap();
        if !node.node_type.allow_children() {
            parser.current = node.parent; // Root node always a ALLOW_CHILDREN.
        }
    }

    // Create a new child node of a specified type.
    // Also changes the parser current node index to the newly created child node.
    fn create_node(nodes: &mut Vec<Node>, parser: &mut ParserStates, node_type: NodeType) {
        Tree::move_to_container(nodes, parser);
        parser.current = Some(Tree::insert(
            nodes,
            &parser.current.take().unwrap(),
            node_type,
        ));
    }

    // Create a new child node of a specified type, and add the Token to this new node.
    // Also changes the parser current node index to the newly created node.
    fn create_node_add_token(
        nodes: &mut Vec<Node>,
        parser: &mut ParserStates,
        node_type: NodeType,
        token: &usize,
    ) {
        Tree::move_to_container(nodes, parser);
        parser.current = Some(Tree::insert(
            nodes,
            &parser.current.take().unwrap(),
            node_type,
        ));
        nodes
            .get_mut(*parser.current.as_ref().unwrap())
            .unwrap()
            .tokens
            .as_mut()
            .unwrap()
            .push(*token);
    }

    // Add to a token to a specified node type.
    // If current node is not the specified node type, a specified node type will be created, and current node is set
    // to it, before adding the token to the node.
    fn add_token(
        nodes: &mut Vec<Node>,
        parser: &mut ParserStates,
        node_type: NodeType,
        token: &usize,
    ) {
        let node = nodes.get_mut(*parser.current.as_ref().unwrap()).unwrap();
        if node.node_type == node_type {
            node.tokens.as_mut().unwrap().push(*token);
        } else {
            Tree::create_node_add_token(nodes, parser, node_type, token);
        }
    }

    // End the current nested state, change back to previous state and move to parent node.
    fn end_nested_state(nodes: &mut [Node], parser: &mut ParserStates) {
        parser.state = match parser.nested_states.pop() {
            Some(s) => s,
            None => ParserState::String,
        };
        parser.current = nodes.get(parser.current.take().unwrap()).unwrap().parent;
    }

    // Check if start of pattern is valid.
    fn pattern_start(
        nodes: &mut Vec<Node>,
        parser: &mut ParserStates,
        tokens: &mut [Token],
        token: &usize,
        patterns: &mut HashMap<String, usize>,
    ) -> Result<(), TreeError> {
        if tokens[*token].token_type == TokenType::Identifier {
            // Multilingual pattern
            Tree::create_node(nodes, parser, NodeType::Pattern);
            if patterns.contains_key(&tokens[*token].string) {
                return Err(TreeError::UniquePattern(tokens[*token].string.to_string()));
            }
            patterns.insert(
                tokens[*token].string.as_str().to_string(),
                *parser.current.as_ref().unwrap(),
            );
            Tree::create_node_add_token(nodes, parser, NodeType::Identifier, token);
            Tree::move_to_container(nodes, parser); // Move back to Pattern node.
            parser.nested_states.push(parser.state);
            parser.state = ParserState::Pattern;
        } else if tokens[*token].token_type == TokenType::Grammar {
            // Future: may allow empty {} to be treated as literal {}
            if tokens[*token].string.as_str() == "`" {
                // Literal pattern.
                Tree::create_node(nodes, parser, NodeType::Text);
                parser.nested_states.push(parser.state);
                parser.nested_states.push(ParserState::Literal);
                parser.state = ParserState::LiteralText;
            } else if tokens[*token].string.as_str() == "#" {
                // Command pattern.
                Tree::create_node(nodes, parser, NodeType::Command);
                parser.nested_states.push(parser.state);
                parser.state = ParserState::Command;
            } else {
                return Err(TreeError::InvalidToken(
                    parser.state,
                    tokens[*token].start_grapheme,
                    tokens[*token].string.to_string(),
                ));
            }
        } else {
            return Err(TreeError::InvalidToken(
                parser.state,
                tokens[*token].start_grapheme,
                tokens[*token].string.to_string(),
            ));
        }
        Ok(())
    }
}

// Various ParserStates the tokens may be in.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ParserState {
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

impl Display for ParserState {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ParserState::Command => write!(f, "Command"),
            ParserState::Keyword => write!(f, "Keyword"),
            ParserState::Literal => write!(f, "Literal"),
            ParserState::LiteralText => write!(f, "LiteralText"),
            ParserState::NamedGroup => write!(f, "NamedGroup"),
            ParserState::NamedString => write!(f, "NamedString"),
            ParserState::Pattern => write!(f, "Pattern"),
            ParserState::String => write!(f, "String"),
            ParserState::SubString => write!(f, "SubString"),
        }
    }
}

// A struct for tracking the parser states.
struct ParserStates {
    current: Option<usize>, // Node index
    state: ParserState,
    nested_states: Vec<ParserState>,
}
