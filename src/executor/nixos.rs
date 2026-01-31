use std::path::PathBuf;
use crate::executor::{run_command, ExecutorError};

/// Status of a NixOS rebuild operation
#[derive(Debug, Clone, PartialEq)]
pub enum RebuildStatus {
    Pending,
    Preparing,
    Downloading,
    Building,
    Activating,
    Success,
    Failed { exit_code: i32, error_summary: String },
    Cancelled,
}

/// A generation entry from nixos-rebuild list-generations
#[derive(Debug, Clone)]
pub struct Generation {
    pub number: i32,
    pub date: String,
    pub current: bool,
}

/// Executor for NixOS rebuild operations
#[derive(Debug)]
pub struct NixOSExecutor {
    flake_path: PathBuf,
    hostname: String,
}

impl NixOSExecutor {
    /// Create new executor for the given flake and hostname
    pub fn new(flake_path: PathBuf, hostname: String) -> Self {
        Self { flake_path, hostname }
    }
    
    /// Check if we're on NixOS (can only run on NixOS)
    pub fn check_nixos() -> Result<bool, ExecutorError> {
        let os_release = std::fs::read_to_string("/etc/os-release")
            .map_err(|e| ExecutorError::IoError {
                message: format!("Failed to read /etc/os-release: {}", e),
            })?;
        
        let is_nixos = os_release.contains("ID=nixos") || os_release.contains("ID_LIKE=nixos");
        Ok(is_nixos)
    }
    
    /// Check if nixos-rebuild is available
    pub fn nixos_rebuild_available() -> bool {
        run_command("nixos-rebuild", &["--version"], None).is_ok()
    }
    
    /// Get rebuild command configured for sudo nixos-rebuild switch --flake
    fn rebuild_command(&self) -> std::process::Command {
        let flake_arg = format!("{}#{}", self.flake_path.display(), self.hostname);
        let mut cmd = std::process::Command::new("sudo");
        cmd.args(&["nixos-rebuild", "switch", "--flake", &flake_arg, "--impure"]);
        cmd
    }
    
    /// Run rebuild with streaming output and status detection
    pub fn rebuild(&self) -> Result<RebuildStatus, ExecutorError> {
        let mut cmd = self.rebuild_command();
        
        // Configure for streaming
        let mut child = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ExecutorError::CommandFailed {
                command: format!("{:?}", cmd),
                exit_code: None,
                stderr: e.to_string(),
            })?;
        
        let mut status = RebuildStatus::Pending;
        let mut stdout_lines = Vec::new();
        let mut stderr_lines = Vec::new();
        
        // Read output with phase detection
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        
        let (tx, rx) = std::sync::mpsc::channel();
        
        // Spawn stdout reader
        let tx_stdout = tx.clone();
        let _stdout_thread = std::thread::spawn(move || {
            let reader = std::io::BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    let _ = tx_stdout.send(("out".to_string(), line_content));
                }
            }
        });
        
        // Spawn stderr reader
        let tx_stderr = tx.clone();
        let _stderr_thread = std::thread::spawn(move || {
            let reader = std::io::BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line_content) = line {
                    let _ = tx_stderr.send(("err".to_string(), line_content));
                }
            }
        });
        
        // Process output lines with phase detection
        for (stream, line) in rx {
            let normalized = line.to_lowercase();
            
            // Update status based on output
            if normalized.contains("building") || normalized.contains("building derivation") {
                status = RebuildStatus::Building;
            } else if normalized.contains("downloading") || normalized.contains("fetching") {
                status = RebuildStatus::Downloading;
            } else if normalized.contains("activating") || normalized.contains("switching") {
                status = RebuildStatus::Activating;
            } else if normalized.contains("error") || normalized.contains("failed") {
                status = RebuildStatus::Failed {
                    exit_code: 1,
                    error_summary: line.clone(),
                };
            }
            
            if stream == "out" {
                stdout_lines.push(line);
            } else {
                stderr_lines.push(line);
            }
        }
        
        // Wait for child and get exit code
        let exit_status = child.wait()?;
        
        // Determine final status
        if status == RebuildStatus::Failed {
            // Already set from output
        } else if exit_status.success() {
            status = RebuildStatus::Success;
        } else {
            status = RebuildStatus::Failed {
                exit_code: exit_status.code().unwrap_or(1),
                error_summary: stderr_lines.last().cloned().unwrap_or_default(),
            };
        }
        
        Ok(status)
    }
    
    /// Get list of generations before rebuild
    pub fn capture_generations(&self) -> Result<Vec<Generation>, ExecutorError> {
        let output = run_command("nixos-rebuild", &["list-generations"], None)?;
        let mut generations = Vec::new();
        
        for line in output.lines {
            // Parse lines like: "1    2024-01-01 12:00:00"
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let Ok(num) = parts[0].parse::<i32>() {
                    generations.push(Generation {
                        number: num,
                        date: parts[1..3].join(" "),
                        current: line.contains("*"),
                    });
                }
            }
        }
        
        Ok(generations)
    }
    
    /// Rollback to previous generation
    pub fn rollback(&self) -> Result<RebuildStatus, ExecutorError> {
        let mut cmd = std::process::Command::new("sudo");
        cmd.args(&["nixos-rebuild", "switch", "--rollback"]);
        
        let mut child = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| ExecutorError::CommandFailed {
                command: "sudo nixos-rebuild switch --rollback".to_string(),
                exit_code: None,
                stderr: e.to_string(),
            })?;
        
        let _ = child.wait()?;
        Ok(RebuildStatus::Success)
    }
    
    /// Detect current phase from nixos-rebuild output line
    fn detect_phase(line: &str) -> Option<RebuildStatus> {
        let normalized = line.to_lowercase();
        if normalized.contains("building") || normalized.contains("building derivation") {
            Some(RebuildStatus::Building)
        } else if normalized.contains("downloading") || normalized.contains("fetching") {
            Some(RebuildStatus::Downloading)
        } else if normalized.contains("activating") || normalized.contains("switching") {
            Some(RebuildStatus::Activating)
        } else if normalized.contains("error") || normalized.contains("failed") {
            Some(RebuildStatus::Failed {
                exit_code: 1,
                error_summary: line.to_string(),
            })
        } else {
            None
        }
    }
    
    /// Format output line for TUI display
    fn format_for_tui(line: &str) -> String {
        // Truncate long lines
        let max_len = 120;
        if line.len() > max_len {
            line[..max_len].to_string() + "..."
        } else {
            line.to_string()
        }
    }
}
