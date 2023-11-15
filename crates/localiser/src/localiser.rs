// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

use crate::LocaliserError;
use i18n_icu::IcuDataProvider;
use i18n_lexer::Lexer;
use i18n_provider::LocalisationProvider;
use i18n_utility::{ LanguageTagRegistry, TaggedString };
use i18n_pattern::{ parse, Formatter, PlaceholderValue, CommandRegistry };

#[cfg( not( feature = "sync" ) )]
use std::cell::RefCell as MutCell;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::{ Arc as RefCount, RwLock as MutCell};

use std::collections::HashMap;

#[cfg( feature = "log" )]
use log::debug;

#[cfg( doc )]
use std::sync::Arc;

#[cfg( doc )]
use std::rc::Rc;

pub struct Localiser<L>
where
    L: LocalisationProvider,
{
    icu_data_provider: RefCount<IcuDataProvider>,
    lexer: MutCell<Lexer>,
    language_registry: RefCount<LanguageTagRegistry>,
    localisation_provider: L,
    command_registry: RefCount<CommandRegistry>,
    fallback: MutCell<bool>,
    caching: MutCell<bool>,
    cache: MutCell<HashMap<RefCount<String>, HashMap<String, CacheData>>>,
    language_tag: MutCell<RefCount<String>>,
}

