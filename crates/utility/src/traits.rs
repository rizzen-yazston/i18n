// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use crate::LocalisationData;
use std::{
    fmt::Debug,
    error::Error,
};

/// Simple trait to indicate the requirement of both [`Error`] and [`LocalisationTrait`] traits.
pub trait LocalisationErrorTrait: LocalisationTrait + Error {} 

/// A trait for localisation of struts and enums.
pub trait LocalisationTrait: Debug {

    /// Obtain the localisation data for the enum or struct that implements this trait.
    fn localisation_data( &self ) -> LocalisationData;
}
