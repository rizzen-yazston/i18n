// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_icu-rizzen-yazston` crate.

use crate::IcuError;
use icu_provider::DataProvider;
use icu_properties::{
    provider::{ PatternSyntaxV1Marker, PatternWhiteSpaceV1Marker },
    sets::{ load_pattern_white_space, load_pattern_syntax, CodePointSetData }
};
use icu_segmenter::{ GraphemeClusterSegmenter, provider::GraphemeClusterBreakDataV1Marker };
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

/// The `IcuDataProvider` type contains a member `data_provider` holding the `&DataProvider` as a `DataProviderWrapper`
/// type.
/// 
/// The `IcuDataProvider` type also contains non-locale based data used within the `i18n_lexer` crate.
/// 
/// `IcuDataProvider` type is used within the `Rc` type as `Rc<IcuDataProvider>` to prevent unnecessary duplication.
pub struct IcuDataProvider<'a, P>
where
    P: ?Sized + DataProvider<PatternSyntaxV1Marker> + DataProvider<PatternWhiteSpaceV1Marker>
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
{
    data_provider: DataProviderWrapper<'a, P>,
    pattern_syntax: CodePointSetData,
    pattern_white_space: CodePointSetData,
    grapheme_segmenter: GraphemeClusterSegmenter,
}

impl<'a, P> IcuDataProvider<'a, P>
where
    P: ?Sized + DataProvider<PatternSyntaxV1Marker> + DataProvider<PatternWhiteSpaceV1Marker>
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
{

    /// Create a `IcuDataProvider` object using the ICU's `DataProvider` as a reference within the
    /// `DataProviderWrapper` type, which is provided by this crate. Besides storing the `DataProvider`, it also
    /// obtains and stores the Pattern_Syntax character set, the Pattern_White_Space character set, and the Grapheme
    /// Cluster Segmenter required for the `Lexer` types to function.
    /// See `i18n_lexer` crate on usage.
    pub fn try_new( data_provider: &'a P ) -> Result<Self, IcuError> {
        let syntax = load_pattern_syntax( data_provider )?;
        let white_space = load_pattern_white_space( data_provider )?;
        let grapheme_segmenter =
            GraphemeClusterSegmenter::try_new_unstable( data_provider )?;
        Ok( IcuDataProvider {
            data_provider: DataProviderWrapper( data_provider ),
            pattern_syntax: syntax,
            pattern_white_space: white_space,
            grapheme_segmenter,
        } )
    }

    /// Get the `DataProviderWrapper` object that can be used in any ICU function that accepts a `DataProvider` as a
    /// parameter, as `data_provider().0`.
    /// See `i18n_lexer` crate on usage.
    pub fn data_provider( &self ) -> &DataProviderWrapper<P> {
        &self.data_provider
    }

    /// Get the preloaded Pattern_Syntax character set.
    /// See `i18n_lexer` crate on usage.
    pub fn pattern_syntax( &self ) -> &CodePointSetData {
        &self.pattern_syntax
    }

    /// Get the preloaded Pattern_White_Space character set.
    /// See `i18n_lexer` crate on usage.
    pub fn pattern_white_space( &self ) -> &CodePointSetData {
        &self.pattern_white_space
    }

    /// Get the Grapheme Cluster Segmenter with preloaded character data set.
    /// See `i18n_lexer` crate on usage.
    pub fn grapheme_segmenter( &self ) -> &GraphemeClusterSegmenter {
        &self.grapheme_segmenter
    }
}

/// A simple tuple struct that holds a reference to a ICU4X `DataProvider` implementation. This tuple struct allows
/// a `DataProvider` reference to be stored within other structs.
pub struct DataProviderWrapper<'a, P: ?Sized>( pub &'a P );
