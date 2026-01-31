//! # milos_niri
//! NixOS TUI Installer - Interactive guided installer for Niri + Noctalia setup

pub mod state;
pub mod event;
pub mod wizard;
pub mod logo;
pub mod error;
pub mod generator;
pub mod executor;

// Re-exports for convenience
pub use state::{WizardState, Step, SharedState};
pub use event::{Event, EventHandler, NavigationAction};
pub use executor::{run_command, run_command_streaming, CommandOutput, executor::output::OutputLine, executor::error::ExecutorError};
