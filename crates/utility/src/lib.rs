// This file is part of `i18n-utility` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `i18n-utility` crate.

/// The `i18n_utility` crate contains various useful components that be handy to the users of the Internationalisation
/// project.
///
/// Contains the follow:
///
/// * LocaleRegistry: A simply registry for holding `ICU4X` Locale objects.

pub mod locale;
pub use locale::*;
