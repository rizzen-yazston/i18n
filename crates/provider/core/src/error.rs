// This file is part of `i18n_provider-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_provider-rizzen-yazston` crate.

use i18n_utility::{ LocalisationTrait, LocalisationErrorTrait };
use std::error::Error;
use core::fmt::{ Display, Formatter, Result };

/// Contains the error that occurred within the provider.
/// 
/// Due to the nature of [`Box`]`<dyn `[`Error`]`>` opaquing the error type, the error type is stored as a
/// `&'static `[`str`] in `error_type` to facilitate in downcasting the error to original error type for further
/// processing.
/// 
/// [`Box`]: std::boxed::Box
/// [`Error`]: std::error::Error
/// [`str`]: core::str
#[derive( Debug )]
pub struct ProviderError {
    pub error_type: &'static str,
    pub source: Box<dyn Error>,
}

impl LocalisationTrait for ProviderError {
    fn component( &self ) -> &str {
        "i18n_provider"
    }
}

impl LocalisationErrorTrait for ProviderError {
    fn error_type( &self ) -> &str {
        "ProviderError"
    }
}

impl Display for ProviderError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        write!(
            formatter, "ProviderError: [{}].", self.source.to_string()
        )
    }
}

impl Error for ProviderError {
    
    /// Source is the actual error that can be downcasted.
    fn source(&self) -> Option<&( dyn Error + 'static )> {
        Some( self.source.as_ref() )
    }
}
