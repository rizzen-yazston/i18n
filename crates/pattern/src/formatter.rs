// This file is part of `i18n_pattern-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_pattern-rizzen-yazston` crate.

//! TODO: Complete crate description
//!
//! # Examples
//!
//! ```
//! // TODO: crate example
//! ```

use crate::*;
use i18n_lexer::Token;
use i18n_lstring::LString;
use icu_provider::prelude::*;
use icu_locid::Locale;
use icu_plurals::{PluralCategory, PluralRules};
use icu_decimal::{FixedDecimalFormatter, options};
use fixed_decimal::{FixedDecimal, DoublePrecision, SignDisplay};
use icu_calendar::DateTime;
use icu_datetime::{options::length, DateTimeFormatter, DateFormatter, TimeFormatter};
use std::collections::HashMap;
use std::rc::Rc;
use core::any::Any;
use tree::Tree;

pub struct Formatter {
    locale: Rc<Locale>,
    patterns: HashMap<String, Vec<Box<dyn Marker>>>,
    numbers: Vec<String>,
    selectors: Vec<HashMap<String, String>>,
    valid_plurals: Vec<String>,
}

impl Formatter {
    /// Creates a Formatter for a language string using parsing results.
    // maybe only ParserResult.tree is needed.
    pub fn try_new(
        locale: &Rc<Locale>,
        parser_result: ParserResult
    ) -> Result<Self, String> {
        let mut patterns = HashMap::<String, Vec<Box<dyn Marker>>>::new();
        let mut numbers = Vec::<String>::new();
        let mut selectors = Vec::<HashMap<String, String>>::new();
        let valid_plurals = vec![
            "zero".to_string(),
            "one".to_string(),
            "two".to_string(),
            "few".to_string(),
            "many".to_string(),
            "other".to_string()
        ];

        // Just temporary entry until named strings has been processed.
        patterns.insert( "_".to_string(), Vec::<Box<dyn Marker>>::new() );

        if !check_node_type( &parser_result.tree, 0, NodeType::Root ) {
            return Err( "Tree root is not a Root node.".to_string() );
        }

        // Process substrings first.
        let Ok( last ) = parser_result.tree.last( 0 ) else {
            return Err( "Last child of Root node not found.".to_string() );
        };
        if check_node_type( &parser_result.tree, last, NodeType::NamedGroup ) {
            let Ok( named_strings ) = parser_result.tree.children( last ) else {
                return Err( "Could not retrieve children for NamedGroup.".to_string() );
            };
            for named in named_strings.iter() {
                let mut pattern = Vec::<Box<dyn Marker>>::new();
                if !check_node_type( &parser_result.tree, *named, NodeType::NamedString ) {
                    return Err( "NamedString node was not found.".to_string() );
                }

                // Get NamedString identifier and check it is not already present.
                let Ok( first ) = parser_result.tree.first( *named ) else {
                    return Err( "First child of NamedString node not found.".to_string() );
                };
                if !check_node_type( &parser_result.tree, first, NodeType::Identifier ) {
                    return Err( "Identifier node was not found.".to_string() );
                }
                let Ok(identifier_data ) = parser_result.tree.data_ref( first ) else {
                    return Err( "Could not retrieve data for Identifier node.".to_string() );
                };
                let Some( identifier_token ) = identifier_data.first().unwrap().downcast_ref::<Rc<Token>>() else {
                    return Err( "Failed to retrieve token for Identifier node.".to_string() ) ;
                };
                if patterns.contains_key( identifier_token.string.as_str() ) {
                    return Err( "NamedString identifiers must be unique and not `_`.".to_string() );
                }

                // Got NamedString identifier
                let Ok( last ) = parser_result.tree.last( *named ) else {
                    return Err( "Last child of NamedString node not found.".to_string() );
                };
                if !check_node_type( &parser_result.tree, last, NodeType::String ) {
                    return Err("String node was not found.".to_string());
                }
                let Ok( children ) = parser_result.tree.children( last ) else {
                    return Err( "Could not retrieve children for String.".to_string() );
                };
                for child in children.iter() {
                    if check_node_type( &parser_result.tree, *child, NodeType::Text ) {
                        part_text( &mut pattern, &parser_result, *child )?;
                    } else if check_node_type( &parser_result.tree, *child, NodeType::NumberSign ) {
                        let len = numbers.len();
                        numbers.push( String::new() );
                        pattern.push( Box::new(
                            NumberSign { index: len }
                        ) );
                    } else if check_node_type( &parser_result.tree, *child, NodeType::Pattern ) {
                        part_pattern(
                            &mut pattern, 
                            &patterns, 
                            &parser_result,
                            *child,
                            &mut selectors,
                            &valid_plurals,
                        )?;
                    } else if check_node_type( &parser_result.tree, *child, NodeType::Command ) {
                        // TODO: convert to a method as both NamedString and main String has Text.
                    } else {
                        return Err( "Invalid node found in String.".to_string() );
                    }
                }
                patterns.insert( identifier_token.string.as_str().to_string(), pattern );
            }
        }

        // Now process main string.
        let mut pattern = Vec::<Box<dyn Marker>>::new();
        let Ok( first ) = parser_result.tree.first( 0 ) else {
            return Err( "First child of Root node not found.".to_string() );
        };
        if !check_node_type(&parser_result.tree, first, NodeType::String) {
            return Err( "String node was not found.".to_string() );
        }
        let Ok( children ) = parser_result.tree.children( first ) else {
            return Err( "Could not retrieve children for String.".to_string() );
        };
        for child in children.iter() {
            if check_node_type( &parser_result.tree, *child, NodeType::Text ) {
                part_text( &mut pattern, &parser_result, *child )?;
            } else if check_node_type( &parser_result.tree, *child, NodeType::Pattern ) {
                part_pattern(
                    &mut pattern, 
                    &patterns, 
                    &parser_result,
                    *child,
                    &mut selectors,
                    &valid_plurals,
                )?;
            } else if check_node_type( &parser_result.tree, *child, NodeType::Command ) {
                // TODO: convert to a method as both NamedString and main String has Text.
            } else {
                return Err( "Invalid node found in String.".to_string() );
            }
        }
        patterns.insert( "_".to_string(), pattern );
        Ok( Formatter {
            locale: Rc::clone( &locale ),
            patterns,
            numbers,
            selectors,
            valid_plurals,
        } )
    }

