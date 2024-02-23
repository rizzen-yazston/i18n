// This file is part of `i18n_localiser-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_localiser-rizzen-yazston` crate.

// Future feature option: language tag wrapping

use crate::{FormatterError, Localiser};
use fixed_decimal::{DoublePrecision, FixedDecimal, SignDisplay};
#[allow(unused_imports)]
use i18n_icu::{DataProvider, IcuDataProvider};
use i18n_lexer::Token;
use i18n_pattern::{CommandRegistry, NodeType};
use i18n_utility::{LanguageTag, PlaceholderValue, TaggedString};
use icu_calendar::{
    types::{IsoHour, IsoMinute, IsoSecond, NanoSecond, Time},
    Date, DateTime, Iso,
};
use icu_datetime::{
    options::length::{Bag, Date as DateLength, Time as TimeLength},
    DateFormatter, DateTimeFormatter, TimeFormatter,
};
use icu_decimal::{options, FixedDecimalFormatter};
use icu_plurals::{PluralCategory, PluralRules};
use icu_provider::prelude::DataLocale;
use tree::Tree;

#[cfg(not(feature = "extend"))]
use icu_locid::LanguageIdentifier as Locale;

#[cfg(feature = "extend")]
use icu_locid::Locale;

#[cfg(feature = "log")]
use log::{debug, trace};

use std::collections::HashMap;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

use std::str::FromStr;

pub struct Formatter {
    language_tag: RefCount<LanguageTag>,
    locale: RefCount<Locale>,
    patterns: HashMap<String, Vec<PatternPart>>,
    numbers: Vec<String>,
    selectors: Vec<HashMap<String, String>>,
}

impl Formatter {
    /// Creates a Formatter for a language string using parsing results.
    /// During the creation of the formatter for the supplied [`Tree`], the semantic analyse is done.
    ///
    /* For now need to see how to modify to test these
    /// # Examples
    ///
    /// ```
    /// use i18n_icu::{ IcuDataProvider, DataProvider };
    /// use i18n_lexer::{ Token, TokenType, Lexer };
    /// use i18n_pattern::{ parse, NodeType, Formatter, FormatterError, PlaceholderValue, CommandRegistry };
    /// use icu_locid::Locale;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    ///
    /// fn pattern_plural() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    ///     let ( tokens, _lengths, _grammar ) =
    ///         lexer.tokenise( "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}\
    /// {one_dog is 1 dog}" );
    ///     let tree = parse( tokens )?;
    ///     let locale: Rc<Locale> = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    ///     let language_tag = Rc::new( locale.to_string() );
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut formatter = Formatter::try_new(
    ///         &icu_data_provider, &language_tag, &locale, &tree, &command_registry
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "dogs_number".to_string(),
    ///         PlaceholderValue::Unsigned( 3 )
    ///     );
    ///     let result = formatter.format( &values )?;
    ///     assert_eq!(
    ///         result.as_str(),
    ///         "There are 3 dogs in the park.",
    ///         "Strings must be the same."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
     */
    ///  
    /// [`Tree`]: tree::Tree
    pub fn try_new(
        localiser: &Localiser,
        language_tag: &RefCount<LanguageTag>,
        tree: &Tree,
    ) -> Result<Formatter, FormatterError> {
        #[cfg(feature = "log")]
        debug!("Creating formatter for language tag '{}'", language_tag);

        let locale = localiser
            .language_tag_registry()
            .locale(language_tag.as_str())
            .unwrap();
        let mut patterns = HashMap::<String, Vec<PatternPart>>::new();
        patterns.insert("_".to_string(), Vec::<PatternPart>::new()); // Insert empty main pattern.
        let mut numbers = Vec::<String>::new();
        let mut selectors = Vec::<HashMap<String, String>>::new();
        if tree.len() == 0 {
            return Ok(Formatter {
                language_tag: RefCount::clone(language_tag),
                locale,
                patterns,
                numbers,
                selectors,
            });
        }
        let option_selectors = OptionSelectors {
            valid_plurals: vec!["zero", "one", "two", "few", "many", "other"],
            calendars: vec![
                "gregorian",
                "buddhist",
                "japanese",
                "ethiopian",
                "indian",
                "coptic",
                "iso",
            ],
        };
        if !check_node_type(tree, 0, NodeType::Root) {
            return Err(FormatterError::InvalidRoot);
        }

        // Process substrings first if present.
        if let Ok(last) = tree.last(0) {
            if check_node_type(tree, last, NodeType::NamedGroup) {
                #[cfg(feature = "log")]
                trace!("Processing NamedGroup.");

                let Ok(named_strings) = tree.children(last) else {
                    return Err(FormatterError::RetrieveChildren(NodeType::NamedGroup));
                };
                for named in named_strings.iter() {
                    let mut pattern = Vec::<PatternPart>::new();
                    if !check_node_type(tree, *named, NodeType::NamedString) {
                        return Err(FormatterError::NodeNotFound(NodeType::NamedString));
                    }

                    // Get NamedString identifier and check it is not already present.
                    let Ok(first) = tree.first(*named) else {
                        return Err(FormatterError::FirstChild(NodeType::NamedString));
                    };
                    if !check_node_type(tree, first, NodeType::Identifier) {
                        return Err(FormatterError::NodeNotFound(NodeType::Identifier));
                    }
                    let Ok(identifier_data) = tree.data_ref(first) else {
                        return Err(FormatterError::RetrieveNodeData(NodeType::Identifier));
                    };
                    let Some(identifier_token) = identifier_data
                        .first()
                        .unwrap()
                        .downcast_ref::<RefCount<Token>>()
                    else {
                        return Err(FormatterError::RetrieveNodeToken(NodeType::Identifier));
                    };
                    if patterns.contains_key(identifier_token.string.as_str()) {
                        return Err(FormatterError::NamedStringIdentifier(
                            identifier_token.string.as_str().to_string(),
                        ));
                    }

                    // Got NamedString identifier
                    let Ok(last) = tree.last(*named) else {
                        return Err(FormatterError::LastChild(NodeType::NamedString));
                    };
                    if !check_node_type(tree, last, NodeType::String) {
                        return Err(FormatterError::NodeNotFound(NodeType::Identifier));
                    }
                    let Ok(children) = tree.children(last) else {
                        return Err(FormatterError::RetrieveChildren(NodeType::String));
                    };
                    for child in children.iter() {
                        if check_node_type(tree, *child, NodeType::Text) {
                            part_text(&mut pattern, tree, *child)?;
                        } else if check_node_type(tree, *child, NodeType::NumberSign) {
                            let len = numbers.len();
                            numbers.push(String::new());
                            pattern.push(PatternPart::NumberSign(len));
                        } else if check_node_type(tree, *child, NodeType::Pattern) {
                            part_pattern(
                                &mut pattern,
                                &patterns,
                                tree,
                                *child,
                                &mut selectors,
                                &option_selectors,
                                &locale,
                            )?;
                        } else if check_node_type(tree, *child, NodeType::Command) {
                            part_command(&mut pattern, tree, *child, localiser.command_registry())?;
                        } else {
                            return Err(FormatterError::InvalidNode(NodeType::String));
                        }
                    }
                    patterns.insert(identifier_token.string.as_str().to_string(), pattern);
                }
            }
        };

        // Now process main string.
        #[cfg(feature = "log")]
        trace!("Processing main string");

        let mut pattern = Vec::<PatternPart>::new();
        let Ok(first) = tree.first(0) else {
            return Err(FormatterError::FirstChild(NodeType::Root));
        };
        if !check_node_type(tree, first, NodeType::String) {
            return Err(FormatterError::NodeNotFound(NodeType::String));
        }
        let Ok(children) = tree.children(first) else {
            return Err(FormatterError::RetrieveChildren(NodeType::String));
        };
        for child in children.iter() {
            if check_node_type(tree, *child, NodeType::Text) {
                part_text(&mut pattern, tree, *child)?;
            } else if check_node_type(tree, *child, NodeType::Pattern) {
                part_pattern(
                    &mut pattern,
                    &patterns,
                    tree,
                    *child,
                    &mut selectors,
                    &option_selectors,
                    &locale,
                )?;
            } else if check_node_type(tree, *child, NodeType::Command) {
                part_command(&mut pattern, tree, *child, localiser.command_registry())?;
            } else {
                return Err(FormatterError::InvalidNode(NodeType::String));
            }
        }
        patterns.insert("_".to_string(), pattern);
        Ok(Formatter {
            language_tag: RefCount::clone(language_tag),
            locale,
            patterns,
            numbers,
            selectors,
        })
    }

