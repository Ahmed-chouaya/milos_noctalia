use std::sync::{Arc, RwLock};

use crate::error::{ErrorModal, ErrorType};

/// Represents the different steps in the wizard
#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    Welcome,
    Timezone,
    Keyboard,
    Account,
    Paths,
    Summary,
    Generate,  // Configuration generation step
    Execution,  // Run git commit and nixos-rebuild
    Completion,  // Final success screen
}

impl Step {
    /// Get the display title for this step
    pub fn title(&self) -> &'static str {
        match self {
            Step::Welcome => "Welcome",
            Step::Timezone => "Timezone",
            Step::Keyboard => "Keyboard Layout",
            Step::Account => "Account",
            Step::Paths => "Paths",
            Step::Summary => "Review",
            Step::Generate => "Generate Configuration",
            Step::Execution => "Apply Configuration",
            Step::Completion => "Complete",
        }
    }

    /// Get the step index
    pub fn index(&self) -> usize {
        match self {
            Step::Welcome => 0,
            Step::Timezone => 1,
            Step::Keyboard => 2,
            Step::Account => 3,
            Step::Paths => 4,
            Step::Summary => 5,
            Step::Generate => 6,
            Step::Execution => 7,
            Step::Completion => 8,
        }
    }

    /// Get all steps in order
    pub fn all_steps() -> Vec<Step> {
        vec![
            Step::Welcome,
            Step::Timezone,
            Step::Keyboard,
            Step::Account,
            Step::Paths,
            Step::Summary,
            Step::Generate,
            Step::Execution,
            Step::Completion,
        ]
    }
}

/// Type alias for StepData trait object
pub trait StepData: Send + Sync {
    fn is_complete(&self) -> bool;
    fn validate(&self) -> Result<(), String>;
}

/// The main wizard state - centralized storage for all wizard data
#[derive(Clone, Debug)]
pub struct WizardState {
    /// Current step index (0-based)
    pub current_step: usize,

    /// Which steps have been completed
    pub completed_steps: Vec<bool>,

    /// Current error mode (if any)
    pub error_mode: Option<ErrorModal>,

    /// Field that has input focus (for validation errors)
    pub focused_field: Option<String>,

    /// Index of focused field in AccountStep (0: hostname, 1: username, 2: full_name, 3: git_username, 4: git_email)
    pub focused_field_index: usize,

    /// Validation error message to display at step bottom
    pub validation_error: Option<String>,

    // Step data fields
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub git_username: Option<String>,
    pub git_email: Option<String>,
    pub password: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub keyboard_layout: Option<String>,
    pub disk_device: Option<String>,
    pub enable_ssh: bool,
    pub ssh_credentials: Option<(String, String)>,
    pub wallpaper_dir: Option<String>,
    pub avatar_path: Option<String>,  // Optional - can be None
    pub screenshot_dir: Option<String>,
}

impl WizardState {
    /// Create a new wizard state with defaults
    pub fn new() -> Self {
        let total_steps = Step::all_steps().len();
        Self {
            current_step: 0,
            completed_steps: vec![false; total_steps],
            error_mode: None,
            focused_field: None,
            focused_field_index: 0,
            validation_error: None,
            hostname: None,
            username: None,
            full_name: None,
            git_username: None,
            git_email: None,
            password: None,
            locale: None,
            timezone: None,
            keyboard_layout: None,
            disk_device: None,
            enable_ssh: false,
            ssh_credentials: None,
            wallpaper_dir: Some("~/Pictures/Wallpapers".to_string()),
            avatar_path: None,  // Optional, user can leave empty
            screenshot_dir: Some("~/Pictures/Screenshots".to_string()),
        }
    }

    /// Get the current step
    pub fn current_step(&self) -> Step {
        Step::all_steps()[self.current_step].clone()
    }

    /// Get current step mutably (for data collection)
    pub fn current_step_mut(&mut self) -> &mut dyn StepData {
        // This is a simplified implementation
        // In a full implementation, each step would have its own data struct
        self
    }

    /// Mark the current step as complete
    pub fn mark_step_complete(&mut self) {
        if self.current_step < self.completed_steps.len() {
            self.completed_steps[self.current_step] = true;
        }
    }

    /// Navigate to a specific step
    pub fn go_to_step(&mut self, step: usize) -> Result<(), String> {
        let total_steps = Step::all_steps().len();
        if step >= total_steps {
            return Err(format!("Step {} does not exist", step));
        }

        // Check if we can navigate forward (only if current step is complete)
        if step > self.current_step && !self.is_current_step_complete() {
            return Err("Current step must be complete before proceeding".to_string());
        }

        self.current_step = step;
        Ok(())
    }

    /// Go to the next step
    pub fn go_next(&mut self) -> Result<(), String> {
        if !self.is_current_step_complete() {
            return Err("Current step must be complete before proceeding".to_string());
        }
        self.go_to_step(self.current_step + 1)
    }

    /// Go to the previous step
    pub fn go_back(&mut self) {
        if self.current_step > 0 {
            self.current_step -= 1;
        }
    }

    /// Check if we can go back
    pub fn can_go_back(&self) -> bool {
        self.current_step > 0
    }

    /// Check if we can go forward
    pub fn can_go_forward(&self) -> bool {
        self.is_current_step_complete() && self.current_step < Step::all_steps().len() - 1
    }

    /// Check if current step is complete
    pub fn is_current_step_complete(&self) -> bool {
        if self.current_step < self.completed_steps.len() {
            self.completed_steps[self.current_step]
        } else {
            false
        }
    }

