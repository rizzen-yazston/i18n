// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

/// A trait for localisation of struts and enums.
pub trait LocalisationTrait {

    /// Localisation string identifier. An empty string is returned if enum variant contains an embedded error. 
    fn identifier( &self ) -> &str { "" }

    /// Localisation string component identifier.
    fn component( &self ) -> &str;
}

/// A trait for localisation of error messages.
pub trait LocalisationErrorTrait : LocalisationTrait {

    /// Obtain the type name as a string. Used for logging and displaying of errors.
    /// 
    /// This is normally used with the `i18n_utility` crate's localisation strings: `error_format`,
    /// `error_format_embedded`, `error_format_enum` and `error_format_enum_embedded`.
    fn error_type( &self ) -> &str; 

    /// For enum based errors, the variant can be obtain as a string, else empty string is returned. Used for logging
    /// and displaying of errors.
    /// 
    /// This is normally used with the `i18n_utility` crate's localisation strings: `error_format_enum` and
    /// `error_format_enum_embedded`.
    fn error_variant( &self ) -> &str { "" }
}