    /// Format the language string with supplied values.
    /// Possible have options to handle nested language strings to wrap inner language string in language tags.
    pub fn format(
        &mut self,
        buffer_provider: &Box<impl BufferProvider + ?Sized>,
        values: &HashMap<String, Box<dyn PlaceholderValue>>
    ) -> Result<LString, String> {
        let mut string = String::new();
        let pattern_string = self.format_pattern(
            buffer_provider,
            values,
            &"_".to_string(),
        )?;
        string.push_str( pattern_string.as_str() );
        Ok( LString::new( string, Rc::clone( &self.locale ) ) )
    }

    pub fn locale( &self ) -> &Rc<Locale> {
        &self.locale
    }

    // Internal methods

    fn part_ref( &self, string: &String, index: usize ) -> Option<&Box<dyn Marker>> {
        if let Some( pattern ) = self.patterns.get( string ) {
            if let Some( part ) = pattern.get( index ) {
                return Some( part );
            }
        }
        None
    }

    fn format_pattern(
        &mut self,
        buffer_provider: &Box<impl BufferProvider + ?Sized>,
        values: &HashMap<String, Box<dyn PlaceholderValue>>,
        named: &String
    ) -> Result<String, String> {
        let mut string = String::new();
        let mut _len = 0usize;
        {
            let Some( pattern ) = self.patterns.get( named ) else {
                return Err( "Failed to retrieve named string pattern.".to_string() );
            };
            _len = pattern.len();
        }
        let mut i = 0usize;
        while i < _len {
            let Some( part ) = self.part_ref( named, i ) else {
                return Err( "Could not retrieve part of pattern.".to_string() );
            };
            match part.get_type() {
                "Text" => {
                    let Some( actual_part ) = part.as_any().downcast_ref::<Text>() else {
                        return Err( "Could not retrieve data for Text.".to_string() );
                    };
                    string.push_str( actual_part.string.as_str() );
                },
                "PatternString" => {
                    let Some( actual_part ) = part.as_any().downcast_ref::<PatternString>() else {
                        return Err( "Could not retrieve data for PatternString.".to_string() );
                    };
                    let Some( value ) = values.get( &actual_part.placeholder ) else {
                        return Err( "Placeholder value is not found for PatternString.".to_string() );
                    };
                    match value.get_type() {
                        "PlaceholderString" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderString>() else {
                                return Err( "Could not retrieve data for PlaceholderString.".to_string() );
                            };
                            string.push_str( actual_value.string.as_str() );
                        },
                        "PlaceholderLString" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderLString>() else {
                                return Err( "Could not retrieve data for PlaceholderLString.".to_string() );
                            };
                            // Currently no option to wrap string in language tags: `[en]substring[/en]`
                            // probably best place for wrap option flag is to put in the struct itself
                            string.push_str( actual_value.string.as_str() );
                        },
                        _ => return Err( "Invalid value type provided for PatternString.".to_string() )
                    }
                },
                "PatternDecimal" => {
                    let Some( actual_part ) = part.as_any().downcast_ref::<PatternDecimal>() else {
                        return Err( "Could not retrieve data for PatternDecimal.".to_string() );
                    };
                    let Some( value ) = values.get( &actual_part.placeholder ) else {
                        return Err( "Placeholder value is not found for PatternDecimal.".to_string() );
                    };
                    let data_locale = DataLocale::from( Rc::as_ref( &self.locale ) );
                    let selectors_index = actual_part.selectors;
                    let Some( selectors ) = self.selectors.get( selectors_index ) else {
                        return Err( "Index not found in selectors.".to_string() );
                    };
                    let option_sign = match selectors.get( "sign" ) {
                        None => None,
                        Some( option ) => {
                            let sign = sign_display( &option )?;
                            Some( sign )
                        }
                    };
                    let mut options: options::FixedDecimalFormatterOptions = Default::default();
                    if let Some( group ) = selectors.get( "group" ) {
                        options.grouping_strategy = decimal_grouping_display( &group )?;
                    }
                    let Ok( fdf ) = FixedDecimalFormatter::try_new_with_buffer_provider(
                        buffer_provider,
                        &data_locale,
                        options,
                    ) else {
                        return Err( "Failed to load decimal formatting information for locale.".to_string() );
                    };
                    match value.get_type() {
                        "PlaceholderFixedDecimal" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderFixedDecimal>() else {
                                return Err( "Could not retrieve data for PlaceholderFixedDecimal.".to_string() );
                            };
                            let mut fixed_decimal = &mut actual_value.number.clone();
                            if option_sign.is_some() {
                                fixed_decimal.apply_sign_display( option_sign.unwrap() );
                            }
                            let number_string = fdf.format( &fixed_decimal ).to_string();
                            string.push_str( number_string.as_str() );
                        },
                        "PlaceholderUnsigned" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderUnsigned>() else {
                                return Err( "Could not retrieve data for PlaceholderUnsigned.".to_string() );
                            };
                            let mut fixed_decimal = FixedDecimal::from( actual_value.number );
                            if option_sign.is_some() {
                                fixed_decimal.apply_sign_display( option_sign.unwrap() );
                            }
                            let number_string = fdf.format( &fixed_decimal ).to_string();
                            string.push_str( number_string.as_str() );
                        },
                        "PlaceholderInteger" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderInteger>() else {
                                return Err( "Could not retrieve data for PlaceholderInteger.".to_string() );
                            };
                            let mut fixed_decimal = FixedDecimal::from( actual_value.number );
                            if option_sign.is_some() {
                                fixed_decimal.apply_sign_display( option_sign.unwrap() );
                            }
                            let number_string = fdf.format( &fixed_decimal ).to_string();
                            string.push_str( number_string.as_str() );
                        },
                        "PlaceholderFloat" => {
                            // Precision is always Floating, for other precisions options use PlaceholderFixedDecimal
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderFloat>() else {
                                return Err( "Could not retrieve data for PlaceholderFloat.".to_string() );
                            };
                            let Ok( mut fixed_decimal ) = FixedDecimal::try_from_f64(
                                actual_value.number, DoublePrecision::Floating
                            ) else {
                                return Err( "Failed to created FixedDecimal for PlaceholderFloat.".to_string() );
                            };
                            if option_sign.is_some() {
                                fixed_decimal.apply_sign_display( option_sign.unwrap() );
                            }
                            let number_string = fdf.format( &fixed_decimal ).to_string();
                            string.push_str( number_string.as_str() );
                        },
                        _ => return Err( "Invalid value type provided for PatternDecimal.".to_string() )
                    }
                },
                "PatternDateTime" => {
                    //TODO: implement options
                    let Some( actual_part ) = part.as_any().downcast_ref::<PatternDateTime>() else {
                        return Err( "Could not retrieve data for PatternDateTime.".to_string() );
                    };
                    let Some( value ) = values.get( &actual_part.placeholder ) else {
                        return Err( "Placeholder value is not found for PatternDateTime.".to_string() );
                    };
                    let data_locale = DataLocale::from( Rc::as_ref( &self.locale ) );
                    let selectors_index = actual_part.selectors;
                    let Some( selectors ) = self.selectors.get( selectors_index ) else {
                        return Err( "Index not found in selectors.".to_string() );
                    };
                    match value.get_type() {
                        "PlaceholderDateTime" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderDateTime>() else {
                                return Err( "Could not retrieve data for PlaceholderDateTime.".to_string() );
                            };
                            // length must be explicitly be set
                            let mut options = length::Bag::from_date_time_style(
                                length::Date::Medium,
                                length::Time::Short,
                            );
                            // probably need to check is a calender is present in locale, if not insert `-u-ca-iso`
                            let Ok( dtf ) = DateTimeFormatter::try_new_with_buffer_provider(
                                buffer_provider,
                                &data_locale,
                                options.into(),
                            ) else {
                                return Err( "Failed to load date-time formatting information for locale.".to_string() );
                            };
                            let Ok( date_string ) = dtf.format_to_string( &actual_value.date_time ) else {
                                return Err( "Could not format date-time for locale.".to_string() );
                            };
                            string.push_str( date_string.as_str() );
                        },
                        "PlaceholderDate" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderDate>() else {
                                return Err( "Could not retrieve data for PlaceholderDate.".to_string() );
                            };
                            // length must be explicitly be set
                            let length = length::Date::Medium;//default value when no `date` option is present
                            // probably need to check is a calender is present in locale, if not insert `-u-ca-iso`
                            let Ok( df ) = DateFormatter::try_new_with_length_with_buffer_provider(
                                buffer_provider,
                                &data_locale,
                                length,
                            ) else {
                                return Err( "Failed to load date formatting information for locale.".to_string() );
                            };
                            let Ok( date_string ) = df.format_to_string( &actual_value.date ) else {
                                return Err( "Could not format date for locale.".to_string() );
                            };
                            string.push_str( date_string.as_str() );
                        },
                        "PlaceholderTime" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderTime>() else {
                                return Err( "Could not retrieve data for PlaceholderTime.".to_string() );
                            };
                            // length must be explicitly be set
                            let length = length::Time::Medium;//default value when no `time` option is present
                            // probably need to check is a calender is present in locale, if not insert `-u-ca-iso`
                            let Ok( tf ) = TimeFormatter::try_new_with_length_with_buffer_provider(
                                buffer_provider,
                                &data_locale,
                                length,
                            ) else {
                                return Err( "Failed to load date formatting information for locale.".to_string() );
                            };
                            let date_string = tf.format_to_string( &actual_value.time );
                            string.push_str( date_string.as_str() );
                        },
                        /*
                        // TODO: complete this as alternative to ICU's date-time objects,
                        // such as date-time strings stored in databases
                        "PlaceholderString" => {
                            let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderString>() else {
                                return Err( "Could not retrieve data for PlaceholderString.".to_string() );
                            };
                            //string must be iso format:
                            // - +YYYY-MM-DD   + is represents -, +, none (which is +) and more Y are allowed (in i32)
                            // - Thh:mm:ss.sss
                            // - +YYYY-MM-DDThh:mm:ss.sss
                            // timezones ( Z (for UCT 00:00), +hh:mm, -hh:mm ) are not supported by ICU4X.

                            //TODO: complete
                        },
                        */
                        _ => return Err( "Invalid value type provided for PatternDecimal.".to_string() )
                    }
                },
                "PatternComplex" => {
                    let Some( actual_part ) = part.as_any().downcast_ref::<PatternComplex>() else {
                        return Err( "Could not retrieve data for PatternComplex.".to_string() );
                    };
                    let Some( value ) = values.get( &actual_part.placeholder ) else {
                        return Err( "Placeholder value is not found for PatternComplex.".to_string() );
                    };
                    let selectors_index = actual_part.selectors;
                    let data_locale = DataLocale::from( Rc::as_ref( &self.locale ) );
                    match actual_part.complex {
                        ComplexType::Plural => {
                            let Ok( plurals ) = PluralRules::try_new_cardinal_with_buffer_provider(
                                buffer_provider,
                                &data_locale,
                            ) else {
                                return Err( "Failed to retrieve plural rules for locale.".to_string() );
                            };
                            match value.get_type() {
                                "PlaceholderFixedDecimal" => {
                                    let Some( actual_value ) =
                                        value.as_any().downcast_ref::<PlaceholderFixedDecimal>() else {
                                        return Err(
                                            "Could not retrieve data for PlaceholderFixedDecimal.".to_string()
                                        );
                                    };
                                    self.find_number_sign(
                                        buffer_provider,
                                        values,
                                        &mut string,
                                        &actual_value.number,
                                        plurals,
                                        &data_locale,
                                        selectors_index,
                                    )?;
                                },
                                "PlaceholderUnsigned" => {
                                    let Some( actual_value ) =
                                        value.as_any().downcast_ref::<PlaceholderUnsigned>() else {
                                        return Err( "Could not retrieve data for PlaceholderUnsigned.".to_string() );
                                    };
                                    let fixed_decimal = FixedDecimal::from( actual_value.number );
                                    self.find_number_sign(
                                        buffer_provider,
                                        values,
                                        &mut string,
                                        &fixed_decimal,
                                        plurals,
                                        &data_locale,
                                        selectors_index,
                                    )?;
                                },
                                "PlaceholderInteger" => {
                                    let Some( actual_value ) =
                                        value.as_any().downcast_ref::<PlaceholderInteger>() else {
                                        return Err( "Could not retrieve data for PlaceholderInteger.".to_string() );
                                    };
                                    let fixed_decimal = FixedDecimal::from( actual_value.number );
                                    self.find_number_sign(
                                        buffer_provider,
                                        values,
                                        &mut string,
                                        &fixed_decimal,
                                        plurals,
                                        &data_locale,
                                        selectors_index,
                                    )?;
                                },
                                "PlaceholderFloat" => {
                                    // Precision is always Floating, for other precisions options use
                                    // PlaceholderFixedDecimal with FixedDecimal::try_from_f64().
                                    let Some( actual_value ) = value.as_any().downcast_ref::<PlaceholderFloat>() else {
                                        return Err( "Could not retrieve data for PlaceholderFloat.".to_string() );
                                    };
                                    let Ok( fixed_decimal ) = FixedDecimal::try_from_f64(
                                        actual_value.number, DoublePrecision::Floating
                                    ) else {
                                        return Err(
                                            "Failed to created FixedDecimal for PlaceholderFloat.".to_string()
                                        );
                                    };
                                    self.find_number_sign(
                                        buffer_provider,
                                        values,
                                        &mut string,
                                        &fixed_decimal,
                                        plurals,
                                        &data_locale,
                                        selectors_index,
                                    )?;
                                },
                                _ => return Err( "Invalid value type provided for PatternOrdinal.".to_string() )
                            }
                        },
                        ComplexType::Ordinal => {
                            // Only positive integers and zero are allowed.
                            let Ok( plurals ) = PluralRules::try_new_ordinal_with_buffer_provider(
                                buffer_provider,
                                &data_locale,
                            ) else {
                                return Err( "Failed to retrieve plural rules for locale.".to_string() );
                            };
                            match value.get_type() {
                                "PlaceholderUnsigned" => {
                                    let Some( actual_value ) =
                                        value.as_any().downcast_ref::<PlaceholderUnsigned>() else {
                                        return Err( "Could not retrieve data for PlaceholderUnsigned.".to_string() );
                                    };
                                    let fixed_decimal = FixedDecimal::from( actual_value.number );
                                    self.find_number_sign(
                                        buffer_provider,
                                        values,
                                        &mut string,
                                        &fixed_decimal,
                                        plurals,
                                        &data_locale,
                                        selectors_index,
                                    )?;
                                },
                                _ => return Err( "Invalid value type provided for PatternOrdinal.".to_string() )
                            }
                        },
                        ComplexType::Select => {
                            match value.get_type() {
                                "PlaceholderString" => {
                                    let Some( actual_value ) =
                                        value.as_any().downcast_ref::<PlaceholderString>() else {
                                        return Err( "Could not retrieve data for PlaceholderString.".to_string() );
                                    };
                                    self.select(
                                        buffer_provider,
                                        values,
                                        &mut string,
                                        &actual_value.string,
                                        selectors_index,
                                    )?;
                                },
                                "PlaceholderLString" => {
                                    // Locale is not used, and LSring is just treated as String for the selector
                                    let Some( actual_value ) =
                                        value.as_any().downcast_ref::<PlaceholderLString>() else {
                                        return Err( "Could not retrieve data for PlaceholderLString.".to_string() );
                                    };
                                    self.select(
                                        buffer_provider,
                                        values,
                                        &mut string,
                                        &actual_value.string.as_str().to_string(),
                                        selectors_index,
                                    )?;
                                },
                                _ => return Err( "Invalid value type provided for PatternOrdinal.".to_string() )
                            }
                        },
                    }
                },
                "NumberSign" => {
                    let Some( actual_part ) = part.as_any().downcast_ref::<NumberSign>() else {
                        return Err( "Could not retrieve data for NumberSign.".to_string() );
                    };
                    let Some( number_string ) = self.numbers.get( actual_part.index ) else {
                        return Err( "Unable to retrieve reference for NumberSign index.".to_string() );
                    };
                    string.push_str( number_string.as_str() );
                },
                _ => { // Should never reach.
                    return Err( "Invalid part type found.".to_string() );
                }
            }
            i += 1;
        }
        Ok( string )
    }

    fn find_number_sign(
        &mut self,
        buffer_provider: &Box<impl BufferProvider + ?Sized>,
        values: &HashMap<String, Box<dyn PlaceholderValue>>,
        string: &mut String,
        fixed_decimal: &FixedDecimal,
        plurals: PluralRules,
        data_locale: &DataLocale,
        selectors_index: usize,
    ) -> Result<(), String> {

        // Format number using graphemes of the locale.
        let Ok( fdf ) = FixedDecimalFormatter::try_new_with_buffer_provider(
            buffer_provider,
            data_locale,
            Default::default(),
        ) else {
            return Err( "Failed to load decimal formatting information for locale.".to_string() );
        };
        let number_string = fdf.format( fixed_decimal ).to_string();
        let category = plural_category( plurals.category_for( fixed_decimal ) ).to_string();

        // Get the named string, and locate number signs to update the string.
        let Some( selectors ) = self.selectors.get( selectors_index ) else {
            return Err( "Index not found in selectors.".to_string() );
        };
        let Some( named ) = selectors.get( &category ) else {
            return Err( "Failed to retrieve named string from selector.".to_string() );
        };
        let mut _len = 0usize;
        {
            let Some( pattern ) = self.patterns.get( named ) else {
                return Err( "Failed to retrieve named string pattern.".to_string() );
            };
            _len = pattern.len();
        }
        let mut i = 0usize;
        while i < _len {
            let Some( part ) = self.part_ref( named, i ) else {
                return Err( "Could not retrieve part of pattern.".to_string() );
            };
            if part.get_type() == "NumberSign" {
                let Some( actual_part ) = part.as_any().downcast_ref::<NumberSign>() else {
                    return Err( "Could not retrieve data for NumberSign.".to_string() );
                };
                let index = actual_part.index; // Gets around the immutable borrow issue.
                let Some( number_string_mut ) = self.numbers.get_mut( index ) else {
                    return Err( "Unable to retrieve reference for NumberSign string.".to_string() );
                };
                *number_string_mut = number_string.to_string();//might change to Rc 
            }
            i += 1;
        }
        let part_string = self.format_pattern(
            buffer_provider,
            values,
            &named.to_string()
        )?;
        string.push_str( part_string.as_str() );
        Ok( () )
    }

    fn select(
        &mut self,
        buffer_provider: &Box<impl BufferProvider + ?Sized>,
        values: &HashMap<String, Box<dyn PlaceholderValue>>,
        string: &mut String,
        string_value: &String,
        selectors_index: usize,
    ) -> Result<(), String> {
    
        // Get the named string, and locate number signs to update the string.
        let Some( selectors ) = self.selectors.get( selectors_index ) else {
            return Err( "Index not found in selectors.".to_string() );
        };
        let Some( named ) = selectors.get( string_value ) else {
            return Err( "Failed to retrieve named string from selector.".to_string() );
        };
        let part_string = self.format_pattern(
            buffer_provider,
            values,
            &named.to_string(),
        )?;
        string.push_str( part_string.as_str() );
        Ok( () )
    }
}

