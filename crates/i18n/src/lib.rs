// This file is part of `i18n-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n-rizzen-yazston` crate.

//! Welcome to the **i18n** crate of the *Internationalisation* (`i18n`) project.
//! 
//! This is the main meta-crate of the project.
//! 
//! This convenience meta crate contains selected available crates:
//! 
//! * `i18n_icu-rizzen-yazston`: Contains ICU4X data provider helper,
//! 
//! * `i18n_lexer-rizzen-yazston`: A simple lexer to tokenise a string,
//! 
//! * `i18n_message-rizzen-yazston`: The multilingual messaging system,
//! 
//! * `i18n_pattern-rizzen-yazston`: Similar to the `icu_pattern` crate of ICU4X, though with the added support of macro functionality,
//! 
//! * `i18n_provider-rizzen-yazston`: Trait for providing language strings, and error struct,
//! 
//! * `i18n_provider_sqlite3-rizzen-yazston`: Implementation of `i18n_provider` using Sqlite3 as its data store,
//! 
//! * `i18n_utility-rizzen-yazston`: Contains the Language tag registry, and the LString type (language tagged string).
//! 
//! # Usage
//! 
//! For most use cases, just the use of `i18n-rizzen-yazston` crate will be sufficient to use the multilingual message
//! system, though the individual crates can be selected individual if the entire `i18n` project is not required.
//! 
//! ## Features
//! 
//! Available features for `i18n_icu` crate:
//! 
//! * `compiled_data`: Allow for the internal data of the various ICU4X components.
//! 
//! * `blob`: Allow for instances of `BlobDataProvider` to be used various ICU4X components that supports [`BufferProvider`].
//! 
//! * `fs`: Allow for instances of `FsDataProvider` to be used various ICU4X components that supports `BufferProvider`.
//! 
//! ## Examples
//! 
//! See the various component crates for usage examples.
//! 
//! [`BufferProvider`]: https://docs.rs/icu_provider/1.2.0/icu_provider/buf/trait.BufferProvider.html

pub use i18n_icu as icu;
pub use i18n_lexer as lexer;
pub use i18n_message as message;
pub use i18n_pattern as pattern;
pub use i18n_provider as provider;
pub use i18n_provider_sqlite3 as provider_sqlite3;
pub use i18n_utility as utility;
