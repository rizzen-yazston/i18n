// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

//! LString provider.
//! 
//! Provides `LString`.
//! 
//! # Examples
//! 
//! ```
//! // TODO
//! ```
//! 

use i18n_lstring::LString;
use icu_locid::Locale;
use std::rc::Rc;


/// LString provider.
/// 
/// TODO:
/// 
/// # Examples
/// 
/// ```
/// // TODO
/// ```
/// [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
/// [`icu_locid`]: https://crates.io/crates/icu_locid

pub trait LStringProvider {
    fn get( &self, identifier: &str, locale: &Rc<Locale> ) -> Result<LString, String>;    
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        /*
        let string = "This is a test string.";
        let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
        let lang_string = LString::new( String::from( string ), Rc::clone( &locale ) );
    
        assert_eq!( lang_string.locale(), locale, "Locale failed." );
        */
    }
}
*/