// Internal structures, enums, etc.

// Check node type.
fn check_node_type( tree: &Tree, index: usize, node_type: NodeType ) -> bool {
    let Ok( node_type_data ) = tree.node_type( index ) else {
        return false;
    };
    let Some( node_type2 ) = node_type_data.downcast_ref::<NodeType>() else {
        return false;
    };
    if node_type != *node_type2 {
        return false;
    }
    true
}

fn part_text(
    pattern: &mut Vec<Box<dyn Marker>>,
    parser_result: &ParserResult,
    index: usize,
) -> Result<(), String> {
    let mut string = String::new();
    let Ok( text_data ) = parser_result.tree.data_ref( index ) else {
        return Err( "Could not retrieve data for Text node.".to_string() );
    };
    for token_data in text_data.iter() {
        let Some( token ) = token_data.downcast_ref::<Rc<Token>>() else {
            return Err( "Could not retrieve token for Text node.".to_string() );
        };
        string.push_str( token.string.as_str() );
    }
    pattern.push( Box::new( Text { string } ) );
    Ok( () )
}

fn part_pattern(
    pattern: &mut Vec<Box<dyn Marker>>,
    patterns: &HashMap<String, Vec<Box<dyn Marker>>>,
    parser_result: &ParserResult,
    index: usize,
    selectors: &mut Vec<HashMap<String, String>>,
    valid_plurals: &Vec<String>,
) -> Result<(), String> {
    let Ok( children ) = parser_result.tree.children( index ) else {
        return Err( "Could not retrieve children for Pattern.".to_string() );
    };
    let mut iterator = children.iter();

    // Identifier - first node
    let placeholder = *iterator.next().unwrap();
    if !check_node_type( &parser_result.tree, placeholder, NodeType::Identifier ) {
        return Err( "Identifier node was not found.".to_string() );
    }
    let Ok( placeholder_data ) = parser_result.tree.data_ref( placeholder ) else {
        return Err( "Could not retrieve data for placeholder node.".to_string() );
    };
    let Some( placeholder_token ) = placeholder_data.first().unwrap().downcast_ref::<Rc<Token>>() else {
        return Err( "Failed to retrieve token for placeholder node.".to_string() ) ;
    };

    // Keyword - second node
    let keyword = match iterator.next() {
        None => {
            // placeholder with no parameters - string value
            pattern.push(
                Box::new(
                    PatternString {
                        placeholder: placeholder_token.string.to_string(),
                    }
                )
            );
            return Ok( () );
        },
        Some( keyword ) => *keyword
    };
    if !check_node_type( &parser_result.tree, keyword, NodeType::Identifier ) {
        return Err( "Identifier node was not found.".to_string() );
    }
    let Ok( keyword_data ) = parser_result.tree.data_ref( keyword ) else {
        return Err( "Could not retrieve data for keyword node.".to_string() );
    };
    let Some( keyword_token ) = keyword_data.first().unwrap().downcast_ref::<Rc<Token>>() else {
        return Err( "Failed to retrieve token for keyword node.".to_string() );
    };
    if keyword_token.string.as_str() == "decimal" {
        let strings = pattern_selector( patterns, parser_result, index, false )?;
        let len = selectors.len();
        selectors.push( strings );
        pattern.push(
            Box::new(
                PatternDecimal {
                    placeholder: placeholder_token.string.to_string(),
                    selectors: len,
                }
            )
        );
    } else if keyword_token.string.as_str() == "date_time" {
        let strings = pattern_selector( patterns, parser_result, index, false )?;
        let len = selectors.len();
        selectors.push( strings );
        pattern.push(
            Box::new(
                PatternDateTime {
                    placeholder: placeholder_token.string.to_string(),
                    selectors: len,
                }
            )
        );
    } else if keyword_token.string.as_str() == "ordinal" {
        let strings = pattern_selector( patterns, parser_result, index, true )?;
        for ( key, value ) in strings.iter() {
            if !valid_plurals.contains( &key ) {
                return Err( "Invalid selector for ordinal pattern.".to_string() );
            }
        }
        let len = selectors.len();
        selectors.push( strings );
        pattern.push( Box::new( PatternComplex {
            placeholder: placeholder_token.string.to_string(),
            complex: ComplexType::Ordinal,
            selectors: len,
        } ) );
    } else if keyword_token.string.as_str() == "plural" {
        let strings = pattern_selector( patterns, parser_result, index, true )?;
        for ( key, value ) in strings.iter() {
            if !valid_plurals.contains( &key ) {
                return Err( "Invalid selector for plural pattern.".to_string() );
            }
        }
        let len = selectors.len();
        selectors.push( strings );
        pattern.push( Box::new( PatternComplex {
            placeholder: placeholder_token.string.to_string(),
            complex: ComplexType::Plural,
            selectors: len,
        } ) );
    } else if keyword_token.string.as_str() == "select" {
        // Just structure checks for errors.
        let strings = pattern_selector( patterns, parser_result, index, true )?;
        let len = selectors.len();
        selectors.push( strings );
        pattern.push( Box::new( PatternComplex {
            placeholder: placeholder_token.string.to_string(),
            complex: ComplexType::Select,
            selectors: len,
        } ) );
    } else {
        return Err( "Invalid placeholder keyword found.".to_string() );
    }
    Ok( () )
}

