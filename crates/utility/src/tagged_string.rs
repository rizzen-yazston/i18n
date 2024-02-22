// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use crate::language::LanguageTag;
use std::fmt;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

/// Tagged string.
///
/// The immutable `TaggedString` type simply associates an identifier tag ([`Rc`]`<`[`LanguageTag`]`>` or
/// [`Arc`]`<LanguageTag>`) to a text string (String).
///
/// In the context of the `i18n` project, the identifier tag is expected to be a [BCP 47 Language Tag] string, even
/// though any identifier could be used.
///
/// # Examples
///
/// ```
/// use i18n_utility::{TaggedString, LanguageTagRegistry};
///
/// let registry = LanguageTagRegistry::new();
/// let string = "This is a test string.";
/// let tag = registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
/// let tagged_string = TaggedString::new(string, &tag);
/// assert_eq!(tagged_string.as_str(), string, "String failed.");
/// assert_eq!(tagged_string.tag(), &tag, "Language tag failed.");
/// ```
///
/// [`Rc`]: std::rc::Rc
/// [`Arc`]: std::sync::Arc
/// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
#[derive(PartialEq, Debug, Clone)]
pub struct TaggedString {
    string: String,
    tag: RefCount<LanguageTag>,
}

impl TaggedString {
    /// Creates a `TaggedString` from string slice [`str`] and an identifier tag as `&`[`Rc`]`<`[`LanguageTag`]`>` or
    /// `&`[`Arc`]`<LanguageTag>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_utility::{TaggedString, LanguageTagRegistry};
    ///
    /// let registry = LanguageTagRegistry::new();
    /// let string = "This is a test string.";
    /// let tag = registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
    /// let tagged_string = TaggedString::new( string, &tag );
    /// assert_eq!(tagged_string.as_str(), string, "String failed.");
    /// assert_eq!(tagged_string.tag(), &tag, "Language tag failed.");
    /// ```
    ///
    /// [`str`]: core::str
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    pub fn new<T: Into<String>>(string: T, tag: &RefCount<LanguageTag>) -> Self {
        TaggedString {
            string: string.into(),
            tag: RefCount::clone(tag),
        }
    }

    /// Returns a reference (`&`[`str`]) to the internal [`String`].
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_utility::{TaggedString, LanguageTagRegistry};
    ///
    /// let registry = LanguageTagRegistry::new();
    /// let string = "This is a test string.";
    /// let tag = registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
    /// let tagged_string = TaggedString::new( string, &tag );
    /// assert_eq!( tagged_string.as_str(), string, "String failed." );
    /// ```
    /// [`str`]: core::str
    pub fn as_str(&self) -> &str {
        &self.string
    }

    /// Returns a reference to the tag [`Rc`]`<`[`LanguageTag`]`>` or [`Arc`]`<LanguageTag>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_utility::{TaggedString, LanguageTagRegistry};
    ///
    /// let registry = LanguageTagRegistry::new();
    /// let tag = registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
    /// let tagged_string = TaggedString::new( "This is a test string.", &tag );
    /// assert_eq!( tagged_string.tag(), &tag, "Locale failed." );
    /// ```
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    pub fn tag(&self) -> &RefCount<LanguageTag> {
        &self.tag
    }
}

impl fmt::Display for TaggedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

#[cfg(test)]
mod tests {
    use crate::LanguageTagRegistry;

    use super::*;

    #[test]
    fn string_with_tag() {
        let registry = LanguageTagRegistry::new();
        let string = "This is a test string.";
        let tag = registry.tag("en-ZA").unwrap();
        let tagged_string = TaggedString::new(string, &tag);
        assert_eq!(tagged_string.tag(), &tag, "Language tag failed.");
        assert_eq!(tagged_string.as_str(), string, "String failed.");
    }
}
