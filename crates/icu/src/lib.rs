// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_icu-rizzen-yazston` crate.

//! [ICU4X] project (maintained by the [Unicode Consortium]) data provider helper.
//! 
//! The `IcuDataProvider` type contains a member `data_provider` holding the ICU4X `&`[`DataProvider`] as a
//! `DataProviderWrapper` type.
//! 
//! The `IcuDataProvider` type also contains non-locale based data used within the `i18n_lexer` crate.
//! 
//! `IcuDataProvider` type is used within the `Rc` type as `Rc<IcuDataProvider>` to prevent unnecessary duplication.
//! 
//! # Examples
//! 
//! See various examples of the `i18n_lexer`, `i18n_pattern`, and `i18n_message` crates.
//! 
//! [ICU4X]: https://github.com/unicode-org/icu4x
//! [Unicode Consortium]: https://home.unicode.org/

#[cfg( doc )]
use icu_provider::DataProvider;

pub mod icu;
pub use icu::*;
pub mod error;
pub use error::*;