    /// Format the language string with supplied values as [`HashMap`]`<`[`String`]`, `[`PlaceholderValue`]`>`.
    ///
    /* For now need to see how to modify to test these
    /// # Examples
    ///
    /// ```
    /// use i18n_icu::{ IcuDataProvider, DataProvider };
    /// use i18n_lexer::{ Token, TokenType, Lexer };
    /// use i18n_utility::PlaceholderValue;
    /// use i18n_pattern::{ parse, NodeType, CommandRegistry };
    /// use i18n_localiser::{ Formatter, FormatterError }
    /// use icu_locid::Locale;
    /// use std::collections::HashMap;
    /// use std::rc::Rc;
    /// use std::error::Error;
    ///
    /// fn pattern_plural() -> Result<(), Box<dyn Error>> {
    ///     let icu_data_provider = Rc::new( IcuDataProvider::try_new( DataProvider::Internal )? );
    ///     let mut lexer = Lexer::new( vec![ '{', '}', '`', '#' ], &icu_data_provider );
    ///     let ( tokens, _lengths, _grammar ) =
    ///         lexer.tokenise( "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}" );
    ///     let tree = parse( tokens )?;
    ///     let locale: Rc<Locale> = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
    ///     let language_tag = Rc::new( locale.to_string() );
    ///     let command_registry = Rc::new( CommandRegistry::new() );
    ///     let mut formatter = Formatter::try_new(
    ///         &icu_data_provider, &language_tag, &locale, &tree, &command_registry
    ///     )?;
    ///     let mut values = HashMap::<String, PlaceholderValue>::new();
    ///     values.insert(
    ///         "dogs_number".to_string(),
    ///         PlaceholderValue::Unsigned( 3 )
    ///     );
    ///     let result = formatter.format( &values )?;
    ///     assert_eq!(
    ///         result.as_str(),
    ///         "There are 3 dogs in the park.",
    ///         "Strings must be the same."
    ///     );
    ///     Ok( () )
    /// }
    /// ```
     */
    pub fn format(
        &mut self,
        localiser: &Localiser,
        values: &HashMap<String, PlaceholderValue>,
    ) -> Result<TaggedString, FormatterError> {
        if self.patterns.get("_").unwrap().is_empty() {
            return Ok(TaggedString::new(String::new(), &self.language_tag));
        }
        let pattern_string = self.format_pattern(localiser, values, &"_".to_string())?;
        Ok(TaggedString::new(pattern_string, &self.language_tag))
    }

    // Internal methods

    fn part_ref(&self, string: &String, index: usize) -> Option<&PatternPart> {
        if let Some(pattern) = self.patterns.get(string) {
            if let Some(part) = pattern.get(index) {
                return Some(part);
            }
        }
        None
    }

