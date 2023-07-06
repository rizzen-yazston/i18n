// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use crate::RegistryError;
use icu_locid::Locale;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::iter::FromIterator;

/// Registry for holding `ICU4X` `Locale` objects.
/// 
/// This module contains the `LanguageTagRegistry` type, to provide a simple container that caches the
/// [BCP 47 Language Tag] strings and the [`Locale`] types for querying language tags. The purpose of the registry is
/// to reduce the need of parsing language tags repeatedly, by storing the result `Locale` for querying language tag in
/// the registry, and uses the existing `Locale` for the querying language tag when requested.
/// 
/// The `Locale` type can be provided by either the [`icu_locid`] crate or the `icu` meta-crate. These two crates
/// are part of the [`ICU4X`] protect developed by the [Unicode Consortium].
/// 
/// This crate makes use of the `Locale` type instead of the [`LanguageIdentifier`] type due to that the `Locale`
/// type supports the entire BCP 47 Language Tag specification, where as the `LanguageIdentifier` type excludes the
/// **extension** subtags of the BCP 47 Language Tag specification.
/// 
/// # Examples
/// 
/// ```
/// use icu_locid::Locale;
/// use std::rc::Rc;
/// use i18n_utility::LanguageTagRegistry;
/// 
/// let registry = LanguageTagRegistry::new();
/// let result = registry.get( "en_ZA" ).expect( "Failed to parse language tag." );
/// let tags = registry.list().iter().count();
/// 
/// assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
/// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
/// ```
/// 
/// [`Locale`]: icu_locid::Locale
/// [`icu_locid`]: icu_locid
/// [`ICU4X`]: https://github.com/unicode-org/icu4x
/// [Unicode Consortium]: https://home.unicode.org/
/// [`LanguageIdentifier`]: icu_locid::LanguageIdentifier
/// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
pub struct LanguageTagRegistry {
    bcp47: RefCell<HashMap<String, ( Rc<String>, Rc<Locale> )>>,
    
    // Well-formed, but with deprecated subtag(s), incorrect case used for subtag(s), and/or underscore (_) used
    // instead of hyphen (-).
    deprecated: RefCell<HashMap<String, ( Rc<String>, Rc<Locale> )>>,
}

