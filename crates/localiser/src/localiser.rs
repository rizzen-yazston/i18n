// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

use crate::{CommandRegistry, Formatter, FormatterError, LocaliserError};
use i18n_lexer::IcuDataProvider;
use i18n_provider::LocalisationProviderTrait;
use i18n_utility::{
    LanguageTag, LanguageTagRegistry, LocalisationData, LocalisationErrorTrait, PlaceholderValue,
    TaggedString,
};
use std::collections::HashMap;

#[cfg(not(feature = "sync"))]
use std::cell::RefCell as MutCell;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::{Arc as RefCount, RwLock as MutCell};

#[cfg(feature = "logging")]
use log::debug;

#[cfg(doc)]
use std::sync::Arc;

#[cfg(doc)]
use std::rc::Rc;

pub struct Localiser {
    icu_data_provider: RefCount<IcuDataProvider>,
    grammar: String,
    language_registry: RefCount<LanguageTagRegistry>,
    localisation_provider: Box<dyn LocalisationProviderTrait>,
    command_registry: RefCount<CommandRegistry>,
    fallback: MutCell<bool>,
    caching: MutCell<bool>,
    cache: MutCell<HashMap<RefCount<LanguageTag>, HashMap<String, CacheData>>>,
    language_tag: MutCell<RefCount<LanguageTag>>,
}