    fn format_pattern(
        &mut self,
        localiser: &Localiser,
        values: &HashMap<String, PlaceholderValue>,
        named: &String,
    ) -> Result<String, FormatterError> {
        let mut string = String::new();
        let mut _len = 0usize;
        {
            let Some(pattern) = self.patterns.get(named) else {
                return Err(FormatterError::PatternNamed(named.to_string()));
            };
            _len = pattern.len();
        }
        let mut i = 0usize;
        while i < _len {
            let Some(part) = self.part_ref(named, i) else {
                return Err(FormatterError::PatternPart(named.to_string(), i));
            };
            match part {
                PatternPart::Text(text) => {
                    #[cfg(feature = "log")]
                    trace!("Formatting PatternPart::Text");

                    string.push_str(text.as_str());
                }
                PatternPart::PatternString(placeholder) => {
                    #[cfg(feature = "log")]
                    trace!("Formatting PatternPart::PatternString");

                    let Some(value) = values.get(placeholder) else {
                        return Err(FormatterError::PlaceholderValue(
                            "PatternString".to_string(),
                            placeholder.to_string(),
                        ));
                    };
                    match value {
                        PlaceholderValue::String(value) => string.push_str(value),
                        PlaceholderValue::TaggedString(value) => string.push_str(value.as_str()),
                        _ => return Err(FormatterError::InvalidValue("PatternString".to_string())),
                    }
                }
                PatternPart::PatternDecimal {
                    placeholder,
                    sign,
                    group,
                } => {
                    #[cfg(feature = "log")]
                    trace!("Formatting PatternPart::PatternDecimal");

                    let Some(value) = values.get(placeholder) else {
                        return Err(FormatterError::PlaceholderValue(
                            "PatternDecimal".to_string(),
                            placeholder.to_string(),
                        ));
                    };
                    let data_locale = DataLocale::from(RefCount::as_ref(&self.locale));
                    let mut options: options::FixedDecimalFormatterOptions = Default::default();
                    if group.is_some() {
                        options.grouping_strategy = group.unwrap();
                    }
                    let fdf = self.fixed_decimal_formatter(localiser, &data_locale, options)?;
                    match value {
                        PlaceholderValue::FixedDecimal(number) => {
                            let fixed_decimal = &mut number.clone();
                            if sign.is_some() {
                                fixed_decimal.apply_sign_display(sign.unwrap());
                            }
                            let number_string = fdf.format(fixed_decimal).to_string();
                            string.push_str(number_string.as_str());
                        }
                        PlaceholderValue::Unsigned(number) => {
                            let mut fixed_decimal = FixedDecimal::from(*number);
                            if sign.is_some() {
                                fixed_decimal.apply_sign_display(sign.unwrap());
                            }
                            let number_string = fdf.format(&fixed_decimal).to_string();
                            string.push_str(number_string.as_str());
                        }
                        PlaceholderValue::Integer(number) => {
                            let mut fixed_decimal = FixedDecimal::from(*number);
                            if sign.is_some() {
                                fixed_decimal.apply_sign_display(sign.unwrap());
                            }
                            let number_string = fdf.format(&fixed_decimal).to_string();
                            string.push_str(number_string.as_str());
                        }
                        PlaceholderValue::Float(number) => {
                            // Precision is always Floating, for other precisions options use PlaceholderFixedDecimal
                            let mut fixed_decimal =
                                FixedDecimal::try_from_f64(*number, DoublePrecision::Floating)?;
                            if sign.is_some() {
                                fixed_decimal.apply_sign_display(sign.unwrap());
                            }
                            let number_string = fdf.format(&fixed_decimal).to_string();
                            string.push_str(number_string.as_str());
                        }
                        _ => {
                            return Err(FormatterError::InvalidValue("PatternDecimal".to_string()))
                        }
                    }
                }
                PatternPart::PatternDateTime {
                    placeholder,
                    length_date,
                    length_time,
                    calendar,
                } => {
                    #[cfg(feature = "log")]
                    trace!("Formatting PatternPart::PatternDateTime");

                    // TODO: add more options as they become non-experimental.
                    // TODO: implement hour for Time/DateTime when no longer experimental
                    let Some(value) = values.get(placeholder) else {
                        return Err(FormatterError::PlaceholderValue(
                            "PatternDateTime".to_string(),
                            placeholder.to_string(),
                        ));
                    };
                    let length_date = match length_date {
                        None => DateLength::Medium,
                        Some(result) => *result,
                    };
                    let length_time = match length_time {
                        None => TimeLength::Medium,
                        Some(result) => *result,
                    };
                    let options = Bag::from_date_time_style(length_date, length_time);
                    let data_locale = match calendar {
                        None => DataLocale::from(RefCount::as_ref(&self.locale)),
                        Some(locale) => DataLocale::from(locale),
                    };
                    match value {
                        PlaceholderValue::DateTime(date_time) => {
                            let dtf = self.date_time_formatter(localiser, &data_locale, options)?;
                            let date_string = dtf.format_to_string(&date_time.to_any())?;
                            string.push_str(date_string.as_str());
                        }
                        PlaceholderValue::Date(date) => {
                            let df = self.date_formatter(localiser, &data_locale, length_date)?;
                            let date_string = df.format_to_string(&date.to_any())?;
                            string.push_str(date_string.as_str());
                        }
                        PlaceholderValue::Time(time) => {
                            let tf = self.time_formatter(localiser, &data_locale, length_time)?;
                            let date_string = tf.format_to_string(time);
                            string.push_str(date_string.as_str());
                        }
                        PlaceholderValue::String(value) => {
                            let date_time_strings: Vec<&str> = value.split('T').collect();
                            if date_time_strings.len() == 2 {
                                if date_time_strings[0].is_empty() {
                                    // time only
                                    let time: Time = decompose_iso_time(date_time_strings[1])?;
                                    let tf =
                                        self.time_formatter(localiser, &data_locale, length_time)?;
                                    let date_string = tf.format_to_string(&time);
                                    string.push_str(date_string.as_str());
                                } else {
                                    // date and time
                                    let date: Date<Iso> = decompose_iso_date(date_time_strings[0])?;
                                    let time: Time = decompose_iso_time(date_time_strings[1])?;
                                    let date_time = DateTime::<Iso>::new(date, time);
                                    let dtf =
                                        self.date_time_formatter(localiser, &data_locale, options)?;
                                    let date_string = dtf.format_to_string(&date_time.to_any())?;
                                    string.push_str(date_string.as_str());
                                }
                            } else {
                                // date only
                                let date: Date<Iso> = decompose_iso_date(date_time_strings[0])?;
                                let df =
                                    self.date_formatter(localiser, &data_locale, length_date)?;
                                let date_string = df.format_to_string(&date.to_any())?;
                                string.push_str(date_string.as_str());
                            }
                        }
                        _ => {
                            return Err(FormatterError::InvalidValue("PatternDateTime".to_string()))
                        }
                    }
                }
                PatternPart::PatternComplex {
                    placeholder,
                    complex,
                    selectors,
                } => {
                    #[cfg(feature = "log")]
                    trace!("Formatting PatternPart::PatternComplex");

                    let Some(value) = values.get(placeholder) else {
                        return Err(FormatterError::PlaceholderValue(
                            "PatternComplex".to_string(),
                            placeholder.to_string(),
                        ));
                    };
                    let selectors_index = selectors;
                    let data_locale = DataLocale::from(RefCount::as_ref(&self.locale));
                    match complex {
                        ComplexType::Plural => {
                            let plurals = self.plural_rules_cardinal(localiser, &data_locale)?;
                            match value {
                                PlaceholderValue::FixedDecimal(number) => self.find_number_sign(
                                    localiser,
                                    values,
                                    &mut string,
                                    number,
                                    plurals,
                                    &data_locale,
                                    *selectors_index,
                                )?,
                                PlaceholderValue::Unsigned(number) => {
                                    let fixed_decimal = FixedDecimal::from(*number);
                                    self.find_number_sign(
                                        localiser,
                                        values,
                                        &mut string,
                                        &fixed_decimal,
                                        plurals,
                                        &data_locale,
                                        *selectors_index,
                                    )?;
                                }
                                PlaceholderValue::Integer(number) => {
                                    let fixed_decimal = FixedDecimal::from(*number);
                                    self.find_number_sign(
                                        localiser,
                                        values,
                                        &mut string,
                                        &fixed_decimal,
                                        plurals,
                                        &data_locale,
                                        *selectors_index,
                                    )?;
                                }
                                PlaceholderValue::Float(number) => {
                                    let fixed_decimal = FixedDecimal::try_from_f64(
                                        *number,
                                        DoublePrecision::Floating,
                                    )?;
                                    self.find_number_sign(
                                        localiser,
                                        values,
                                        &mut string,
                                        &fixed_decimal,
                                        plurals,
                                        &data_locale,
                                        *selectors_index,
                                    )?;
                                }
                                _ => {
                                    return Err(FormatterError::InvalidValue(
                                        "PatternPlural".to_string(),
                                    ))
                                }
                            }
                        }
                        ComplexType::Ordinal => {
                            // Only positive integers and zero are allowed.
                            let plurals = self.plural_rules_ordinal(localiser, &data_locale)?;
                            match value {
                                PlaceholderValue::Unsigned(number) => {
                                    let fixed_decimal = FixedDecimal::from(*number);
                                    self.find_number_sign(
                                        localiser,
                                        values,
                                        &mut string,
                                        &fixed_decimal,
                                        plurals,
                                        &data_locale,
                                        *selectors_index,
                                    )?;
                                }
                                _ => {
                                    return Err(FormatterError::InvalidValue(
                                        "PatternOrdinal".to_string(),
                                    ))
                                }
                            }
                        }
                        ComplexType::Select => {
                            match value {
                                PlaceholderValue::String(value) => {
                                    self.select(
                                        localiser,
                                        values,
                                        &mut string,
                                        value,
                                        *selectors_index,
                                    )?;
                                }
                                PlaceholderValue::TaggedString(value) => {
                                    // Locale is not used, and LSring is just treated as String for the selector
                                    self.select(
                                        localiser,
                                        values,
                                        &mut string,
                                        &value.as_str().to_string(),
                                        *selectors_index,
                                    )?;
                                }
                                _ => {
                                    return Err(FormatterError::InvalidValue(
                                        "PatternSelect".to_string(),
                                    ))
                                }
                            }
                        }
                    }
                }
                PatternPart::NumberSign(index) => {
                    #[cfg(feature = "log")]
                    trace!("Formatting PatternPart::NumberSign");

                    let Some(number_string) = self.numbers.get(*index) else {
                        return Err(FormatterError::NumberSignString(*index));
                    };
                    string.push_str(number_string.as_str());
                }
                PatternPart::Command { strings } => {
                    #[cfg(feature = "log")]
                    trace!("Formatting PatternPart::Command");

                    let mut parameters = Vec::<PlaceholderValue>::new();
                    let mut iterator = strings.iter();

                    // First string is always command identifier.
                    let first = iterator.next().unwrap();
                    let command = match first {
                        PlaceholderValue::String(string) => string,

                        // Never reached. Always PlaceholderValue::String.
                        _ => return Err(FormatterError::NeverReached),
                    };
                    parameters.push(first.clone());

                    // If parameter is same as placeholder, take the placeholder value instead.
                    for parameter in iterator {
                        let string = match parameter {
                            PlaceholderValue::String(string) => string,

                            // Never reached. Always PlaceholderValue::String.
                            _ => return Err(FormatterError::NeverReached),
                        };
                        if let Some(value) = values.get(string) {
                            parameters.push(value.clone());
                        } else {
                            parameters.push(parameter.clone());
                        }
                    }
                    let function = localiser.command_registry().command(command)?;
                    string.push_str(&function(parameters)?);
                }
            }
            i += 1;
        }
        Ok(string)
    }

