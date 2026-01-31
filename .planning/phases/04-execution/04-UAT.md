---
status: testing
phase: 04-execution
source: 04-01-SUMMARY.md, 04-02-SUMMARY.md, 04-03-SUMMARY.md, 04-04-SUMMARY.md, 04-05-SUMMARY.md
started: 2026-01-31T21:30:00Z
updated: 2026-01-31T21:40:00Z
---

## Current Test

number: 2
name: Account Step Works
expected: |
  User can enter hostname, username, full name, and Git credentials. Input validation works (no spaces in hostname/username). User can navigate with arrow keys, Tab, Enter.
awaiting: user response

## Tests

### 1. Script Launches Installer
expected: User runs `./milos` from the cloned milos_niri directory. Script auto-detects hostname from /etc/hostname, builds if needed, and launches the TUI installer showing the MILOS logo and Welcome step.
result: issue
reported: "Build errors: missing chrono, private exports, wrong run_command signature, PathBuf not imported, string type mismatch"
severity: blocker

### 2. Account Step Works
expected: User can enter hostname, username, full name, and Git credentials. Input validation works (no spaces in hostname/username). User can navigate with arrow keys, Tab, Enter.
result: pending

### 3. Timezone & Keyboard Selection Works
expected: User can scroll through region/city list to select timezone. User can select keyboard layout from common options. Type-to-filter works for searching.
result: pending

### 4. Path Configuration Works
expected: User can set wallpaper directory, optional avatar path, and screenshot path. Defaults are pre-filled and user can edit or accept them.
result: pending

### 5. Generate Step Creates Configs
expected: User completes Summary step, clicks Next. Generate step runs, shows progress. Generated configs appear in ./milos-output/ directory with flake.nix, users.nix, git.nix, locale.nix, noctalia.nix, niri.nix, nix.conf.
result: pending

### 6. Execution Commits Configs
expected: Execution step starts. Git repo is initialized (if needed), configs are staged, commit is created with message like "milos: Generate {hostname} config for {username}". User sees commit confirmation.
result: pending

### 7. Execution Rebuilds System
expected: After git commit, installer runs `nixos-rebuild switch --flake`. User sees real-time output with phase indicators: "Downloading", "Building", "Activating".
result: pending

### 8. Errors Displayed Gracefully
expected: If errors occur (e.g., validation failure, build error), user sees clear error message in color-eyre format, not raw Rust panics.
result: pending

### 9. Completion Screen Shows
expected: On successful rebuild, Completion step displays hostname, username, git repo location, and next steps. Enter exits the installer.
result: pending

### 10. Full Wizard Flow Complete
expected: User completes entire flow: Welcome → Account → Timezone → Paths → Summary → Generate → Execution → Completion. All steps render correctly, navigation works, data persists between steps.
result: pending

## Summary

total: 10
passed: 0
issues: 1
pending: 9
skipped: 0

## Gaps

- truth: "Script builds and launches installer successfully"
  status: failed
  reason: "User reported: Build errors - missing chrono, private exports, wrong run_command signature, PathBuf not imported, string type mismatch"
  severity: blocker
  test: 1
  root_cause: "Code executed via Task tool had issues - chrono missing from Cargo.toml, OutputLine and ExecutorError not properly exported, run_command signature missing working_dir parameter, command.rs missing PathBuf import, wizard.rs had &str in Vec<String>"
  artifacts:
    - path: "Cargo.toml"
      issue: "Missing chrono dependency"
    - path: "src/lib.rs"
      issue: "OutputLine and ExecutorError not properly exported"
    - path: "src/executor/mod.rs"
      issue: "run_command signature missing working_dir parameter"
    - path: "src/executor/command.rs"
      issue: "Missing PathBuf import"
    - path: "src/wizard.rs"
      issue: "&str in Vec<String>"
    - path: "src/executor/git.rs"
      issue: "Using old API, stdout field doesn't exist"
    - path: "src/executor/nixos.rs"
      issue: "Using old API, missing working_dir parameter"
  missing:
    - "Add chrono to Cargo.toml"
    - "Export OutputLine and ExecutorError from lib.rs"
    - "Add working_dir parameter to run_command and run_command_streaming"
    - "Add PathBuf import to command.rs"
    - "Add .to_string() to wizard.rs"
    - "Update git.rs to use new API with stdout_lines()"
    - "Update nixos.rs to use new API"
  debug_session: ""
