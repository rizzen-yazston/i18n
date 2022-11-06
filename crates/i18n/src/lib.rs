// This file is part of `i18n-rizzen-yazston` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n-rizzen-yazston` crate.

//! `i18n` is the main meta-crate of the Internationalisation (`i18n`) project.
//! 
//! This convenience meta crate contains selected available crates:
//! 
//! * i18n_lstring-rizzen-yazston
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
//! icu_locid = "1.0.0"
//! i18n-rizzen-yazston = "0.2.0"
//! ```
//! 
//! ## Examples
//! 
//! See the various component crates for usage examples.

pub use i18n_lstring-rizzen-yazston as lstring;
//pub use pattern-rizzen-yazston as pattern;
//pub use message-rizzen-yazston as message;
