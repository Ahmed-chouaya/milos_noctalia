//! Executor module for running shell commands with streaming output capture.
//!
//! This module provides functionality to execute system commands (like nixos-rebuild and git)
//! with real-time output streaming for TUI display.

pub mod command;
pub mod error;
pub mod output;

use crate::executor::error::ExecutorError;
use crate::executor::output::{OutputLine, OutputStream};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Output from a executed command containing all captured output and metadata.
#[derive(Debug, Clone)]
pub struct CommandOutput {
    /// All lines captured from stdout and stderr
    pub lines: Vec<OutputLine>,
    /// The exit code returned by the command
    pub exit_code: Option<i32>,
    /// Whether the command was killed or failed to run
    pub success: bool,
    /// Duration of command execution
    pub duration: Duration,
    /// The command that was executed (for display purposes)
    pub command: String,
}

impl CommandOutput {
    /// Create a new CommandOutput
    pub fn new(command: String) -> Self {
        Self {
            lines: Vec::new(),
            exit_code: None,
            success: false,
            duration: Duration::ZERO,
            command,
        }
    }

    /// Add a line to the output
    pub fn add_line(&mut self, line: OutputLine) {
        self.lines.push(line);
    }

    /// Get all stdout lines
    pub fn stdout_lines(&self) -> Vec<&str> {
        self.lines.iter().filter(|l| !l.is_stderr).map(|l| l.line.as_str()).collect()
    }

    /// Get all stderr lines
    pub fn stderr_lines(&self) -> Vec<&str> {
        self.lines.iter().filter(|l| l.is_stderr).map(|l| l.line.as_str()).collect()
    }
}

/// Run a command with streaming output capture.
///
/// This function spawns a process and captures its stdout and stderr in real-time,
/// yielding output lines through a channel as they are produced.
///
/// # Arguments
///
/// * `program` - The path or name of the program to execute
/// * `args` - Arguments to pass to the program
///
/// # Returns
///
/// Returns `Ok(CommandOutput)` on success, or `Err(ExecutorError)` on failure.
///
/// # Example
///
/// ```ignore
/// use executor::run_command;
///
/// let output = run_command("echo", &["Hello, World!"])?;
/// assert!(output.success);
/// ```
pub fn run_command(program: &str, args: &[&str]) -> Result<CommandOutput, ExecutorError> {
    let start = Instant::now();
    let command_str = format!("{} {}", program, args.join(" "));

    let mut cmd = std::process::Command::new(program);
    cmd.args(args);

    // Setup pipes for stdout and stderr
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    // Spawn the process
    let mut child = cmd
        .spawn()
        .map_err(|e| ExecutorError::IoError {
            message: format!("Failed to spawn command '{}': {}", command_str, e),
        })?;

    // Create channel for output lines
    let (tx, rx) = mpsc::channel();

    // Clone handles for the reader threads
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    // Spawn thread to read stdout
    let tx_stdout = tx.clone();
    let _stdout_thread = thread::spawn(move || {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    let _ = tx_stdout.send(OutputLine::stdout(line_content));
                }
                Err(_) => {
                    // Reader closed, exit loop
                    break;
                }
            }
        }
    });

    // Spawn thread to read stderr
    let tx_stderr = tx.clone();
    let _stderr_thread = thread::spawn(move || {
        let reader = std::io::BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    let _ = tx_stderr.send(OutputLine::stderr(line_content));
                }
                Err(_) => {
                    // Reader closed, exit loop
                    break;
                }
            }
        }
    });

    // Collect all output lines
    let mut lines: Vec<OutputLine> = Vec::new();
    for received in rx {
        lines.push(received);
    }

    // Wait for the process to complete
    let exit_status = child
        .wait()
        .map_err(|e| ExecutorError::IoError {
            message: format!("Failed to wait for command '{}': {}", command_str, e),
        })?;

    let duration = start.elapsed();
    let exit_code = exit_status.code();

    Ok(CommandOutput {
        lines,
        exit_code,
        success: exit_status.success(),
        duration,
        command: command_str,
    })
}

/// Run a command and return an OutputStream for real-time output viewing.
///
/// This is useful when you want to display output as it's being produced
/// rather than waiting for the command to complete.
///
/// # Arguments
///
/// * `program` - The path or name of the program to execute
/// * `args` - Arguments to pass to the program
///
/// # Returns
///
/// Returns `Ok((OutputStream, JoinHandle<CommandOutput>))` where the handle
/// can be used to wait for completion and get the final output.
pub fn run_command_streaming(
    program: &str,
    args: &[&str],
) -> Result<(OutputStream, std::thread::JoinHandle<CommandOutput>), ExecutorError> {
    use crate::executor::output::OutputStream;
    use std::sync::mpsc;
    use std::thread;

    let command_str = format!("{} {}", program, args.join(" "));

    let mut cmd = std::process::Command::new(program);
    cmd.args(args);

    // Setup pipes for stdout and stderr
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    // Spawn the process
    let mut child = cmd
        .spawn()
        .map_err(|e| ExecutorError::IoError {
            message: format!("Failed to spawn command '{}': {}", command_str, e),
        })?;

    // Create channel for output lines
    let (tx, rx) = mpsc::channel();

    // Create OutputStream
    let stream = OutputStream::new(rx);

    // Clone handles for the reader threads
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    // Spawn thread to read stdout
    let tx_stdout = tx.clone();
    let _stdout_thread = thread::spawn(move || {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    let _ = tx_stdout.send(OutputLine::stdout(line_content));
                }
                Err(_) => {
                    break;
                }
            }
        }
    });

    // Spawn thread to read stderr
    let tx_stderr = tx.clone();
    let _stderr_thread = thread::spawn(move || {
        let reader = std::io::BufReader::new(stderr);
        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    let _ = tx_stderr.send(OutputLine::stderr(line_content));
                }
                Err(_) => {
                    break;
                }
            }
        }
    });

    let start = Instant::now();

    // Create handle that waits for completion
    let handle = thread::spawn(move || {
        // Wait for the process to complete
        let exit_status = child.wait().expect("Failed to wait for child");
        let duration = start.elapsed();
        let exit_code = exit_status.code();

        CommandOutput {
            lines: Vec::new(), // Lines already sent through stream
            exit_code,
            success: exit_status.success(),
            duration,
            command: command_str,
        }
    });

    Ok((stream, handle))
}
