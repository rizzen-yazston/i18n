// This file is part of `i18n_provider_sqlite3-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider_sqlite3-rizzen-yazston` crate.

//! Sqlite3 provider for `LString`s.
//! 
//! Provides `LString`.
//! 
//! # Examples
//! 
//! ```
//! // TODO
//! ```
//! 

use i18n_utility::locale::LocaleRegistry;
use i18n_lstring::LString;
use i18n_provider::LStringProvider;
use icu_locid::Locale;
use std::collections::HashMap;
use std::{rc::Rc, cell::RefCell};
use std::path::PathBuf;
use rusqlite::{params, Connection, Result as SqliteResult};

/// LString provider.
/// 
/// TODO:
/// 
/// # Examples
/// 
/// ```
/// // TODO
/// ```
/// [`Locale`]: https://docs.rs/icu/latest/icu/locid/struct.Locale.html
/// [`icu_locid`]: https://crates.io/crates/icu_locid
pub struct ProviderSqlite3 {
    root: PathBuf,
    locale_registry: Rc<RefCell<LocaleRegistry>>,
    connections: HashMap<String, Connection>,
}


impl ProviderSqlite3 {

    /// Complete the docs
    pub fn try_new<T: TryInto<PathBuf>>(
        root: T,
        locale_registry: &Rc<RefCell<LocaleRegistry>>
    ) -> Result<Self, String> {
        let Ok( root ) = root.try_into() else {
            return Err( "Invalid path.".to_string() )
        };
        Ok( ProviderSqlite3 {
            root,
            locale_registry: Rc::clone( locale_registry ),
            connections: HashMap::<String, Connection>::new(),
        } )
    }
}

impl LStringProvider for ProviderSqlite3 {

    /// Complete the docs
    fn get( &self, identifier: &str, locale: &Rc<Locale> ) -> Result<LString, String> {
        //split identifier into crate and string identifier, separator /
        //check if connection is cached
        //open db if not and store in cache
        //get string
    



        Err( "Temp return.".to_string() )
    }    
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        /*
        let string = "This is a test string.";
        let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
        let lang_string = LString::new( String::from( string ), Rc::clone( &locale ) );
    
        assert_eq!( lang_string.locale(), locale, "Locale failed." );
        */
    }
}
*/
