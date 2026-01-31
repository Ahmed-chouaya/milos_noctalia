//! Noctalia shell configuration generator.
//!
//! This module generates Noctalia shell configuration files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;
use crate::generator::validate::validate_no_unsubstituted;

#[derive(Template)]
#[template(path = "noctalia.nix", escape = "none")]
struct NoctaliaContext {
    wallpaper_dir: String,
    avatar_path: Option<String>,
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
            avatar_path: config.avatar_path.clone(),
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