    /// Check if all steps are complete
    pub fn is_complete(&self) -> bool {
        self.completed_steps.iter().all(|&completed| completed)
    }

    /// Set an error
    pub fn set_error(&mut self, error: ErrorModal) {
        self.error_mode = Some(error);
    }

    /// Set an error from ErrorType
    pub fn set_error_type(&mut self, error_type: ErrorType) {
        self.error_mode = Some(ErrorModal::new(error_type));
    }

    /// Clear the current error
    pub fn clear_error(&mut self) {
        self.error_mode = None;
        self.focused_field = None;
    }

    /// Check if there's an error
    pub fn has_error(&self) -> bool {
        self.error_mode.is_some()
    }

    /// Get current error
    pub fn current_error(&self) -> Option<&ErrorModal> {
        self.error_mode.as_ref()
    }

    /// Set focus to a field
    pub fn set_focus(&mut self, field: &str) {
        self.focused_field = Some(field.to_string());
    }

    /// Get focus field
    pub fn focused_field(&self) -> Option<&String> {
        self.focused_field.as_ref()
    }

    /// Validate the hostname field
    pub fn validate_hostname(&self) -> Result<(), String> {
        if let Some(hostname) = &self.hostname {
            if hostname.is_empty() {
                return Err("Hostname cannot be empty".to_string());
            }
            if hostname.contains(' ') {
                return Err("Hostname cannot contain spaces".to_string());
            }
            if hostname.len() > 63 {
                return Err("Hostname cannot be longer than 63 characters".to_string());
            }
            Ok(())
        } else {
            Err("Hostname is required".to_string())
        }
    }

    /// Validate the username field
    pub fn validate_username(&self) -> Result<(), String> {
        if let Some(username) = &self.username {
            if username.is_empty() {
                return Err("Username cannot be empty".to_string());
            }
            if !username.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
                return Err("Username must be lowercase alphanumeric with underscores".to_string());
            }
            if username.starts_with('_') {
                return Err("Username cannot start with underscore".to_string());
            }
            Ok(())
        } else {
            Err("Username is required".to_string())
        }
    }

    /// Validate the full name field
    pub fn validate_full_name(&self) -> Result<(), String> {
        if let Some(full_name) = &self.full_name {
            if full_name.is_empty() {
                return Err("Full name cannot be empty".to_string());
            }
            Ok(())
        } else {
            Err("Full name is required".to_string())
        }
    }

    /// Validate the git username field
    pub fn validate_git_username(&self) -> Result<(), String> {
        if let Some(git_username) = &self.git_username {
            if git_username.is_empty() {
                return Err("Git username cannot be empty".to_string());
            }
            if git_username.len() > 39 {
                return Err("Git username cannot exceed 39 characters".to_string());
            }
            if !git_username.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                return Err("Git username can only contain alphanumeric characters and hyphens".to_string());
            }
            Ok(())
        } else {
            Err("Git username is required".to_string())
        }
    }

    /// Validate the git email field
    pub fn validate_git_email(&self) -> Result<(), String> {
        if let Some(git_email) = &self.git_email {
            if git_email.is_empty() {
                return Err("Git email cannot be empty".to_string());
            }
            // Basic email validation regex
            if !git_email.contains('@') {
                return Err("Git email must be a valid email address".to_string());
            }
            if !git_email.contains('.') {
                return Err("Git email must be a valid email address".to_string());
            }
            Ok(())
        } else {
            Err("Git email is required".to_string())
        }
    }

    /// Validate a path field (required)
    pub fn validate_path(&self, path: &Option<String>, field_name: &str) -> Result<(), String> {
        if let Some(path) = path {
            if path.is_empty() {
                return Err(format!("{} cannot be empty", field_name));
            }
            // Basic sanity check for tilde expansion
            if path.starts_with('~') && path.len() == 1 {
                return Err(format!("{} needs a path after the tilde (e.g., ~/Pictures)", field_name));
            }
            Ok(())
        } else {
            Err(format!("{} is required", field_name))
        }
    }

    /// Validate the wallpaper directory field
    pub fn validate_wallpaper_dir(&self) -> Result<(), String> {
        self.validate_path(&self.wallpaper_dir, "Wallpaper directory")
    }

    /// Validate the avatar path field (optional)
    pub fn validate_avatar_path(&self) -> Result<(), String> {
        // Avatar path is optional - None is valid
        if self.avatar_path.is_none() {
            return Ok(());
        }
        // If provided, validate it's not empty
        self.validate_path(&self.avatar_path, "Avatar path")
    }

    /// Validate the screenshot directory field
    pub fn validate_screenshot_dir(&self) -> Result<(), String> {
        self.validate_path(&self.screenshot_dir, "Screenshot directory")
    }

    /// Validate a field and set error if invalid
    pub fn validate_field(&mut self, field: &str, value: Option<String>, validator: fn(Option<&String>) -> Result<(), String>) {
        let result = validator(value.as_ref());
        if let Err(e) = result {
            self.set_error_type(ErrorType::InputValidation {
                field: field.to_string(),
                message: e,
                suggestion: None,
            });
            self.set_focus(field);
        } else {
            self.clear_error();
        }
    }
}

impl Default for WizardState {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement StepData for WizardState to allow dynamic dispatch
impl StepData for WizardState {
    fn is_complete(&self) -> bool {
        self.is_complete()
    }

    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Shared state wrapper for event loop
pub type SharedState = Arc<RwLock<WizardState>>;
