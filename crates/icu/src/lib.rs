// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_icu-rizzen-yazston` crate.

//! ICU4X data provider helper.
//! 
//! The `IcuDataProvider` type contains a member `data_provider` holding the `DataProvider`, which is a deserialised
//! `BufferProvider`.
//! 
//! The `IcuDataProvider` type also contains non-locale based data used within the `i18n_lexer` crate.
//! 
//! # Examples
//! 
//! See various examples of `i18n_lexer`, `i18n_pattern`, and `i18n_message` crates.

pub mod icu;
pub use icu::*;
pub mod error;
pub use error::*;
