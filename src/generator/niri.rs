//! Niri compositor configuration generator.
//!
//! This module generates Niri compositor configuration files.

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;

#[derive(Debug)]
pub struct NiriGenerator;

impl Generator for NiriGenerator {
    fn generate(&self, _config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        // TODO: Implement Niri configuration generation
        Ok(vec![])
    }

    fn output_base_path(&self, _config: &UserConfig) -> std::path::PathBuf {
        std::path::PathBuf::from("/etc/nixos")
    }
}
