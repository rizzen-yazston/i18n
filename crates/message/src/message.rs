// This file is part of `i18n_message-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_message-rizzen-yazston` crate.

use crate::MessageError;
use i18n_icu::IcuDataProvider;
use i18n_lexer::{ Lexer, Token, TokenType };
use i18n_provider::{ LStringProvider, LStringProviderWrapper };
use i18n_registry::LanguageTagRegistry;
use i18n_lstring::LString;
use icu_provider::DataProvider;
use icu_properties::{ provider::{ PatternSyntaxV1Marker, PatternWhiteSpaceV1Marker } };
use icu_segmenter::provider::GraphemeClusterBreakDataV1Marker;
use icu_plurals::provider::{ CardinalV1Marker, OrdinalV1Marker };
use icu_decimal::provider::DecimalSymbolsV1Marker;
use icu_datetime::provider::calendar::{
    TimeSymbolsV1Marker,
    TimeLengthsV1Marker,
    GregorianDateLengthsV1Marker,
    BuddhistDateLengthsV1Marker,
    JapaneseDateLengthsV1Marker,
    JapaneseExtendedDateLengthsV1Marker,
    CopticDateLengthsV1Marker,
    IndianDateLengthsV1Marker,
    EthiopianDateLengthsV1Marker,
    GregorianDateSymbolsV1Marker,
    BuddhistDateSymbolsV1Marker,
    JapaneseDateSymbolsV1Marker,
    JapaneseExtendedDateSymbolsV1Marker,
    CopticDateSymbolsV1Marker,
    IndianDateSymbolsV1Marker,
    EthiopianDateSymbolsV1Marker,
};
use icu_calendar::provider::{ WeekDataV1Marker, JapaneseErasV1Marker, JapaneseExtendedErasV1Marker };
use std::rc::Rc;

pub struct Message<'a, I, L>
where
    I: ?Sized + DataProvider<PatternSyntaxV1Marker> + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<GraphemeClusterBreakDataV1Marker> + DataProvider<CardinalV1Marker>
        + DataProvider<OrdinalV1Marker> + DataProvider<DecimalSymbolsV1Marker> + DataProvider<TimeSymbolsV1Marker>
        + DataProvider<TimeLengthsV1Marker> + DataProvider<WeekDataV1Marker>
        + DataProvider<GregorianDateLengthsV1Marker> + DataProvider<BuddhistDateLengthsV1Marker>
        + DataProvider<JapaneseDateLengthsV1Marker> + DataProvider<JapaneseExtendedDateLengthsV1Marker>
        + DataProvider<CopticDateLengthsV1Marker> + DataProvider<IndianDateLengthsV1Marker>
        + DataProvider<EthiopianDateLengthsV1Marker> + DataProvider<GregorianDateSymbolsV1Marker>
        + DataProvider<BuddhistDateSymbolsV1Marker> + DataProvider<JapaneseDateSymbolsV1Marker>
        + DataProvider<JapaneseExtendedDateSymbolsV1Marker> + DataProvider<CopticDateSymbolsV1Marker>
        + DataProvider<IndianDateSymbolsV1Marker> + DataProvider<EthiopianDateSymbolsV1Marker>
        + DataProvider<JapaneseErasV1Marker> + DataProvider<JapaneseExtendedErasV1Marker>,
    L: ?Sized + LStringProvider,
{
    icu_data_provider: Rc<IcuDataProvider<'a, I>>,
    registry: Rc<LanguageTagRegistry>,
    lexer: Rc<Lexer<'a, I>>,
    lstring_provider: LStringProviderWrapper<'a, L>,
    fallback: bool,
    caching: bool,
}

impl<'a, I, L> Message<'a, I, L>
where
    I: ?Sized + DataProvider<PatternSyntaxV1Marker> + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<GraphemeClusterBreakDataV1Marker> + DataProvider<CardinalV1Marker>
        + DataProvider<OrdinalV1Marker> + DataProvider<DecimalSymbolsV1Marker> + DataProvider<TimeSymbolsV1Marker>
        + DataProvider<TimeLengthsV1Marker> + DataProvider<WeekDataV1Marker>
        + DataProvider<GregorianDateLengthsV1Marker> + DataProvider<BuddhistDateLengthsV1Marker>
        + DataProvider<JapaneseDateLengthsV1Marker> + DataProvider<JapaneseExtendedDateLengthsV1Marker>
        + DataProvider<CopticDateLengthsV1Marker> + DataProvider<IndianDateLengthsV1Marker>
        + DataProvider<EthiopianDateLengthsV1Marker> + DataProvider<GregorianDateSymbolsV1Marker>
        + DataProvider<BuddhistDateSymbolsV1Marker> + DataProvider<JapaneseDateSymbolsV1Marker>
        + DataProvider<JapaneseExtendedDateSymbolsV1Marker> + DataProvider<CopticDateSymbolsV1Marker>
        + DataProvider<IndianDateSymbolsV1Marker> + DataProvider<EthiopianDateSymbolsV1Marker>
        + DataProvider<JapaneseErasV1Marker> + DataProvider<JapaneseExtendedErasV1Marker>,
    L: ?Sized + LStringProvider,
{

    /// TODO
    // TODO: Add struct contain callback functions for commands
    pub fn try_new(
        icu_data_provider: &Rc<IcuDataProvider<'a, I>>,
        language_registry: &Rc<LanguageTagRegistry>,
        lexer: &Rc<Lexer<'a, I>>,
        lstring_provider: &'a L,
        fallback: bool, //true = fallback to default language
        caching: bool,
    ) -> Result<Self, MessageError> {
        Ok( Message {
            icu_data_provider: Rc::clone( icu_data_provider ),
            registry: Rc::clone( language_registry ),
            lexer: Rc::clone( lexer ),
            lstring_provider: LStringProviderWrapper( lstring_provider ),
            fallback,
            caching,
        } )
    }

    pub fn format<T: AsRef<str>>(
        &self,
        identifier: T,
        language_tag: &Rc<String>,
        fallback: Option<bool>, //true = fallback to default language, None = use the Message default.
        caching: Option<bool>, //cache the resultant Format for repeating use with different values.
    ) -> Result<LString, MessageError> {
        let lstring = self.lstring_provider.0.get( identifier, language_tag );



        Ok( LString::new( "blah", &Rc::new( "blah".to_string() ) ) )//temp to get rid of compiling error.
    }

}
