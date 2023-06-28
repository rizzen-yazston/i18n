// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use std::rc::Rc;

/// Language string.
/// 
/// This crate contains the `LString` type (aka LanguageString), for associating a text string ([`String`]) to a
/// specific language ([`Rc`]`<String>`).
/// 
/// The specified language is expected to be a [BCP 47 Language Tag] string, though any identifier could be used.
/// 
/// # Examples
/// 
/// ```
/// use icu_locid::Locale;
/// use std::rc::Rc;
/// use i18n_utility::LString;
/// 
/// let string = "This is a test string.";
/// let tag = Rc::new(
///     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
/// );
/// let lang_string = LString::new( string, &tag );
/// assert_eq!( lang_string.as_str(), string, "String failed." );
/// assert_eq!( lang_string.language_tag(), &tag, "Language tag failed." );
/// ```
/// 
/// [`String`]: alloc::string::String
/// [`Rc`]: std::rc::Rc
/// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
#[derive( PartialEq, Debug, Clone )]
pub struct LString {
    string: String,
    language_tag: Rc<String>,
}

impl LString {
    /// Creates a `LString` from string slice [`str`] and a language tag as `&`[`Rc`]`<`[`String`]`>`.
    /// 
    /// *WARNING:* No checks are done on the supplied language tag to see if it conforms to the [BCP 47 Language Tag]
    /// specification in terms of:
    /// 
    /// * _well-formed_ - syntactically correct,
    /// 
    /// * _valid_ - well-formed and only uses registered language subtags, extensions, keywords, types…,
    /// 
    /// * _canonical_ - valid and no deprecated codes or structure.
    /// 
    /// If required the [`Locale`]`::canonicalize()` function of [`icu_locid`] crate can be used to perform the
    /// conformance checks.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LString;
    /// 
    /// let string = "This is a test string.";
    /// let tag = Rc::new(
    ///     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
    /// );
    /// let lang_string = LString::new( string, &tag );
    /// assert_eq!( lang_string.as_str(), string, "String failed." );
    /// assert_eq!( lang_string.language_tag(), &tag, "Language tag failed." );
    /// ```
    /// 
    /// [`str`]: core::str
    /// [`String`]: alloc::string::String
    /// [`Rc`]: std::rc::Rc
    /// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
    /// [`Locale`]: icu_locid::Locale
    /// [`icu_locid`]: icu_locid
    pub fn new<T: Into<String>>( string: T, language_tag: &Rc<String> ) -> Self {
        LString { string: string.into(), language_tag: Rc::clone( language_tag ) }
    }

    /// Returns a reference (`&`[`str`]) to the internal [`String`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LString;
    /// 
    /// let string = "This is a test string.";
    /// let tag = Rc::new(
    ///     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
    /// );
    /// let lang_string = LString::new( string, &tag );
    /// assert_eq!( lang_string.as_str(), string, "String failed." );
    /// ```
    /// [`str`]: core::str
    /// [`String`]: alloc::string::String
    pub fn as_str( &self ) -> &str {
        &self.string
    }

    /// Returns a reference to the language tag [`Rc`]`<`[`String`]`>`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::LString;
    /// 
    /// let tag = Rc::new(
    ///     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
    /// );
    /// let lang_string = LString::new( "This is a test string.", &tag );
    /// assert_eq!( lang_string.language_tag(), &tag, "Locale failed." );
    /// ```
    /// [`Rc`]: std::rc::Rc
    /// [`String`]: alloc::string::String
    pub fn language_tag( &self ) -> &Rc<String> {
        &self.language_tag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_with_language_tag() {
        let string = "This is a test string.";
        let tag = Rc::new( "en-ZA".to_string() );
        let lang_string = LString::new( string, &tag );
        assert_eq!( lang_string.language_tag(), &tag, "Language tag failed." );
        assert_eq!( lang_string.as_str(), string, "String failed." );
    }
}
