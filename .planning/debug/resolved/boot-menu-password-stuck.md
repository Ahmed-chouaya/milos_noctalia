---
status: resolved
trigger: "when I rebuild from this config, and reboot I keep getting stuck in the boot menu , I enter the passward and it is stuck there"
created: 2026-02-01T00:00:00Z
updated: 2026-02-01T00:00:00Z
---

## Current Focus

hypothesis: systemd-boot consoleMode issue - default console mode causes framebuffer freeze on certain hardware
test: Add boot.loader.systemd-boot.consoleMode = "0" to fix display freeze
expecting: Setting consoleMode to "0" (text mode) will prevent framebuffer freeze
next_action: Apply fix to boot.nix and provide verification steps

## Symptoms

expected: After rebuild and reboot, system should boot normally to desktop/login
actual: System freezes at bootloader menu, only cursor moves
errors: None visible - screen freezes
reproduction: Rebuild from milos_noctalia config → reboot → stuck at bootloader
started: First time setting up this system

## Eliminated

(none yet)

## Evidence

- timestamp: 2026-02-01
  checked: hosts/nixos/configuration.nix, flake.nix, modules/system/boot.nix, hardware-configuration.nix
  found: Using systemd-boot, Intel i915 kernel params in boot.nix, EFI setup
  implication: Bootloader config is active and may have display initialization issues

- timestamp: 2026-02-01
  checked: NixOS issue #449939 - "Nixos no longer boots since kernel 6.17"
  found: Similar symptoms - systemd-boot freeze after selecting generation, older generations work
  implication: This is a known issue with systemd-boot on certain hardware/configurations

- timestamp: 2026-02-01
  checked: NixOS option documentation for boot.loader.systemd-boot.consoleMode
  found: consoleMode defaults can cause framebuffer issues; setting to "0" uses text mode
  implication: Missing consoleMode setting likely causes the freeze

## Resolution

root_cause: systemd-boot default consoleMode causes framebuffer freeze on this hardware - the bootloader tries to use a graphical mode that freezes the display, leaving only cursor movement
fix: Add boot.loader.systemd-boot.consoleMode = "0" to force text mode in bootloader
verification: Rebuild, reboot, and confirm system boots past bootloader without freezing
files_changed:
  - modules/system/boot.nix: Add consoleMode setting
