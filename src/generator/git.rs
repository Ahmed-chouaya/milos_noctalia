//! Git configuration generator.
//!
//! This module generates Git configuration files for the user.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;
use crate::generator::validate::validate_no_unsubstituted;

#[derive(Template)]
#[template(path = "git.nix")]
struct GitContext {
    full_name: String,
    git_email: String,
    git_username: String,
}

#[derive(Debug)]
pub struct GitGenerator;

impl GitGenerator {
    /// Get the template name for validation.
    pub fn template_name(&self) -> &'static str {
        "git.nix"
    }
}

impl Generator for GitGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = GitContext {
            full_name: config.full_name.clone(),
            git_email: config.git_email.clone(),
            git_username: config.git_username.clone(),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("git.nix"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("modules")
    }

    fn template_name(&self) -> &'static str {
        "git.nix"
    }
}
