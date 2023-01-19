// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! TODO: Complete crate description
//! 
//! # Examples
//! 
//! ```
//! // TODO: crate example
//! ```

use i18n_lstring::LString;
use fixed_decimal::FixedDecimal;
use icu_calendar::{AnyCalendar, Date, DateTime, types::Time };
use tree::Tree;
use std::collections::HashMap;
use core::any::Any;
use std::fmt;

/// Enum of the node types that can exist in the generate AST.
/// The following node types are available:
/// * Root: [Container] The top level, which may optional contained NamedGroup container, and required String container.
/// * NamedGroup: [Container] Container: Exists if at least 1 named substring (NamedString node) is detected.
/// * NamedString: [Container] Contains the Identifier leaf, and its substring String container
/// * String: [Container] represents either whole string, or a substring for a plural or select pattern.
/// * Text: [Leaf] Just literal text, and consist of 1 or more tokens (of any type that are treated as text).
/// * NumberSign: [Leaf] The number pattern `#` in text.
/// * Command: [Container] Contains command pattern data.
/// * Pattern: [Container] Usually a multilingual pattern data. 2nd node indicates pattern type.
/// * Identifier: [Leaf] An identifier. Always 1 token.
/// * Selector: [Container] Contains 2 Identifier nodes. Used for `plural` and `select` patterns.
#[derive( PartialEq )]
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

impl fmt::Display for NodeType {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            NodeType::Root => write!( f, "Root" ),
            NodeType::NamedGroup => write!( f, "NamedGroup" ),
            NodeType::NamedString => write!( f, "NamedString" ),
            NodeType::String => write!( f, "String" ),
            NodeType::Text => write!( f, "Text" ),
            NodeType::NumberSign => write!( f, "NumberSign" ),
            NodeType::Command => write!( f, "Command" ),
            NodeType::Pattern => write!( f, "Pattern" ),
            NodeType::Identifier => write!( f, "Identifier" ),
            NodeType::Selector => write!( f, "Selector" )
        }
    }
}

pub struct ParserResult {
    pub tree: Tree,
    pub named_strings: HashMap<String, usize>,
    pub patterns: HashMap<String, usize>,
}

// --- placeholder value types ---
// Once the complex `FixedDecimal` from ICU4X is complete it will be added as a type

pub trait PlaceholderValue {
    fn get_type( &self ) -> &str;
    fn as_any( &self ) -> &dyn Any;
}

pub struct PlaceholderString {
    pub string: String,
}

impl PlaceholderValue for PlaceholderString {
    fn get_type( &self ) -> &str {
        "PlaceholderString"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}

pub struct PlaceholderLString {
    pub string: LString,
}

impl PlaceholderValue for PlaceholderLString {
    fn get_type( &self ) -> &str {
        "PlaceholderLString"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}

pub struct PlaceholderInteger {
    pub number: i128,
}

impl PlaceholderValue for PlaceholderInteger {
    fn get_type( &self ) -> &str {
        "PlaceholderInteger"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}

pub struct PlaceholderUnsigned {
    pub number: u128,
}

impl PlaceholderValue for PlaceholderUnsigned {
    fn get_type( &self ) -> &str {
        "PlaceholderUnsigned"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}

pub struct PlaceholderFloat {
    pub number: f64,
}

impl PlaceholderValue for PlaceholderFloat {
    fn get_type( &self ) -> &str {
        "PlaceholderFloat"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}

pub struct PlaceholderFixedDecimal {
    pub number: FixedDecimal,
}

impl PlaceholderValue for PlaceholderFixedDecimal {
    fn get_type( &self ) -> &str {
        "PlaceholderFixedDecimal"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}

pub struct PlaceholderDateTime {
    pub date_time: DateTime<AnyCalendar>,
}

impl PlaceholderValue for PlaceholderDateTime {
    fn get_type( &self ) -> &str {
        "PlaceholderDateTime"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}

pub struct PlaceholderDate {
    pub date: Date<AnyCalendar>,
}

impl PlaceholderValue for PlaceholderDate {
    fn get_type( &self ) -> &str {
        "PlaceholderDate"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}

pub struct PlaceholderTime {
    pub time: Time,
}

impl PlaceholderValue for PlaceholderTime {
    fn get_type( &self ) -> &str {
        "PlaceholderTime"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }
}


/*
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
*/
