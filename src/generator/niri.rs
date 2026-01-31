//! Niri compositor configuration generator.
//!
//! This module generates Niri compositor configuration files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;

#[derive(Template)]
#[template(path = "niri/config.kdl")]
struct NiriContext {
    username: String,
    screenshot_dir: String,
}

#[derive(Debug)]
pub struct NiriGenerator;

impl Generator for NiriGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = NiriContext {
            username: config.username.clone(),
            screenshot_dir: config.screenshot_dir.clone(),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("config.kdl"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("niri")
    }
}