impl Localiser {
    /// Create a new `Localiser` instance, that is connected to a language string provider
    /// [`LocalisationProviderTrait`]. A reference to the language tag registry [`Rc`]`<`[`LanguageTagRegistry`]`>` or
    /// [`Arc`]`<LanguageTagRegistry>` instance and reference to the ICU data provider `Rc<`[`IcuDataProvider`]`>` or
    /// `Arc<IcuDataProvider>` are stored within the `Localiser` to facilitate the parsing of language string patterns,
    /// and for formatting strings.
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
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser};
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
    ///         &icu_data_provider, &language_tag_registry, Box::new( localisation_provider ),
    ///         &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let tag = language_tag_registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
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
    ///         &tag,
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
    pub fn try_new(
        icu_data_provider: &RefCount<IcuDataProvider>,
        language_tag_registry: &RefCount<LanguageTagRegistry>,
        localisation_provider: Box<dyn LocalisationProviderTrait>,
        command_registry: &RefCount<CommandRegistry>,
        fallback: bool,
        caching: bool,
        language_tag: &str,
    ) -> Result<Localiser, LocaliserError> {
        let tag = language_tag_registry.tag(language_tag)?;
        Ok(Localiser {
            icu_data_provider: RefCount::clone(icu_data_provider),
            grammar: "{}`#".to_string(),
            language_registry: RefCount::clone(language_tag_registry),
            localisation_provider,
            command_registry: RefCount::clone(command_registry),
            fallback: MutCell::new(fallback),
            caching: MutCell::new(caching),
            cache: MutCell::new(
                HashMap::<RefCount<LanguageTag>, HashMap<String, CacheData>>::new(),
            ),
            language_tag: MutCell::new(tag),
        })
    }

    /// For the specified string identifier, format a string for the specified language tag with the supplied values
    /// for the placeholders. Optionally specify whether to fallback to the default language tag of string identifier
    /// when there is no string pattern for the specified language. Optionally specify whether the parsed string should
    /// be cache for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser};
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
    ///         &icu_data_provider, &language_tag_registry, Box::new( localisation_provider ),
    ///         &command_registry, true, true, "en-ZA"
    ///     )?;
    ///     let tag = language_tag_registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
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
    ///         &tag,
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
    pub fn format(
        &self,
        component: &str,
        identifier: &str,
        values: &HashMap<String, PlaceholderValue>,
        language_tag: &RefCount<LanguageTag>,
        fallback: Option<bool>, // true = fallback to default language, None = use the Localiser default.
        caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg(feature = "logging")]
        debug!("Localiser is using format().");

        #[cfg(not(feature = "sync"))]
        let bool_fallback = fallback.unwrap_or(*self.fallback.borrow());

        #[cfg(not(feature = "sync"))]
        let bool_caching = caching.unwrap_or(*self.caching.borrow());

        #[cfg(feature = "sync")]
        let bool_fallback = fallback.unwrap_or(*self.fallback.read().unwrap());

        #[cfg(feature = "sync")]
        let bool_caching = caching.unwrap_or(*self.caching.read().unwrap());

        self.actual_format(
            component,
            identifier,
            values,
            language_tag,
            bool_fallback,
            bool_caching,
        )
    }

    /// For the specified string identifier, format a string for the specified language tag with the supplied values
    /// for the placeholders using the `Localiser` instance defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser};
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
    ///         &icu_data_provider, &language_tag_registry, Box::new( localisation_provider ),
    ///         &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "component".to_string(),
    ///         PlaceholderValue::String( "i18n_localiser".to_string() )
    ///     );
    ///     values.insert(
    ///         "identifier".to_string(),
    ///         PlaceholderValue::String( "cache_entry".to_string() )
    ///     );
    ///     let lstring = message_system.format_with_defaults(
    ///         "i18n_localiser",
    ///         "cache_entry",
    ///         &values,
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "Unable to get the string for the component ‘i18n_localiser’ with the identifier ‘cache_entry’ as the \
    ///         cache entry requires values for formatting.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format_with_defaults(
        &self,
        component: &str,
        identifier: &str,
        values: &HashMap<String, PlaceholderValue>,
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg(feature = "logging")]
        debug!("Localiser is using format_with_defaults().");

        #[cfg(not(feature = "sync"))]
        let tag = &RefCount::clone(&self.language_tag.borrow());

        #[cfg(not(feature = "sync"))]
        let bool_fallback = *self.fallback.borrow();

        #[cfg(not(feature = "sync"))]
        let bool_caching = *self.caching.borrow();

        #[cfg(feature = "sync")]
        let tag = &RefCount::clone(&self.language_tag.read().unwrap());

        #[cfg(feature = "sync")]
        let bool_fallback = *self.fallback.read().unwrap();

        #[cfg(feature = "sync")]
        let bool_caching = *self.caching.read().unwrap();

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
    /// Optionally specify whether to fallback to the default language tag of string identifier when there is no
    /// string pattern for the specified language. Optionally specify whether the parsed string should be cache for
    /// reuse.
    /// # Examples
    ///
    /// ```
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser};
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
    ///         &icu_data_provider, &language_tag_registry, Box::new( localisation_provider ),
    ///         &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let tag = language_tag_registry.tag("en-ZA").expect("Failed to canonicalise language tag.");
    ///     let lstring = message_system.literal(
    ///         "i18n_localiser",
    ///         "cache_entry",
    ///         &tag,
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "Unable to get the string for the component ‘{component}’ with the identifier ‘{identifier}’ as the \
    ///         cache entry requires values for formatting.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn literal(
        &self,
        component: &str,
        identifier: &str,
        language_tag: &RefCount<LanguageTag>,
        fallback: Option<bool>, // true = fallback to default language, None = use the Localiser default.
        caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg(feature = "logging")]
        debug!("Localiser is using literal().");

        #[cfg(not(feature = "sync"))]
        let bool_fallback = fallback.unwrap_or(*self.fallback.borrow());

        #[cfg(not(feature = "sync"))]
        let bool_caching = caching.unwrap_or(*self.caching.borrow());

        #[cfg(feature = "sync")]
        let bool_fallback = fallback.unwrap_or(*self.fallback.read().unwrap());

        #[cfg(feature = "sync")]
        let bool_caching = caching.unwrap_or(*self.caching.read().unwrap());

        self.actual_literal(
            component,
            identifier,
            language_tag,
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
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser};
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
    ///         &icu_data_provider, &language_tag_registry, Box::new( localisation_provider ),
    ///         &command_registry, true, true, "en-ZA",
    ///     )?;
    ///     let lstring = message_system.literal_with_defaults(
    ///         "i18n_localiser",
    ///         "cache_entry",
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "Unable to get the string for the component ‘{component}’ with the identifier ‘{identifier}’ as the \
    ///         cache entry requires values for formatting.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn literal_with_defaults(
        &self,
        component: &str,
        identifier: &str,
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg(feature = "logging")]
        debug!("Localiser is using literal_with_defaults().");

        #[cfg(not(feature = "sync"))]
        let tag = &RefCount::clone(&self.language_tag.borrow());

        #[cfg(not(feature = "sync"))]
        let bool_fallback = *self.fallback.borrow();

        #[cfg(not(feature = "sync"))]
        let bool_caching = *self.caching.borrow();

        #[cfg(feature = "sync")]
        let tag = &RefCount::clone(&self.language_tag.read().unwrap());

        #[cfg(feature = "sync")]
        let bool_fallback = *self.fallback.read().unwrap();

        #[cfg(feature = "sync")]
        let bool_caching = *self.caching.read().unwrap();

        self.actual_literal(component, identifier, tag, bool_fallback, bool_caching)
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
    pub fn defaults(
        &self,
        language_tag: Option<RefCount<LanguageTag>>,
        fallback: Option<bool>,
        caching: Option<bool>,
    ) -> Result<(), LocaliserError> {
        if let Some(language_tag) = language_tag {
            #[cfg(not(feature = "sync"))]
            self.language_tag.replace(language_tag);

            #[cfg(feature = "sync")]
            {
                *self.language_tag.write().unwrap() = language_tag;
            }
        }
        if let Some(fallback) = fallback {
            #[cfg(not(feature = "sync"))]
            self.fallback.replace(fallback);

            #[cfg(feature = "sync")]
            {
                *self.fallback.write().unwrap() = fallback;
            }
        }
        if let Some(caching) = caching {
            #[cfg(not(feature = "sync"))]
            self.caching.replace(caching);

            #[cfg(feature = "sync")]
            {
                *self.caching.write().unwrap() = caching;
            }
        }
        Ok(())
    }

    /// Obtain the default language for the `Localiser` instance.
    pub fn default_language(&self) -> RefCount<LanguageTag> {
        #[cfg(not(feature = "sync"))]
        let binding = self.language_tag.borrow();

        #[cfg(feature = "sync")]
        let binding = self.language_tag.read().unwrap();

        RefCount::clone(&binding)
    }

    /// Obtain the localisation provider for the `Localiser` instance.
    #[allow(clippy::borrowed_box)]
    pub fn localisation_provider(&self) -> &Box<dyn LocalisationProviderTrait> {
        &self.localisation_provider
    }

    /// Obtain the language tag registry for the `Localiser` instance.
    pub fn language_tag_registry(&self) -> &RefCount<LanguageTagRegistry> {
        &self.language_registry
    }

    /// Obtain the command registry for the `Localiser` instance.
    pub fn command_registry(&self) -> &RefCount<CommandRegistry> {
        &self.command_registry
    }

    /// Obtain the ICU data provider for the `Localiser` instance.
    pub fn icu_data_provider(&self) -> &RefCount<IcuDataProvider> {
        &self.icu_data_provider
    }

    /// Obtain the grammar syntax characters that is used by the localiser.
    pub fn grammar(&self) -> &str {
        self.grammar.as_str()
    }

    /// For the provided [`LocalisationData`], format a string for the specified language tag with the supplied values
    /// for the placeholders. Optionally specify whether to fallback to the default language tag of string identifier
    /// when there is no string pattern for the specified language. Optionally specify whether the parsed string should
    /// be cache for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, LocalisationData, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser};
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let localiser = Localiser::try_new(
    ///         &icu_data_provider,
    ///         &language_tag_registry,
    ///         Box::new( lstring_provider ),
    ///         &command_registry,
    ///         true,
    ///         true,
    ///         "en-ZA",
    ///     )?;
    ///     let language = language_tag_registry.tag( "en-ZA" ).unwrap();
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
    ///     let data = LocalisationData {
    ///         component: "i18n_localiser".to_string(),
    ///         identifier: "string_not_found".to_string(),
    ///         values: Some( values ),
    ///     };
    ///     let lstring = localiser.format_localisation_data(
    ///         &data,
    ///         &language,
    ///         None,
    ///         None
    ///     )?;
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No string was found for the component ‘i18n_localiser’ with identifier ‘string_not_found’ for the language \
    ///         tag ‘en-ZA’. Fallback was used: True.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format_localisation_data(
        &self,
        data: &LocalisationData,
        language_tag: &RefCount<LanguageTag>,
        fallback: Option<bool>, // true = fallback to default language, None = use the Localiser default.
        caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg(feature = "logging")]
        debug!("Localiser is using format_error().");

        #[cfg(not(feature = "sync"))]
        let bool_fallback = fallback.unwrap_or(*self.fallback.borrow());

        #[cfg(not(feature = "sync"))]
        let bool_caching = caching.unwrap_or(*self.caching.borrow());

        #[cfg(feature = "sync")]
        let bool_fallback = fallback.unwrap_or(*self.fallback.read().unwrap());

        #[cfg(feature = "sync")]
        let bool_caching = caching.unwrap_or(*self.caching.read().unwrap());

        self.actual_format_localisation_data(data, language_tag, bool_fallback, bool_caching)
    }

    /// For the provided [`LocalisationData`] instance, format using the `Localiser` instance defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, LocalisationData, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser};
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let localiser = Localiser::try_new(
    ///         &icu_data_provider,
    ///         &language_tag_registry,
    ///         Box::new( lstring_provider ),
    ///         &command_registry,
    ///         true,
    ///         true,
    ///         "en-ZA",
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
    ///     let data = LocalisationData {
    ///         component: "i18n_localiser".to_string(),
    ///         identifier: "string_not_found".to_string(),
    ///         values: Some( values ),
    ///     };
    ///     let lstring = localiser.format_localisation_data_with_defaults( &data ).unwrap();
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "No string was found for the component ‘i18n_localiser’ with identifier ‘string_not_found’ for the language \
    ///         tag ‘en-ZA’. Fallback was used: True.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format_localisation_data_with_defaults(
        &self,
        data: &LocalisationData,
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg(feature = "logging")]
        debug!("Localiser is using format_error_with_defaults().");

        #[cfg(not(feature = "sync"))]
        let tag = &RefCount::clone(&self.language_tag.borrow());

        #[cfg(not(feature = "sync"))]
        let bool_fallback = *self.fallback.borrow();

        #[cfg(not(feature = "sync"))]
        let bool_caching = *self.caching.borrow();

        #[cfg(feature = "sync")]
        let tag = &RefCount::clone(&self.language_tag.read().unwrap());

        #[cfg(feature = "sync")]
        let bool_fallback = *self.fallback.read().unwrap();

        #[cfg(feature = "sync")]
        let bool_caching = *self.caching.read().unwrap();

        self.actual_format_localisation_data(data, tag, bool_fallback, bool_caching)
    }

    /// Format the provided error implementing the [`LocalisationErrorTrait`] trait. Optionally specify whether to
    /// fallback to the default language tag of string identifier when there is no string pattern for the specified
    /// language. Optionally specify whether the parsed string should be cache for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser, LocaliserError};
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let localiser = Localiser::try_new(
    ///         &icu_data_provider,
    ///         &language_tag_registry,
    ///         Box::new( lstring_provider ),
    ///         &command_registry,
    ///         true,
    ///         true,
    ///         "en-ZA",
    ///     )?;
    ///     let language = language_tag_registry.tag( "en-ZA" ).unwrap();
    ///     let error = LocaliserError::StringNotFound(
    ///         "application".to_string(),
    ///         "not_exists".to_string(),
    ///         language.as_str().to_string(),
    ///         false
    ///     );
    ///     let lstring = localiser.format_error(
    ///         &error, &language, None, None
    ///     ).unwrap();
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "LocaliserError::StringNotFound: ‘No string was found for the component ‘application’ with identifier \
    ///         ‘not_exists’ for the language tag ‘en-ZA’. Fallback was used: False.’.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format_error(
        &self,
        error: &impl LocalisationErrorTrait,
        language_tag: &RefCount<LanguageTag>,
        fallback: Option<bool>, // true = fallback to default language, None = use the Localiser default.
        caching: Option<bool>, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg(feature = "logging")]
        debug!("Localiser is using format_error().");

        #[cfg(not(feature = "sync"))]
        let bool_fallback = fallback.unwrap_or(*self.fallback.borrow());

        #[cfg(not(feature = "sync"))]
        let bool_caching = caching.unwrap_or(*self.caching.borrow());

        #[cfg(feature = "sync")]
        let bool_fallback = fallback.unwrap_or(*self.fallback.read().unwrap());

        #[cfg(feature = "sync")]
        let bool_caching = caching.unwrap_or(*self.caching.read().unwrap());

        let data = error.localisation_data();
        self.actual_format_localisation_data(&data, language_tag, bool_fallback, bool_caching)
    }

    /// For the provided error implementing the [`LocalisationErrorTrait`] trait, format using the [`Localiser`]
    /// instance defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use i18n_lexer::{IcuDataProvider, DataProvider};
    /// use i18n_utility::{LanguageTagRegistry, PlaceholderValue};
    /// use i18n_provider_sqlite3::LocalisationProviderSqlite3;
    /// use i18n_localiser::{CommandRegistry, Localiser, LocaliserError};
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let language_tag_registry = Rc::new( LanguageTagRegistry::new() );
    ///     let lstring_provider = LocalisationProviderSqlite3::try_new(
    ///         "./l10n/", &language_tag_registry, false
    ///     )?;
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let localiser = Localiser::try_new(
    ///         &icu_data_provider,
    ///         &language_tag_registry,
    ///         Box::new( lstring_provider ),
    ///         &command_registry,
    ///         true,
    ///         true,
    ///         "en-ZA",
    ///     )?;
    ///     let language = language_tag_registry.tag( "en-ZA" ).unwrap();
    ///     let error = LocaliserError::StringNotFound(
    ///         "application".to_string(),
    ///         "not_exists".to_string(),
    ///         language.as_str().to_string(),
    ///         false
    ///     );
    ///     let lstring = localiser.format_error_with_defaults( &error ).unwrap();
    ///     assert_eq!(
    ///         lstring.as_str(),
    ///         "LocaliserError::StringNotFound: ‘No string was found for the component ‘application’ with identifier \
    ///         ‘not_exists’ for the language tag ‘en-ZA’. Fallback was used: False.’.",
    ///         "Check placeholder values."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
    pub fn format_error_with_defaults(
        &self,
        error: &impl LocalisationErrorTrait,
    ) -> Result<TaggedString, LocaliserError> {
        #[cfg(feature = "logging")]
        debug!("Localiser is using format_error_with_defaults().");

        #[cfg(not(feature = "sync"))]
        let tag = &RefCount::clone(&self.language_tag.borrow());

        #[cfg(not(feature = "sync"))]
        let bool_fallback = *self.fallback.borrow();

        #[cfg(not(feature = "sync"))]
        let bool_caching = *self.caching.borrow();

        #[cfg(feature = "sync")]
        let tag = &RefCount::clone(&self.language_tag.read().unwrap());

        #[cfg(feature = "sync")]
        let bool_fallback = *self.fallback.read().unwrap();

        #[cfg(feature = "sync")]
        let bool_caching = *self.caching.read().unwrap();

        let data = error.localisation_data();
        self.actual_format_localisation_data(&data, tag, bool_fallback, bool_caching)
    }

    // Internal methods

    // For the specified string identifier, format a string for the specified language tag with the supplied values
    // for the placeholders. Optionally specify whether to fallback to the default language tag of string identifier
    // when there is no string pattern for the specified language. Optionally specify whether the parsed string should
    // be cache for reuse.
    fn actual_format(
        &self,
        component: &str,
        identifier: &str,
        values: &HashMap<String, PlaceholderValue>,
        language_tag: &RefCount<LanguageTag>,
        fallback: bool, // true = fallback to default language.
        caching: bool, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        let mut combined = component.to_string();
        combined.push('/');
        combined.push_str(identifier.as_ref());
        let mut _language_entry = false;
        {
            #[cfg(not(feature = "sync"))]
            let binding = self.cache.borrow();

            #[cfg(feature = "sync")]
            let binding = self.cache.read().unwrap();

            if let Some(result) = binding.get(language_tag) {
                _language_entry = true;
                if let Some(result2) = result.get(&combined) {
                    return match result2 {
                        CacheData::TaggedString(lstring) => Ok(lstring.clone()),

                        #[cfg(not(feature = "sync"))]
                        CacheData::Formatter(formatter) => {
                            Ok(formatter.borrow_mut().format(self, values)?)
                        }

                        #[cfg(feature = "sync")]
                        CacheData::Formatter(formatter) => {
                            Ok(formatter.write().unwrap().format(self, values)?)
                        }
                    };
                }
            }
        }

        // Not in cache.
        // Get pattern string for specified language, though returned `TaggedString` may be for another language.
        let lstring =
            match self
                .localisation_provider
                .string(component, identifier, language_tag)?
            {
                Some(result) => result,
                None => {
                    if !fallback {
                        return Err(LocaliserError::StringNotFound(
                            component.to_string(),
                            identifier.to_string(),
                            language_tag.as_str().to_string(),
                            false,
                        ));
                    }
                    let default_language = &self
                        .localisation_provider
                        .component_details(component.as_ref())?
                        .default;
                    match self.localisation_provider.string(
                        component.as_ref(),
                        identifier.as_ref(),
                        default_language,
                    )? {
                        Some(result) => result,
                        None => {
                            return Err(LocaliserError::StringNotFound(
                                component.to_string(),
                                identifier.to_string(),
                                language_tag.as_str().to_owned(),
                                true,
                            ))
                        }
                    }
                }
            };

        // Tokenise the pattern string.
        // If string is empty, skip over formatter, simply cache (if allowed) and return the empty string.
        // If pattern string has no grammar syntax characters, simply cache (if allowed) and return the string.
        if lstring.as_str().is_empty() {
            if caching {
                self.add_string_to_cache(_language_entry, &combined, &lstring);
            }
            return Ok(lstring);
        }
        let mut formatter = match Formatter::try_new(self, language_tag, lstring.as_str()) {
            Ok(value) => value,
            Err(error) => match error {
                FormatterError::NoGrammar => {
                    if caching {
                        self.add_string_to_cache(_language_entry, &combined, &lstring);
                    }
                    return Ok(lstring);
                }
                _ => return Err(LocaliserError::Formatter(error)),
            },
        };

        // If caching is not allowed, simple use `Formatter` to get the TaggedString.
        if !caching {
            return Ok(formatter.format(self, values)?);
        }

        // Cache the `Formatter`.
        {
            #[cfg(feature = "logging")]
            debug!("Caching formatting string.");

            if !_language_entry {
                let mut data_entry = HashMap::<String, CacheData>::new();
                data_entry.insert(
                    combined.clone(),
                    CacheData::Formatter(MutCell::new(formatter)),
                );

                #[cfg(not(feature = "sync"))]
                self.cache
                    .borrow_mut()
                    .insert(RefCount::clone(language_tag), data_entry);

                #[cfg(feature = "sync")]
                self.cache
                    .write()
                    .unwrap()
                    .insert(RefCount::clone(language_tag), data_entry);
            } else {
                #[cfg(not(feature = "sync"))]
                let mut binding = self.cache.borrow_mut();

                #[cfg(feature = "sync")]
                let mut binding = self.cache.write().unwrap();

                let data_entry = binding.get_mut(language_tag);
                data_entry.unwrap().insert(
                    combined.clone(),
                    CacheData::Formatter(MutCell::new(formatter)),
                );
            }
        }

        // Get `Formatter` and use it to get the TaggedString.
        #[cfg(not(feature = "sync"))]
        let binding = self.cache.borrow();

        #[cfg(feature = "sync")]
        let binding = self.cache.read().unwrap();

        let result = binding.get(language_tag).unwrap();
        let result2 = result.get(&combined).unwrap();
        match result2 {
            CacheData::TaggedString(lstring) => Ok(lstring.clone()),

            #[cfg(not(feature = "sync"))]
            CacheData::Formatter(formatter) => Ok(formatter.borrow_mut().format(self, values)?),

            #[cfg(feature = "sync")]
            CacheData::Formatter(formatter) => {
                Ok(formatter.write().unwrap().format(self, values)?)
            }
        }
    }

    // Simply get the language string without any formatting being done.
    fn actual_literal(
        &self,
        component: &str,
        identifier: &str,
        language_tag: &RefCount<LanguageTag>,
        fallback: bool, // true = fallback to default language, None = use the Localiser default.
        caching: bool, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        let mut combined = component.to_string();
        combined.push('/');
        combined.push_str(identifier);
        let mut _language_entry = false;
        {
            #[cfg(not(feature = "sync"))]
            let binding = self.cache.borrow();

            #[cfg(feature = "sync")]
            let binding = self.cache.read().unwrap();

            if let Some(result) = binding.get(language_tag) {
                _language_entry = true;
                if let Some(result2) = result.get(&combined) {
                    return match result2 {
                        CacheData::TaggedString(lstring) => Ok(lstring.clone()),
                        CacheData::Formatter(_formatter) => Err(LocaliserError::CacheEntry(
                            component.to_string(),
                            identifier.to_string(),
                        )),
                    };
                }
            }
        }

        // Not in cache.
        // Get pattern string for specified language, though returned `TaggedString` may be for another language.
        let lstring =
            match self
                .localisation_provider
                .string(component, identifier, language_tag)?
            {
                Some(result) => result,
                None => {
                    if !fallback {
                        return Err(LocaliserError::StringNotFound(
                            component.to_string(),
                            identifier.to_string(),
                            language_tag.as_str().to_string(),
                            false,
                        ));
                    }
                    let default_language = &self
                        .localisation_provider
                        .component_details(component.as_ref())?
                        .default;
                    match self.localisation_provider.string(
                        component.as_ref(),
                        identifier.as_ref(),
                        default_language,
                    )? {
                        Some(result) => result,
                        None => {
                            return Err(LocaliserError::StringNotFound(
                                component.to_string(),
                                identifier.to_string(),
                                language_tag.as_str().to_owned(),
                                true,
                            ))
                        }
                    }
                }
            };

        // Since pattern string is treated as literal, simply cache (if allowed) and return the string.
        if caching {
            self.add_string_to_cache(_language_entry, &combined, &lstring);
        }
        Ok(lstring)
    }

    fn actual_format_localisation_data(
        &self,
        data: &LocalisationData,
        language_tag: &RefCount<LanguageTag>,
        fallback: bool, // true = fallback to default language
        caching: bool, // true = cache the resultant Formatter for repeating use with different values.
    ) -> Result<TaggedString, LocaliserError> {
        Ok(if data.values.is_none() {
            self.actual_literal(
                data.component.as_str(),
                data.identifier.as_str(),
                language_tag,
                fallback,
                caching,
            )?
        } else {
            let mut values_new = HashMap::<String, PlaceholderValue>::new();
            for (placeholder, value) in data.values.as_ref().unwrap() {
                match value {
                    PlaceholderValue::LocalisationData(inner) => {
                        let _ = values_new.insert(
                            placeholder.clone(),
                            PlaceholderValue::TaggedString(self.actual_format_localisation_data(
                                inner,
                                language_tag,
                                fallback,
                                caching,
                            )?),
                        );
                    }
                    _ => {
                        let _ = values_new.insert(placeholder.to_string(), value.clone());
                    }
                }
            }
            self.actual_format(
                data.component.as_str(),
                data.identifier.as_str(),
                &values_new,
                language_tag,
                fallback,
                caching,
            )?
        })
    }

    fn add_string_to_cache(&self, language_entry: bool, combined: &String, lstring: &TaggedString) {
        #[cfg(feature = "logging")]
        debug!("Caching literal string.");

        if !language_entry {
            let mut data_entry = HashMap::<String, CacheData>::new();
            data_entry.insert(
                combined.to_string(),
                CacheData::TaggedString(lstring.clone()),
            );

            #[cfg(not(feature = "sync"))]
            self.cache
                .borrow_mut()
                .insert(RefCount::clone(lstring.tag()), data_entry);

            #[cfg(feature = "sync")]
            self.cache
                .write()
                .unwrap()
                .insert(RefCount::clone(lstring.tag()), data_entry);
        } else {
            #[cfg(not(feature = "sync"))]
            let mut binding = self.cache.borrow_mut();

            #[cfg(feature = "sync")]
            let mut binding = self.cache.write().unwrap();

            let data_entry = binding.get_mut(lstring.tag());
            data_entry
                .unwrap()
                .insert(combined.clone(), CacheData::TaggedString(lstring.clone()));
        }
    }
}

// Internal structs, enums, etc

enum CacheData {
    TaggedString(TaggedString),
    Formatter(MutCell<Formatter>),
}
