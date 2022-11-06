// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

//! Module for locale related components:
//! 
//! - Registry for holding [`ICU4X`] [`Locale`] types.
//! 
//! # Registry for holding [`ICU4X`] [`Locale`] types.
//! 
//! This module contains the `LocaleRegistry` type, to provide a simple container that contains [`Locale`] types for
//! provided language tags. The purpose of the registry is to reduce the need of parsing language tags repeatedly, by
//! storing the result [`Locale`] for provided language tag in the registry, and uses the existing [`Locale`] for the 
//! language tag when requested.
//! 
//! The [`Locale`] type can be provided by either the [`icu_locid`] crate or the [`icu`] meta-crate. These two crates
//! are part of the [`ICU4X`] protect developed by the [Unicode Consortium].
//! 
//! This crate makes use of the [`Locale`] type instead of the [`LanguageIdentifier`] type due to that the [`Locale`]
//! type supports the entire [BCP 47 Language Tag] specification, where as the [`LanguageIdentifier`] type excludes the
//! **extension** subtags of the BCP 47 Language Tag specification.
//! 
//! ## Examples
//! 
//! ```
//! use icu_locid::Locale;
//! use std::rc::Rc;
//! use i18n_utility::locale::LocaleRegistry;
//! 
//! let mut registry = LocaleRegistry::new();
//! let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
//! let tags = registry.list().iter().count();
//! 
//! assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
//! assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
//! ```
//! 
//! [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
//! [`icu_locid`]: https://crates.io/crates/icu_locid
//! [`icu`]: https://crates.io/crates/icu
//! [`ICU4X`]: https://github.com/unicode-org/icu4x
//! [Unicode Consortium]: https://home.unicode.org/
//! [`LanguageIdentifier`]: https://docs.rs/icu/latest/icu/locid/struct.LanguageIdentifier.html
//! [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt

use icu_locid::Locale;
use std::rc::Rc;
use std::collections::HashMap;
use std::iter::FromIterator;

/// Registry for holding [`ICU4X`] [`Locale`] types.
/// 
/// This module contains the `LocaleRegistry` type, to provide a simple container that contains [`Locale`] types for
/// provided language tags. The purpose of the registry is to reduce the need of parsing language tags repeatedly, by
/// storing the result [`Locale`] for provided language tag in the registry, and uses the existing [`Locale`] for the 
/// language tag when requested.
/// 
/// The [`Locale`] type can be provided by either the [`icu_locid`] crate or the [`icu`] meta-crate. These two crates
/// are part of the [`ICU4X`] protect developed by the [Unicode Consortium].
/// 
/// This crate makes use of the [`Locale`] type instead of the [`LanguageIdentifier`] type due to that the [`Locale`]
/// type supports the entire [BCP 47 Language Tag] specification, where as the [`LanguageIdentifier`] type excludes the
/// **extension** subtags of the BCP 47 Language Tag specification.
/// 
/// # Examples
/// 
/// ```
/// use icu_locid::Locale;
/// use std::rc::Rc;
/// use i18n_utility::locale::LocaleRegistry;
/// 
/// let mut registry = LocaleRegistry::new();
/// let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
/// let tags = registry.list().iter().count();
/// 
/// assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
/// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
/// ```
/// 
/// [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
/// [`icu_locid`]: https://crates.io/crates/icu_locid
/// [`icu`]: https://crates.io/crates/icu
/// [`ICU4X`]: https://github.com/unicode-org/icu4x
/// [Unicode Consortium]: https://home.unicode.org/
/// [`LanguageIdentifier`]: https://docs.rs/icu/latest/icu/locid/struct.LanguageIdentifier.html
/// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
pub struct LocaleRegistry {
    bcp47: HashMap<String, Rc<Locale>>,
    icu: HashMap<String, Rc<Locale>>,
}

