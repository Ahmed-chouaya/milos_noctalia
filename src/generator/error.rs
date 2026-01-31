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
    #[error("Unsubstituted placeholder in template '{template}': {placeholders}")]
    UnsubstitutedPlaceholders {
        /// The template name.
        template: String,
        /// The unsubstituted placeholders found.
        placeholders: String,
    },

    /// Nix syntax validation failed.
    #[error("Nix syntax error: {0}")]
    NixSyntax(String),

    /// Regex error during validation.
    #[error("Regex error during validation: {0}")]
    RegexError(String),

    /// Strip prefix error when computing relative paths.
    #[error("Path error: {0}")]
    PathError(String),

    /// An unknown error occurred.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<regex::Error> for GeneratorError {
    fn from(e: regex::Error) -> Self {
        GeneratorError::RegexError(e.to_string())
    }
}

impl From<std::path::StripPrefixError> for GeneratorError {
    fn from(e: std::path::StripPrefixError) -> Self {
        GeneratorError::PathError(e.to_string())
    }
}

impl From<atomicwrites::Error<std::io::Error>> for GeneratorError {
    fn from(e: atomicwrites::Error<std::io::Error>) -> Self {
        GeneratorError::FileWrite {
            path: std::path::PathBuf::from("unknown"),
            source: e.into(),
        }
    }
}
