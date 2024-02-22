// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! Used by `parser` module.

use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

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
