//! Generator error types.
//!
//! This module defines the `GeneratorError` enum which represents all possible
//! errors that can occur during configuration generation.

use thiserror::Error;

/// Errors that can occur during configuration generation.
#[derive(Debug, Error)]
pub enum GeneratorError {
    /// Error occurred while rendering a template.
    #[error("Template rendering error: {0}")]
    TemplateError(#[from] askama::Error),

    /// A required configuration value was missing.
    #[error("Required value missing: {0}")]
    ValueMissing(&'static str),

    /// Validation failed for a configuration value.
    #[error("Validation failed: {0}")]
    Validation(String),

    /// Failed to write a file to disk.
    #[error("Failed to write file '{path}': {source}")]
    FileWrite {
        /// The path to the file that failed to write.
        path: std::path::PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },

    /// A template placeholder was not substituted.
    #[error("Unsubstituted placeholder in template '{0}': {1}")]
    UnsubstitutedPlaceholder(&'static str, String),

    /// An unknown error occurred.
    #[error("Unknown error: {0}")]
    Unknown(String),
}
