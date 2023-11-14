// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n_icu-rizzen-yazston` crate.

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
//! # Features
//! 
//! Available features for `i18n_icu` crate:
//! 
//! * `compiled_data` (Preferred): Allow for the internal data of the various ICU4X components. Also recommended by
//! ICU4X.
//! 
//! * `blob`: Allow for instances of [`BlobDataProvider`] to be used various ICU4X components that supports
//! [`BufferProvider`]. An alternative provider when the internal data of ICU4X components are insufficient for a
//! particular use case.
//! 
//! * `fs`: Allow for instances of [`FsDataProvider`] to be used various ICU4X components that supports
//! `BufferProvider`. An alternative provider when the internal data of ICU4X components are insufficient for a
//! particular use case.
//! 
//! * `sync`: Allow for rust's concurrency capabilities to be used. Use of `Arc` and [`Mutex`] instead `Rc` and
//! [`RefCell`].
//! 
//! * `log`: Enable log support.
//! 
//! # Examples
//! 
//! See various examples of the `i18n_lexer`, `i18n_pattern`, and `i18n_message` crates.
//! 
//! [ICU4X]: https://github.com/unicode-org/icu4x
//! [Unicode Consortium]: https://home.unicode.org/
//! [`BlobDataProvider`]: https://docs.rs/icu_provider_blob/1.2.0/icu_provider_blob/struct.BlobDataProvider.html
//! [`FsDataProvider`]: https://docs.rs/icu_provider_fs/1.2.1/icu_provider_fs/struct.FsDataProvider.html
//! [`BufferProvider`]: https://docs.rs/icu_provider/1.2.0/icu_provider/buf/trait.BufferProvider.html

#[cfg( doc )]
use std::sync::{ Arc, Mutex };

#[cfg( doc )]
use std::rc::Rc;

#[cfg( doc )]
use std::cell::RefCell;

pub mod icu;
pub use icu::*;
pub mod error;
pub use error::*;
