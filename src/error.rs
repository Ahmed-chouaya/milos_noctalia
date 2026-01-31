//! # Error Handling
//!
//! This module provides beautiful error handling with:
//! - Input validation errors (inline, with suggestions)
//! - System errors (modal overlay, blocking navigation)
//! - Backtrace toggle using color-eyre
//! - Context-dependent modal actions (Retry, Cancel, Exit)

use std::error::Error;
use std::fmt;
use std::sync::Arc;
use ratatui::{
    widgets::{Block, Borders, Paragraph},
    layout::Rect,
    Frame,
    style::{Style, Color, Modifier},
};

/// Error types that can occur in the wizard
#[derive(Clone, Debug)]
pub enum ErrorType {
    /// Input validation error with field name, message, and optional suggestion
    InputValidation {
        field: String,
        message: String,
        suggestion: Option<String>,
    },
    /// Disk-related error (e.g., device not found, permission denied)
    DiskError {
        message: String,
        device: Option<String>,
    },
    /// Network-related error (e.g., connection failed)
    NetworkError {
        message: String,
        operation: String,
    },
    /// Configuration error (e.g., invalid setting)
    ConfigError {
        message: String,
        setting: String,
    },
    /// Generic error
    Other {
        message: String,
    },
    /// Error from a std::error::Error source
    Source {
        message: String,
        source: Option<Arc<dyn Error + Send + Sync>>,
    },
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::InputValidation { field: _, message, suggestion } => {
                write!(f, "{}", message)?;
                if let Some(suggestion) = suggestion {
                    write!(f, "\nSuggestion: {}", suggestion)?;
                }
                Ok(())
            }
            ErrorType::DiskError { message, device } => {
                write!(f, "Disk Error: {}", message)?;
                if let Some(device) = device {
                    write!(f, "\nDevice: {}", device)?;
                }
                Ok(())
            }
            ErrorType::NetworkError { message, operation } => {
                write!(f, "Network Error during {}: {}", operation, message)?;
                Ok(())
            }
            ErrorType::ConfigError { message, setting } => {
                write!(f, "Config Error for '{}': {}", setting, message)?;
                Ok(())
            }
            ErrorType::Other { message } => {
                write!(f, "Error: {}", message)?;
                Ok(())
            }
            ErrorType::Source { message, .. } => {
                write!(f, "{}", message)?;
                Ok(())
            }
        }
    }
}

/// Actions available in error modal
#[derive(Clone, Debug, PartialEq)]
pub enum ErrorAction {
    /// Retry the failed operation (for recoverable errors)
    Retry,
    /// Cancel the operation (for optional operations)
    Cancel,
    /// Exit the application (for fatal errors)
    Exit,
    /// Dismiss the error (for non-blocking warnings)
    Dismiss,
    /// Toggle backtrace visibility
    ToggleBacktrace,
}

/// Modal for displaying errors
#[derive(Clone, Debug)]
pub struct ErrorModal {
    /// The type of error
    pub error_type: ErrorType,
    /// Whether the backtrace is currently visible
    pub backtrace_visible: bool,
    /// Currently selected action button
    pub selected_action: ErrorAction,
    /// Source error for color-eyre backtrace
    pub source_error: Option<Arc<dyn Error + Send + Sync>>,
}

impl ErrorModal {
    /// Create a new error modal from an ErrorType
    pub fn new(error_type: ErrorType) -> Self {
        Self {
            error_type,
            backtrace_visible: false,
            selected_action: ErrorAction::Dismiss,
            source_error: None,
        }
    }

    /// Create from a std::error::Error
    pub fn from_error<E: Error + 'static + Send + Sync>(error: E) -> Self {
        Self {
            error_type: ErrorType::Source {
                message: error.to_string(),
                source: Some(Arc::new(error)),
            },
            backtrace_visible: false,
            selected_action: ErrorAction::Dismiss,
            source_error: None,
        }
    }

    /// Get available actions for this error type
    pub fn available_actions(&self) -> Vec<ErrorAction> {
        match &self.error_type {
            ErrorType::InputValidation { .. } => vec![ErrorAction::Dismiss],
            ErrorType::DiskError { .. } => vec![ErrorAction::Retry, ErrorAction::Exit],
            ErrorType::NetworkError { .. } => vec![ErrorAction::Retry, ErrorAction::Cancel, ErrorAction::Exit],
            ErrorType::ConfigError { .. } => vec![ErrorAction::Cancel, ErrorAction::Exit],
            ErrorType::Other { .. } => vec![ErrorAction::Dismiss, ErrorAction::Exit],
            ErrorType::Source { .. } => vec![ErrorAction::Retry, ErrorAction::Exit],
        }
    }

    /// Check if the error is fatal (cannot continue)
    pub fn is_fatal(&self) -> bool {
        matches!(
            &self.error_type,
            ErrorType::DiskError { .. }
                | ErrorType::NetworkError { .. }
                | ErrorType::ConfigError { .. }
        )
    }

    /// Get the user-friendly message
    pub fn user_message(&self) -> String {
        match &self.error_type {
            ErrorType::InputValidation { field, message, suggestion } => {
                let mut msg = format!("Invalid {}: {}", field, message);
                if let Some(suggestion) = suggestion {
                    msg.push_str(&format!("\nHint: {}", suggestion));
                }
                msg
            }
            ErrorType::DiskError { message, .. } => message.clone(),
            ErrorType::NetworkError { message, .. } => message.clone(),
            ErrorType::ConfigError { message, .. } => message.clone(),
            ErrorType::Other { message } => message.clone(),
            ErrorType::Source { .. } => "An error occurred".to_string(),
        }
    }

    /// Handle keyboard input in the modal
    pub fn handle_input(&mut self, key_code: crossterm::event::KeyCode, _key_modifiers: crossterm::event::KeyModifiers) -> Option<ErrorAction> {
        match key_code {
            crossterm::event::KeyCode::Tab => {
                self.cycle_action();
                None
            }
            crossterm::event::KeyCode::Left => {
                self.cycle_action_backward();
                None
            }
            crossterm::event::KeyCode::Right => {
                self.cycle_action();
                None
            }
            crossterm::event::KeyCode::Enter => Some(self.selected_action.clone()),
            crossterm::event::KeyCode::Char('t') | crossterm::event::KeyCode::Char('d') => {
                self.backtrace_visible = !self.backtrace_visible;
                None
            }
            crossterm::event::KeyCode::Esc => {
                // Default to Dismiss on Escape
                Some(ErrorAction::Dismiss)
            }
            _ => None,
        }
    }

    /// Cycle to next available action
    fn cycle_action(&mut self) {
        let actions = self.available_actions();
        if let Some(current_idx) = actions.iter().position(|a| a == &self.selected_action) {
            let next_idx = (current_idx + 1) % actions.len();
            self.selected_action = actions[next_idx].clone();
        } else if !actions.is_empty() {
            self.selected_action = actions[0].clone();
        }
    }

    /// Cycle to previous available action
    fn cycle_action_backward(&mut self) {
        let actions = self.available_actions();
        if let Some(current_idx) = actions.iter().position(|a| a == &self.selected_action) {
            let prev_idx = if current_idx == 0 { actions.len() - 1 } else { current_idx - 1 };
            self.selected_action = actions[prev_idx].clone();
        } else if !actions.is_empty() {
            self.selected_action = actions[0].clone();
        }
    }
}

