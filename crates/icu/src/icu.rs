// This file is part of `i18n_icu-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_icu-rizzen-yazston` crate.

use crate::IcuError;
#[cfg( feature = "buffer" )]
#[allow( unused_imports )]
use icu_provider::AsDeserializingBufferProvider;
#[cfg( feature = "blob" )]
use icu_provider_blob::BlobDataProvider;
#[cfg( feature = "fs" )]
use icu_provider_fs::FsDataProvider;
use icu_segmenter::GraphemeClusterSegmenter;
#[allow( unused_imports )]
use icu_properties::sets::CodePointSetData;
#[cfg( feature = "buffer" )]
#[allow( unused_imports )]
use icu_properties::{
    sets::{ load_pattern_syntax, load_pattern_white_space },
    provider::{ PatternSyntaxV1Marker, PatternWhiteSpaceV1Marker }
};

/// Indicates which data provider to use for various supported ICU4X components:
/// 
/// * Internal: Will use the internal BakedDateProvider of various ICU4X components. Requires the `compiled_data` feature.
/// 
/// * Blob: The BlobDataProvider will be used for the various ICU4X components. Requires the `blob` feature.
/// 
/// * Fs: The FsDataProvider will be used for the various ICU4X components. Requires the `fs` feature.
#[derive( Clone )]
pub enum DataProvider {
    #[cfg( feature = "compiled_data" )]
    Internal,
    #[cfg( feature = "blob" )]
    Blob( BlobDataProvider ),
    #[cfg( feature = "fs" )]
    Fs( FsDataProvider )
}

/// The `IcuBufferProvider` type containing the `DataProvider` enum.
/// 
/// The `IcuBufferProvider` type also contains non-locale based data used within the `i18n_lexer` crate.
/// 
/// `IcuBufferProvider` type is used within the `Rc` type as `Rc<IcuBufferProvider>` to prevent unnecessary duplication.
pub struct IcuBufferProvider {
    data_provider: DataProvider,
    grapheme_segmenter: Option<GraphemeClusterSegmenter>,
    #[allow( dead_code )]
    syntax: Option<CodePointSetData>,
    #[allow( dead_code )]
    white_space: Option<CodePointSetData>,
}

impl IcuBufferProvider {

    /// Create a `IcuBufferProvider` instance. Depending on the DataProvider choice, the instance may include the
    /// Grapheme Cluster Segmenter and character properties sets.
    pub fn try_new( data_provider: DataProvider ) -> Result<Self, IcuError> {
        #[allow( unused_mut )]
        #[allow( unused_variables )]
        let mut grapheme_segmenter = None;
        #[allow( unused_mut )]
        #[allow( unused_variables )]
        let mut syntax = None;
        #[allow( unused_mut )]
        #[allow( unused_variables )]
        let mut white_space = None;
        #[allow( unreachable_code )]
        match data_provider.clone() {
            #[cfg( feature = "blob" )]
            DataProvider::Blob( blob ) => {
                grapheme_segmenter = Some( GraphemeClusterSegmenter::try_new_with_buffer_provider( &blob )? );
                // Below left as comment until figured out how to use BufferProvider implementations for properties.
                /*
                let blob_provider = blob.as_deserializing();
                syntax = Some( load_pattern_syntax( &blob_provider )? ); // <---- issue here
                white_space = Some( load_pattern_white_space( &blob_provider )? ); // <---- issue here
                */
            },
            #[cfg( feature = "fs" )]
            DataProvider::Fs( fs ) => {
                grapheme_segmenter = Some( GraphemeClusterSegmenter::try_new_with_buffer_provider( &fs )? );
                // Below left as comment until figured out how to use BufferProvider implementations for properties.
                /*
                let fs_provider = fs.as_deserializing();
                syntax = Some( load_pattern_syntax( &fs_provider )? ); // <---- issue here
                white_space = Some( load_pattern_white_space( &fs_provider )? ); // <---- issue here
                */
            },
            #[allow( unreachable_patterns )]
            _ => {}
        };
        Ok( IcuBufferProvider {
            data_provider,
            grapheme_segmenter,
            syntax,
            white_space,
        } )
    }

    /// Get the `DataProvider` enum.
    /// See `i18n_lexer` crate on usage.
    pub fn data_provider( &self ) -> &DataProvider {
        &self.data_provider
    }

    /// Get the Grapheme Cluster Segmenter from preloaded character data set.
    /// See `i18n_lexer` crate on usage.
    pub fn grapheme_segmenter( &self ) -> Option<&GraphemeClusterSegmenter> {
        match &self.grapheme_segmenter {
            None => return None,
            Some( value ) => return Some( &value )
        }
    }

    /// Get the Pattern_Syntax data from preloaded character data set.
    pub fn syntax( &self ) -> Option<&CodePointSetData> {
        match &self.syntax {
            None => return None,
            Some( value ) => return Some( &value )
        }
    }

    /// Get the Pattern_White_Space data from preloaded character data set.
    pub fn white_space( &self ) -> Option<&CodePointSetData> {
        match &self.white_space {
            None => return None,
            Some( value ) => return Some( &value )
        }
    }
}
