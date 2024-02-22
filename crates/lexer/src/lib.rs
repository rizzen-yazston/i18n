// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! String lexer and resultant tokens.
//!
//! The `Lexer` is initialised using [`DataProvider`] enum of supported data providers to an
//! [Unicode Consortium] [CLDR] data repository (even a custom database). Usually the repository is just a local copy
//! of the CLDR in the application's data directory. Once the `Lexer` has been initialised using `new()` method, it may
//! be used to tokenise strings, without needing to re-initialising the `Lexer` before use.
//!
//! Consult the [ICU4X] website for instructions on generating a suitable data repository for the application, by
//! leaving out data that is not used by the application.
//!
//! Strings are tokenised using the method `tokenise()` taking string slice and a vector containing grammar syntax
//! characters.
//!
//! # Features
//!
//! Available features for `i18n_lexer` crate:
//!
//! * `compiled_data` (Preferred): Enable the `compiled_data` feature on `i18n_icu`. Allow for the internal data of
//! the various ICU4X components.
//!
//! * `blob`: Enable the `blob` feature on `i18n_icu`. Allow for instances of `BlobDataProvider` to be used
//! various ICU4X components that supports [`BufferProvider`].
//!
//! * `fs`: Enable the `fs` feature on `i18n_icu`. Allow for instances of `FsDataProvider` to be used
//! various ICU4X components that supports `BufferProvider`.
//!
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of [`Arc`] and [`Mutex`] instead [`Rc`] and
//! [`RefCell`].
//!
//! * `log`: Enables logging on `i18n_icu` crate.
//!
//! # Examples
//!
//! ```
//! use i18n_icu::{ IcuDataProvider, DataProvider };
//! use i18n_lexer::{Token, TokenType, Lexer};
//! use std::rc::Rc;
//! use std::error::Error;
//!
//! fn test_tokenise() -> Result<(), Box<dyn Error>> {
//!     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
//!     let mut lexer = Lexer::new( vec![ '{', '}' ], &icu_data_provider );
//!     let ( tokens, lengths, grammar ) =
//!         lexer.tokenise( "String contains a {placeholder}." );
//!     let mut grammar_tokens = 0;
//!     assert_eq!( lengths.bytes, 32, "Supposed to be a total of 32 bytes." );
//!     assert_eq!( lengths.characters, 32, "Supposed to be a total of 32 characters." );
//!     assert_eq!( lengths.graphemes, 32, "Supposed to be a total of 32 graphemes." );
//!     assert_eq!( lengths.tokens, 10, "Supposed to be a total of 10 tokens." );
//!     for token in tokens.iter() {
//!     if token.token_type == TokenType::Grammar {
//!          grammar_tokens += 1;
//!         }
//!     }
//!     assert_eq!( grammar_tokens, 2, "Supposed to be 2 grammar tokens." );
//!     assert!(grammar, "There supposed to be grammar tokens." );
//!     Ok( () )
//! }
//! ```
//!
//! [`DataProvider`]: i18n_icu::DataProvider
//! [Unicode Consortium]: https://home.unicode.org/
//! [CLDR]: https://cldr.unicode.org/
//! [ICU4X]: https://github.com/unicode-org/icu4x
//! [`BufferProvider`]: https://docs.rs/icu_provider/1.2.0/icu_provider/buf/trait.BufferProvider.html

#[cfg(doc)]
use std::sync::{Arc, Mutex};

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use std::cell::RefCell;

pub mod lexer;
pub use lexer::*;
