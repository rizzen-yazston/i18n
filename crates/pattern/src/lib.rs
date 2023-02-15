// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! The `i18n_pattern` crate contains the messaging system.
//!
//! Contains the follow modules:
//!
//! * `types`: Contains the enum `NodeType`, and placeholder values types, and enum.
//!
//! * `parser`: Parsers a vector of tokens into a `ParserResult`.
//! 
//! * `formatter`: Takes an `ParserResult` with supplied values to create a language string of the specified `Locale`.

pub mod types;
pub mod parser;
//pub mod formatter;

pub use types::*;
pub use parser::*;
//pub use formatter::*;
