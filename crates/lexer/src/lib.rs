// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_lexer-rizzen-yazston` crate.

//! Welcome to the **`i18n_lexer`** crate of the *Internationalisation* (i18n) project.
//!
//! This crate contains three modules:
//!
//! * [`icu`]: The ICU data provider wrapper.
//!
//! * [`lexer`]: The lexer iterator.
//!
//! * [`error`]: The error enums for the other modules.
//!
//! ## Features
//!
//! Available features for `i18n_lexer` crate:
//!
//! * `icu_blob`: Allow for instances of `BlobDataProvider` to be used various ICU4X components that supports
//! [`BufferProvider`]. An alternative provider when the internal data of ICU4X components are insufficient for a
//! particular use case.
//!
//! * `icu_compiled_data` \[default\]: Allow for the internal data of the various ICU4X components.
//!
//! * `icu_extended`: Include the extended data of various ICU's components.
//!
//! * `icu_fs`: Allow for instances of `FsDataProvider` to be used various ICU4X components that supports
//! `BufferProvider`. An alternative provider when the internal data of ICU4X components are insufficient for a
//! particular use case.
//!
//! * `logging`: To provide some logging information.
//!
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of `Arc` and `Mutex` instead `Rc` and
//! `RefCell`.
//!
//! ## Modules
//!
//! ### `icu`: ICU data provider wrapper
//!
//! [ICU4X] project (maintained by the [Unicode Consortium]) data provider helper.
//!
//! The `IcuDataProvider` type contains the `DataProvider` enum of supported implementations of ICU4X [`DataProvider`].
//! Depending on the features selected, they are: `Internal` (internally uses the BakedDataProvider),
//! [`BlobDataProvider`], or [`FsDataProvider`].
//!
//! When data provider is not `Internal` and depending on the data provider used, the `IcuDataProvider` may contain
//! non-locale based data, such as the grapheme cluster segmenter and the selected character properties set data.
//!
//! `IcuDataProvider` type is used within the [`Rc`] type as `Rc<IcuDataProvider>` or [`Arc`] type as
//! `Arc<IcuDataProvider>` to prevent unnecessary duplication.
//!
//! ### `lexer`: The Lexer Iterator
//!
//! The `LexerIterator` is created with the `DataProvider` enum of supported data providers. Usually the ICU
//! components includes internal compiled data, thus other data providers are not required. The crate's default
//! feature includes the `icu_compiled_data` feature. If the internal compiled data is not used, the data repository
//! is just a customised local copy of the [Unicode Consortium] [CLDR], and usually located in the application's data
//!  directory.
//!
//! Consult the {icu4x}[ICU4X] website for instructions on generating a suitable data repository for the application,
//! by leaving out data that is not used by the application.
//!
//! The grammar syntax characters along with the string are also passed at time of creating the `LexerInterator`.
//!
//! ### `error`
//!
//! Contains the error enums for both ICU and lexer modules.
//!
//! ## Examples
//!
//! ```
//! use i18n_lexer::{Token, TokenType, LexerIterator, IcuDataProvider, DataProvider};
//! use std::rc::Rc;
//! use std::error::Error;
//!
//! fn tokenise_single_byte_character_string() -> Result<(), Box<dyn Error>> {
//!     let icu_data_provider = Rc::new(
//!         IcuDataProvider::try_new(DataProvider::Internal)?
//!     );
//!     let mut grammar_tokens = 0;
//!     let mut syntax_tokens = 0;
//!     let mut white_space_tokens = 0;
//!     let mut identifier_tokens = 0;
//!     let mut length_bytes = 0;
//!     let mut length_characters = 0;
//!     let mut length_graphemes = 0;
//!     let mut length_tokens = 0;
//!     for token in LexerIterator::try_new(
//!         "String contains a {placeholder}.",
//!         "{}",
//!         &icu_data_provider
//!     ).unwrap() {
//!         match token.token_type {
//!             TokenType::Grammar => grammar_tokens += 1,
//!             TokenType::Syntax => syntax_tokens += 1,
//!             TokenType::WhiteSpace => white_space_tokens += 1,
//!             TokenType::Identifier => identifier_tokens += 1,
//!         }
//!         length_bytes += token.length_bytes;
//!         length_characters += token.length_characters;
//!         length_graphemes += token.length_graphemes;
//!         length_tokens += 1;
//!     }
//!     assert_eq!(length_bytes, 32, "Supposed to be a total of 32 bytes.");
//!     assert_eq!(length_characters, 32, "Supposed to be a total of 32 characters.");
//!     assert_eq!(length_graphemes, 32, "Supposed to be a total of 32 graphemes.");
//!     assert_eq!(length_tokens, 10, "Supposed to be a total of 10 tokens.");
//!     assert_eq!(grammar_tokens, 2, "Supposed to be 2 grammar tokens.");
//!     assert_eq!(syntax_tokens, 1, "Supposed to be 1 syntax token.");
//!     assert_eq!(white_space_tokens, 3, "Supposed to be 3 white space tokens.");
//!     assert_eq!(identifier_tokens, 4, "Supposed to be 4 identifier tokens.");
//!     Ok(())
//! }
//! ```
//!
//! ```
//! use i18n_lexer::{Token, TokenType, LexerIterator, IcuDataProvider, DataProvider};
//! use std::rc::Rc;
//! use std::error::Error;
//!
//! fn tokenise_single_byte_character_string() -> Result<(), Box<dyn Error>> {
//!     let icu_data_provider = Rc::new(
//!         IcuDataProvider::try_new(DataProvider::Internal)?
//!     );
//!     let mut grammar_tokens = 0;
//!     let mut syntax_tokens = 0;
//!     let mut white_space_tokens = 0;
//!     let mut identifier_tokens = 0;
//!     let mut length_bytes = 0;
//!     let mut length_characters = 0;
//!     let mut length_graphemes = 0;
//!     let mut length_tokens = 0;
//!     for token in LexerIterator::try_new(
//!         "Earth = \u{1F30D}. 각",
//!         "{}",
//!         &icu_data_provider
//!     ).unwrap() {
//!         match token.token_type {
//!             TokenType::Grammar => grammar_tokens += 1,
//!             TokenType::Syntax => syntax_tokens += 1,
//!             TokenType::WhiteSpace => white_space_tokens += 1,
//!             TokenType::Identifier => identifier_tokens += 1,
//!         }
//!         length_bytes += token.length_bytes;
//!         length_characters += token.length_characters;
//!         length_graphemes += token.length_graphemes;
//!         length_tokens += 1;
//!     }
//!     assert_eq!(length_bytes, 20, "Supposed to be a total of 20 bytes.");
//!     assert_eq!(length_characters, 13, "Supposed to be a total of 13 characters.");
//!     assert_eq!(length_graphemes, 12, "Supposed to be a total of 12 graphemes.");
//!     assert_eq!(length_tokens, 8, "Supposed to be a total of 8 tokens.");
//!     assert_eq!(grammar_tokens, 0, "Supposed to be 0 grammar tokens.");
//!     assert_eq!(syntax_tokens, 2, "Supposed to be 2 syntax tokens.");
//!     assert_eq!(white_space_tokens, 3, "Supposed to be 3 white space tokens.");
//!     assert_eq!(identifier_tokens, 3, "Supposed to be 3 identifier tokens.");
//!     Ok(())
//! }
//! ```
//!
//! [`DataProvider`]: crate::DataProvider
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

#[cfg(doc)]
use crate::DataProvider;

#[cfg(doc)]
use icu_provider_blob::BlobDataProvider;

#[cfg(doc)]
use icu_provider_fs::FsDataProvider;

pub mod icu;
pub use icu::*;
pub mod lexer;
pub use lexer::*;
pub mod error;
pub use error::*;