fn pattern_selector(
    patterns: &HashMap<String, Vec<Box<dyn Marker>>>,
    parser_result: &ParserResult,
    index: usize,
    complex: bool,
) -> Result<HashMap<String, String>, String> {
    let Ok( children ) = parser_result.tree.children( index ) else {
        return Err( "Could not retrieve children for Pattern.".to_string() );
    };
    let mut iterator = children.iter().skip( 2 );
    let mut strings = HashMap::<String, String>::new();
    let mut other = false;
    while let Some( selector ) = iterator.next() {
        if !check_node_type( &parser_result.tree, *selector, NodeType::Selector ) {
            return Err( "Selector node was not found.".to_string() );
        }
        let Ok( first ) = parser_result.tree.first( *selector ) else {
            return Err( "First child of Selector node not found.".to_string() );
        };
        if !check_node_type( &parser_result.tree, first, NodeType::Identifier ) {
            return Err( "Identifier node was not found.".to_string() );
        }
        let Ok( first_data ) = parser_result.tree.data_ref( first ) else {
            return Err( "Could not retrieve data for selector first node.".to_string() );
        };
        let Some( first_token ) = first_data.first().unwrap().downcast_ref::<Rc<Token>>() else {
            return Err( "Failed to retrieve token for selector first node.".to_string() ) ;
        };
        if first_token.string.as_str() == "other" {
            other = true;
        }
        let Ok( last ) = parser_result.tree.last( *selector ) else {
            return Err( "Last child of Selector node not found.".to_string() );
        };
        if !check_node_type( &parser_result.tree, last, NodeType::Identifier ) {
            return Err( "Identifier node was not found.".to_string() );
        }
        let Ok( last_data ) = parser_result.tree.data_ref( last ) else {
            return Err( "Could not retrieve data for selector last node.".to_string() );
        };
        let Some( last_token ) = last_data.first().unwrap().downcast_ref::<Rc<Token>>() else {
            return Err( "Failed to retrieve token for selector last node.".to_string() );
        };
        let name = last_token.string.to_string();
        if complex && !patterns.contains_key( &name ) {
            return Err( "Named string not found for selector.".to_string() );
        }
        strings.insert( first_token.string.to_string(), name );
    }
    if complex && !other {
        return Err( "Required 'other' selector was not found for keyword 'select'.".to_string() );
    }
    Ok( strings )
}

