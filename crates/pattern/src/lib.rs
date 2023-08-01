// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! Parsing of string tokens into an Abstract Syntax Tree (AST), checking the grammar of patterns is valid.
//! The parser only does the syntactic analysis of the supplied Token vector.
//! 
//! The formatter takes the Abstract Syntax Tree (AST) that was generated by the parser and constructs a string
//! template for values to be substituted into. The formatter also does the semantic analysis of the grammar. Once a
//! string template has been constructed, it can be used multiple times by simply supplying new placeholder values when
//! executing the `format()` method. Depending on the placeholder type of the pattern, a suitable selection of the
//! available data types can be used, these include the basic `String`, integers, unsigned integers and floats. In
//! addition to these special ICU4X types are supported such as `FixedDecimal`, `Date`, `Time` and `DateTime` structs.
//! 
//! See `pattern strings.asciidoc` in `docs` of `pattern` for the pattern formatting specification.
//! 
//! # Features
//! 
//! Available features for `i18n_pattern` crate:
//! 
//! * `compiled_data`: Allow for the internal data of the various ICU4X components.
//! 
//! * `blob`: Allow for instances of `BlobDataProvider` to be used various ICU4X components that supports [`BufferProvider`].
//! 
//! * `fs`: Allow for instances of `FsDataProvider` to be used various ICU4X components that supports `BufferProvider`.
//! 
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//! [`RefCell`].
//! 
//! # Examples
//! 
//! ```
//! use i18n_icu::{ IcuDataProvider, DataProvider };
//! use i18n_lexer::{Token, TokenType, Lexer};
//! use i18n_pattern::{
//!     parse, NodeType, Formatter, FormatterError, PlaceholderValue, CommandRegistry, english_a_or_an
//! };
//! use icu_locid::Locale;
//! use std::collections::HashMap;
//! use std::rc::Rc;
//! use std::error::Error;
//! 
//! fn pattern_plural() -> Result<(), Box<dyn Error>> {
//!     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
//!     let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
//!     let ( tokens, _lengths, _grammar ) =
//!         lexer.tokenise( "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}" );
//!     let tree = parse( tokens )?;
//!     let locale: Rc<Locale> = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
//!     let language_tag = Rc::new( locale.to_string() );
//!     let command_registry = Rc::new( CommandRegistry::new() );
//!     let mut formatter = Formatter::try_new(
//!         &icu_data_provider, &language_tag, &locale, &tree, &command_registry
//!     )?;
//!     let mut values = HashMap::<String, PlaceholderValue>::new();
//!     values.insert(
//!         "dogs_number".to_string(),
//!         PlaceholderValue::Unsigned( 3 )
//!     );
//!     let result = formatter.format( &values )?;
//!     assert_eq!( result.as_str(), "There are 3 dogs in the park.", "Strings must be the same." );
//!     Ok( () )
//! }
//! 
//! fn command_delayed() -> Result<(), Box<dyn Error>> {
//!     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
//!     let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
//!     let ( tokens, _lengths, _grammar ) =
//!         lexer.tokenise( "At night {#english_a_or_an# hunter} {hunter} stalked {#english_a_or_an # prey} {prey}." );
//!     let tree = parse( tokens )?;
//!     let locale: Rc<Locale> = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
//!     let language_tag = Rc::new( locale.to_string() );
//!     let command_registry = Rc::new( CommandRegistry::new() );
//!     command_registry.insert( "english_a_or_an", english_a_or_an )?;
//!     let mut formatter =
//!         Formatter::try_new( &icu_data_provider, &language_tag, &locale, &tree, &command_registry )?;
//!     let mut values = HashMap::<String, PlaceholderValue>::new();
//!     values.insert(
//!         "hunter".to_string(),
//!         PlaceholderValue::String( "owl".to_string() )
//!     );
//!     values.insert(
//!         "prey".to_string(),
//!         PlaceholderValue::String( "mouse".to_string() )
//!     );
//!     let result = formatter.format( &values )?;
//!     assert_eq!(
//!         result.as_str(),
//!         "At night an owl stalked a mouse.",
//!         "Strings must be the same."
//!     );
//!     Ok( () )
//! }
//! ```
//! 
//! [`BufferProvider`]: https://docs.rs/icu_provider/1.2.0/icu_provider/buf/trait.BufferProvider.html

#[cfg( doc )]
use std::sync::{ Arc, Mutex };

#[cfg( doc )]
use std::rc::Rc;

#[cfg( doc )]
use std::cell::RefCell;

pub mod types;
pub use types::*;
pub mod parser;
pub use parser::*;
pub mod command;
pub use command::*;
pub mod formatter;
pub use formatter::*;
pub mod error;
pub use error::*;