/// Render an error modal overlay
pub fn render_error_modal(
    frame: &mut Frame,
    modal: &ErrorModal,
    area: Rect,
) {
    // Create centered modal area
    let modal_width = std::cmp::min(area.width.saturating_sub(4), 60);
    let modal_height = std::cmp::min(area.height.saturating_sub(4), if modal.backtrace_visible { 20 } else { 12 });

    let modal_area = Rect::new(
        area.x + (area.width - modal_width) / 2,
        area.y + (area.height - modal_height) / 2,
        modal_width,
        modal_height,
    );

    // Semi-transparent overlay
    let overlay = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::NONE);

    frame.render_widget(overlay, area);

    // Modal block
    let modal_block = Block::default()
        .title(" Error ")
        .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(Color::Rgb(20, 20, 20)));

    frame.render_widget(modal_block, modal_area);

    // Inner area for content
    let inner_area = Rect::new(
        modal_area.x + 2,
        modal_area.y + 2,
        modal_area.width - 4,
        modal_area.height - 4,
    );

    // Error message
    let message = modal.user_message();
    let message_para = Paragraph::new(message)
        .style(Style::default().fg(Color::White))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(message_para, Rect::new(inner_area.x, inner_area.y, inner_area.width, 4));

    // Toggle backtrace hint
    let toggle_hint = if modal.backtrace_visible {
        "[H] Hide backtrace"
    } else {
        "[T] Show backtrace"
    };

    let toggle_para = Paragraph::new(toggle_hint)
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(toggle_para, Rect::new(inner_area.x, inner_area.y + 5, inner_area.width, 1));

    // Action buttons
    let actions = modal.available_actions();
    let buttons_area = Rect::new(
        inner_area.x,
        inner_area.y + inner_area.height - 3,
        inner_area.width,
        3,
    );

    let button_width = 12;
    let total_button_width = actions.len() as u16 * button_width + (actions.len().saturating_sub(1) as u16);
    let button_start_x = buttons_area.x + (buttons_area.width - total_button_width) / 2;

    for (i, action) in actions.iter().enumerate() {
        let button_x = button_start_x + (i as u16 * (button_width + 1));
        let button_area = Rect::new(button_x, buttons_area.y, button_width, 3);

        let label = match action {
            ErrorAction::Retry => "[ Retry ]",
            ErrorAction::Cancel => "[ Cancel ]",
            ErrorAction::Exit => "[  Exit  ]",
            ErrorAction::Dismiss => "[ Dismiss ]",
            ErrorAction::ToggleBacktrace => "[ Toggle ]",
        };

        let is_selected = *action == modal.selected_action;
        let button_style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let button = Paragraph::new(label)
            .style(button_style)
            .block(Block::default().borders(Borders::ALL).border_style(
                if is_selected {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::DarkGray)
                }
            ));

        frame.render_widget(button, button_area);
    }
}

/// Render inline validation error below a field
pub fn render_inline_error(
    frame: &mut Frame,
    message: &str,
    area: Rect,
) {
    let error_para = Paragraph::new(message)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::ITALIC))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(error_para, area);
}

/// Helper to create a validation error
pub fn validation_error(field: &str, message: &str, suggestion: Option<&str>) -> ErrorType {
    ErrorType::InputValidation {
        field: field.to_string(),
        message: message.to_string(),
        suggestion: suggestion.map(|s| s.to_string()),
    }
}

/// Helper to create a disk error
pub fn disk_error(message: &str, device: Option<&str>) -> ErrorType {
    ErrorType::DiskError {
        message: message.to_string(),
        device: device.map(|s| s.to_string()),
    }
}

/// Helper to create a network error
pub fn network_error(operation: &str, message: &str) -> ErrorType {
    ErrorType::NetworkError {
        message: message.to_string(),
        operation: operation.to_string(),
    }
}

/// Helper to create a config error
pub fn config_error(setting: &str, message: &str) -> ErrorType {
    ErrorType::ConfigError {
        message: message.to_string(),
        setting: setting.to_string(),
    }
}