fn plural_category( category: PluralCategory ) -> &'static str {
    match category {
        PluralCategory::Zero => "zero",
        PluralCategory::One => "one",
        PluralCategory::Two => "two",
        PluralCategory::Few => "few",
        PluralCategory::Many => "many",
        PluralCategory::Other => "other",
    }
}

fn sign_display( sign: &str ) -> Result<SignDisplay, String> {
    match sign {
        "auto" => return Ok( SignDisplay::Auto ),
        "never" => return Ok( SignDisplay::Never ),
        "always" => return Ok( SignDisplay::Always ),
        "except_zero" => return Ok( SignDisplay::ExceptZero ),
        "negative" => return Ok( SignDisplay::Negative ),
        _ => return Err( "Invalid decimal sign option.".to_string() )
    }
}

fn decimal_grouping_display( group: &str ) -> Result<options::GroupingStrategy, String> {
    match group {
        "auto" => return Ok( options::GroupingStrategy::Auto ),
        "never" => return Ok( options::GroupingStrategy::Never ),
        "always" => return Ok( options::GroupingStrategy::Always ),
        "min2" => return Ok( options::GroupingStrategy::Min2 ),
        _ => return Err( "Invalid decimal group option.".to_string() )
    }
}

enum ComplexType {
    Plural,
    Ordinal,
    Select,
}