impl LanguageTagRegistry {
    /// Creates an empty registry.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    /// 
    /// let registry = LanguageTagRegistry::new();
    /// let result = registry.get( "en_ZA" ).expect( "Failed to parse language tag." );
    /// let tags = registry.list().iter().count();
    /// 
    /// assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    pub fn new() -> Self {
        LanguageTagRegistry {
            bcp47: RefCell::new( HashMap::<String, ( Rc<String>, Rc<Locale> )>::new() ),
            deprecated: RefCell::new( HashMap::<String, ( Rc<String>, Rc<Locale> )>::new() )
        }
    }

    /// Obtain a tuple pair of referenced counted language tag and ICU4X locale
    /// `( `[`Rc`]`<`[`String`]`>, Rc<`[`Locale`]`> )`.
    /// 
    /// An error will be returned if the querying tag is malformed, that is does not conform to the
    /// [BCP 47 Language Tag] specification for being _Well-formed_. 
    /// 
    /// However deprecated tags (containing deprecated subtag(s) and/or the deprecated underscore (_)) may still
    /// produce a valid BCP 47 Language Tag during the ICU4X locale canonicalise process of the querying tag. Thus will
    /// be stored in the registry with both the querying tag and the resultant BCP 47 Language Tag, for the ICU4X
    /// [`Locale`] value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    /// 
    /// let registry = LanguageTagRegistry::new();
    /// let result = registry.get( "en_ZA" ).expect( "Failed to parse language tag." );
    /// let tags = registry.list().iter().count();
    /// 
    /// assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    /// 
    /// [`Rc`]: std::rc::Rc
    /// [`Locale`]: icu_locid::Locale
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    pub fn get<T: AsRef<str>>( &self, language_tag: T ) -> Result<( Rc<String>, Rc<Locale> ), RegistryError> {
        if let Some( result ) = self.bcp47.borrow().get( language_tag.as_ref() ) {
            return Ok( ( Rc::clone( &result.0 ), Rc::clone( &result.1 ) ) );
        }
        if let Some( result ) = self.deprecated.borrow().get( language_tag.as_ref() ) {
            return Ok( ( Rc::clone( &result.0 ), Rc::clone( &result.1 ) ) );
        }
        let new_locale = Locale::try_from_bytes( language_tag.as_ref().as_bytes() )?;
        let new_tag = new_locale.to_string();
        {
            if let Some( result ) = self.bcp47.borrow().get( &new_tag ) {
                self.deprecated.borrow_mut().insert(
                    language_tag.as_ref().to_string(),
                    ( Rc::clone( &result.0 ), Rc::clone( &result.1 ) )
                );
                return Ok( ( Rc::clone( &result.0 ), Rc::clone( &result.1 ) ) );
            }
        }
        let rc_new_locale = Rc::new( new_locale );
        let rc_new_tag = Rc::new( new_tag );
        if language_tag.as_ref() != rc_new_tag.as_str() {
            self.deprecated.borrow_mut().insert(
                language_tag.as_ref().to_string(),
                ( Rc::clone( &rc_new_tag ), Rc::clone( &rc_new_locale ) )
            );
        }
        self.bcp47.borrow_mut().insert(
            rc_new_tag.as_str().to_string(),
            ( Rc::clone( &rc_new_tag ), Rc::clone( &rc_new_locale ) )
        );
        return Ok( ( Rc::clone( &rc_new_tag ), Rc::clone( &rc_new_locale ) ) );
    }

    /// Obtain a referenced counted language tag [`Rc`]`<`[`String`]`>`.
    /// 
    /// An error will be returned if the querying tag is malformed, that is does not conform to the
    /// [BCP 47 Language Tag] specification for being _Well-formed_. 
    /// 
    /// However deprecated tags (containing deprecated subtag(s) and/or the deprecated underscore (_)) may still
    /// produce a valid BCP 47 Language Tag during the ICU4X locale canonicalise process of the querying tag. Thus will
    /// be stored in the registry with both the querying tag and the resultant BCP 47 Language Tag, for the ICU4X
    /// [`Locale`] value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    /// 
    /// let registry = LanguageTagRegistry::new();
    /// let tag = registry.get_language_tag( "en_ZA" ).expect( "Failed to parse language tag." );
    /// let tags = registry.list().iter().count();
    /// 
    /// assert_eq!( tag.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    /// 
    /// [`Rc`]: std::rc::Rc
    /// [`Locale`]: icu_locid::Locale
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    pub fn get_language_tag<T: AsRef<str>>( &self, language_tag: T ) -> Result<Rc<String>, RegistryError> {
        let result = self.get( language_tag.as_ref() )?;
        Ok( result.0 )
    }

    /// Obtain a ICU4X locale [`Rc`]`<`[`Locale`]`>`.
    /// 
    /// An error will be returned if the querying tag is malformed, that is does not conform to the
    /// [BCP 47 Language Tag] specification for being _Well-formed_. 
    /// 
    /// However deprecated tags (containing deprecated subtag(s) and/or the deprecated underscore (_)) may still
    /// produce a valid BCP 47 Language Tag during the ICU4X locale canonicalise process of the querying tag. Thus will
    /// be stored in the registry with both the querying tag and the resultant BCP 47 Language Tag, for the ICU4X
    /// [`Locale`] value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    /// 
    /// let registry = LanguageTagRegistry::new();
    /// let locale = registry.get_locale( "en_ZA" ).expect( "Failed to parse language tag." );
    /// let tags = registry.list().iter().count();
    /// 
    /// assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    /// 
    /// [`Rc`]: std::rc::Rc
    /// [`Locale`]: icu_locid::Locale
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    pub fn get_locale<T: AsRef<str>>( &self, language_tag: T ) -> Result<Rc<Locale>, RegistryError> {
        let result = self.get( language_tag.as_ref() )?;
        Ok( result.1 )
    }

    /// Returns a vector list of all the registered language tags of the [BCP 47 Language Tag] specification.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    /// 
    /// let registry = LanguageTagRegistry::new();
    /// let result = registry.get( "en_ZA" ).expect( "Failed to parse language tag." ); // Just adding deprecated tag.
    /// let tags = registry.list().iter().count();
    /// 
    /// assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    /// 
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    pub fn list( &self ) -> Vec<String> {
        Vec::from_iter( self.bcp47.borrow().keys().map( |x| x.to_string() ) )
    }

    /// Returns a vector list of all the registered language tags of deprecated specification.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    /// 
    /// let registry = LanguageTagRegistry::new();
    /// let result = registry.get( "en_ZA" ).expect( "Failed to parse language tag." ); // Just adding deprecated tag.
    /// let deprecated_tags = registry.list_deprecated().iter().count();
    /// 
    /// assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( deprecated_tags, 1, "Supposed to be 2 entries: en_ZA." )
    /// ```
    pub fn list_deprecated( &self ) -> Vec<String> {
        Vec::from_iter( self.deprecated.borrow().keys().map( |x| x.to_string() ) )
    }

    /// Returns a vector list of all the registered language tags.
    /// 
    /// Note: Each language tag included in the list can either be conforming [BCP 47 Language Tag] specification or
    /// deprecated specification.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    /// 
    /// let registry = LanguageTagRegistry::new();
    /// let result = registry.get( "en_ZA" ).expect( "Failed to parse language tag." ); // Just adding deprecated tag.
    /// let all_tags = registry.list_all().iter().count();
    /// 
    /// assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( all_tags, 2, "Supposed to be 2 entries: en_ZA and en-ZA." )
    /// ```
    /// 
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    pub fn list_all( &self ) -> Vec<String> {
        let mut list = Vec::from_iter( self.bcp47.borrow().keys().map( |x| x.to_string() ) );
        let mut deprecated = Vec::from_iter(
            self.deprecated.borrow().keys().map( |x| x.to_string() )
        );
        list.append( &mut deprecated );
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn get() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        let result = registry.get( "en_ZA" )?;
        assert_eq!( result.0.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
        Ok( () )
    }

    #[test]
    fn get_language_tag() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        let tag = registry.get_language_tag( "en_ZA" )?;
        assert_eq!( tag.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
        Ok( () )
    }

    #[test]
    fn get_locale() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        let locale = registry.get_locale( "en_ZA" )?;
        assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
        Ok( () )
    }

    #[test]
    fn list() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        registry.get( "en_ZA" )?;
        let pcb47 = registry.list().iter().count();
        assert_eq!( pcb47, 1, "Supposed to be 1 entries: en-ZA." );
        Ok( () )
    }

    #[test]
    fn list_all() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        registry.get( "en_ZA" )?;
        let all = registry.list_all().iter().count();
        assert_eq!( all, 2, "Supposed to be 2 entries: en_ZA and en-ZA." );
        Ok( () )
    }

    #[test]
    fn list_deprecated() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        registry.get( "en_ZA" )?;
        let deprecated = registry.list_deprecated().iter().count();
        assert_eq!( deprecated, 1, "Supposed to be 1 entries: en_ZA." );
        Ok( () )
    }

    #[test]
    fn invalid_language_tag() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        match registry.get_language_tag( "hnfg_lku" ) {
            Ok( _ ) => panic!( "Must fail as tag is invalid." ),
            Err( _ ) => Ok( () )
        }
    }
}
