// This file is part of `i18n-lstring` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n-lstring` crate.

/// `LString` (short for `LanguageString`) is a simple struct for associating a text string to a specific language.
/// 
/// The ICU's `Locale` is used instead of the ICU's `LanguageIdentifier` due to that the `Locale` supports the entire
/// [BCP47 Language Tag](https://www.rfc-editor.org/rfc/bcp/bcp47.txt), where as the `LanguageIdentifier` excludes
/// the **extension** subtags of the BCP47 Language Tag.

use icu_locid::Locale;
use std::rc::Rc;

pub struct LString {
    string: String,
    locale: Rc<Locale>,
}

impl LString {
    /// Creates a LString object from a String and a reference counter Locale.
    pub fn new( string: String, locale: Rc<Locale> ) -> Self {
        LString { string, locale }
    }

    /// Returns a reference to the internal String.
    pub fn string( &self ) -> &str {
        &self.string
    }

    /// Returns a reference counter Locale.
    pub fn locale( &self ) -> Rc<Locale> {
        Rc::clone( &self.locale )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let string = "This is a test string.";
        let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
        let lang_string = LString::new( String::from( string ), Rc::clone( &locale ) );
    
        assert_eq!( lang_string.locale(), locale, "Locale failed." );
    }
}
