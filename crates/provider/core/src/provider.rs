// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

use crate::ProviderError;
use i18n_lstring::LString;
use std::rc::Rc;

/// A trait for providing language strings in the form of a `LString` vector, and obtaining the default language tag
/// used for the crate's messages.
/// 
/// For an implementation example, see the `i18n_provider_sqlite3-rizzen-yazston` crate, which uses Sqlite3 for its
/// data store.
pub trait LStringProvider {

    /// Ideally a single exact match should be returned, yet may not be for the requested language tag. If no strings
    /// is found for the requested tag, the right most subtag is removed sequentially until either at least 1 `LString`
    /// is found, or `None returned when there are no more subtags to be removed. Multiple `LString` may be returned
    /// when there are multiple entries of language tags having additional subtags than the requested language tag. 
    /// 
    /// Return of `None` indicates no strings was found matching the requested language tag, or its more general form.
    /// 
    /// Return of `ProviderError` indicates there was an error, usually in the data store.
    fn get<T: AsRef<str>>(
        &self, identifier: T,
        language_tag: &Rc<String>
    ) -> Result<Vec<LString>, ProviderError>;

    /// Retrieve the default language tag of the crate's data store.
    /// 
    /// Return of `None` indicates no default language tag was found with in the provider's data store.
    /// 
    /// Return of `ProviderError` indicates there was an error to retrieve the string from the provider's data store.
    fn default_language_tag<T: AsRef<str>>( &self, identifier: T ) -> Result<Option<String>, ProviderError>;
}
