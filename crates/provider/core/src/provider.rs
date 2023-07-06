// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

use crate::ProviderError;
use i18n_utility::LString;
use std::rc::Rc;

/// A trait for providing language strings in the form of [`Vec`]`<`[`LString`]`>`, and obtaining the default language
/// tag used for the crate's messages.
/// 
/// For an implementation example, see the `i18n_provider_sqlite3-rizzen-yazston` crate, which uses Sqlite3 for its
/// data store.
pub trait LStringProvider {

    /// Ideally a single exact match should be returned, yet may not be for the requested language tag. If no strings
    /// are found for the requested tag, the right most subtag is removed sequentially until there are no more subtags.
    /// Multiple [`LString`]'s may be returned when there are multiple entries of language tags having additional 
    /// subtags than the requested language tag.
    /// 
    /// Return of `ProviderError` indicates there was an error, usually in the data store.
    fn get<T: AsRef<str>>(
        &self, identifier: T,
        language_tag: &Rc<String>
    ) -> Result<Vec<LString>, ProviderError>;

    /// Similar to `get()` method, except that `get_one()` will only return a single [`LString`] if multiple strings
    /// are available.
    /// 
    /// `None` is returned when there is no strings available for the language tag.
    fn get_one<T: AsRef<str>>(
        &self, identifier: T,
        language_tag: &Rc<String>
    ) -> Result<Option<LString>, ProviderError>;

    /// Retrieve the default language tag of the crate's data store.
    /// 
    /// Return of `None` indicates no default language tag was found with in the provider's data store.
    /// 
    /// Return of `ProviderError` indicates there was an error to retrieve the string from the provider's data store.
    fn default_language_tag<T: AsRef<str>>( &self, identifier: T ) -> Result<Option<String>, ProviderError>;
}

/// A wrapper struct tuple to hold a reference to an `impl LStringProvider`, so that the Provider can be stored in
/// structs.
pub struct LStringProviderWrapper<'a, P: ?Sized>( pub &'a P );
