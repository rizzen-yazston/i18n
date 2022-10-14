// This file is part of `i18n-utility` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n-utility` crate.

/// `LocaleRegistry` is a simple container that contains `Locale` objects for provided language tags. The registry aids 
/// in reducing the need of parsing language tags repeatedly, by storing the result Locale for provided language tag in
/// the registry.
/// 
/// The ICU's `Locale` is used instead of the ICU's `LanguageIdentifier` due to that the `Locale` supports the entire
/// [BCP47 Language Tag](https://www.rfc-editor.org/rfc/bcp/bcp47.txt), where as the `LanguageIdentifier` excludes
/// the **extension** subtags of the BCP47 Language Tag.

use icu_locid::Locale;
use std::rc::Rc;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct LocaleRegistry {
    registry: HashMap<String, Rc<Locale>>
//    string: String,
//    locale: Rc<Locale>,
}

impl LocaleRegistry {
    /// Creates an empty registry. Use get( String ) method to obtain a locale.
    pub fn new() -> Self {
        LocaleRegistry { registry: HashMap::<String, Rc<Locale>>::new() }
    }

    /// Obtain a Locale reference for the specified language tag (either BCP 47 Language Tag format or old format),
    /// though the Locale always formats the string as BCP 47 Language Tag.
    /// An error is returned if the language tag is malformed.
    /// Note: If the language tag is of the old format, that is containing underscores (_), then the old format
    /// language tag and the BCP 47 Language Tag will be both be added to the registry using the same Locale.
    pub fn get( &mut self, language_tag: String ) -> Result<Rc<Locale>, String> {
        if let Some( locale ) = self.registry.get( &language_tag ) {
            return Ok( Rc::clone( locale ) );
        }
        match Locale::try_from_bytes( language_tag.as_bytes() ) {
            Err( _error ) => return Err( "Invalid language tag.".to_string() ),
            Ok( locale_new ) => {
                let tag = locale_new.to_string();
                let mut locale: Option<Rc<Locale>> = None;
                {
                    if let Some( _locale ) = self.registry.get( &tag ) {
                        locale = Some( Rc::clone( _locale ) );
                    }
                }
                {
                    if !locale.is_none() {
                        let locale2 = locale.unwrap();
                        self.registry.insert( language_tag, Rc::clone( &locale2 ) );
                        return Ok( Rc::clone( &locale2 ) );
                    }
                }
                let rc_locale_new = Rc::new( locale_new );
                if language_tag != tag {
                    self.registry.insert( tag, Rc::clone( &rc_locale_new ) );
                }
                self.registry.insert( language_tag, Rc::clone( &rc_locale_new ) );
                return Ok( Rc::clone( &rc_locale_new ) );
            }
        }
    }

    /// Returns a vector list of all the registered language tags of both old format and BCP 47 Language Tag format.
    pub fn list( &self ) -> Vec<&String> {
        Vec::from_iter( self.registry.keys() )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut registry = LocaleRegistry::new();
        let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
        let entries = registry.list().iter().count();
    
        assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
        assert_eq!( entries, 2, "Supposed to be 2 entries: en_ZA and en-ZA." )
    }
}
