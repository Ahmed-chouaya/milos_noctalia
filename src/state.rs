use std::sync::{Arc, RwLock};
use std::time::Duration;

/// Represents the different steps in the wizard
#[derive(Clone, Debug, PartialEq)]
pub enum Step {
    Welcome,
    Locale,
    Keyboard,
    Network,
    Disk,
    User,
    Packages,
    Summary,
}

impl Step {
    /// Get the display title for this step
    pub fn title(&self) -> &'static str {
        match self {
            Step::Welcome => "Welcome",
            Step::Locale => "Language & Locale",
            Step::Keyboard => "Keyboard Layout",
            Step::Network => "Network",
            Step::Disk => "Disk Selection",
            Step::User => "User Account",
            Step::Packages => "Package Selection",
            Step::Summary => "Review & Install",
        }
    }

    /// Get the step index
    pub fn index(&self) -> usize {
        match self {
            Step::Welcome => 0,
            Step::Locale => 1,
            Step::Keyboard => 2,
            Step::Network => 3,
            Step::Disk => 4,
            Step::User => 5,
            Step::Packages => 6,
            Step::Summary => 7,
        }
    }

    /// Get all steps in order
    pub fn all_steps() -> Vec<Step> {
        vec![
            Step::Welcome,
            Step::Locale,
            Step::Keyboard,
            Step::Network,
            Step::Disk,
            Step::User,
            Step::Packages,
            Step::Summary,
        ]
    }
}

/// Type alias for StepData trait object
pub trait StepData: Send + Sync {
    fn is_complete(&self) -> bool;
    fn validate(&self) -> Result<(), String>;
}

/// Error mode for handling different error types
#[derive(Clone, Debug)]
pub enum ErrorMode {
    InputValidation {
        field: String,
        message: String,
        suggestion: Option<String>,
    },
    SystemError {
        message: String,
        recoverable: bool,
    },
}

/// The main wizard state - centralized storage for all wizard data
#[derive(Clone, Debug)]
pub struct WizardState {
    /// Current step index (0-based)
    pub current_step: usize,

    /// Which steps have been completed
    pub completed_steps: Vec<bool>,

    /// Current error mode (if any)
    pub error_mode: Option<ErrorMode>,

    /// Field that has input focus (for validation errors)
    pub focused_field: Option<String>,

    // Step data fields
    pub hostname: Option<String>,
    pub username: Option<String>,
    pub full_name: Option<String>,
    pub password: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub keyboard_layout: Option<String>,
    pub disk_device: Option<String>,
    pub enable_ssh: bool,
    pub ssh_credentials: Option<(String, String)>,
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
            hostname: None,
            username: None,
            full_name: None,
            password: None,
            locale: None,
            timezone: None,
            keyboard_layout: None,
            disk_device: None,
            enable_ssh: false,
            ssh_credentials: None,
        }
    }

    /// Get the current step
    pub fn current_step(&self) -> Step {
        Step::all_steps()[self.current_step]
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
    pub fn set_error(&mut self, error: ErrorMode) {
        self.error_mode = Some(error);
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
    pub fn current_error(&self) -> Option<&ErrorMode> {
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

    /// Validate a field and set error if invalid
    pub fn validate_field(&mut self, field: &str, value: Option<String>, validator: fn(Option<&String>) -> Result<(), String>) {
        let result = validator(value.as_ref());
        if let Err(e) = result {
            self.set_error(ErrorMode::InputValidation {
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

/// Shared state wrapper for event loop
pub type SharedState = Arc<RwLock<WizardState>>;