trait Marker {
    fn get_type( &self ) -> &str;

    fn as_any( &self ) -> &dyn Any;

    fn as_any_mut( &mut self ) -> &mut dyn Any;
}

struct Text {
     string: String,
}

impl Marker for Text {
    fn get_type( &self ) -> &str {
        "Text"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }
}

struct NumberSign {
    index: usize,
}

impl Marker for NumberSign {
    fn get_type( &self ) -> &str {
        "NumberSign"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }
}

struct PatternString {
    placeholder: String,
}

impl Marker for PatternString {
    fn get_type( &self ) -> &str {
        "PatternString"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }
}

struct PatternDecimal {
    placeholder: String,
    selectors: usize,
}

impl Marker for PatternDecimal {
    fn get_type( &self ) -> &str {
        "PatternDecimal"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }
}

struct PatternDateTime {
    placeholder: String,
    selectors: usize,
}

impl Marker for PatternDateTime {
    fn get_type( &self ) -> &str {
        "PatternDateTime"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }
}

struct PatternComplex {
    placeholder: String,
    complex: ComplexType,
    selectors: usize,//HashMap<String, String>,
}

impl Marker for PatternComplex {
    fn get_type( &self ) -> &str {
        "PatternComplex"
    }

    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use icu_testdata::buffer;
    use i18n_lexer::Lexer;

