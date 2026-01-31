//! Locale and keyboard configuration generator.
//!
//! This module generates locale and keyboard configuration files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;
use crate::generator::validate::validate_no_unsubstituted;

#[derive(Template)]
#[template(path = "locale.nix")]
struct LocaleContext {
    timezone: String,
    keyboard_layout: String,
}

#[derive(Debug)]
pub struct LocaleGenerator;

impl LocaleGenerator {
    /// Validate generated locale.nix content.
    ///
    /// # Arguments
    /// * `content` - The generated content to validate.
    ///
    /// # Returns
    /// Returns `Ok(())` if no unsubstituted placeholders remain.
    pub fn validate(&self, content: &str) -> Result<(), GeneratorError> {
        validate_no_unsubstituted(content, "locale.nix")
    }
}

impl Generator for LocaleGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = LocaleContext {
            timezone: config.timezone.clone(),
            keyboard_layout: config.keyboard_layout.clone(),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("locale.nix"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("modules")
    }
}
