// This file is part of `i18n_lexer-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_lexer-rizzen-yazston` crate.

#![allow(unexpected_cfgs)]

use crate::IcuError;

#[allow(unused_imports)]
use icu_properties::sets::CodePointSetData;

#[cfg(feature = "icu_compiled_data")]
use icu_properties::sets::{pattern_syntax, pattern_white_space};

#[cfg(feature = "buffer")]
#[allow(unused_imports)]
use icu_properties::{
    provider::{PatternSyntaxV1Marker, PatternWhiteSpaceV1Marker},
    sets::{load_pattern_syntax, load_pattern_white_space},
};

#[cfg(feature = "buffer")]
#[allow(unused_imports)]
use icu_provider::AsDeserializingBufferProvider;

#[cfg(feature = "blob")]
use icu_provider_blob::BlobDataProvider;

#[cfg(feature = "fs")]
use icu_provider_fs::FsDataProvider;

use icu_segmenter::GraphemeClusterSegmenter;

#[cfg(feature = "logging")]
use log::{debug, error};

#[cfg(doc)]
use icu_provider_blob::BlobDataProvider;

#[cfg(doc)]
use icu_provider_fs::FsDataProvider;

#[cfg(doc)]
use std::sync::{Arc, Mutex};

#[cfg(doc)]
use std::rc::Rc;

#[cfg(doc)]
use std::cell::RefCell;

/// Indicates which data provider to use for various supported ICU4X components:
///
/// * Internal (Preferred): Will use the internal BakedDateProvider of various ICU4X components. Requires the
///   `icu_compiled_data` feature. The internal data of ICU4X components are sufficient for most use cases needing
///   localisation, and is recommended by ICU4X.
///
/// * Blob: The [`BlobDataProvider`] will be used for the various ICU4X components. Requires the `blob` feature. An
///   alternative provider when the internal data of ICU4X components are insufficient for a particular use case.
///
/// * Fs: The [`FsDataProvider`] will be used for the various ICU4X components. Requires the `fs` feature. An
///   alternative provider when the internal data of ICU4X components are insufficient for a particular use case.
#[derive(Debug, Clone)]
pub enum DataProvider {
    #[cfg(feature = "icu_compiled_data")]
    Internal,

    #[cfg(feature = "blob")]
    Blob(BlobDataProvider),

    #[cfg(feature = "fs")]
    Fs(FsDataProvider),
}

/// The `IcuDataProvider` type containing the `DataProvider` enum.
///
/// `IcuDataProvider` type is normally used within the [`Rc`] type as `Rc<IcuDataProvider>` or [`Arc`] type as
/// `Arc<IcuDataProvider>` to prevent unnecessary duplication.
pub struct IcuDataProvider {
    data_provider: DataProvider,
    grapheme_segmenter: GraphemeClusterSegmenter,

    #[allow(dead_code)]
    syntax: CodePointSetData,

    #[allow(dead_code)]
    white_space: CodePointSetData,
}

impl IcuDataProvider {
    /// Create a `IcuDataProvider` instance. Depending on the DataProvider choice, the instance may include the
    /// Grapheme Cluster Segmenter and character properties sets.
    pub fn try_new(data_provider: DataProvider) -> Result<Self, IcuError> {
        // Temporary values.
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut grapheme_segmenter = None;

        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut syntax = None;

        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut white_space = None;

        #[allow(unreachable_code)]
        match data_provider.clone() {
            #[cfg(feature = "blob")]
            DataProvider::Blob(blob) => {
                #[cfg(feature = "logging")]
                debug!("BlobDataProvider was selected.");

                grapheme_segmenter = Some(GraphemeClusterSegmenter::try_new_with_buffer_provider(
                    &blob,
                )?);
                let blob_provider = blob.as_deserializing();
                syntax = Some(load_pattern_syntax(&blob_provider)?);
                white_space = Some(load_pattern_white_space(&blob_provider)?);
            }

            #[cfg(feature = "fs")]
            DataProvider::Fs(fs) => {
                #[cfg(feature = "logging")]
                debug!("FsDataProvider was selected.");

                grapheme_segmenter =
                    Some(GraphemeClusterSegmenter::try_new_with_buffer_provider(&fs)?);
                let fs_provider = fs.as_deserializing();
                syntax = Some(load_pattern_syntax(&fs_provider)?);
                white_space = Some(load_pattern_white_space(&fs_provider)?);
            }

            #[cfg(feature = "icu_compiled_data")]
            DataProvider::Internal => {
                #[cfg(feature = "logging")]
                debug!("Internal data was selected.");

                grapheme_segmenter = Some(GraphemeClusterSegmenter::new());
                syntax = Some(pattern_syntax().static_to_owned());
                white_space = Some(pattern_white_space().static_to_owned());
            }

            #[allow(unreachable_patterns)]
            _ => {}
        };

        // Do sanity check: None of `grapheme`, `white_space` or `syntax` can be `None`.
        if grapheme_segmenter.is_none() {
            #[cfg(feature = "logging")]
            error!("Missing grapheme segmenter.");

            return Err(IcuError::Syntax);
        }
        if syntax.is_none() {
            #[cfg(feature = "logging")]
            error!("Missing Pattern_Syntax properties.");

            return Err(IcuError::Syntax);
        }
        if white_space.is_none() {
            #[cfg(feature = "logging")]
            error!("Missing Pattern_White_Space properties.");

            return Err(IcuError::WhiteSpace);
        }
        Ok(IcuDataProvider {
            data_provider,
            grapheme_segmenter: grapheme_segmenter.unwrap(),
            syntax: syntax.unwrap(),
            white_space: white_space.unwrap(),
        })
    }

    /// Get the `DataProvider` enum.
    /// See `i18n_lexer` crate on usage.
    pub fn data_provider(&self) -> &DataProvider {
        &self.data_provider
    }

    /// Get the Grapheme Cluster Segmenter from preloaded character data set.
    /// See `i18n_lexer` crate on usage.
    pub fn grapheme_segmenter(&self) -> &GraphemeClusterSegmenter {
        &self.grapheme_segmenter
    }

    /// Get the Pattern_Syntax data from preloaded character data set.
    /// See `i18n_lexer` crate on usage.
    pub fn syntax(&self) -> &CodePointSetData {
        &self.syntax
    }

    /// Get the Pattern_White_Space data from preloaded character data set.
    /// See `i18n_lexer` crate on usage.
    pub fn white_space(&self) -> &CodePointSetData {
        &self.white_space
    }
}