    #[test]
    fn plain_text() {
        fn format_string() -> Result<(), String> {
            let buffer_provider = Box::new( buffer() );
            let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
            let tokens = lexer.tokenise(
                "A simple plain text string.", &vec![ '{', '}', '`', '#' ]
            );
            let parser_result = parse( tokens )?;
            let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
            let mut formatter = Formatter::try_new( &locale, parser_result )?;
            let values = HashMap::<String, Box::<dyn PlaceholderValue>>::new();
            let result = formatter.format( &buffer_provider, &values )?;
            assert_eq!( result.as_str(), "A simple plain text string.", "Strings must be the same." );
            Ok( () )
        }
        match format_string() {
            Err( error ) => {
                println!( "Error: {}", error );
                assert!( false );
                return;
            },
            Ok( _ ) => {}
        };
    }

    #[test]
    fn pattern_string() {
        fn format_string() -> Result<(), String> {
            let buffer_provider = Box::new( buffer() );
            let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
            let tokens = lexer.tokenise(
                "Expecting a string for placeholder: {string}", &vec![ '{', '}', '`', '#' ]
            );
            let parser_result = parse( tokens )?;
            let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
            let mut formatter = Formatter::try_new( &locale, parser_result )?;
            let mut values = HashMap::<String, Box::<dyn PlaceholderValue>>::new();
            values.insert(
                "string".to_string(),
                Box::new( PlaceholderString { string: "This is a string.".to_string() } )
            );
            let result = formatter.format( &buffer_provider, &values )?;
            assert_eq!(
                result.as_str(),
                "Expecting a string for placeholder: This is a string.",
                "Strings must be the same."
            );
            Ok( () )
        }
        match format_string() {
            Err( error ) => {
                println!( "Error: {}", error );
                assert!( false );
                return;
            },
            Ok( _ ) => {}
        };
    }

