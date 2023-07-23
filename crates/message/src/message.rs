// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

use crate::MessageError;
use i18n_icu::IcuDataProvider;
use i18n_lexer::Lexer;
use i18n_provider::{ LStringProvider, LStringProviderWrapper };
use i18n_utility::{ LanguageTagRegistry, LString };
use i18n_pattern::{ parse, Formatter, PlaceholderValue, CommandRegistry };
use std::{ rc::Rc, cell::RefCell, collections::HashMap };

pub struct Message<'a, L>
where
    L: ?Sized + LStringProvider,
{
    icu_data_provider: Rc<IcuDataProvider>,
    lexer: Lexer,
    language_tag_registry: Rc<LanguageTagRegistry>,
    lstring_provider: LStringProviderWrapper<'a, L>,
    command_registry: Rc<CommandRegistry>,
    fallback: bool,
    caching: bool,
    cache: RefCell<HashMap<Rc<String>, HashMap<String, CacheData>>>,
}

impl<'a, L> Message<'a, L>
where
    L: ?Sized + LStringProvider,
{

    /// Create a new `Message` instance, that is connected to a language string provider [`LStringProvider`]. A
    /// reference to the language tag registry [`Rc`]`<`[`LanguageTagRegistry`]`>` instance and reference to the ICU
    /// data provider [`Rc`]`<`[`IcuDataProvider`]`>` are stored within the `Message` to facilitate the parsing of
    /// language string patterns, and for formatting strings.
    /// 
    /// Two boolean flags `fallback` and `caching` are also set to be the defaults of the `Message` instance. These
    /// flags govern whether parsed strings are cached for reuse, and if no string is found for the specified language
    /// whether the `format()` method should fallback to the default language tag of the string identifier.
    ///  
    /// # Examples
    /// 
    /// ```
    /// use i18n_icu::{ IcuDataProvider, DataProvider };
    /// use i18n_utility::LanguageTagRegistry;
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_pattern::{ PlaceholderValue, CommandRegistry };
    /// use i18n_message::Message;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn message() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = ProviderSqlite3::try_new(
    ///         "./i18n/", &language_tag_registry
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Message::try_new(
    ///         &icu_data_provider, &language_tag_registry, &lstring_provider, &command_registry, true, true
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "identifier".to_string(),
    ///         PlaceholderValue::String( "i18n_message/string_not_found".to_string() )
    ///     );
    ///     values.insert(
    ///         "language_tag".to_string(),
    ///         PlaceholderValue::String( "en-ZA".to_string() )
    ///     );
    ///     values.insert(
    ///         "fallback".to_string(),
    ///         PlaceholderValue::String( "true".to_string() )
    ///     );
    ///     let lstring = message_system.format(
    ///         "i18n_message/string_not_found",
    ///         &values,
    ///         &language_tag_registry.get_language_tag( "en-ZA" ).unwrap(),
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No string was found for identifier ‘i18n_message/string_not_found’ and language tag ‘en-ZA’. Fallback used: True.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn try_new(
        icu_data_provider: &Rc<IcuDataProvider>,
        language_tag_registry: &Rc<LanguageTagRegistry>,
        lstring_provider: &'a L,
        command_registry: &Rc<CommandRegistry>,
        fallback: bool,
        caching: bool,
    ) -> Result<Self, MessageError> {
        Ok( Message {
            icu_data_provider: Rc::clone( icu_data_provider ),
            lexer: Lexer::new( vec![ '{', '}', '`', '#' ], icu_data_provider ),
            language_tag_registry: Rc::clone( language_tag_registry ),
            lstring_provider: LStringProviderWrapper( lstring_provider ),
            command_registry: Rc::clone( command_registry ),
            fallback,
            caching,
            cache: RefCell::new( HashMap::<Rc<String>, HashMap<String, CacheData>>::new() ),
        } )
    }

    /// For the specified string identifier, format a string for the specified language tag with the supplied values
    /// for the placeholders. Optionally specify whether to fallback to the default language tag of string identifier
    /// when there is no string pattern for the specified language. Optionally specify whether the parsed string should
    /// be cache for reuse.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_icu::{ IcuDataProvider, DataProvider };
    /// use i18n_utility::LanguageTagRegistry;
    /// use i18n_provider_sqlite3::ProviderSqlite3;
    /// use i18n_pattern::{ PlaceholderValue, CommandRegistry };
    /// use i18n_message::Message;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn message() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = ProviderSqlite3::try_new(
    ///         "./i18n/", &language_tag_registry
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Message::try_new(
    ///         &icu_data_provider, &language_tag_registry, &lstring_provider, &command_registry, true, true
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "identifier".to_string(),
    ///         PlaceholderValue::String( "i18n_message/string_not_found".to_string() )
    ///     );
    ///     values.insert(
    ///         "language_tag".to_string(),
    ///         PlaceholderValue::String( "en-ZA".to_string() )
    ///     );
    ///     values.insert(
    ///         "fallback".to_string(),
    ///         PlaceholderValue::String( "true".to_string() )
    ///     );
    ///     let lstring = message_system.format(
    ///         "i18n_message/string_not_found",
    ///         &values,
    ///         &language_tag_registry.get_language_tag( "en-ZA" ).unwrap(),
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No string was found for identifier ‘i18n_message/string_not_found’ and language tag ‘en-ZA’. Fallback used: True.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format<T: AsRef<str>>(
        &mut self,
        identifier: T,
        values: &HashMap<String, PlaceholderValue>,
        language_tag: &Rc<String>,
        mut fallback: Option<bool>, // true = fallback to default language, None = use the Message default.
        mut caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<LString, MessageError> {
        let mut _language_entry = false;
        {
            let binding = self.cache.borrow();
            if let Some( result ) = binding.get( language_tag ) {
                _language_entry = true;
                if let Some( result2 ) = result.get( identifier.as_ref() ) {
                    return match result2 {
                        CacheData::LSring( lstring) => Ok( lstring.clone() ),
                        CacheData::Formatter( formatter ) =>
                            Ok( formatter.borrow_mut().format( values )? )
                    }
                }
            }
        }

        // Not in cache.
        // Get pattern string for specified language, though returned `LString` may be for another language.
        let lstring = match self.lstring_provider.0.get_one(
            identifier.as_ref().to_string(), language_tag
        )? {
            Some( result ) => result,
            None => {
                if fallback.is_none() {
                    fallback = Some( self.fallback );
                }
                if !fallback.unwrap() {
                    return Err( MessageError::StringNotFound(
                        identifier.as_ref().to_string(), language_tag.as_str().to_owned(), false
                    ) );
                }
                let default_language = match self.lstring_provider.0.default_language_tag(
                    identifier.as_ref().to_string()
                )? {
                    None => return Err( MessageError::NoDefaultLanguageTag( identifier.as_ref().to_string() ) ),
                    Some( result ) => self.language_tag_registry.get_language_tag( result )?
                };
                match self.lstring_provider.0.get_one(
                    identifier.as_ref().to_string(), &default_language
                )? {
                    Some( result ) => result,
                    None => return Err( MessageError::StringNotFound(
                        identifier.as_ref().to_string(), language_tag.as_str().to_owned(), true
                    ) )
                }
            }
        };

        // Tokenise the pattern string.
        // If pattern string has no grammar syntax characters, simply cache (if allowed) and return the string.
        if caching.is_none() {
            caching = Some( self.caching );
        }
        let ( tokens, _lengths, grammar_found ) = self.lexer.tokenise(
            lstring.as_str() );
        if !grammar_found {
            if caching.unwrap() {
                if !_language_entry {
                    let mut data_entry = HashMap::<String, CacheData>::new();
                    data_entry.insert(
                        identifier.as_ref().to_string(),
                        CacheData::LSring( lstring.clone() )
                    );
                    self.cache.borrow_mut().insert(
                        Rc::clone( language_tag ),
                        data_entry
                    );
                } else {
                    let mut binding = self.cache.borrow_mut();
                    let data_entry = binding.get_mut( language_tag );
                    data_entry.unwrap().insert(
                        identifier.as_ref().to_string(),
                        CacheData::LSring( lstring.clone() )
                    );
                }
            }
            return Ok( lstring );
        }

        // Has grammar syntax characters.
        // Parse tokens and create `Formatter`
        let tree = parse( tokens )?;
        let mut formatter = Formatter::try_new(
            &self.icu_data_provider,
            language_tag,
            &self.language_tag_registry.get_locale( language_tag.as_str() )?,
            &tree,
            &self.command_registry,
        )?;

        // If caching is not allowed, simple use `Formatter` to get the LString.
        if !caching.unwrap() {
            return Ok( formatter.format( values )? );
        }

        // Cache the `Formatter`.
        {
            if !_language_entry {
                let mut data_entry = HashMap::<String, CacheData/*<'a, I>*/>::new();
                data_entry.insert(
                    identifier.as_ref().to_string(),
                    CacheData::Formatter( RefCell::new( formatter ) )
                );
                self.cache.borrow_mut().insert(
                    Rc::clone( language_tag ),
                    data_entry
                );
            } else {
                let mut binding = self.cache.borrow_mut();
                let data_entry = binding.get_mut( language_tag );
                data_entry.unwrap().insert(
                    identifier.as_ref().to_string(),
                    CacheData::Formatter( RefCell::new( formatter ) )
                );
            }
        }

        // Get `Formatter` and use it to get the LString.
        let binding = self.cache.borrow();
        let result = binding.get( language_tag ).unwrap();
        let result2 = result.get( identifier.as_ref() ).unwrap();
        match result2 {
            CacheData::LSring( lstring) => Ok( lstring.clone() ),
            CacheData::Formatter( formatter ) =>
                Ok( formatter.borrow_mut().format( values )? )
        }
    }

}

// Internal structs, enums, etc

enum CacheData {
    LSring( LString ),
    Formatter( RefCell<Formatter> ),
}
