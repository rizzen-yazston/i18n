// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! String lexer and resultant tokens.
//! 
//! The `Lexer` is initialised using a data provider [`BufferProvider`] to an [Unicode Consortium] [CLDR] data
//! repository, usually it is just a local copy of the CLDR in the application's data directory. Once the `Lexer` has
//! been initialised it may be used to tokenise strings, without needing to re-initialising the `Lexer` before use.
//! Consult the [ICU4X] website for instructions on generating a suitable data repository for the application, by
//! leaving out data that is not used by the application. 
//! 
//! Strings are tokenised using the method `tokenise()` taking string slice and a vector containing grammar syntax
//! characters.
//! 
//! # Examples
//! 
//! ```
//! use i18n_icu::IcuDataProvider;
//! use i18n_lexer::{Token, TokenType, Lexer};
//! use icu_testdata::buffer;
//! use icu_provider::serde::AsDeserializingBufferProvider;
//! use std::rc::Rc;
//! use std::error::Error;
//! 
//! fn tokenise() -> Result<(), Box<dyn Error>> {
//!     let buffer_provider = buffer();
//!     let data_provider = buffer_provider.as_deserializing();
//!     let icu_data_provider = IcuDataProvider::try_new( data_provider )?;
//!     let mut lexer = Lexer::try_new( &Rc::new( icu_data_provider ) )?;
//!     let tokens = lexer.tokenise(
//!         "String contains a {placeholder}.", &vec![ '{', '}' ]
//!     );
//!     let mut grammar = 0;
//!     assert_eq!( tokens.0.iter().count(), 10, "Supposed to be a total of 10 tokens." );
//!     for token in tokens.0.iter() {
//!         if token.token_type == TokenType::Grammar {
//!             grammar += 1;
//!         }
//!     }
//!     assert_eq!( grammar, 2, "Supposed to be 2 grammar tokens." );
//!     Ok( () )
//! }
//! ```
//! 
//! [`BufferProvider`]: https://docs.rs/icu_provider/latest/icu_provider/buf/trait.BufferProvider.html
//! [Unicode Consortium]: https://home.unicode.org/
//! [CLDR]: https://cldr.unicode.org/
//! [ICU4X]: https://github.com/unicode-org/icu4x

pub mod lexer;
pub use lexer::*;
pub mod error;
pub use error::*;
