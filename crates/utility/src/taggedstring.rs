// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

/// Tagged string.
/// 
/// The immutable `TaggedString` type simply associates an identifier tag ([`Rc`]`<String>` or [`Arc`]`<String>`) to a
/// text string ([`String`]).
/// 
/// In the context of the `i18n` project, the identifier tag is expected to be a [BCP 47 Language Tag] string, even
/// though any identifier could be used.
/// 
/// # Examples
/// 
/// ```
/// use icu_locid::Locale;
/// use std::rc::Rc;
/// use i18n_utility::TaggedString;
/// 
/// let string = "This is a test string.";
/// let tag = Rc::new(
///     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
/// );
/// let lang_string = TaggedString::new( string, &tag );
/// assert_eq!( lang_string.as_str(), string, "String failed." );
/// assert_eq!( lang_string.tag(), &tag, "Language tag failed." );
/// ```
/// 
/// [`Rc`]: std::rc::Rc
/// [`Arc`]: std::sync::Arc
/// [BCP 47 Language Tag]: https://www.rfc-editor.org/rfc/bcp/bcp47.txt
#[derive( PartialEq, Debug, Clone )]
pub struct TaggedString {
    string: String,
    tag: RefCount<String>,
}

impl TaggedString {
    /// Creates a `TaggedString` from string slice [`str`] and an identifier tag as `&`[`Rc`]`<`[`String`]`>` or
    /// `&`[`Arc`]`<String>`.
    /// 
   /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::TaggedString;
    /// 
    /// let string = "This is a test string.";
    /// let tag = Rc::new(
    ///     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
    /// );
    /// let lang_string = TaggedString::new( string, &tag );
    /// assert_eq!( lang_string.as_str(), string, "String failed." );
    /// assert_eq!( lang_string.tag(), &tag, "Language tag failed." );
    /// ```
    /// 
    /// [`str`]: core::str
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    pub fn new<T: Into<String>>(
        string: T,
        tag: &RefCount<String>,
    ) -> Self {
        TaggedString {
            string: string.into(),
            tag: RefCount::clone( tag ),
        }
    }

    /// Returns a reference (`&`[`str`]) to the internal [`String`].
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::TaggedString;
    /// 
    /// let string = "This is a test string.";
    /// let tag = Rc::new(
    ///     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
    /// );
    /// let lang_string = TaggedString::new( string, &tag );
    /// assert_eq!( lang_string.as_str(), string, "String failed." );
    /// ```
    /// [`str`]: core::str
    pub fn as_str( &self ) -> &str {
        &self.string
    }

    /// Returns a reference to the tag [`Rc`]`<`[`String`]`>` or [`Arc`]`<String>`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use icu_locid::Locale;
    /// use std::rc::Rc;
    /// use i18n_utility::TaggedString;
    /// 
    /// let tag = Rc::new(
    ///     Locale::canonicalize( "en-ZA" ).expect( "Failed to canonicalise language tag." )
    /// );
    /// let lang_string = TaggedString::new( "This is a test string.", &tag );
    /// assert_eq!( lang_string.tag(), &tag, "Locale failed." );
    /// ```
    /// [`Rc`]: std::rc::Rc
    /// [`Arc`]: std::sync::Arc
    pub fn tag( &self ) -> &RefCount<String> {
        &self.tag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_with_tag() {
        let string = "This is a test string.";
        let tag = RefCount::new( "en-ZA".to_string() );
        let lang_string = TaggedString::new( string, &tag );
        assert_eq!( lang_string.tag(), &tag, "Language tag failed." );
        assert_eq!( lang_string.as_str(), string, "String failed." );
    }
}
