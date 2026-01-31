//! Output types for streaming command execution.
//!
//! This module provides types for capturing and streaming command output
/// A single line of output from a command execution.
///
/// Contains the line content, whether it's from stderr, and a timestamp.
#[derive(Debug, Clone)]
pub struct OutputLine {
    /// The text content of the line
    pub line: String,
    /// Whether this line came from stderr (true) or stdout (false)
    pub is_stderr: bool,
    /// When this line was captured
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl OutputLine {
    /// Create a new stdout line.
    pub fn stdout(line: String) -> Self {
        Self {
            line,
            is_stderr: false,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a new stderr line.
    pub fn stderr(line: String) -> Self {
        Self {
            line,
            is_stderr: true,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Get the line content.
    pub fn content(&self) -> &str {
        self.line.as_str()
    }

    /// Check if this is a stderr line.
    pub fn is_stderr(&self) -> bool {
        self.is_stderr
    }

    /// Check if this is a stdout line.
    pub fn is_stdout(&self) -> bool {
        !self.is_stderr
    }
}

/// A receiver for streaming output lines.
///
/// This struct wraps an mpsc receiver and provides a convenient interface
/// for receiving output lines as they are produced by a running command.
#[derive(Debug)]
pub struct OutputStream {
    /// The receiver for output lines
    receiver: std::sync::mpsc::Receiver<OutputLine>,
}

impl OutputStream {
    /// Create a new OutputStream from a receiver.
    pub fn new(receiver: std::sync::mpsc::Receiver<OutputLine>) -> Self {
        Self { receiver }
    }

    /// Receive the next output line.
    ///
    /// This will block until a line is available.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(OutputLine))` if a line was received,
    /// or `Ok(None)` if the sender has been disconnected.
    pub fn recv(&self) -> Result<Option<OutputLine>, std::sync::mpsc::RecvError> {
        self.receiver.recv().map(Some)
    }

    /// Try to receive the next output line without blocking.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(OutputLine))` if a line was available,
    /// `Ok(None)` if no line was available yet, or `Err` if the sender disconnected.
    pub fn try_recv(&self) -> Result<Option<OutputLine>, std::sync::mpsc::TryRecvError> {
        self.receiver.try_recv().map(Some)
    }

    /// Get an iterator over all available lines.
    ///
    /// This will return all lines that are currently available without blocking.
    /// Use `recv()` in a loop if you want to wait for new lines.
    pub fn try_iter(&self) -> TryOutputLineIterator<'_> {
        TryOutputLineIterator { stream: self }
    }

    /// Get the underlying receiver for advanced use cases.
    pub fn receiver(&self) -> &std::sync::mpsc::Receiver<OutputLine> {
        &self.receiver
    }
}

/// An iterator over available output lines.
#[derive(Debug)]
pub struct TryOutputLineIterator<'a> {
    stream: &'a OutputStream,
}

impl<'a> Iterator for TryOutputLineIterator<'a> {
    type Item = OutputLine;

    fn next(&mut self) -> Option<Self::Item> {
        self.stream.try_recv().unwrap_or(None)
    }
}
