// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

use crate::ProviderError;
use i18n_utility::TaggedString;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "sync" )]
#[cfg( target_has_atomic = "ptr" )]
use std::sync::Arc as RefCount;

/// A trait for retrieving language strings from a localisation data repository via a provider that implements the
/// trait methods: `strings()` and `string()`. In addition, there are other trait methods for retrieving the default
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
pub trait LocalisationProvider {

    /// Obtain a localisation string ([`TaggedString`]) from the data repository for the provided parameters, though
    /// if an exact match is not found then search using similar language tags, else [`None`] returned indicating no
    /// possible match was found.
    /// 
    /// If no string is found for the requested tag, the provider must remove the right most subtag sequentially until
    /// either a match is found or there are no more subtags remaining, at which the result is `None` (not found).
    /// 
    /// If more than one string matches the requested tag, then only one string is returned. This trait does not
    /// specify how the string is to be selected to be returned, thus varied results may be experienced. Look at
    /// `strings()` method to obtain all the strings, that matches the requested tag.
    ///  
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository.
    fn string<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
    ) -> Result<Option<TaggedString>, ProviderError>;

    /// Obtain a localisation string ([`TaggedString`]) only if there is an exact match in the data repository for the
    /// provided parameters, else [`None`] returned indicating no match was found.
    /// 
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository.
    fn string_exact_match<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
    ) -> Result<Option<TaggedString>, ProviderError>;

    /// Similar to `string()`, except all the strings are returned for the matching requested tag.
    /// 
    /// Empty [`Vec`] returned indicates no match was found.
    ///  
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository.
    fn strings<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
        language_tag: &RefCount<String>,
    ) -> Result<Vec<TaggedString>, ProviderError>;

    /// Obtain the information details [`IdentifierDetails`] of an identifier within a component. 
    /// 
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository.
    fn identifier_details<T: AsRef<str>>(
        &self,
        component: T,
        identifier: T,
    ) -> Result<IdentifierDetails, ProviderError>;
    
    /// Obtain the information details [`ComponentDetails`] of a component. 
    /// 
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository.
    fn component_details<T: AsRef<str>>(
        &self,
        component: T,
    ) -> Result<RefCount<ComponentDetails>, ProviderError>;
    
    /// Obtain the information details [`RepositoryDetails`] of the provider's repository.
    /// 
    /// Return of [`ProviderError`] indicates there was an error in accessing the data repository.
    fn repository_details( &self ) -> Result<RefCount<RepositoryDetails>, ProviderError>;
}

/// Contains a list of available languages for an identifier of a component in the provider's repository, where there
/// exists a string for the language. The default language of the identifier is also provided.
//#[derive( Clone )]
pub struct IdentifierDetails {
    pub languages: Vec<RefCount<String>>, // The list of available languages for the identifier.
    pub default: RefCount<String>, // The default language for the identifier.
}

/// Contains a list of available languages of a component in the provider's repository. For each language: a string
/// count, ratio against the default language, and contributors list is provided. The default language is indicated,
/// and also total number of strings found for the component.
//#[derive( Clone )]
pub struct ComponentDetails {
    pub languages: Vec<LanguageData>, // The list of available languages for the component. 
    pub default: RefCount<String>, // The default language of the component. 
    pub total_strings: usize, // The total number of strings for the component.
}

/// Data about an available language of a component in the provider's repository.
//#[derive( Clone )]
pub struct LanguageData {
    pub language: RefCount<String>,
    pub count: usize, // The number of strings for this language.
    pub ratio: f32, // The ratio between this language and the default language of the component.
    pub contributors: Vec<String>, // The list of contributors for this language.
}

/// Contains a list of available languages in the provider's repository. The default language is indicated, also total
/// number of strings, and all the contributors for localisation.
//#[derive( Clone )]
pub struct RepositoryDetails {
    pub languages: Vec<RefCount<String>>, // The unique list of all the languages of all the components.
    pub default: Option<RefCount<String>>, // The default language of repository, usually the application component.
    pub total_strings: usize, // The total number of localisation strings of all the components.
    pub contributors: Vec<String>, // The unique list of all the contributors of all the components.
    pub components: Vec<String>, // The list of components.
}