    #[test]
    fn pattern_plural() {
        fn format_string() -> Result<(), String> {
            let buffer_provider = Box::new( buffer() );
            let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
            let tokens = lexer.tokenise(
                "There {dogs_number plural one#one_dog other#dogs} in the park.#{dogs are # dogs}{one_dog is 1 dog}",
                &vec![ '{', '}', '`', '#' ]
            );
            let parser_result = parse( tokens )?;
            let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
            let mut formatter = Formatter::try_new( &locale, parser_result )?;
            let mut values = HashMap::<String, Box::<dyn PlaceholderValue>>::new();
            values.insert(
                "dogs_number".to_string(),
                Box::new( PlaceholderUnsigned { number: 3 } )
            );
            let result = formatter.format( &buffer_provider, &values )?;
            assert_eq!(
                result.as_str(),
                "There are 3 dogs in the park.",
                "Strings must be the same."
            );
            Ok( () )
        }
        match format_string() {
            Err( error ) => {
                println!( "Error: {}", error );
                assert!( false );
                return;
            },
            Ok( _ ) => {}
        };
    }

    #[test]
    fn pattern_decimal() {
        fn format_string() -> Result<(), String> {
            let buffer_provider = Box::new( buffer() );
            let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
            let tokens = lexer.tokenise(
                "There is {amount decimal} kg of rice in the container.",
                &vec![ '{', '}', '`', '#' ]
            );
            let parser_result = parse( tokens )?;
            let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
            let mut formatter = Formatter::try_new( &locale, parser_result )?;
            let mut values = HashMap::<String, Box::<dyn PlaceholderValue>>::new();
            values.insert(
                "amount".to_string(),
                Box::new( PlaceholderFloat { number: 3.678 } )
            );
            let result = formatter.format( &buffer_provider, &values )?;
            assert_eq!(
                result.as_str(),
                "There is 3,678 kg of rice in the container.",
                "Strings must be the same."
            );
            Ok( () )
        }
        match format_string() {
            Err( error ) => {
                println!( "Error: {}", error );
                assert!( false );
                return;
            },
            Ok( _ ) => {}
        };
    }

    #[test]
    fn pattern_decimal_with_option() {
        fn format_string() -> Result<(), String> {
            let buffer_provider = Box::new( buffer() );
            let mut lexer = Lexer::try_new( &buffer_provider ).expect( "Failed to initialise lexer." );
            let tokens = lexer.tokenise(
                "There is {amount decimal sign#always} kg of rice in the container.",
                &vec![ '{', '}', '`', '#' ]
            );
            let parser_result = parse( tokens )?;
            let locale = Rc::new( "en-ZA".parse().expect( "Failed to parse language tag." ) );
            let mut formatter = Formatter::try_new( &locale, parser_result )?;
            let mut values = HashMap::<String, Box::<dyn PlaceholderValue>>::new();
            values.insert(
                "amount".to_string(),
                Box::new( PlaceholderFloat { number: 3.678 } )
            );
            let result = formatter.format( &buffer_provider, &values )?;
            assert_eq!(
                result.as_str(),
                "There is +3,678 kg of rice in the container.",
                "Strings must be the same."
            );
            Ok( () )
        }
        match format_string() {
            Err( error ) => {
                println!( "Error: {}", error );
                assert!( false );
                return;
            },
            Ok( _ ) => {}
        };
    }
}
