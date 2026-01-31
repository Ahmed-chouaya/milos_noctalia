//! Noctalia shell configuration generator.
//!
//! This module generates Noctalia shell configuration files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;
use crate::generator::validate::validate_no_unsubstituted;

/// Wrapper that implements the traits Askama needs for conditionals
#[derive(Clone, Copy)]
struct BoolWrapper(bool);

impl BoolWrapper {
    fn new(b: bool) -> Self {
        BoolWrapper(b)
    }
}

/// For displaying string values
#[derive(Clone)]
struct StringWrapper(String);

impl StringWrapper {
    fn new(s: Option<String>) -> Self {
        StringWrapper(s.unwrap_or_default())
    }
}

impl std::fmt::Display for StringWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Template)]
#[template(path = "noctalia.nix", escape = "none")]
struct NoctaliaContext {
    wallpaper_dir: String,
    avatar_path: StringWrapper,
    avatar_path_set: BoolWrapper,
}

#[derive(Debug)]
pub struct NoctaliaGenerator;

impl NoctaliaGenerator {
    /// Validate generated noctalia.nix content.
    ///
    /// # Arguments
    /// * `content` - The generated content to validate.
    ///
    /// # Returns
    /// Returns `Ok(())` if no unsubstituted placeholders remain.
    pub fn validate(&self, content: &str) -> Result<(), GeneratorError> {
        validate_no_unsubstituted(content, "noctalia.nix")
    }
}

impl Generator for NoctaliaGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = NoctaliaContext {
            wallpaper_dir: config.wallpaper_dir.clone(),
            avatar_path: StringWrapper::new(config.avatar_path.clone()),
            avatar_path_set: BoolWrapper::new(config.avatar_path.is_some()),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("noctalia.nix"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("modules")
    }

    fn template_name(&self) -> &'static str {
        "noctalia.nix"
    }
}
