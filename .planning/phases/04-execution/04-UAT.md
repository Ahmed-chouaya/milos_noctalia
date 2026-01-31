---
status: testing
phase: 04-execution
source: 04-01-SUMMARY.md, 04-02-SUMMARY.md, 04-03-SUMMARY.md, 04-04-SUMMARY.md, 04-05-SUMMARY.md
started: 2026-01-31T21:30:00Z
updated: 2026-01-31T21:35:00Z
---

## Current Test

number: 1
name: Git Commit Created
expected: |
  User runs `./milos` from their NixOS config directory (with flake.nix).
  The script enters nix-shell, builds if needed, and launches the TUI installer.
  User can navigate through the full wizard flow: Welcome → Account → Timezone → Paths → Summary → Generate → Execution → Completion.
awaiting: user response

## Tests

### 1. Script Launches Installer
expected: User runs `./milos` from their NixOS config directory (with flake.nix). Script enters nix-shell, builds if needed, and launches the TUI installer showing the MILOS logo and Welcome step.
result: pending

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
issues: 0
pending: 10
skipped: 0

## Gaps

[none yet]
