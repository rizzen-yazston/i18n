// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use crate::RegistryError;

#[cfg(not(feature = "extend"))]
use icu_locid::LanguageIdentifier as IcuLanguage;

#[cfg(feature = "extend")]
use icu_locid::Locale as IcuLanguage;

#[cfg(not(feature = "sync"))]
use std::cell::RefCell as MutCell;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::{Arc as RefCount, Mutex as MutCell};

use std::collections::HashMap;
use std::iter::FromIterator;

#[cfg(doc)]
use icu_locid::LanguageIdentifier;

#[cfg(doc)]
use std::sync::{Arc, Mutex};

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use std::cell::RefCell;

/// An immutable canonicalised [BCP 47 Language Tag] string.
///
/// This struct holds the result of the [`LanguageIdentifier::canonicalize()`] method.
///
/// If the cargo feature `icu_extended` is enabled, the [`Locale::canonicalize()`] method will
/// be used instead.
///
/// Currently the `canonicalize()` method (internally the `try_from_bytes()` method) only checks
/// that the language tag is __well-formed__. In future the method will also do the __valid__
/// and the __canonical__ checks.
///
/// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
/// [`LanguageIdentifier::canonicalize()`]: icu_locid::LanguageIdentifier::try_from_bytes()
/// [`Locale::canonicalize()`]: icu_locid::Locale::try_from_bytes()
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageTag {
    tag: String,
}

impl LanguageTag {
    /// Obtain a reference [`&str`] to the internal string.
    pub fn as_str(&self) -> &str {
        self.tag.as_str()
    }

    /// Create a [`LanguageIdentifier`] or [`Locale`] (using feature `ice_extended`) instance
    /// directly from the language tag, without requiring the [`LanguageTagRegistry`] instance.
    ///
    /// [`LanguageIdentifier`]: icu_locid::LanguageIdentifier
    /// [`Locale`]: icu_locid::Locale
    pub fn icu_language(&self) -> IcuLanguage {
        IcuLanguage::try_from_bytes(self.tag.as_bytes()).unwrap()
    }
}

/// Registry for holding the validated [BCP 47 Language Tag] strings, and optionally holding the
/// `ICU4X` [`LanguageIdentifier`] or [`Locale`] (using feature `icu_extended`) instances.
///
/// The purpose of the registry is to reduce the need of parsing language tags repeatedly, by
/// storing the validated language tag against the requested language tag.
///
/// The `LanguageIdentifier` or `Locale` type can be provided by either the [`icu_locid`] crate
/// or the `icu` meta-crate. These two crates are part of the [ICU4X] project developed by the
/// [Unicode Consortium].
///
/// # Examples
///
/// ```
/// use icu_locid::Locale;
/// use std::rc::Rc;
/// use i18n_utility::LanguageTagRegistry;
///
/// let registry = LanguageTagRegistry::new();
/// let result = registry.tag( "en_ZA" ).expect( "Failed to parse language tag." );
/// let tags = registry.list().iter().count();
///
/// assert_eq!( result.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
/// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
/// ```
///
/// [`icu_locid`]: icu_locid
/// [ICU4X]: https://github.com/unicode-org/icu4x
/// [Unicode Consortium]: https://home.unicode.org/
/// [`LanguageIdentifier`]: icu_locid::LanguageIdentifier
/// [`Locale`]: icu_locid::Locale
/// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
#[allow(clippy::type_complexity)]
pub struct LanguageTagRegistry {
    // Well-formed [BCP 47 Language Tag] strings.
    tags: MutCell<HashMap<String, RefCount<LanguageTag>>>,

    // Well-formed strings, but older ICU language identifiers with deprecated subtag(s),
    // incorrect case used for subtag(s), and/or underscore (_) used instead of hyphen (-).
    deprecated: MutCell<HashMap<String, RefCount<LanguageTag>>>,

    // The optional cache of `LanguageIdentifier` or `Locale` instances.
    icu: MutCell<HashMap<RefCount<LanguageTag>, RefCount<IcuLanguage>>>,
}

impl LanguageTagRegistry {
    pub fn new() -> Self {
        LanguageTagRegistry {
            tags: MutCell::new(HashMap::<String, RefCount<LanguageTag>>::new()),
            deprecated: MutCell::new(HashMap::<String, RefCount<LanguageTag>>::new()),

            icu: MutCell::new(HashMap::<RefCount<LanguageTag>, RefCount<IcuLanguage>>::new()),
        }
    }

