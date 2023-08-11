// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

use crate::ProviderError;
use i18n_utility::LString;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

/// A trait for retrieving language strings from a localisation data repository via a provider that implements the
/// trait methods: `get()` and `get_one()`. In addition, there are other trait methods for retrieving the default
/// language for a component, and supported languages for entire repository, component and identifier
/// respectively.
/// 
/// The parameter `component` is simply a collection of `identifier`s typically representing a dependant library
/// (simply using the crate's name), though can also be application data grouped into components. Within a data
/// repository all `component`s must be unique to avoid `identifier`s clashing from different components.
/// 
/// The parameter `identifier` is an unique name given to a collection of localisation language strings, that expresses
/// the same meaning across the supported languages.
/// 
/// Both the `component` and the `identifier` may consist of Unicode characters not having the properties of either
/// Unicode's Pattern_Syntax or Pattern_White_Space ([UAX #31]).
/// 
/// For an implementation example, see the `i18n_provider_sqlite3-rizzen-yazston` crate, which uses Sqlite3 for its
/// data store.
/// 
/// [UAX #31]: https://www.unicode.org/reports/tr31/
pub trait LStringProvider {

    /// Ideally a single exact match should be returned, yet may not be the case for the requested language tag. If no
    /// strings are found for the requested tag, the provider must remove the right most subtag sequentially until
    /// there are no more subtags. Multiple [`LString`]'s may be returned when there are multiple entries of language
    /// tags having additional subtags than the requested language tag.
    /// 
    /// Return of `ProviderError` indicates there was an error, usually from within the data repository.
    fn get<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
    ) -> Result<Vec<LString>, ProviderError>;

    /// Similar to `get()` method, except that `get_one()` will only return a single [`LString`] if multiple strings
    /// are available.
    /// 
    /// `None` is returned when there is no strings available for the language tag.
    /// 
    /// Return of `ProviderError` indicates there was an error, usually from within the data repository.
    fn get_one<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
    ) -> Result<Option<LString>, ProviderError>;

    /// Retrieve the default language tag of the component in the data repository.
    /// 
    /// Return of `None` indicates no default language tag was found for the component.
    /// 
    /// Return of `ProviderError` indicates there was an error, usually from within the data repository.
    fn default_language_tag<T: AsRef<str>>(
        &self,
        component: T,
    ) -> Result<Option<RefCount<String>>, ProviderError>;

    /// Obtain a list of all the supported languages for a specific identifier.
    /// 
    /// Return of `ProviderError` indicates there was an error, usually from within the data repository.
    fn identifier_language_tags<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
    ) -> Result<Vec<RefCount<String>>, ProviderError>; 

    /// Obtain a list of all the supported languages for a specific component.
    /// 
    /// Return of `ProviderError` indicates there was an error, usually from within the data repository.
    fn component_language_tags<T: AsRef<str>>(
        &self,
        component: T,
    ) -> Result<Vec<LanguageData>, ProviderError>; 

    /// Obtain a list of all the languages having localisation pattern string in the entire repository.
    /// 
    /// Return of `ProviderError` indicates there was an error, usually from within the data repository.
    fn repository_language_tags( &self ) -> Result<Vec<RefCount<String>>, ProviderError>;

    // FUTURE: Idea to retrieve list of components in the data repository.
    /*
    fn components<T: AsRef<str>>(
        &self,
    ) -> Result<Vec<String>, ProviderError>; 
     */ 

     // FUTURE: Idea to retrieve contributor list for a language of a component in the data repository.
    /*
    fn component_contributors<T: AsRef<str>>(
        &self,
        component: T,
    ) -> Result<Vec<LanguageContributor>, ProviderError>; 
     */ 
}

/// A wrapper struct tuple to hold a reference to an `impl LStringProvider`, so that the Provider can be stored in
/// structs.
pub struct LStringProviderWrapper<'a, P: ?Sized>( pub &'a P );

/// Data about a specific language for a component in the provider's repository.
/// `language` is the language tag.
/// `count` is the number of pattern strings for the language tag.
/// `ratio` is the indication of the degree of translations done for the language against the default language.
pub struct LanguageData {
    pub language: RefCount<String>,
    pub count: usize,
    pub ratio: f32,
}