impl LocaleRegistry {
    /// Creates an empty registry.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::locale::LocaleRegistry;
    /// 
    /// let mut registry = LocaleRegistry::new();
    /// let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
    /// let tags = registry.list().iter().count();
    /// 
    /// assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    pub fn new() -> Self {
        LocaleRegistry { bcp47: HashMap::<String, Rc<Locale>>::new(), icu: HashMap::<String, Rc<Locale>>::new() }
    }

    /// Obtain a [`Locale`] reference for the specified language tag. The language tag may use either the
    /// [BCP 47 Language Tag] specification or the [ICU Locale] specification, though the resultant [`Locale`] will
    /// always formats the string according to the BCP 47 Language Tag specification.
    /// 
    /// An error is returned if the language tag is malformed.
    /// 
    /// Note: If the language tag is of the ICU Locale specification (that is containing underscores (_)), then both the
    /// ICU Locale] specification language tag and the resultant BCP 47 Language Tag will be added to the registry using
    /// the resultant [`Locale`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::locale::LocaleRegistry;
    /// 
    /// let mut registry = LocaleRegistry::new();
    /// let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
    /// let tags = registry.list().iter().count();
    /// 
    /// assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    /// 
    /// [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    /// [ICU Locale]: https://unicode-org.github.io/icu/userguide/locale/
    pub fn get( &mut self, language_tag: String ) -> Result<Rc<Locale>, String> {
        if let Some( locale ) = self.bcp47.get( &language_tag ) {
            return Ok( Rc::clone( locale ) );
        }
        if let Some( locale ) = self.icu.get( &language_tag ) {
            return Ok( Rc::clone( locale ) );
        }
        match Locale::try_from_bytes( language_tag.as_bytes() ) {
            Err( _error ) => return Err( "Invalid language tag.".to_string() ),
            Ok( locale_new ) => {
                let tag = locale_new.to_string();
                let mut locale: Option<Rc<Locale>> = None;
                {
                    if let Some( _locale ) = self.bcp47.get( &tag ) {
                        locale = Some( Rc::clone( _locale ) );
                    }
                }
                {
                    if !locale.is_none() {
                        let locale2 = locale.unwrap();
                        self.icu.insert( language_tag, Rc::clone( &locale2 ) );
                        return Ok( Rc::clone( &locale2 ) );
                    }
                }
                let rc_locale_new = Rc::new( locale_new );
                if language_tag != tag {
                    self.icu.insert( language_tag, Rc::clone( &rc_locale_new ) );
                }
                self.bcp47.insert( tag, Rc::clone( &rc_locale_new ) );
                return Ok( Rc::clone( &rc_locale_new ) );
            }
        }
    }

    /// Returns a vector list of all the registered language tags of the [BCP 47 Language Tag] specification.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::locale::LocaleRegistry;
    /// 
    /// let mut registry = LocaleRegistry::new();
    /// let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
    /// let tags = registry.list().iter().count();
    /// 
    /// assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    /// 
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    /// [ICU Locale]: https://unicode-org.github.io/icu/userguide/locale/
    pub fn list( &self ) -> Vec<&String> {
        Vec::from_iter( self.bcp47.keys() )
    }

    /// Returns a vector list of all the registered language tags of the [ICU Locale] specification.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::locale::LocaleRegistry;
    /// 
    /// let mut registry = LocaleRegistry::new();
    /// let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
    /// let icu_tags = registry.list_icu().iter().count();
    /// 
    /// assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( icu_tags, 1, "Supposed to be 2 entries: en_ZA." )
    /// ```
    /// 
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    /// [ICU Locale]: https://unicode-org.github.io/icu/userguide/locale/
    pub fn list_icu( &self ) -> Vec<&String> {
        Vec::from_iter( self.icu.keys() )
    }

    /// Returns a vector list of all the registered language tags.
    /// 
    /// Note: Each language tag included in the list can be either the [BCP 47 Language Tag] specification or the
    /// [ICU Locale] specification.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::locale::LocaleRegistry;
    /// 
    /// let mut registry = LocaleRegistry::new();
    /// let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
    /// let all_tags = registry.list_all().iter().count();
    /// 
    /// assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( all_tags, 2, "Supposed to be 2 entries: en_ZA and en-ZA." )
    /// ```
    /// 
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    /// [ICU Locale]: https://unicode-org.github.io/icu/userguide/locale/
    pub fn list_all( &self ) -> Vec<&String> {
        let mut list = Vec::from_iter( self.bcp47.keys() );
        let mut icu = Vec::from_iter( self.icu.keys() );
        list.append( &mut icu );
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut registry = LocaleRegistry::new();
        let locale = registry.get( "en_ZA".to_string() ).expect( "Failed to parse language tag." );
        let pcb47 = registry.list().iter().count();
        let icu = registry.list_icu().iter().count();
        let all = registry.list_all().iter().count();
    
        assert_eq!( locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
        assert_eq!( pcb47, 1, "Supposed to be 1 entries: en-ZA." );
        assert_eq!( icu, 1, "Supposed to be 1 entries: en_ZA." );
        assert_eq!( all, 2, "Supposed to be 2 entries: en_ZA and en-ZA." );
    }
}