    /// Obtain a referenced counted language tag [`Rc`]`<`[`LanguageTag`]`>` or
    /// [`Arc`]`<LanguageTag>` (using feature `sync`).
    ///
    /// An error will be returned if the querying tag is malformed, that is does not conform
    /// to the [BCP 47 Language Tag] specification for being _Well-formed_.
    ///
    /// However deprecated tags (containing deprecated subtag(s) and/or the deprecated
    /// underscore (_)) may still produce a valid BCP 47 Language Tag during the ICU4X locale
    /// canonicalise process of the querying tag. Thus will be stored in the registry with both
    /// the querying tag and the resultant BCP 47 Language Tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    ///
    /// let registry = LanguageTagRegistry::new();
    /// let tag = registry.tag( "en_ZA" ).expect( "Failed to parse language tag." );
    /// let tags = registry.list().iter().count();
    ///
    /// assert_eq!( tag.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    ///
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    pub fn tag(&self, language_tag: &str) -> Result<RefCount<LanguageTag>, RegistryError> {
        // Check if requested tag is already in tags.
        #[cfg(not(feature = "sync"))]
        if let Some(result) = self.tags.borrow().get(language_tag) {
            return Ok(RefCount::clone(result));
        }

        #[cfg(feature = "sync")]
        if let Some(result) = self.tags.lock().unwrap().get(language_tag) {
            return Ok(RefCount::clone(result));
        }

        // Check if requested tag is already in deprecated.
        #[cfg(not(feature = "sync"))]
        if let Some(result) = self.deprecated.borrow().get(language_tag) {
            return Ok(RefCount::clone(result));
        }

        #[cfg(feature = "sync")]
        if let Some(result) = self.deprecated.lock().unwrap().get(language_tag) {
            return Ok(RefCount::clone(result));
        }

        // Requested tag is not in registry, validate the supplied tag.
        let new_tag = IcuLanguage::canonicalize(language_tag)?;

        // Check if requested tag is just another deprecated tag of already registered tag.
        {
            #[cfg(not(feature = "sync"))]
            if let Some(result) = self.tags.borrow().get(&new_tag) {
                self.deprecated
                    .borrow_mut()
                    .insert(language_tag.to_string(), RefCount::clone(result));
                return Ok(RefCount::clone(result));
            }

            #[cfg(feature = "sync")]
            if let Some(result) = self.tags.lock().unwrap().get(&new_tag) {
                self.deprecated
                    .lock()
                    .unwrap()
                    .insert(language_tag.to_string(), RefCount::clone(result));
                return Ok(RefCount::clone(result));
            }
        }

        // Requested tag is not in tags or deprecated, add to registry and return reference counted
        // tag.
        let rc_new_tag = RefCount::new(LanguageTag { tag: new_tag });
        if language_tag != rc_new_tag.as_str() {
            #[cfg(not(feature = "sync"))]
            self.deprecated
                .borrow_mut()
                .insert(language_tag.to_string(), RefCount::clone(&rc_new_tag));

            #[cfg(feature = "sync")]
            self.deprecated
                .lock()
                .unwrap()
                .insert(language_tag.to_string(), RefCount::clone(&rc_new_tag));
        }

        #[cfg(not(feature = "sync"))]
        self.tags.borrow_mut().insert(
            rc_new_tag.as_str().to_string(),
            RefCount::clone(&rc_new_tag),
        );

        #[cfg(feature = "sync")]
        self.tags.lock().unwrap().insert(
            rc_new_tag.as_str().to_string(),
            RefCount::clone(&rc_new_tag),
        );

        Ok(RefCount::clone(&rc_new_tag))
    }

    /// Obtain a reference counted [`Rc`] or [`Arc`] (using feature `sync`) ICU4X
    /// [`LanguageIdentifier`] or [`Locale`] (using feature `icu_extended`) instances from
    /// the optional cache.
    ///
    /// If the `LanguageIdentifier` or `Locale` instance is not present in the cache, then
    /// the the method will create the instance from the validated [`LanguageTag`] and add to
    /// the cache.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    ///
    /// let registry = LanguageTagRegistry::new();
    /// let tag = registry.tag( "en_ZA" ).expect( "Failed to parse language tag." );
    /// let locale = registry.icu_language(&tag);
    ///
    /// assert_eq!(locale.to_string(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format.");
    /// ```
    ///
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    /// [`LanguageIdentifier`]: icu_locid::LanguageIdentifier
    /// [`Locale`]: icu_locid::Locale
    pub fn icu_language(&self, language_tag: &RefCount<LanguageTag>) -> RefCount<IcuLanguage> {
        #[cfg(not(feature = "sync"))]
        if let Some(result) = self.icu.borrow().get(language_tag) {
            return RefCount::clone(result);
        }

        #[cfg(feature = "sync")]
        if let Some(result) = self.icu.lock().unwrap().get(language_tag) {
            return RefCount::clone(result);
        }

        let icu_language = RefCount::new(language_tag.icu_language());

        #[cfg(not(feature = "sync"))]
        self.icu.borrow_mut().insert(
            RefCount::clone(language_tag),
            RefCount::clone(&icu_language),
        );

        #[cfg(feature = "sync")]
        self.icu.lock().unwrap().insert(
            RefCount::clone(language_tag),
            RefCount::clone(&icu_language),
        );

        icu_language
    }

