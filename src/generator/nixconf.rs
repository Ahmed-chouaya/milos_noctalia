//! General nix.conf configuration generator.
//!
//! This module generates general NixOS configuration files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;
use crate::generator::validate::validate_no_unsubstituted;

#[derive(Template)]
#[template(path = "nix.conf")]
struct NixConfContext {
    hostname: String,
    username: String,
}

#[derive(Debug)]
pub struct NixConfGenerator;

impl NixConfGenerator {
    /// Validate generated nix.conf content.
    ///
    /// # Arguments
    /// * `content` - The generated content to validate.
    ///
    /// # Returns
    /// Returns `Ok(())` if no unsubstituted placeholders remain.
    pub fn validate(&self, content: &str) -> Result<(), GeneratorError> {
        validate_no_unsubstituted(content, "nix.conf")
    }
}

impl Generator for NixConfGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = NixConfContext {
            hostname: config.hostname.clone(),
            username: config.username.clone(),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("nix.conf"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from(".")
    }
}
