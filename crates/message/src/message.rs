// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

use crate::MessageError;
use i18n_icu::IcuDataProvider;
use i18n_lexer::Lexer;
use i18n_provider::LanguageStringProvider;
use i18n_utility::{ LanguageTagRegistry, TaggedString };
use i18n_pattern::{ parse, Formatter, PlaceholderValue, CommandRegistry };

#[cfg( not( feature = "sync" ) )]
use std::cell::RefCell as MutCell;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::{ Arc as RefCount, Mutex as MutCell};

use std::collections::HashMap;

#[cfg( doc )]
use std::sync::Arc;

#[cfg( doc )]
use std::rc::Rc;

pub struct Message<L>
where
    L: LanguageStringProvider,
{
    icu_data_provider: RefCount<IcuDataProvider>,
    lexer: Lexer,
    language_tag_registry: RefCount<LanguageTagRegistry>,
    lstring_provider: L,
    command_registry: RefCount<CommandRegistry>,
    fallback: MutCell<bool>,
    caching: MutCell<bool>,
    cache: MutCell<HashMap<RefCount<String>, HashMap<String, CacheData>>>,
    language_tag: MutCell<RefCount<String>>,
}

impl<L> Message<L>
where
    L: LanguageStringProvider,
{

    /// Create a new `Message` instance, that is connected to a language string provider [`LanguageStringProvider`]. A
    /// reference to the language tag registry [`Rc`]`<`[`LanguageTagRegistry`]`>` or [`Arc`]`<LanguageTagRegistry>`
    /// instance and reference to the ICU data provider `Rc<`[`IcuDataProvider`]`>` or `Arc<IcuDataProvider>`
    /// are stored within the `Message` to facilitate the parsing of language string patterns, and for formatting
    /// strings.
    /// 
    /// Two boolean flags `fallback` and `caching` are also set to be the defaults of the `Message` instance. These
    /// flags govern whether parsed strings are cached for reuse, and if no string is found for the specified language
    /// whether the `format()` method should fallback to the default language tag of the string identifier.
    /// 
    /// The `language_tag` parameter is for the default language for this `Message` instance, allows for simpler
    /// formatting function `format_with_defaults()`.
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
    ///         "./l10n/", &language_tag_registry
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Message::try_new(
    ///         &icu_data_provider, &language_tag_registry, lstring_provider, &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "component".to_string(),
    ///         PlaceholderValue::String( "i18n_message".to_string() )
    ///     );
    ///     values.insert(
    ///         "identifier".to_string(),
    ///         PlaceholderValue::String( "string_not_found".to_string() )
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
    ///         "i18n_message",
    ///         "string_not_found",
    ///         &values,
    ///         &language_tag_registry.get_tag( "en-ZA" ).unwrap(),
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No string was found for the component ‘i18n_message’ with identifier ‘string_not_found’ and the \
    ///             language tag ‘en-ZA’. Fallback was used: True.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn try_new<T: AsRef<str>>(
        icu_data_provider: &RefCount<IcuDataProvider>,
        language_tag_registry: &RefCount<LanguageTagRegistry>,
        lstring_provider: L,
        command_registry: &RefCount<CommandRegistry>,
        fallback: bool,
        caching: bool,
        language_tag: T
    ) -> Result<Message<L>, MessageError> {
        let tag = language_tag_registry.get_tag( language_tag )?;
        Ok( Message {
            icu_data_provider: RefCount::clone( icu_data_provider ),
            lexer: Lexer::new( vec![ '{', '}', '`', '#' ], icu_data_provider ),
            language_tag_registry: RefCount::clone( language_tag_registry ),
            lstring_provider,
            command_registry: RefCount::clone( command_registry ),
            fallback: MutCell::new( fallback ),
            caching: MutCell::new( caching ),
            cache: MutCell::new( HashMap::<RefCount<String>, HashMap<String, CacheData>>::new() ),
            language_tag: MutCell::new( tag ),
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
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = ProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Message::try_new(
    ///         &icu_data_provider, &language_tag_registry, lstring_provider, &command_registry, true, true, "en-ZA"
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "component".to_string(),
    ///         PlaceholderValue::String( "i18n_message".to_string() )
    ///     );
    ///     values.insert(
    ///         "identifier".to_string(),
    ///         PlaceholderValue::String( "string_not_found".to_string() )
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
    ///         "i18n_message",
    ///         "string_not_found",
    ///         &values,
    ///         "en-ZA",
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No string was found for the component ‘i18n_message’ with identifier ‘string_not_found’ for the \
    ///             language tag ‘en-ZA’. Fallback was used: True.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format<T: AsRef<str>>(
        &mut self,
        component: T,
        identifier: T,
        values: &HashMap<String, PlaceholderValue>,
        language_tag: T,
        fallback: Option<bool>, // true = fallback to default language, None = use the Message default.
        caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, MessageError> {
        let tag = self.language_tag_registry.get_tag( language_tag )?;

        #[cfg( not( feature = "sync" ) )]
        let bool_fallback = fallback.unwrap_or( self.fallback.borrow().clone() );

        #[cfg( not( feature = "sync" ) )]
        let bool_caching = caching.unwrap_or( self.caching.borrow().clone() );
 
        #[cfg( feature = "sync" )]
        let bool_fallback = fallback.unwrap_or( self.fallback.lock().unwrap().clone() );

        #[cfg( feature = "sync" )]
        let bool_caching = caching.unwrap_or( self.caching.lock().unwrap().clone() );
       
        self.actual_format(
            component,
            identifier,
            values,
            &tag,
            bool_fallback,
            bool_caching,
        )
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
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = ProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Message::try_new(
    ///         &icu_data_provider, &language_tag_registry, lstring_provider, &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "component".to_string(),
    ///         PlaceholderValue::String( "i18n_message".to_string() )
    ///     );
    ///     let lstring = message_system.format_with_defaults(
    ///         "i18n_message",
    ///         "no_default_language_tag",
    ///         &values,
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No default language tag was found for the component ‘i18n_message’.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format_with_defaults<T: AsRef<str>>(
        &mut self,
        component: T,
        identifier: T,
        values: &HashMap<String, PlaceholderValue>,
    ) -> Result<TaggedString, MessageError> {

        #[cfg( not( feature = "sync" ) )]
        let tag = &RefCount::clone( &self.language_tag.borrow() );

        #[cfg( not( feature = "sync" ) )]
        let bool_fallback = self.fallback.borrow().clone();

        #[cfg( not( feature = "sync" ) )]
        let bool_caching = self.caching.borrow().clone();

        #[cfg( feature = "sync" )]
        let tag = &RefCount::clone( &self.language_tag.lock().unwrap() );
 
        #[cfg( feature = "sync" )]
        let bool_fallback = self.fallback.lock().unwrap().clone();

        #[cfg( feature = "sync" )]
        let bool_caching = self.caching.lock().unwrap().clone();

        self.actual_format(
            component,
            identifier,
            values,
            tag,
            bool_fallback,
            bool_caching,
        )
    }

    /// Simply get a language string without any formatting being done, typically strings with no placeholder patterns.
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
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = ProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Message::try_new(
    ///         &icu_data_provider, &language_tag_registry, lstring_provider, &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let lstring = message_system.get(
    ///         "i18n_message",
    ///         "no_default_language_tag",
    ///         "en-ZA",
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No default language tag was found for the component ‘{component}’.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn get<T: AsRef<str>>(
        &mut self,
        component: T,
        identifier: T,
        language_tag: T,
        fallback: Option<bool>, // true = fallback to default language, None = use the Message default.
        caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, MessageError> {
        let tag = self.language_tag_registry.get_tag( language_tag )?;

        #[cfg( not( feature = "sync" ) )]
        let bool_fallback = fallback.unwrap_or( self.fallback.borrow().clone() );

        #[cfg( not( feature = "sync" ) )]
        let bool_caching = caching.unwrap_or( self.caching.borrow().clone() );
 
        #[cfg( feature = "sync" )]
        let bool_fallback = fallback.unwrap_or( self.fallback.lock().unwrap().clone() );

        #[cfg( feature = "sync" )]
        let bool_caching = caching.unwrap_or( self.caching.lock().unwrap().clone() );

        self.actual_get(
            component,
            identifier,
            &tag,
            bool_fallback,
            bool_caching,
        )
    }

    /// Simply get a language string without any formatting being done, typically strings with no placeholder patterns
    /// using the `Message` instance defaults.
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
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = ProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Message::try_new(
    ///         &icu_data_provider, &language_tag_registry, lstring_provider, &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let lstring = message_system.get_with_defaults(
    ///         "i18n_message",
    ///         "no_default_language_tag",
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No default language tag was found for the component ‘{component}’.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn get_with_defaults<T: AsRef<str>>(
        &mut self,
        component: T,
        identifier: T,
    ) -> Result<TaggedString, MessageError> {

        #[cfg( not( feature = "sync" ) )]
        let tag = &RefCount::clone( &self.language_tag.borrow() );

        #[cfg( not( feature = "sync" ) )]
        let bool_fallback = self.fallback.borrow().clone();

        #[cfg( not( feature = "sync" ) )]
        let bool_caching = self.caching.borrow().clone();

        #[cfg( feature = "sync" )]
        let tag = &RefCount::clone( &self.language_tag.lock().unwrap() );
 
        #[cfg( feature = "sync" )]
        let bool_fallback = self.fallback.lock().unwrap().clone();

        #[cfg( feature = "sync" )]
        let bool_caching = self.caching.lock().unwrap().clone();

        self.actual_get(
            component,
            identifier,
            tag,
            bool_fallback,
            bool_caching,
        )
    }

    /// Reset the defaults of `Message` instance.
    /// 
    /// The following can be reset:
    /// 
    /// * `fallback`: bool
    /// 
    /// * `caching`: bool
    /// 
    /// * `language_tag`: &str
    pub fn defaults<T: AsRef<str>>(
        &mut self,
        fallback: bool,
        caching: bool,
        language_tag: T
    ) -> Result<(), MessageError> {
        let tag = self.language_tag_registry.get_tag( language_tag )?;

        #[cfg( not( feature = "sync" ) )]
        {
            self.language_tag.replace( tag );
            self.fallback.replace( fallback );
            self.caching.replace( caching );
        }

        #[cfg( feature = "sync" )]
        {
            *self.language_tag.get_mut().unwrap() = tag;
            *self.fallback.get_mut().unwrap() = fallback;
            *self.caching.get_mut().unwrap() = caching;
        }

        Ok( () )
    }

    // Internal methods

    // For the specified string identifier, format a string for the specified language tag with the supplied values
    // for the placeholders. Optionally specify whether to fallback to the default language tag of string identifier
    // when there is no string pattern for the specified language. Optionally specify whether the parsed string should
    // be cache for reuse.
    fn actual_format<T: AsRef<str>>(
        &mut self,
        component: T,
        identifier: T,
        values: &HashMap<String, PlaceholderValue>,
        language_tag: &RefCount<String>,
        fallback: bool, // true = fallback to default language, None = use the Message default.
        caching: bool, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, MessageError> {
        let mut combined = component.as_ref().to_string();
        combined.push( '/' );
        combined.push_str( identifier.as_ref() );
        let mut _language_entry = false;
        {
            #[cfg( not( feature = "sync" ) )]
            let binding = self.cache.borrow();

            #[cfg( feature = "sync" )]
            let binding = self.cache.lock().unwrap();

            if let Some( result ) = binding.get( language_tag ) {
                _language_entry = true;
                if let Some( result2 ) = result.get( &combined ) {
                    return match result2 {
                        CacheData::TaggedString( lstring) => Ok( lstring.clone() ),

                        #[cfg( not( feature = "sync" ) )]
                        CacheData::Formatter( formatter ) => 
                            Ok( formatter.borrow_mut().format( values )? ),

                        #[cfg( feature = "sync" )]
                        CacheData::Formatter( formatter ) => Ok( formatter.lock().unwrap().format( values )? )
                    }
                }
            }
        }

        // Not in cache.
        // Get pattern string for specified language, though returned `TaggedString` may be for another language.
        let lstring = match self.lstring_provider.get_one(
            component.as_ref().to_string(), identifier.as_ref().to_string(), language_tag
        )? {
            Some( result ) => result,
            None => {
                if !fallback {
                    return Err( MessageError::StringNotFound(
                        component.as_ref().to_string(),
                        identifier.as_ref().to_string(),
                        language_tag.as_str().to_string(),
                        false
                    ) );
                }
                let default_language = match self.lstring_provider.default_language(
                    component.as_ref()
                )? {
                    None => return Err( MessageError::NoDefaultLanguageTag( component.as_ref().to_string() ) ),
                    Some( result ) => result
                };
                match self.lstring_provider.get_one(
                    component.as_ref().to_string(),
                    identifier.as_ref().to_string(),
                    &default_language
                )? {
                    Some( result ) => result,
                    None => return Err( MessageError::StringNotFound(
                        component.as_ref().to_string(),
                        identifier.as_ref().to_string(),
                        language_tag.as_str().to_owned(),
                        true
                    ) )
                }
            }
        };

        // Tokenise the pattern string.
        // If pattern string has no grammar syntax characters, simply cache (if allowed) and return the string.
        let ( tokens, _lengths, grammar_found ) = self.lexer.tokenise(
            lstring.as_str() );
        if !grammar_found {
            if caching {
                if !_language_entry {
                    let mut data_entry = HashMap::<String, CacheData>::new();
                    data_entry.insert(
                        identifier.as_ref().to_string(),
                        CacheData::TaggedString( lstring.clone() )
                    );

                    #[cfg( not( feature = "sync" ) )]
                    self.cache.borrow_mut().insert(
                        RefCount::clone( language_tag ),
                        data_entry
                    );

                    #[cfg( feature = "sync" )]
                    self.cache.lock().unwrap().insert(
                        RefCount::clone( language_tag ),
                        data_entry
                    );
                } else {
                    #[cfg( not( feature = "sync" ) )]
                    let mut binding = self.cache.borrow_mut();

                    #[cfg( feature = "sync" )]
                    let mut binding = self.cache.lock().unwrap();

                    let data_entry = binding.get_mut( language_tag );
                    data_entry.unwrap().insert(
                        combined.clone(),
                        CacheData::TaggedString( lstring.clone() )
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

        // If caching is not allowed, simple use `Formatter` to get the TaggedString.
        if !caching {
            return Ok( formatter.format( values )? );
        }

        // Cache the `Formatter`.
        {
            if !_language_entry {
                let mut data_entry = HashMap::<String, CacheData>::new();
                data_entry.insert(
                    combined.clone(),
                    CacheData::Formatter( MutCell::new( formatter ) )
                );

                #[cfg( not( feature = "sync" ) )]
                self.cache.borrow_mut().insert(
                    RefCount::clone( language_tag ),
                    data_entry
                );

                #[cfg( feature = "sync" )]
                self.cache.lock().unwrap().insert(
                    RefCount::clone( language_tag ),
                    data_entry
                );
            } else {
                #[cfg( not( feature = "sync" ) )]
                let mut binding = self.cache.borrow_mut();

                #[cfg( feature = "sync" )]
                let mut binding = self.cache.lock().unwrap();

                let data_entry = binding.get_mut( language_tag );
                data_entry.unwrap().insert(
                    combined.clone(),
                    CacheData::Formatter( MutCell::new( formatter ) )
                );
            }
        }

        // Get `Formatter` and use it to get the TaggedString.
        #[cfg( not( feature = "sync" ) )]
        let binding = self.cache.borrow();

        #[cfg( feature = "sync" )]
        let binding = self.cache.lock().unwrap();

        let result = binding.get( language_tag ).unwrap();
        let result2 = result.get( &combined ).unwrap();
        match result2 {
            CacheData::TaggedString( lstring) => Ok( lstring.clone() ),

            #[cfg( not( feature = "sync" ) )]
            CacheData::Formatter( formatter ) => 
                Ok( formatter.borrow_mut().format( values )? ),

            #[cfg( feature = "sync" )]
            CacheData::Formatter( formatter ) => Ok( formatter.lock().unwrap().format( values )? )
        }
    }

    // Simply get the language string without any formatting being done.
    fn actual_get<T: AsRef<str>>(
        &mut self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
        fallback: bool, // true = fallback to default language, None = use the Message default.
        caching: bool, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, MessageError> {
        let mut combined = component.as_ref().to_string();
        combined.push( '/' );
        combined.push_str( identifier.as_ref() );
        let mut _language_entry = false;
        {
            #[cfg( not( feature = "sync" ) )]
            let binding = self.cache.borrow();

            #[cfg( feature = "sync" )]
            let binding = self.cache.lock().unwrap();

            if let Some( result ) = binding.get( language_tag ) {
                _language_entry = true;
                if let Some( result2 ) = result.get( &combined ) {
                    return match result2 {
                        CacheData::TaggedString( lstring) => Ok( lstring.clone() ),
                        CacheData::Formatter( _formatter ) =>
                            Err( MessageError::CacheEntry(
                                component.as_ref().to_string(),
                                identifier.as_ref().to_string(),
                            ) ),
                    }
                }
            }
        }

        // Not in cache.
        // Get pattern string for specified language, though returned `TaggedString` may be for another language.
        let lstring = match self.lstring_provider.get_one(
            component.as_ref().to_string(), identifier.as_ref().to_string(), language_tag
        )? {
            Some( result ) => result,
            None => {
                if !fallback {
                    return Err( MessageError::StringNotFound(
                        component.as_ref().to_string(),
                        identifier.as_ref().to_string(),
                        language_tag.as_str().to_string(),
                        false
                    ) );
                }
                let default_language = match self.lstring_provider.default_language(
                    component.as_ref()
                )? {
                    None => return Err( MessageError::NoDefaultLanguageTag( component.as_ref().to_string() ) ),
                    Some( result ) => result
                };
                match self.lstring_provider.get_one(
                    component.as_ref().to_string(),
                    identifier.as_ref().to_string(),
                    &default_language
                )? {
                    Some( result ) => result,
                    None => return Err( MessageError::StringNotFound(
                        component.as_ref().to_string(),
                        identifier.as_ref().to_string(),
                        language_tag.as_str().to_owned(),
                        true
                    ) )
                }
            }
        };

        // If pattern string has no grammar syntax characters, simply cache (if allowed) and return the string.
        if caching {
            if !_language_entry {
                let mut data_entry = HashMap::<String, CacheData>::new();
                data_entry.insert(
                    identifier.as_ref().to_string(),
                    CacheData::TaggedString( lstring.clone() )
                );

                #[cfg( not( feature = "sync" ) )]
                self.cache.borrow_mut().insert(
                    RefCount::clone( language_tag ),
                    data_entry
                );

                #[cfg( feature = "sync" )]
                self.cache.lock().unwrap().insert(
                    RefCount::clone( language_tag ),
                    data_entry
                );
            } else {
                #[cfg( not( feature = "sync" ) )]
                let mut binding = self.cache.borrow_mut();

                #[cfg( feature = "sync" )]
                let mut binding = self.cache.lock().unwrap();

                let data_entry = binding.get_mut( language_tag );
                data_entry.unwrap().insert(
                    combined.clone(),
                    CacheData::TaggedString( lstring.clone() )
                );
            }
        }
        return Ok( lstring );
    }
}

// Internal structs, enums, etc

enum CacheData {
    TaggedString( TaggedString ),
    Formatter( MutCell<Formatter> ),
}
