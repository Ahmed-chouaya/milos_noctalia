//! # milos_niri
//! NixOS TUI Installer - Interactive guided installer for Niri + Noctalia setup

pub mod state;
pub mod event;
pub mod wizard;
pub mod logo;
pub mod error;
pub mod generator;

// Re-exports for convenience
pub use state::{WizardState, Step, ErrorMode, SharedState};
pub use event::{Event, EventHandler, NavigationAction};
