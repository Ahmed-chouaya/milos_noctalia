//! Users and groups configuration generator.
//!
//! This module generates user and group configuration files.

use askama::Template;
use std::path::PathBuf;

use crate::generator::{Generator, GeneratedFile, GeneratorError};
use crate::generator::context::UserConfig;
use crate::generator::validate::validate_no_unsubstituted;

#[derive(Template)]
#[template(path = "users.nix")]
struct UsersContext {
    username: String,
    full_name: String,
    git_email: String,
    git_username: String,
}

#[derive(Debug)]
pub struct UsersGenerator;

impl UsersGenerator {
    /// Validate generated users.nix content.
    ///
    /// # Arguments
    /// * `content` - The generated content to validate.
    ///
    /// # Returns
    /// Returns `Ok(())` if no unsubstituted placeholders remain.
    pub fn validate(&self, content: &str) -> Result<(), GeneratorError> {
        validate_no_unsubstituted(content, "users.nix")
    }
}

impl Generator for UsersGenerator {
    fn generate(&self, config: &UserConfig) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let context = UsersContext {
            username: config.username.clone(),
            full_name: config.full_name.clone(),
            git_email: config.git_email.clone(),
            git_username: config.git_username.clone(),
        };
        let content = context.render()?;
        Ok(vec![GeneratedFile {
            path: PathBuf::from("users.nix"),
            content,
        }])
    }

    fn output_base_path(&self, _config: &UserConfig) -> PathBuf {
        PathBuf::from("modules")
    }
}