    #[allow(clippy::too_many_arguments)]
    fn find_number_sign(
        &mut self,
        localiser: &Localiser,
        values: &HashMap<String, PlaceholderValue>,
        string: &mut String,
        fixed_decimal: &FixedDecimal,
        plurals: PluralRules,
        data_locale: &DataLocale,
        selectors_index: usize,
    ) -> Result<(), FormatterError> {
        let mut _named = String::new();

        // Format number using graphemes of the locale.
        let fdf = self.fixed_decimal_formatter(localiser, data_locale, Default::default())?;
        let number_string = fdf.format(fixed_decimal).to_string();
        let category = plural_category(plurals.category_for(fixed_decimal)).to_string();

        // Get the named string, and locate number signs to update the string.
        {
            let Some(selectors) = self.selectors.get(selectors_index) else {
                return Err(FormatterError::SelectorsIndex(selectors_index));
            };
            let Some(named) = selectors.get(&category) else {
                return Err(FormatterError::SelectorsIndexNamed(
                    category,
                    selectors_index,
                ));
            };
            _named = named.to_string();
        }
        {
            let mut _len = 0usize;
            {
                let Some(pattern) = self.patterns.get(&_named) else {
                    return Err(FormatterError::PatternNamed(_named));
                };
                _len = pattern.len();
            }
            let mut i = 0usize;
            while i < _len {
                let mut _part: Option<&PatternPart> = None;
                {
                    let Some(part) = self.part_ref(&_named, i) else {
                        return Err(FormatterError::PatternPart(_named, i));
                    };
                    _part = Some(part);
                }
                if let PatternPart::NumberSign(index) = _part.unwrap() {
                    self.update_number_part(*index, &number_string)?
                }
                i += 1;
            }
        }
        let part_string = self.format_pattern(localiser, values, &_named)?;
        string.push_str(part_string.as_str());
        Ok(())
    }

    fn select(
        &mut self,
        localiser: &Localiser,
        values: &HashMap<String, PlaceholderValue>,
        string: &mut String,
        string_value: &String,
        selectors_index: usize,
    ) -> Result<(), FormatterError> {
        // Get the named string, and locate number signs to update the string.
        let Some(selectors) = self.selectors.get(selectors_index) else {
            return Err(FormatterError::SelectorsIndex(selectors_index));
        };
        let Some(named) = selectors.get(string_value) else {
            return Err(FormatterError::SelectorsIndexNamed(
                string_value.to_string(),
                selectors_index,
            ));
        };
        let part_string = self.format_pattern(localiser, values, &named.to_string())?;
        string.push_str(part_string.as_str());
        Ok(())
    }

    fn update_number_part(
        &mut self,
        index: usize,
        number_string: &String,
    ) -> Result<(), FormatterError> {
        let Some(number_string_mut) = self.numbers.get_mut(index) else {
            return Err(FormatterError::NumberSignString(index));
        };
        *number_string_mut = number_string.to_string();
        Ok(())
    }

    fn fixed_decimal_formatter(
        &self,
        localiser: &Localiser,
        _data_locale: &DataLocale,
        _options: options::FixedDecimalFormatterOptions,
    ) -> Result<FixedDecimalFormatter, FormatterError> {
        match localiser.icu_data_provider().data_provider() {
            #[cfg(feature = "icu_compiled_data")]
            DataProvider::Internal => Ok(FixedDecimalFormatter::try_new(_data_locale, _options)?),

            #[cfg(feature = "blob")]
            DataProvider::Blob(provider) => {
                Ok(FixedDecimalFormatter::try_new_with_buffer_provider(
                    provider,
                    _data_locale,
                    _options,
                )?)
            }

            #[cfg(feature = "fs")]
            DataProvider::Fs(provider) => Ok(FixedDecimalFormatter::try_new_with_buffer_provider(
                provider,
                _data_locale,
                _options,
            )?),

            #[allow(unreachable_patterns)]
            _ => Err(FormatterError::NoIcuProvider),
        }
    }

    fn date_time_formatter(
        &self,
        localiser: &Localiser,
        _data_locale: &DataLocale,
        _options: Bag,
    ) -> Result<DateTimeFormatter, FormatterError> {
        match localiser.icu_data_provider().data_provider() {
            #[cfg(feature = "icu_compiled_data")]
            DataProvider::Internal => {
                Ok(DateTimeFormatter::try_new(_data_locale, _options.into())?)
            }

            #[cfg(feature = "blob")]
            DataProvider::Blob(provider) => Ok(DateTimeFormatter::try_new_with_buffer_provider(
                provider,
                _data_locale,
                _options.into(),
            )?),

            #[cfg(feature = "fs")]
            DataProvider::Fs(provider) => Ok(DateTimeFormatter::try_new_with_buffer_provider(
                provider,
                _data_locale,
                _options.into(),
            )?),

            #[allow(unreachable_patterns)]
            _ => Err(FormatterError::NoIcuProvider),
        }
    }