impl<L> Localiser<L>
where
    L: LocalisationProvider,
{

    /// Create a new `Localiser` instance, that is connected to a language string provider [`LocalisationProvider`]. A
    /// reference to the language tag registry [`Rc`]`<`[`LanguageTagRegistry`]`>` or [`Arc`]`<LanguageTagRegistry>`
    /// instance and reference to the ICU data provider `Rc<`[`IcuDataProvider`]`>` or `Arc<IcuDataProvider>`
    /// are stored within the `Localiser` to facilitate the parsing of language string patterns, and for formatting
    /// strings.
    /// 
    /// Two boolean flags `fallback` and `caching` are also set to be the defaults of the `Localiser` instance. These
    /// flags govern whether parsed strings are cached for reuse, and if no string is found for the specified language
    /// whether the `format()` method should fallback to the default language tag of the string identifier.
    /// 
    /// The `language_tag` parameter is for the default language for this `Localiser` instance, allows for simpler
    /// formatting function `format_with_defaults()`.
    ///  
    /// # Examples
    /// 
    /// ```
    /// use i18n_icu::{ IcuDataProvider, DataProvider };
    /// use i18n_utility::LanguageTagRegistry;
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_pattern::{ PlaceholderValue, CommandRegistry };
    /// use i18n_localiser::Localiser;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn message() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let localisation_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Localiser::try_new(
    ///         &icu_data_provider, &language_tag_registry, localisation_provider,
    ///         &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "component".to_string(),
    ///         PlaceholderValue::String( "i18n_localiser".to_string() )
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
    ///         "i18n_localiser",
    ///         "string_not_found",
    ///         &values,
    ///         &language_tag_registry.tag( "en-ZA" ).unwrap(),
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No string was found for the component ‘i18n_localiser’ with identifier ‘string_not_found’ and the \
    ///             language tag ‘en-ZA’. Fallback was used: True.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn try_new<T: AsRef<str>>(
        icu_data_provider: &RefCount<IcuDataProvider>,
        language_tag_registry: &RefCount<LanguageTagRegistry>,
        localisation_provider: L,
        command_registry: &RefCount<CommandRegistry>,
        fallback: bool,
        caching: bool,
        language_tag: T
    ) -> Result<Localiser<L>, LocaliserError> {
        let tag = language_tag_registry.tag( language_tag )?;
        Ok( Localiser {
            icu_data_provider: RefCount::clone( icu_data_provider ),
            lexer: MutCell::new( Lexer::new( vec![ '{', '}', '`', '#' ], icu_data_provider ) ),
            language_registry: RefCount::clone( language_tag_registry ),
            localisation_provider,
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
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_pattern::{ PlaceholderValue, CommandRegistry };
    /// use i18n_localiser::Localiser;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let localisation_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Localiser::try_new(
    ///         &icu_data_provider, &language_tag_registry, localisation_provider,
    ///         &command_registry, true, true, "en-ZA"
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "component".to_string(),
    ///         PlaceholderValue::String( "i18n_localiser".to_string() )
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
    ///         "i18n_localiser",
    ///         "string_not_found",
    ///         &values,
    ///         "en-ZA",
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No string was found for the component ‘i18n_localiser’ with identifier ‘string_not_found’ for the \
    ///             language tag ‘en-ZA’. Fallback was used: True.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        values: &HashMap<String, PlaceholderValue>,
        language_tag: T,
        fallback: Option<bool>, // true = fallback to default language, None = use the Localiser default.
        caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg( feature = "log" )]
        debug!( "Localiser is using format().");

        let tag = self.language_registry.tag( language_tag )?;

        #[cfg( not( feature = "sync" ) )]
        let bool_fallback = fallback.unwrap_or( self.fallback.borrow().clone() );

        #[cfg( not( feature = "sync" ) )]
        let bool_caching = caching.unwrap_or( self.caching.borrow().clone() );
 
        #[cfg( feature = "sync" )]
        let bool_fallback = fallback.unwrap_or( self.fallback.read().unwrap().clone() );

        #[cfg( feature = "sync" )]
        let bool_caching = caching.unwrap_or( self.caching.read().unwrap().clone() );
       
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
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_pattern::{ PlaceholderValue, CommandRegistry };
    /// use i18n_localiser::Localiser;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let localisation_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Localiser::try_new(
    ///         &icu_data_provider, &language_tag_registry, localisation_provider,
    ///         &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "component".to_string(),
    ///         PlaceholderValue::String( "i18n_localiser".to_string() )
    ///     );
    ///     let lstring = message_system.format_with_defaults(
    ///         "i18n_localiser",
    ///         "no_default_language_tag",
    ///         &values,
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No default language tag was found for the component ‘i18n_localiser’.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format_with_defaults<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        values: &HashMap<String, PlaceholderValue>,
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg( feature = "log" )]
        debug!( "Localiser is using format_with_defaults().");

        #[cfg( not( feature = "sync" ) )]
        let tag = &RefCount::clone( &self.language_tag.borrow() );

        #[cfg( not( feature = "sync" ) )]
        let bool_fallback = self.fallback.borrow().clone();

        #[cfg( not( feature = "sync" ) )]
        let bool_caching = self.caching.borrow().clone();

        #[cfg( feature = "sync" )]
        let tag = &RefCount::clone( &self.language_tag.read().unwrap() );
 
        #[cfg( feature = "sync" )]
        let bool_fallback = self.fallback.read().unwrap().clone();

        #[cfg( feature = "sync" )]
        let bool_caching = self.caching.read().unwrap().clone();

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
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_pattern::{ PlaceholderValue, CommandRegistry };
    /// use i18n_localiser::Localiser;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let localisation_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Localiser::try_new(
    ///         &icu_data_provider, &language_tag_registry, localisation_provider,
    ///         &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let lstring = message_system.literal(
    ///         "i18n_localiser",
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
    pub fn literal<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: T,
        fallback: Option<bool>, // true = fallback to default language, None = use the Localiser default.
        caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg( feature = "log" )]
        debug!( "Localiser is using literal().");

        let tag = self.language_registry.tag( language_tag )?;

        #[cfg( not( feature = "sync" ) )]
        let bool_fallback = fallback.unwrap_or( self.fallback.borrow().clone() );

        #[cfg( not( feature = "sync" ) )]
        let bool_caching = caching.unwrap_or( self.caching.borrow().clone() );
 
        #[cfg( feature = "sync" )]
        let bool_fallback = fallback.unwrap_or( self.fallback.read().unwrap().clone() );

        #[cfg( feature = "sync" )]
        let bool_caching = caching.unwrap_or( self.caching.read().unwrap().clone() );

        self.actual_literal(
            component,
            identifier,
            &tag,
            bool_fallback,
            bool_caching,
        )
    }

    /// Simply get a language string without any formatting being done, typically strings with no placeholder patterns
    /// using the `Localiser` instance defaults.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use i18n_icu::{ IcuDataProvider, DataProvider };
    /// use i18n_utility::LanguageTagRegistry;
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_pattern::{ PlaceholderValue, CommandRegistry };
    /// use i18n_localiser::Localiser;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    /// 
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let localisation_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut message_system = Localiser::try_new(
    ///         &icu_data_provider, &language_tag_registry, localisation_provider,
    ///         &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let lstring = message_system.literal_with_defaults(
    ///         "i18n_localiser",
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
    pub fn literal_with_defaults<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg( feature = "log" )]
        debug!( "Localiser is using literal_with_defaults().");

        #[cfg( not( feature = "sync" ) )]
        let tag = &RefCount::clone( &self.language_tag.borrow() );

        #[cfg( not( feature = "sync" ) )]
        let bool_fallback = self.fallback.borrow().clone();

        #[cfg( not( feature = "sync" ) )]
        let bool_caching = self.caching.borrow().clone();

        #[cfg( feature = "sync" )]
        let tag = &RefCount::clone( &self.language_tag.read().unwrap() );
 
        #[cfg( feature = "sync" )]
        let bool_fallback = self.fallback.read().unwrap().clone();

        #[cfg( feature = "sync" )]
        let bool_caching = self.caching.read().unwrap().clone();

        self.actual_literal(
            component,
            identifier,
            tag,
            bool_fallback,
            bool_caching,
        )
    }

    /// Change the defaults of `Localiser` instance.
    /// 
    /// The following can be reset:
    /// 
    /// * `language_tag`: `Option<&str>`
    /// 
    /// * `fallback`: `Option<bool>`
    /// 
    /// * `caching`: `Option<bool>`
    /// 
    /// A value of `None` indicates no change.
    pub fn defaults<T: AsRef<str>>(
        &self,
        language_tag: Option<T>,
        fallback: Option<bool>,
        caching: Option<bool>,
    ) -> Result<(), LocaliserError> {
        if language_tag.is_some() {
            let tag = self.language_registry.tag( language_tag.unwrap() )?;

            #[cfg( not( feature = "sync" ) )]
            self.language_tag.replace( tag );
    
            #[cfg( feature = "sync" )]
            {
                *self.language_tag.write().unwrap() = tag;
            }
        }
        if fallback.is_some() {
            #[cfg( not( feature = "sync" ) )]
            self.fallback.replace( fallback.unwrap() );
    
            #[cfg( feature = "sync" )]
            {
                *self.fallback.write().unwrap() = fallback.unwrap();
            }
        }
        if caching.is_some() {
            #[cfg( not( feature = "sync" ) )]
            self.caching.replace( caching.unwrap() );
    
            #[cfg( feature = "sync" )]
            {
                *self.caching.write().unwrap() = caching.unwrap();
            }
        }
        Ok( () )
    }

    /// Obtain the default language for the `Localiser` instance.
    pub fn default_language( &self ) -> RefCount<String> {

        #[cfg( not( feature = "sync" ) )]
        let binding = self.language_tag.borrow();

        #[cfg( feature = "sync" )]
        let binding = self.language_tag.read().unwrap();

        RefCount::clone( &binding )
    }

    /// Obtain the localisation provider for the `Localiser` instance.
    pub fn localisation_provider( &self ) -> &L {
        &self.localisation_provider
    }

    /// Obtain the language tag registry for the `Localiser` instance.
    pub fn language_tag_registry( &self ) -> &RefCount<LanguageTagRegistry> {
        &self.language_registry
    }
    
    /// Obtain the command registry for the `Localiser` instance.
    pub fn command_registry( &self ) -> &RefCount<CommandRegistry> {
        &self.command_registry
    }

    /// Obtain the ICU data provider for the `Localiser` instance.
    pub fn icu_data_provider( &self ) -> &RefCount<IcuDataProvider> {
        &self.icu_data_provider
    }

    // Internal methods

    // For the specified string identifier, format a string for the specified language tag with the supplied values
    // for the placeholders. Optionally specify whether to fallback to the default language tag of string identifier
    // when there is no string pattern for the specified language. Optionally specify whether the parsed string should
    // be cache for reuse.
    fn actual_format<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        values: &HashMap<String, PlaceholderValue>,
        language_tag: &RefCount<String>,
        fallback: bool, // true = fallback to default language, None = use the Localiser default.
        caching: bool, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        let mut combined = component.as_ref().to_string();
        combined.push( '/' );
        combined.push_str( identifier.as_ref() );
        let mut _language_entry = false;
        {
            #[cfg( not( feature = "sync" ) )]
            let binding = self.cache.borrow();

            #[cfg( feature = "sync" )]
            let binding = self.cache.read().unwrap();

            if let Some( result ) = binding.get( language_tag ) {
                _language_entry = true;
                if let Some( result2 ) = result.get( &combined ) {
                    return match result2 {
                        CacheData::TaggedString( lstring) => Ok( lstring.clone() ),

                        #[cfg( not( feature = "sync" ) )]
                        CacheData::Formatter( formatter ) => 
                            Ok( formatter.borrow_mut().format( values )? ),

                        #[cfg( feature = "sync" )]
                        CacheData::Formatter( formatter ) => Ok( formatter.write().unwrap().format( values )? )
                    }
                }
            }
        }

        // Not in cache.
        // Get pattern string for specified language, though returned `TaggedString` may be for another language.
        let lstring = match self.localisation_provider.string(
            component.as_ref().to_string(), identifier.as_ref().to_string(), language_tag
        )? {
            Some( result ) => result,
            None => {
                if !fallback {
                    return Err( LocaliserError::StringNotFound(
                        component.as_ref().to_string(),
                        identifier.as_ref().to_string(),
                        language_tag.as_str().to_string(),
                        false
                    ) );
                }
                let default_language = &self.localisation_provider.component_details(
                    component.as_ref()
                )?.default;
                match self.localisation_provider.string(
                    component.as_ref().to_string(),
                    identifier.as_ref().to_string(),
                    &default_language
                )? {
                    Some( result ) => result,
                    None => return Err( LocaliserError::StringNotFound(
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

        #[cfg( not( feature = "sync" ) )]
        let ( tokens, _lengths, grammar_found ) = self.lexer.borrow_mut().tokenise(
            lstring.as_str() );

        #[cfg( feature = "sync" )]
        let ( tokens, _lengths, grammar_found ) = self.lexer.write().unwrap().tokenise(
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
                    self.cache.write().unwrap().insert(
                        RefCount::clone( language_tag ),
                        data_entry
                    );
                } else {
                    #[cfg( not( feature = "sync" ) )]
                    let mut binding = self.cache.borrow_mut();

                    #[cfg( feature = "sync" )]
                    let mut binding = self.cache.write().unwrap();

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
            &self.language_registry.locale( language_tag.as_str() )?,
            &tree,
            &self.command_registry,
        )?;

        // If caching is not allowed, simple use `Formatter` to get the TaggedString.
        if !caching {
            return Ok( formatter.format( values )? );
        }

        // Cache the `Formatter`.
        {
            #[cfg( feature = "log" )]
            debug!( "Caching formatting string.");

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
                self.cache.write().unwrap().insert(
                    RefCount::clone( language_tag ),
                    data_entry
                );
            } else {
                #[cfg( not( feature = "sync" ) )]
                let mut binding = self.cache.borrow_mut();

                #[cfg( feature = "sync" )]
                let mut binding = self.cache.write().unwrap();

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
        let binding = self.cache.read().unwrap();

        let result = binding.get( language_tag ).unwrap();
        let result2 = result.get( &combined ).unwrap();
        match result2 {
            CacheData::TaggedString( lstring) => Ok( lstring.clone() ),

            #[cfg( not( feature = "sync" ) )]
            CacheData::Formatter( formatter ) => 
                Ok( formatter.borrow_mut().format( values )? ),

            #[cfg( feature = "sync" )]
            CacheData::Formatter( formatter ) => Ok( formatter.write().unwrap().format( values )? )
        }
    }

    // Simply get the language string without any formatting being done.
    fn actual_literal<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
        fallback: bool, // true = fallback to default language, None = use the Localiser default.
        caching: bool, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        let mut combined = component.as_ref().to_string();
        combined.push( '/' );
        combined.push_str( identifier.as_ref() );
        let mut _language_entry = false;
        {
            #[cfg( not( feature = "sync" ) )]
            let binding = self.cache.borrow();

            #[cfg( feature = "sync" )]
            let binding = self.cache.read().unwrap();

            if let Some( result ) = binding.get( language_tag ) {
                _language_entry = true;
                if let Some( result2 ) = result.get( &combined ) {
                    return match result2 {
                        CacheData::TaggedString( lstring) => Ok( lstring.clone() ),
                        CacheData::Formatter( _formatter ) =>
                            Err( LocaliserError::CacheEntry(
                                component.as_ref().to_string(),
                                identifier.as_ref().to_string(),
                            ) ),
                    }
                }
            }
        }

        // Not in cache.
        // Get pattern string for specified language, though returned `TaggedString` may be for another language.
        let lstring = match self.localisation_provider.string(
            component.as_ref().to_string(), identifier.as_ref().to_string(), language_tag
        )? {
            Some( result ) => result,
            None => {
                if !fallback {
                    return Err( LocaliserError::StringNotFound(
                        component.as_ref().to_string(),
                        identifier.as_ref().to_string(),
                        language_tag.as_str().to_string(),
                        false
                    ) );
                }
                let default_language = &self.localisation_provider.component_details(
                    component.as_ref()
                )?.default;
                match self.localisation_provider.string(
                    component.as_ref().to_string(),
                    identifier.as_ref().to_string(),
                    &default_language
                )? {
                    Some( result ) => result,
                    None => return Err( LocaliserError::StringNotFound(
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
            #[cfg( feature = "log" )]
            debug!( "Caching literal string.");

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
                self.cache.write().unwrap().insert(
                    RefCount::clone( language_tag ),
                    data_entry
                );
            } else {
                #[cfg( not( feature = "sync" ) )]
                let mut binding = self.cache.borrow_mut();

                #[cfg( feature = "sync" )]
                let mut binding = self.cache.write().unwrap();

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
