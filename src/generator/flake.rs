//! Flake configuration generator.
//!
//! This module generates the flake.nix and flake.lock files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;
use crate::generator::validate::validate_no_unsubstituted;

#[derive(Template)]
#[template(path = "flake.nix")]
struct FlakeContext {
    hostname: String,
    username: String,
    nixpkgs_ref: String,
}

#[derive(Debug)]
pub struct FlakeGenerator;

impl FlakeGenerator {
    /// Get the template name for validation.
    pub fn template_name(&self) -> &'static str {
        "flake.nix"
    }
}

impl Generator for FlakeGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = FlakeContext {
            hostname: config.hostname.clone(),
            username: config.username.clone(),
            nixpkgs_ref: "nixos-unstable".to_string(),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("flake.nix"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from(".")
    }

    fn template_name(&self) -> &'static str {
        "flake.nix"
    }
}
