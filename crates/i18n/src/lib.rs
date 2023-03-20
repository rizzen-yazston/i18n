// This file is part of `i18n-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n-rizzen-yazston` crate.

//! `i18n` is the main meta-crate of the Internationalisation (`i18n`) project.
//! 
//! This convenience meta crate contains selected available crates:
//! 
//! * i18n_lexer-rizzen-yazston
//! 
//! * i18n_lstring-rizzen-yazston
//! 
//! * i18n_pattern-rizzen-yazston
//! 
//! * i18n_message-rizzen-yazston
//! 
//! * i18n_provider-rizzen-yazston
//! 
//! * i18n_provider_sqlite3-rizzen-yazston
//! 
//! * i18n_registry-rizzen-yazston
//! 
//! # Usage
//! 
//! For most use cases, just the use of `i18n-rizzen-yazston` crate will be sufficient to use the multilingual message
//! system, though the individual crates can be selected individual if the entire `i18n` project is not required.
//! 
//! ## Cargo.toml
//! 
//! ```
//! [dependencies]
//! i18n-rizzen-yazston = "0.5.0"
//! ```
//! 
//! ## Examples
//! 
//! See the various component crates for usage examples.

pub use i18n_lexer-rizzen-yazston as lexer;
pub use i18n_lstring-rizzen-yazston as lstring;
pub use i18n_message-rizzen-yazston as message;
pub use i18n_pattern-rizzen-yazston as pattern;
pub use i18n_provider-rizzen-yazston as provider;
pub use i18n_provider_sqlite3-rizzen-yazston as provider_sqlite3;
pub use i18n_registry-rizzen-yazston as registry;
