//! User configuration context for generation.
//!
//! This module provides the `UserConfig` struct which contains all the
//! user-provided configuration values collected during the wizard flow.
//! It also provides the conversion from `WizardState` to `UserConfig`.

use crate::state::WizardState;

/// The user configuration extracted from the wizard state.
///
/// This struct contains all the values needed by generators to produce
/// configuration files. All fields are guaranteed to be non-empty Strings
/// (optional fields like `avatar_path` use `Option<String>`).
#[derive(Debug, Clone, PartialEq)]
pub struct UserConfig {
    /// The machine hostname.
    pub hostname: String,

    /// The primary user username.
    pub username: String,

    /// The user's full name (for GECOS/comment field).
    pub full_name: String,

    /// Git username for commit authorship.
    pub git_username: String,

    /// Git email for commit authorship.
    pub git_email: String,

    /// Selected timezone (e.g., "America/New_York").
    pub timezone: String,

    /// Selected keyboard layout (e.g., "us", "de", "fr").
    pub keyboard_layout: String,

    /// Directory for wallpapers.
    pub wallpaper_dir: String,

    /// Optional path to user avatar image.
    pub avatar_path: Option<String>,

    /// Directory for screenshots.
    pub screenshot_dir: String,
}

impl UserConfig {
    /// Create a new `UserConfig` from a `WizardState`.
    ///
    /// This constructor extracts all the relevant fields from the wizard state
    /// and converts them to the format expected by generators.
    ///
    /// # Arguments
    /// * `state` - The wizard state to convert from.
    ///
    /// # Returns
    /// A new `UserConfig` with all fields populated from the wizard state.
    ///
    /// # Panics
    /// This function will panic if any required field in the wizard state is `None`.
    /// In normal operation, the wizard should validate all fields before allowing
    /// the user to proceed to generation.
    #[must_use]
    pub fn from_wizard_state(state: &WizardState) -> Self {
        Self {
            hostname: state.hostname.clone().unwrap_or_else(|| {
                panic!("hostname must be set before generation")
            }),
            username: state.username.clone().unwrap_or_else(|| {
                panic!("username must be set before generation")
            }),
            full_name: state.full_name.clone().unwrap_or_else(|| {
                panic!("full_name must be set before generation")
            }),
            git_username: state.git_username.clone().unwrap_or_else(|| {
                panic!("git_username must be set before generation")
            }),
            git_email: state.git_email.clone().unwrap_or_else(|| {
                panic!("git_email must be set before generation")
            }),
            timezone: state.timezone.clone().unwrap_or_else(|| {
                panic!("timezone must be set before generation")
            }),
            keyboard_layout: state.keyboard_layout.clone().unwrap_or_else(|| {
                panic!("keyboard_layout must be set before generation")
            }),
            wallpaper_dir: state.wallpaper_dir.clone().unwrap_or_else(|| {
                panic!("wallpaper_dir must be set before generation")
            }),
            avatar_path: state.avatar_path.clone(),
            screenshot_dir: state.screenshot_dir.clone().unwrap_or_else(|| {
                panic!("screenshot_dir must be set before generation")
            }),
        }
    }
}
