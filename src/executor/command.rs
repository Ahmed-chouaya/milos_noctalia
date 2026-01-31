use std::path::{Path, PathBuf};

//! Command builders for nixos-rebuild and git operations.
//!
//! This module provides convenient builder functions for creating configured
/// Build a command for running nixos-rebuild switch with a flake.
///
/// # Arguments
///
/// * `flake_path` - Path to the flake directory containing flake.nix
///
/// # Returns
///
/// A configured `std::process::Command` ready to execute.
///
/// # Example
///
/// ```ignore
/// use executor::command::nixos_rebuild_cmd;
///
/// let mut cmd = nixos_rebuild_cmd("/home/user/milos");
/// // cmd is now configured as: nixos-rebuild switch --flake /home/user/milos
/// ```
pub fn nixos_rebuild_cmd(flake_path: &Path) -> std::process::Command {
    let mut cmd = std::process::Command::new("nixos-rebuild");
    cmd.arg("switch").arg("--flake").arg(flake_path);
    cmd
}

/// Build a command for initializing a git repository.
///
/// # Arguments
///
/// * `directory` - The directory where git init should be run
///
/// # Returns
///
/// A configured `std::process::Command` for git init.
pub fn git_init_cmd(directory: &std::path::Path) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("-C").arg(directory).arg("init");
    cmd
}

/// Build a command for staging files in git.
///
/// # Arguments
///
/// * `path` - The file or directory to stage
///
/// # Returns
///
/// A configured `std::process::Command` for git add.
pub fn git_add_cmd(path: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("add").arg(path);
    cmd
}

/// Build a command for creating a git commit.
///
/// # Arguments
///
/// * `message` - The commit message
///
/// # Returns
///
/// A configured `std::process::Command` for git commit.
///
/// # Example
///
/// ```ignore
/// use executor::command::git_commit_cmd;
///
/// let cmd = git_commit_cmd("Add generated NixOS configuration");
/// // cmd is now configured as: git commit -m "Add generated NixOS configuration"
/// ```
pub fn git_commit_cmd(message: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("commit").arg("-m").arg(message);
    cmd
}

/// Build a command for configuring git user.
///
/// # Arguments
///
/// * `name` - The user name to set
/// * `email` - The user email to set
///
/// # Returns
///
/// A configured `std::process::Command` for git config.
pub fn git_config_cmd(name: &str, email: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    cmd.arg("config").arg("--global").arg("user.name").arg(name);
    cmd
}

/// Build a command for checking git status.
///
/// # Arguments
///
/// * `directory` - The directory to check (optional)
///
/// # Returns
///
/// A configured `std::process::Command` for git status.
pub fn git_status_cmd(directory: Option<&std::path::Path>) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    if let Some(dir) = directory {
        cmd.arg("-C").arg(dir);
    }
    cmd.arg("status");
    cmd
}

/// Build a command for showing the current git commit.
///
/// # Arguments
///
/// * `directory` - The directory to check (optional)
///
/// # Returns
///
/// A configured `std::process::Command` for git show.
pub fn git_show_head_cmd(directory: Option<&std::path::Path>) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    if let Some(dir) = directory {
        cmd.arg("-C").arg(dir);
    }
    cmd.arg("show").arg("HEAD").arg("--stat");
    cmd
}

/// Build a command for adding a remote to a git repository.
///
/// # Arguments
///
/// * `name` - The remote name (e.g., "origin")
/// * `url` - The remote URL
/// * `directory` - The directory to operate on (optional)
///
/// # Returns
///
/// A configured `std::process::Command` for git remote add.
pub fn git_remote_add_cmd(
    name: &str,
    url: &str,
    directory: Option<&std::path::Path>,
) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    if let Some(dir) = directory {
        cmd.arg("-C").arg(dir);
    }
    cmd.arg("remote").arg("add").arg(name).arg(url);
    cmd
}

/// Build a command for pushing to a git remote.
///
/// # Arguments
///
/// * `remote` - The remote to push to
/// * `branch` - The branch to push
/// * `directory` - The directory to operate on (optional)
///
/// # Returns
///
/// A configured `std::process::Command` for git push.
pub fn git_push_cmd(
    remote: &str,
    branch: &str,
    directory: Option<&std::path::Path>,
) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    if let Some(dir) = directory {
        cmd.arg("-C").arg(dir);
    }
    cmd.arg("push").arg(remote).arg(branch);
    cmd
}

/// Build a command for getting git user information.
///
/// # Arguments
///
/// * `directory` - The directory to check (optional)
///
/// # Returns
///
/// A configured `std::process::Command` for git config.
pub fn git_get_user_cmd(directory: Option<&std::path::Path>) -> std::process::Command {
    let mut cmd = std::process::Command::new("git");
    if let Some(dir) = directory {
        cmd.arg("-C").arg(dir);
    }
    cmd.arg("config").arg("--list").arg("--show-origin");
    cmd
}
