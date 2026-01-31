---
status: testing
phase: 01-tui-foundation
source: 01-01-SUMMARY.md, 01-02-SUMMARY.md, 01-03-SUMMARY.md, 01-04-SUMMARY.md, 01-05-SUMMARY.md
started: 2026-01-31T12:41:00Z
updated: 2026-01-31T12:41:00Z
---

## Current Test

number: 2
name: Binary runs
expected: |
  Run `./target/debug/milos_niri` (or `cargo run --bin milos_niri`)
  Expected: Binary starts and displays the startup screen with "Press ENTER to begin" prompt
awaiting: user response

## Tests

### 1. Build compiles
expected: `cargo check` passes without errors, showing Ratatui, Crossterm, color-eyre dependencies resolved
result: pass

### 2. Binary runs
expected: Run `./target/debug/milos_niri` - binary starts, shows "Press ENTER to begin" prompt
result: pending

### 3. Animated logo displays
expected: On startup, MILOS letters appear one-by-one (typewriter effect) in neon green with amber glow, followed by blinking cursor
result: pending

### 4. Navigation works
expected: Press Enter to begin, arrow keys navigate between Locale/Keyboard options, Enter confirms selection
result: pending

### 5. Step progression
expected: Completing a step (Enter on selection) marks it complete and moves to next step. Back button (Escape) returns to previous step
result: pending

### 6. Sidebar shows progress
expected: Left sidebar shows all steps with ✓ (completed), ▶ (current), ○ (pending) markers and step titles
result: pending

### 7. Error modal appears
expected: Enter invalid data (hostname with spaces), error modal appears blocking navigation, shows user-friendly message
result: pending

### 8. Error actions work
expected: In error modal, Tab/Left/Right cycles buttons, Enter selects action (Dismiss/Retry/Exit), 't' toggles backtrace
result: pending

## Summary

total: 8
passed: 1
issues: 0
pending: 7
skipped: 0

## Gaps

[none yet]
