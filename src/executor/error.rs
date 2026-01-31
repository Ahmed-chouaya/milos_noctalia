//! Error types for the executor module.

use thiserror::Error;

/// Errors that can occur when executing commands.
#[derive(Debug, Error)]
pub enum ExecutorError {
    /// The command was executed but returned a non-zero exit code.
    #[error("Command '{command}' failed with exit code {exit_code:?}: {stderr}")]
    CommandFailed {
        /// The command that was executed
        command: String,
        /// The exit code returned (if available)
        exit_code: Option<i32>,
        /// The stderr output from the command
        stderr: String,
    },

    /// The command was not found or is not executable.
    #[error("Command not found: '{command}'")]
    CommandNotFound {
        /// The command that was not found
        command: String,
    },

    /// An I/O error occurred while executing the command.
    #[error("I/O error: {message}")]
    IoError {
        /// Description of the I/O error
        message: String,
    },

    /// An error occurred while capturing command output.
    #[error("Output capture error: {message}")]
    OutputCaptureError {
        /// Description of the output capture error
        message: String,
    },

    /// The command was killed or timed out.
    #[error("Command was killed: {message}")]
    CommandKilled {
        /// Description of why the command was killed
        message: String,
    },

    /// The command timed out.
    #[error("Command timed out after {timeout:?}: {command}")]
    CommandTimeout {
        /// The command that timed out
        command: String,
        /// The timeout duration
        timeout: std::time::Duration,
    },
}

impl ExecutorError {
    /// Create a CommandFailed error from a command, exit code, and stderr.
    pub fn command_failed(command: String, exit_code: Option<i32>, stderr: String) -> Self {
        Self::CommandFailed {
            command,
            exit_code,
            stderr,
        }
    }

    /// Create a CommandNotFound error.
    pub fn command_not_found(command: String) -> Self {
        Self::CommandNotFound { command }
    }

    /// Create an IoError.
    pub fn io_error(message: String) -> Self {
        Self::IoError { message }
    }

    /// Create an OutputCaptureError.
    pub fn output_capture_error(message: String) -> Self {
        Self::OutputCaptureError { message }
    }
}
