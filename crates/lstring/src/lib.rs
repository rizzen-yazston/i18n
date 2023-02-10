// This file is part of `i18n_lstring-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lstring-rizzen-yazston` crate.

//! Language string.
//! 
//! This crate contains the `LString` type (aka LanguageString), for associating a text string ([`String`]) to a
//! specific language ([`Locale`]).
//! 
//! The [`Locale`] type can be provided by either the [`icu_locid`] crate or the [`icu`] meta-crate. These two crates
//! are part of the [ICU4X] protect developed by the [Unicode Consortium].
//! 
//! This crate makes use of the [`Locale`] type instead of the [`LanguageIdentifier`] type due to that the [`Locale`]
//! type supports the entire [BCP 47 Language Tag] specification, where as the [`LanguageIdentifier`] type excludes the
//! **extension** subtags of the BCP 47 Language Tag specification.
//! 
//! # Examples
//! 
//! ```
//! use icu_locid::Locale;
//! use std::rc::Rc;
//! use i18n_lstring::LString;
//! 
//! let string = "This is a test string.";
//! let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
//! let lang_string = LString::new( string, &locale );
//! 
//! assert_eq!( lang_string.as_str(), string, "String failed." );
//! assert_eq!( lang_string.locale(), &locale, "Locale failed." );
//! ```
//! 
//! [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
//! [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
//! [`icu_locid`]: https://crates.io/crates/icu_locid
//! [`icu`]: https://crates.io/crates/icu
//! [ICU4X]: https://github.com/unicode-org/icu4x
//! [Unicode Consortium]: https://home.unicode.org/
//! [`LanguageIdentifier`]: https://docs.rs/icu/latest/icu/locid/struct.LanguageIdentifier.html
//! [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt

use icu_locid::Locale;
use std::rc::Rc;

/// Language string.
/// 
/// This crate contains the `LString` type (aka LanguageString), for associating a text string ([`String`]) to a
/// specific language ([`Locale`]).
/// 
/// The [`Locale`] type can be provided by either the [`icu_locid`] crate or the [`icu`] meta-crate. These two crates
/// are part of the [ICU4X] protect developed by the [Unicode Consortium].
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
/// use i18n_lstring::LString;
/// 
/// let string = "This is a test string.";
/// let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
/// let lang_string = LString::new( string, &locale );
/// 
/// assert_eq!( lang_string.as_str(), string, "String failed." );
/// assert_eq!( lang_string.locale(), &locale, "Locale failed." );
/// ```
/// 
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
/// [`icu_locid`]: https://crates.io/crates/icu_locid
/// [`icu`]: https://crates.io/crates/icu
/// [ICU4X]: https://github.com/unicode-org/icu4x
/// [Unicode Consortium]: https://home.unicode.org/
/// [`LanguageIdentifier`]: https://docs.rs/icu/latest/icu/locid/struct.LanguageIdentifier.html
/// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
#[derive( PartialEq, Debug )]
pub struct LString {
    string: String,
    locale: Rc<Locale>,
}

impl LString {
    /// Creates a `LString` object from a reference to a string slice [`str`] and a reference counted [`Locale`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_lstring::LString;
    /// 
    /// let string = "This is a test string.";
    /// let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    /// let lang_string = LString::new( string, &locale );
    /// 
    /// assert_eq!( lang_string.as_str(), string, "String failed." );
    /// assert_eq!( lang_string.locale(), &locale, "Locale failed." );
    /// ```
    /// 
    /// [`str`]: https://doc.rust-lang.org/nightly/core/primitive.str.html
    /// [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
    pub fn new( string: &str, locale: &Rc<Locale> ) -> Self {
        LString { string: String::from( string ), locale: Rc::clone( locale ) }
    }

    /// Returns a reference to the internal [`String`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_lstring::LString;
    /// 
    /// let string = "This is a test string.";
    /// let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    /// let lang_string = LString::new( string, &locale );
    /// 
    /// assert_eq!( lang_string.as_str(), string, "String failed." );
    /// ```
    /// 
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    pub fn as_str( &self ) -> &str {
        &self.string
    }

    /// Returns a reference counted for [`Locale`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_lstring::LString;
    /// 
    /// let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    /// let lang_string = LString::new( "This is a test string.", &locale );
    /// 
    /// assert_eq!( lang_string.locale(), &locale, "Locale failed." );
    /// ```
    /// 
    /// [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
    pub fn locale( &self ) -> &Rc<Locale> {
        &self.locale
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locale_and_string() {
        let string = "This is a test string.";
        let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
        let lang_string = LString::new( string, &locale );
    
        assert_eq!( lang_string.locale(), &locale, "Locale failed." );
        assert_eq!( lang_string.as_str(), string, "String failed." );
    }
}