    fn date_formatter(
        &self,
        localiser: &Localiser,
        _data_locale: &DataLocale,
        _length_date: icu_datetime::options::length::Date,
    ) -> Result<DateFormatter, FormatterError> {
        match localiser.icu_data_provider().data_provider() {
            #[cfg(feature = "icu_compiled_data")]
            DataProvider::Internal => Ok(DateFormatter::try_new_with_length(
                _data_locale,
                _length_date,
            )?),

            #[cfg(feature = "blob")]
            DataProvider::Blob(provider) => {
                Ok(DateFormatter::try_new_with_length_with_buffer_provider(
                    provider,
                    _data_locale,
                    _length_date,
                )?)
            }

            #[cfg(feature = "fs")]
            DataProvider::Fs(provider) => {
                Ok(DateFormatter::try_new_with_length_with_buffer_provider(
                    provider,
                    _data_locale,
                    _length_date,
                )?)
            }

            #[allow(unreachable_patterns)]
            _ => Err(FormatterError::NoIcuProvider),
        }
    }

    fn time_formatter(
        &self,
        localiser: &Localiser,
        _data_locale: &DataLocale,
        _length_time: icu_datetime::options::length::Time,
    ) -> Result<TimeFormatter, FormatterError> {
        match localiser.icu_data_provider().data_provider() {
            #[cfg(feature = "icu_compiled_data")]
            DataProvider::Internal => Ok(TimeFormatter::try_new_with_length(
                _data_locale,
                _length_time,
            )?),

            #[cfg(feature = "blob")]
            DataProvider::Blob(provider) => {
                Ok(TimeFormatter::try_new_with_length_with_buffer_provider(
                    provider,
                    _data_locale,
                    _length_time,
                )?)
            }

            #[cfg(feature = "fs")]
            DataProvider::Fs(provider) => {
                Ok(TimeFormatter::try_new_with_length_with_buffer_provider(
                    provider,
                    _data_locale,
                    _length_time,
                )?)
            }

            #[allow(unreachable_patterns)]
            _ => Err(FormatterError::NoIcuProvider),
        }
    }

    fn plural_rules_cardinal(
        &self,
        localiser: &Localiser,
        _data_locale: &DataLocale,
    ) -> Result<PluralRules, FormatterError> {
        match localiser.icu_data_provider().data_provider() {
            #[cfg(feature = "icu_compiled_data")]
            DataProvider::Internal => Ok(PluralRules::try_new_cardinal(_data_locale)?),

            #[cfg(feature = "blob")]
            DataProvider::Blob(provider) => Ok(PluralRules::try_new_cardinal_with_buffer_provider(
                provider,
                _data_locale,
            )?),

            #[cfg(feature = "fs")]
            DataProvider::Fs(provider) => Ok(PluralRules::try_new_cardinal_with_buffer_provider(
                provider,
                _data_locale,
            )?),

            #[allow(unreachable_patterns)]
            _ => Err(FormatterError::NoIcuProvider),
        }
    }

    fn plural_rules_ordinal(
        &self,
        localiser: &Localiser,
        _data_locale: &DataLocale,
    ) -> Result<PluralRules, FormatterError> {
        match localiser.icu_data_provider().data_provider() {
            #[cfg(feature = "icu_compiled_data")]
            DataProvider::Internal => Ok(PluralRules::try_new_ordinal(_data_locale)?),

            #[cfg(feature = "blob")]
            DataProvider::Blob(provider) => Ok(PluralRules::try_new_ordinal_with_buffer_provider(
                provider,
                _data_locale,
            )?),

            #[cfg(feature = "fs")]
            DataProvider::Fs(provider) => Ok(PluralRules::try_new_ordinal_with_buffer_provider(
                provider,
                _data_locale,
            )?),

            #[allow(unreachable_patterns)]
            _ => Err(FormatterError::NoIcuProvider),
        }
    }
}

/// Decomposes an ISO 8601 date string into a [`Date`]`<`[`Iso`]`>` struct.
///
/// Supported ISO 8601 extended and basic formats:
///   YYYY-MM-DD or YYYYMMDD
///   YYYY-MM
///   YYYY
/// where:
///   YYYY refers to a zero-padded year, range of 0000 to 9999 (1 BC to 9999 AD).
///   MM refers to a zero-padded month, range of 01 to 12, and defaults to 01 when not present.
///   DD refers to a zero-padded day, range of 01 to 31, and defaults to 01 when not present.
///
/// Currently ISO 8601 does not allow the YYYYMM format due to historical data using the obsolete YYMMDD format.
///
/// Supported ISO 8601 expanded formats:
///   -YYYY-MM-DD
///   -YYYY-MM
///   -YYYY
///   ±YYYYY-MM-DD
///   ±YYYYY-MM
///   ±YYYYY
/// where:
///   ± refers to either + or -.
///   YYYY refers to zero-padded year with - prefix, range of -0001 to -9999 (2 BC to 10000 BC).
///   YYYYY refers to 5 or more digits with either + or - prefix, range before -9999 (10000 BC) or after 9999.
///   MM refers to a zero-padded month, range of 01 to 12, and defaults to 01 when not present.
///   DD refers to a zero-padded day, range of 01 to 31, and defaults to 01 when not present.
///
/// ICU4X `Date` supports the year to be in the range of `-2_147_483_648` to `2_147_483_647`.
///
/// ISO 8601 _Week_ and _Ordinal date_ formats are not supported as there are currently no methods available for
/// ICU4X `Date<Iso>` for creating structs using the week number or the ordinal day of the year.
pub fn decompose_iso_date(string: &str) -> Result<Date<Iso>, FormatterError> {
    #[cfg(feature = "log")]
    trace!("Decompose ISO date string into ICU Date<Iso> instance.");

    let no_plus = string.trim_start_matches('+');
    let mut year: i32 = 0;
    let mut month: u8 = 1;
    let mut day: u8 = 1;
    if no_plus.is_empty() {
        // no year
        let date = Date::try_new_iso_date(year, month, day)?;
        return Ok(date);
    }
    let parts: Vec<&str> = no_plus.split('-').collect();
    if parts.len() == 1 {
        // YYYYMMDD or YYYY. ISO 8601 may re-add YYYYMM in the future, once most historical data has been
        // converted from YYMMDD.
        if let Some(slice) = parts[0].get(..4) {
            year = i32::from_str(slice)?;

            // try month
            if let Some(slice) = parts[0].get(4..6) {
                month = u8::from_str(slice)?;

                // try day
                if let Some(slice) = parts[0].get(6..8) {
                    day = u8::from_str(slice)?;
                }
            }
        }
    } else {
        let mut index = 0usize;
        let mut year_string = String::new();
        if parts[0].is_empty() {
            // negative dates
            year_string.push('-');
            index = 1;
        }
        year_string.push_str(parts[index]);
        year = i32::from_str(&year_string)?;

        // try month
        index += 1;
        if let Some(slice) = parts.get(index) {
            month = u8::from_str(slice)?;

            // try day
            index += 1;
            if let Some(slice) = parts.get(index) {
                day = u8::from_str(slice)?;
            }
        }
    }
    let result = Date::try_new_iso_date(year, month, day)?;
    Ok(result)
}

