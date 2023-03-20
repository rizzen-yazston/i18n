// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_icu-rizzen-yazston` crate.

use crate::IcuError;
use icu_provider::serde::DeserializingBufferProvider;
use icu_properties::sets::{ load_pattern_white_space, load_pattern_syntax, CodePointSetData };
use icu_segmenter::GraphemeClusterSegmenter;

/// The `IcuDataProvider` type contains a member `data_provider` holding the `DataProvider`, which is a deserialised
/// `BufferProvider`.
/// 
/// The `IcuDataProvider` type also contains non-locale based data used within the `i18n_lexer` crate.
/// 
/// `IcuDataProvider` type is used within the `Rc` type to prevent unnecessary duplication.
pub struct IcuDataProvider<'a, P>
where
    P: ?Sized
{
    data_provider: DeserializingBufferProvider<'a, P>,
    pattern_syntax: CodePointSetData,
    pattern_white_space: CodePointSetData,
    grapheme_segmenter: GraphemeClusterSegmenter,
}

impl<'a, P> IcuDataProvider<'a, P> 
where
    P: ?Sized + icu_provider::BufferProvider
{

    /// Create a `IcuDataProvider` object using the ICU's `DeserializingBufferProvider` as the `DataProvider`. Besides
    /// storing the `DataProvider`, it also obtains and stores the Pattern_Syntax character set, the
    /// Pattern_White_Space character set, and the Grapheme Cluster Segmenter required for the `Lexer` types to
    /// function.
    pub fn try_new( data_provider: DeserializingBufferProvider<'a, P> ) -> Result<Self, IcuError> {
        let syntax = load_pattern_syntax( &data_provider )?;
        let white_space = load_pattern_white_space( &data_provider )?;
        let grapheme_segmenter =
            GraphemeClusterSegmenter::try_new_unstable( &data_provider )?;
        Ok( IcuDataProvider {
            data_provider,
            pattern_syntax: syntax,
            pattern_white_space: white_space,
            grapheme_segmenter,
        } )
    }

    /// Get the `DataProvider` object that can be used in any ICU function that accepts a `DataProvider` as a
    /// parameter.
    pub fn data_provider( &self ) -> &DeserializingBufferProvider<P> {
        &self.data_provider
    }

    /// Get the preloaded Pattern_Syntax character set.
    pub fn pattern_syntax( &self ) -> &CodePointSetData {
        &self.pattern_syntax
    }

    /// Get the preloaded Pattern_White_Space character set.
    pub fn pattern_white_space( &self ) -> &CodePointSetData {
        &self.pattern_white_space
    }

    /// Get the Grapheme Cluster Segmenter with preloaded character data set.
    pub fn grapheme_segmenter( &self ) -> &GraphemeClusterSegmenter {
        &self.grapheme_segmenter
    }
}