    /// Returns a vector list of all the registered language tags of the [BCP 47 Language Tag]
    /// specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    ///
    /// let registry = LanguageTagRegistry::new();
    ///
    /// // Just adding deprecated tag.
    /// let result = registry.tag( "en_ZA" ).expect( "Failed to parse language tag." );
    ///
    /// let tags = registry.list().iter().count();
    ///
    /// assert_eq!( result.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( tags, 1, "Supposed to be 1 entries: en-ZA." )
    /// ```
    ///
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    pub fn list(&self) -> Vec<RefCount<LanguageTag>> {
        #[cfg(not(feature = "sync"))]
        return Vec::from_iter(self.tags.borrow().values().map(RefCount::clone));

        #[cfg(feature = "sync")]
        Vec::from_iter(
            self.tags
                .lock()
                .unwrap()
                .values()
                .map(RefCount::clone),
        )
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
    ///
    /// // Just adding deprecated tag.
    /// let result = registry.tag( "en_ZA" ).expect( "Failed to parse language tag." );
    ///
    /// let deprecated_tags = registry.list_deprecated().iter().count();
    ///
    /// assert_eq!( result.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( deprecated_tags, 1, "Supposed to be 2 entries: en_ZA." )
    /// ```
    pub fn list_deprecated(&self) -> Vec<RefCount<LanguageTag>> {
        #[cfg(not(feature = "sync"))]
        return Vec::from_iter(
            self.deprecated
                .borrow()
                .values()
                .map(RefCount::clone),
        );

        #[cfg(feature = "sync")]
        Vec::from_iter(
            self.deprecated
                .lock()
                .unwrap()
                .values()
                .map(RefCount::clone),
        )
    }

    /// Returns a vector list of all the registered language tags.
    ///
    /// Note: Each language tag included in the list can either be conforming
    /// [BCP 47 Language Tag] specification or deprecated specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LanguageTagRegistry;
    ///
    /// let registry = LanguageTagRegistry::new();
    ///
    /// // Just adding deprecated tag.
    /// let result = registry.tag( "en_ZA" ).expect( "Failed to parse language tag." );
    ///
    ///  let all_tags = registry.list_all().iter().count();
    ///
    /// assert_eq!( result.as_str(), "en-ZA", "Did not convert en_ZA to en-ZA BCP 47 format." );
    /// assert_eq!( all_tags, 2, "Supposed to be 2 entries: en_ZA and en-ZA." )
    /// ```
    ///
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    pub fn list_all(&self) -> Vec<RefCount<LanguageTag>> {
        #[cfg(not(feature = "sync"))]
        let mut list = Vec::from_iter(
            self.tags.borrow().values().map(RefCount::clone)
        );

        #[cfg(feature = "sync")]
        let mut list = Vec::from_iter(
            self.tags
                .lock()
                .unwrap()
                .values()
                .map(RefCount::clone),
        );

        #[cfg(not(feature = "sync"))]
        let mut deprecated = Vec::from_iter(
            self.deprecated
                .borrow()
                .values()
                .map(RefCount::clone),
        );

        #[cfg(feature = "sync")]
        let mut deprecated = Vec::from_iter(
            self.deprecated
                .lock()
                .unwrap()
                .values()
                .map(RefCount::clone),
        );

        list.append(&mut deprecated);
        list
    }
}

impl Default for LanguageTagRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn tag() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        let tag = registry.tag("en_ZA")?;
        assert_eq!(
            tag.as_str(),
            "en-ZA",
            "Did not convert en_ZA to en-ZA BCP 47 format."
        );
        Ok(())
    }

    #[test]
    fn icu_language() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        let tag = registry.tag("en_ZA")?;
        let locale = registry.icu_language(&tag);
        assert_eq!(
            locale.to_string(),
            "en-ZA",
            "Did not convert en_ZA to en-ZA BCP 47 format."
        );
        Ok(())
    }

    #[test]
    fn list() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        registry.tag("en_ZA")?;
        let pcb47 = registry.list().iter().count();
        assert_eq!(pcb47, 1, "Supposed to be 1 entries: en-ZA.");
        Ok(())
    }

    #[test]
    fn list_all() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        registry.tag("en_ZA")?;
        let all = registry.list_all().iter().count();
        assert_eq!(all, 2, "Supposed to be 2 entries: en_ZA and en-ZA.");
        Ok(())
    }

    #[test]
    fn list_deprecated() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        registry.tag("en_ZA")?;
        let deprecated = registry.list_deprecated().iter().count();
        assert_eq!(deprecated, 1, "Supposed to be 1 entries: en_ZA.");
        Ok(())
    }

    #[test]
    fn invalid_tag() -> Result<(), Box<dyn Error>> {
        let registry = LanguageTagRegistry::new();
        match registry.tag("hnfg_lku") {
            Ok(_) => panic!("Must fail as tag is invalid."),
            Err(_) => Ok(()),
        }
    }
}
