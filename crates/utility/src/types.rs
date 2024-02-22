// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use crate::TaggedString;
use fixed_decimal::FixedDecimal;
use icu_calendar::{types::Time, Date, DateTime, Iso};
use std::collections::HashMap;

/// A simple data structure containing the data for localisation of an enum or struct.
///
/// `component`: Is the component identifier to use when retrieving a localisation string from a repository,
///
/// `identifier`: Is the pattern string identifier to use when retrieving a localisation string from a repository,
///
/// `values`: Is the values to be used when localisation is carried out. A value of `None` indicates a literal string
/// is being used.
#[derive(Debug, Clone)]
pub struct LocalisationData {
    pub component: String,
    pub identifier: String,
    pub values: Option<HashMap<String, PlaceholderValue>>,
}

/// An enum consists of a selection of Rust primitives, ICU4X types, and [`TaggedString`] for messages.
/// The following are types are available:
///
/// * String( [`String`] ): Can also be used for date (ISO format), time (ISO format), fixed decimal,
///
/// * Integer( [`i128`] ),
///
/// * Unsigned( [`u128`] ),
///
/// * Float( [`f64`] ),
///
/// * TaggedString( [`TaggedString`] ),
///
/// * FixedDecimal( [`FixedDecimal`] ),
///
/// * DateTime( [`DateTime`]`<`[`Iso`]`>` ),
///
/// * Date( [`Date`]`<`[`Iso`]`>` ),
///
/// * Time( [`Time`] ),
///
/// * LocalisationData( [`LocalisationData`] ): Indicates there is an inner localisation string.
#[derive(Debug, Clone)]
pub enum PlaceholderValue {
    String(String), // Can also be used for date (ISO format), time (ISO format), fixed decimal.
    Integer(i128),
    Unsigned(u128),
    Float(f64),
    TaggedString(TaggedString),
    FixedDecimal(FixedDecimal),
    DateTime(DateTime<Iso>),
    Date(Date<Iso>),
    Time(Time),
    LocalisationData(LocalisationData),
}
