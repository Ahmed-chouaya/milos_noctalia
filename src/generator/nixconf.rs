//! General nix.conf configuration generator.
//!
//! This module generates general NixOS configuration files.

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;

#[derive(Debug)]
pub struct NixConfGenerator;

impl Generator for NixConfGenerator {
    fn generate(&self, _config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        // TODO: Implement nix.conf generation
        Ok(vec![])
    }

    fn output_base_path(&self, _config: &UserConfig) -> std::path::PathBuf {
        std::path::PathBuf::from("/etc/nixos")
    }
}