/// Decomposes an ISO time string into a [`Time`] struct.
///
/// Supported ISO 8601 extended and basic formats:
///   Thh:mm:ss.nnn or Thhmmss.nnn
///   Thh:mm:ss or Thhmmss
///   Thh:mm or Thhmm
///   Thh
///   T
/// where:
///   T refers to the time separator from date. Even though required by ISO 8601, it is optional here.
///   hh refers to a zero-padded hour, range of 00 to 24 (24 used to represent last instance of the day 24:00:00).
///   mm refers to a zero-padded minute, range of 00 to 59.
///   ss refers to a zero-padded second, range of 00 to 60 (where 60 is only used to denote an added leap second).
///   nnn refers to a nanosecond, range of 000 to 999.
///
/// All time components not included are treated as zero.
///
/// Time zones are not supported by ICU4X `Time`, thus will be ignored.
/// - time zones ( Z (for UCT 00:00), +hh:mm, -hh:mm, +hhmm, -hhmm ).
///   -00:00 or -0000 are not supported by ISO 8601.
pub fn decompose_iso_time(string: &str) -> Result<Time, FormatterError> {
    #[cfg(feature = "log")]
    trace!("Decompose ISO time string into ICU Time instance.");

    let no_t = string.trim_start_matches('T');
    let no_plus = match no_t.find('+') {
        None => no_t,
        Some(pos) => &no_t[..pos],
    };
    let trimmed = match no_plus.find('-') {
        None => no_plus,
        Some(pos) => &no_plus[..pos],
    };
    if trimmed.is_empty() {
        return Ok(Time::new(
            IsoHour::zero(),
            IsoMinute::zero(),
            IsoSecond::zero(),
            NanoSecond::zero(),
        ));
    }
    let mut hour = IsoHour::zero();
    let mut minute = IsoMinute::zero();
    let mut second = IsoSecond::zero();
    let mut nano = NanoSecond::zero();
    if let Some(slice) = trimmed.get(..2) {
        hour = IsoHour::from_str(slice)?;

        // try minute
        let mut no_hour = &trimmed[2..];
        no_hour = no_hour.trim_start_matches(':');
        if let Some(slice) = no_hour.get(..2) {
            minute = IsoMinute::from_str(slice)?;

            // try second
            let mut no_minute = &no_hour[2..];
            no_minute = no_minute.trim_start_matches(':');
            if let Some(slice) = no_minute.get(..2) {
                second = IsoSecond::from_str(slice)?;

                // try nanosecond
                let mut no_second = &no_minute[2..];
                no_second = no_second.trim_start_matches('.');
                nano = NanoSecond::from_str(no_second)?;
            }
        }
    }
    Ok(Time::new(hour, minute, second, nano))
}

// Internal structures, enums, etc.

// Check node type.
fn check_node_type(tree: &Tree, index: usize, node_type: NodeType) -> bool {
    let Ok(node_type_data) = tree.node_type(index) else {
        return false;
    };
    let Some(node_type2) = node_type_data.as_ref().unwrap().downcast_ref::<NodeType>() else {
        return false;
    };
    if node_type != *node_type2 {
        return false;
    }
    true
}

fn part_text(
    pattern: &mut Vec<PatternPart>,
    tree: &Tree,
    index: usize,
) -> Result<(), FormatterError> {
    #[cfg(feature = "log")]
    trace!("Processing text node.");

    let mut string = String::new();
    let Ok(text_data) = tree.data_ref(index) else {
        return Err(FormatterError::RetrieveNodeData(NodeType::Text));
    };
    for token_data in text_data.iter() {
        let Some(token) = token_data.downcast_ref::<RefCount<Token>>() else {
            return Err(FormatterError::RetrieveNodeToken(NodeType::Text));
        };
        string.push_str(token.string.as_str());
    }
    pattern.push(PatternPart::Text(string));
    Ok(())
}

fn part_pattern(
    pattern: &mut Vec<PatternPart>,
    patterns: &HashMap<String, Vec<PatternPart>>,
    tree: &Tree,
    index: usize,
    selectors: &mut Vec<HashMap<String, String>>,
    option_selectors: &OptionSelectors,
    locale: &RefCount<Locale>,
) -> Result<(), FormatterError> {
    #[cfg(feature = "log")]
    trace!("Processing pattern node.");

    let Ok(children) = tree.children(index) else {
        return Err(FormatterError::RetrieveChildren(NodeType::Pattern));
    };
    let mut iterator = children.iter();

    // Identifier - first node
    let Some(placeholder) = iterator.next() else {
        return Err(FormatterError::NoChildren(NodeType::Pattern));
    };
    if !check_node_type(tree, *placeholder, NodeType::Identifier) {
        return Err(FormatterError::NodeNotFound(NodeType::Identifier));
    }
    let Ok(placeholder_data) = tree.data_ref(*placeholder) else {
        return Err(FormatterError::RetrieveNodeData(NodeType::Identifier));
    };
    let Some(placeholder_token) = placeholder_data
        .first()
        .unwrap()
        .downcast_ref::<RefCount<Token>>()
    else {
        return Err(FormatterError::RetrieveNodeToken(NodeType::Identifier));
    };

    // Keyword - second node
    let keyword = match iterator.next() {
        None => {
            // placeholder with no parameters - defaults to string value
            pattern.push(PatternPart::PatternString(
                placeholder_token.string.to_string(),
            ));
            return Ok(());
        }
        Some(keyword) => *keyword,
    };
    if !check_node_type(tree, keyword, NodeType::Identifier) {
        return Err(FormatterError::NodeNotFound(NodeType::Identifier));
    }
    let Ok(keyword_data) = tree.data_ref(keyword) else {
        return Err(FormatterError::RetrieveNodeData(NodeType::Identifier));
    };
    let Some(keyword_token) = keyword_data
        .first()
        .unwrap()
        .downcast_ref::<RefCount<Token>>()
    else {
        return Err(FormatterError::RetrieveNodeToken(NodeType::Identifier));
    };

    // Options and selectors for keywords.
    // TODO: add more options as they become non-experimental.
    if keyword_token.string.as_str() == "decimal" {
        // Currently the option can be repeated, though only final value is used.
        let strings = pattern_selectors(tree, index)?;
        let mut sign: Option<SignDisplay> = None;
        let mut group: Option<options::GroupingStrategy> = None;
        for (key, value) in strings.iter() {
            if key.as_str() == "sign" {
                sign = Some(sign_display(value.as_str())?);
            } else if key.as_str() == "group" {
                group = Some(decimal_grouping_display(value.as_str())?);
            } else {
                return Err(FormatterError::InvalidOption(
                    value.as_str().to_string(),
                    "decimal".to_string(),
                    placeholder_token.string.as_str().to_string(),
                ));
            }
        }
        pattern.push(PatternPart::PatternDecimal {
            placeholder: placeholder_token.string.to_string(),
            sign,
            group,
        });
    } else if keyword_token.string.as_str() == "date_time" {
        // Currently the option can be repeated, though only final value is used.
        let strings = pattern_selectors(tree, index)?;
        let mut length_date: Option<DateLength> = None;
        let mut length_time: Option<TimeLength> = None;
        let mut calendar: Option<Locale> = None;
        for (key, value) in strings.iter() {
            if key.as_str() == "date" {
                length_date = Some(date_length(value.as_str())?);
            } else if key.as_str() == "time" {
                length_time = Some(time_length(value.as_str())?);
            } else if key.as_str() == "calendar" {
                if !option_selectors.calendars.contains(&value.as_str()) {
                    return Err(FormatterError::InvalidOption(
                        value.as_str().to_string(),
                        "ordinal".to_string(),
                        placeholder_token.string.as_str().to_string(),
                    ));
                }
                let mut new_calendar = "-u-ca-".to_string();
                new_calendar.push_str(value.as_str());
                let mut locale_string = locale.to_string();
                match locale_string.find("-u-ca-") {
                    None => {
                        locale_string.push_str(new_calendar.as_str());
                    }
                    Some(tag_position) => {
                        let mut hyphens = locale_string.match_indices('-');
                        while let Some(pair) = hyphens.next() {
                            if pair.0 == tag_position {
                                hyphens.next(); // There is an experimental advance_by( 2 ) to replace two next()
                                hyphens.next();
                                match hyphens.next() {
                                    None => {
                                        // The calendar tag is last tag
                                        let old_calendar =
                                            locale_string.get(tag_position..).unwrap();
                                        locale_string = locale_string
                                            .replace(old_calendar, new_calendar.as_str());
                                    }
                                    Some(end) => {
                                        new_calendar.push('-');
                                        let old_calendar =
                                            locale_string.get(tag_position..=end.0).unwrap();
                                        locale_string = locale_string
                                            .replace(old_calendar, new_calendar.as_str());
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
                let calendar_locale = locale_string.parse()?;
                calendar = Some(calendar_locale);
            } else {
                return Err(FormatterError::InvalidOption(
                    value.as_str().to_string(),
                    "date-time".to_string(),
                    placeholder_token.string.as_str().to_string(),
                ));
            }
        }
        pattern.push(PatternPart::PatternDateTime {
            placeholder: placeholder_token.string.to_string(),
            length_date,
            length_time,
            calendar,
        });
    } else if keyword_token.string.as_str() == "ordinal" {
        let strings = pattern_selectors(tree, index)?;
        let mut other = false;
        for (selector, named) in strings.iter() {
            if !option_selectors.valid_plurals.contains(&selector.as_str()) {
                return Err(FormatterError::InvalidSelector(
                    selector.as_str().to_string(),
                    "ordinal".to_string(),
                    placeholder_token.string.as_str().to_string(),
                ));
            }
            if !patterns.contains_key(named) {
                return Err(FormatterError::SelectorNamed(
                    named.as_str().to_string(),
                    selector.as_str().to_string(),
                    placeholder_token.string.as_str().to_string(),
                ));
            }
            if selector.as_str() == "other" {
                other = true;
            }
        }
        if !other {
            return Err(FormatterError::SelectorOther(
                "ordinal".to_string(),
                placeholder_token.string.as_str().to_string(),
            ));
        }
        let len = selectors.len();
        selectors.push(strings);
        pattern.push(PatternPart::PatternComplex {
            placeholder: placeholder_token.string.to_string(),
            complex: ComplexType::Ordinal,
            selectors: len,
        });
    } else if keyword_token.string.as_str() == "plural" {
        let strings = pattern_selectors(tree, index)?;
        let mut other = false;
        for (selector, named) in strings.iter() {
            if !option_selectors.valid_plurals.contains(&selector.as_str()) {
                return Err(FormatterError::InvalidSelector(
                    selector.as_str().to_string(),
                    "plural".to_string(),
                    placeholder_token.string.as_str().to_string(),
                ));
            }
            if !patterns.contains_key(named) {
                return Err(FormatterError::SelectorNamed(
                    named.as_str().to_string(),
                    selector.as_str().to_string(),
                    placeholder_token.string.as_str().to_string(),
                ));
            }
            if selector.as_str() == "other" {
                other = true;
            }
        }
        if !other {
            return Err(FormatterError::SelectorOther(
                "plural".to_string(),
                placeholder_token.string.as_str().to_string(),
            ));
        }
        let len = selectors.len();
        selectors.push(strings);
        pattern.push(PatternPart::PatternComplex {
            placeholder: placeholder_token.string.to_string(),
            complex: ComplexType::Plural,
            selectors: len,
        });
    } else if keyword_token.string.as_str() == "select" {
        let strings = pattern_selectors(tree, index)?;
        for (selector, named) in strings.iter() {
            if !patterns.contains_key(named) {
                return Err(FormatterError::SelectorNamed(
                    named.as_str().to_string(),
                    selector.as_str().to_string(),
                    placeholder_token.string.as_str().to_string(),
                ));
            }
        }
        let len = selectors.len();
        selectors.push(strings);
        pattern.push(PatternPart::PatternComplex {
            placeholder: placeholder_token.string.to_string(),
            complex: ComplexType::Select,
            selectors: len,
        });
    } else {
        return Err(FormatterError::InvalidKeyword(
            keyword_token.string.as_str().to_string(),
            placeholder_token.string.as_str().to_string(),
        ));
    }
    Ok(())
}

// Commands always returns static text
fn part_command(
    pattern: &mut Vec<PatternPart>,
    tree: &Tree,
    index: usize,
    command_registry: &RefCount<CommandRegistry>,
) -> Result<(), FormatterError> {
    #[cfg(feature = "log")]
    trace!("Processing command node.");

    let mut delay = false;
    let mut parameters = Vec::<PlaceholderValue>::new();
    let Ok(children) = tree.children(index) else {
        return Err(FormatterError::RetrieveChildren(NodeType::Pattern));
    };
    let mut iterator = children.iter();

    // Identifier - first node
    let Some(command) = iterator.next() else {
        return Err(FormatterError::NoChildren(NodeType::Pattern));
    };
    if !check_node_type(tree, *command, NodeType::Identifier) {
        return Err(FormatterError::NodeNotFound(NodeType::Identifier));
    }
    let Ok(command_data) = tree.data_ref(*command) else {
        return Err(FormatterError::RetrieveNodeData(NodeType::Identifier));
    };
    let Some(command_token) = command_data
        .first()
        .unwrap()
        .downcast_ref::<RefCount<Token>>()
    else {
        return Err(FormatterError::RetrieveNodeToken(NodeType::Identifier));
    };
    parameters.push(PlaceholderValue::String(
        command_token.as_ref().string.to_string(),
    ));

    // Check if delay command marker `#` is present.
    //peek ahead so not to interfere with while if not present.
    let mut iterator_peeking = iterator.clone();
    let command_next = iterator_peeking.next();
    if command_next.is_some() && check_node_type(tree, *command_next.unwrap(), NodeType::NumberSign)
    {
        delay = true;
        iterator = iterator_peeking;
    }

    // Rest can be either Identifier or Text nodes.
    for parameter in iterator {
        if check_node_type(tree, *parameter, NodeType::Identifier) {
            let Ok(identifier_data) = tree.data_ref(*parameter) else {
                return Err(FormatterError::RetrieveNodeData(NodeType::Identifier));
            };
            let Some(identifier_token) = identifier_data
                .first()
                .unwrap()
                .downcast_ref::<RefCount<Token>>()
            else {
                return Err(FormatterError::RetrieveNodeToken(NodeType::Identifier));
            };
            parameters.push(PlaceholderValue::String(
                identifier_token.as_ref().string.to_string(),
            ));
        } else if check_node_type(tree, *parameter, NodeType::Text) {
            let mut string = String::new();
            let Ok(text_data) = tree.data_ref(*parameter) else {
                return Err(FormatterError::RetrieveNodeData(NodeType::Text));
            };
            for token_data in text_data.iter() {
                let Some(token) = token_data.downcast_ref::<RefCount<Token>>() else {
                    return Err(FormatterError::RetrieveNodeToken(NodeType::Text));
                };
                string.push_str(token.string.as_str());
            }
            parameters.push(PlaceholderValue::String(string));
        } else {
            return Err(FormatterError::InvalidNode(NodeType::Command));
        }
    }
    if delay {
        pattern.push(PatternPart::Command {
            strings: parameters,
        });
    } else {
        let function = command_registry.command(&command_token.as_ref().string)?;
        pattern.push(PatternPart::Text(function(parameters)?));
    }
    Ok(())
}

fn pattern_selectors(tree: &Tree, index: usize) -> Result<HashMap<String, String>, FormatterError> {
    #[cfg(feature = "log")]
    trace!("Processing pattern selectors.");

    // Work around for inability to pass iterators, thus the iterator needs to be recreated.
    let Ok(children) = tree.children(index) else {
        panic!("Failed to recreate children iterator.");
    };
    let iterator = children.iter().skip(2);
    let mut pairs = HashMap::<String, String>::new();
    for selector in iterator {
        if !check_node_type(tree, *selector, NodeType::Selector) {
            return Err(FormatterError::NodeNotFound(NodeType::Selector));
        }
        let Ok(first) = tree.first(*selector) else {
            return Err(FormatterError::FirstChild(NodeType::Selector));
        };
        if !check_node_type(tree, first, NodeType::Identifier) {
            return Err(FormatterError::NodeNotFound(NodeType::Identifier));
        }
        let Ok(first_data) = tree.data_ref(first) else {
            return Err(FormatterError::RetrieveNodeData(NodeType::Identifier));
        };
        let Some(first_token) = first_data
            .first()
            .unwrap()
            .downcast_ref::<RefCount<Token>>()
        else {
            return Err(FormatterError::RetrieveNodeToken(NodeType::Identifier));
        };
        let Ok(last) = tree.last(*selector) else {
            return Err(FormatterError::LastChild(NodeType::Selector));
        };
        if !check_node_type(tree, last, NodeType::Identifier) {
            return Err(FormatterError::NodeNotFound(NodeType::Identifier));
        }
        let Ok(last_data) = tree.data_ref(last) else {
            return Err(FormatterError::RetrieveNodeData(NodeType::Identifier));
        };
        let Some(last_token) = last_data.first().unwrap().downcast_ref::<RefCount<Token>>() else {
            return Err(FormatterError::RetrieveNodeToken(NodeType::Identifier));
        };
        pairs.insert(
            first_token.string.to_string(),
            last_token.string.to_string(),
        );
    }
    Ok(pairs)
}

fn plural_category(category: PluralCategory) -> &'static str {
    match category {
        PluralCategory::Zero => "zero",
        PluralCategory::One => "one",
        PluralCategory::Two => "two",
        PluralCategory::Few => "few",
        PluralCategory::Many => "many",
        PluralCategory::Other => "other",
    }
}

fn sign_display(sign: &str) -> Result<SignDisplay, FormatterError> {
    match sign {
        "auto" => Ok(SignDisplay::Auto),
        "never" => Ok(SignDisplay::Never),
        "always" => Ok(SignDisplay::Always),
        "except_zero" => Ok(SignDisplay::ExceptZero),
        "negative" => Ok(SignDisplay::Negative),
        _ => Err(FormatterError::InvalidOptionValue(
            sign.to_string(),
            "sign".to_string(),
            "decimal".to_string(),
        )),
    }
}

fn decimal_grouping_display(group: &str) -> Result<options::GroupingStrategy, FormatterError> {
    match group {
        "auto" => Ok(options::GroupingStrategy::Auto),
        "never" => Ok(options::GroupingStrategy::Never),
        "always" => Ok(options::GroupingStrategy::Always),
        "min2" => Ok(options::GroupingStrategy::Min2),
        _ => Err(FormatterError::InvalidOptionValue(
            group.to_string(),
            "group".to_string(),
            "decimal".to_string(),
        )),
    }
}

fn date_length(len: &str) -> Result<DateLength, FormatterError> {
    match len {
        "full" => Ok(DateLength::Full),
        "long" => Ok(DateLength::Long),
        "medium" => Ok(DateLength::Medium),
        "short" => Ok(DateLength::Short),
        _ => Err(FormatterError::InvalidOptionValue(
            len.to_string(),
            "date".to_string(),
            "date_time".to_string(),
        )),
    }
}

fn time_length(len: &str) -> Result<TimeLength, FormatterError> {
    match len {
        "full" => Ok(TimeLength::Full),
        "long" => Ok(TimeLength::Long),
        "medium" => Ok(TimeLength::Medium),
        "short" => Ok(TimeLength::Short),
        _ => Err(FormatterError::InvalidOptionValue(
            len.to_string(),
            "time".to_string(),
            "date_time".to_string(),
        )),
    }
}

struct OptionSelectors<'a> {
    valid_plurals: Vec<&'a str>,
    calendars: Vec<&'a str>,
}

enum ComplexType {
    Plural,
    Ordinal,
    Select,
}

enum PatternPart {
    Text(String),
    NumberSign(usize),
    PatternString(String),
    PatternDecimal {
        placeholder: String,
        sign: Option<SignDisplay>,
        group: Option<options::GroupingStrategy>,
    },
    PatternDateTime {
        placeholder: String,
        length_date: Option<DateLength>,
        length_time: Option<TimeLength>,
        calendar: Option<Locale>,
    },
    PatternComplex {
        placeholder: String,
        complex: ComplexType,
        selectors: usize,
    },
    Command {
        strings: Vec<PlaceholderValue>,
    },
}
