---
status: resolved
trigger: "After fixing systemd-boot freeze with consoleMode, rebooted and now stuck at bootloader GUI. TTY shows error: Failed to start Home Manager environment for milgraph"
created: 2026-02-01T00:00:00Z
updated: 2026-02-01T00:00:00Z
symptoms_prefilled: true
---

## Current Focus

hypothesis: Home Manager service failing due to configuration error or missing dependency
test: Check previous changes made to home-manager config and identify what could cause service failure
expecting: Find configuration issue in home.nix or related files that prevents home-manager service from starting
next_action: Review the boot.nix changes and any home-manager configuration changes made

## Symptoms

expected: After rebuild and reboot, system should boot to desktop/login manager
actual: Stuck at bootloader GUI after password entry, can access TTY. Home Manager service fails to start.
errors: "Failed to start Home Manager environment for milgraph. See 'systemctl status home-manager-milgraph.service'"
reproduction: Rebuild with consoleMode fix → reboot → stuck at bootloader, Home Manager service failure
started: After applying systemd-boot consoleMode fix

## Eliminated

- hypothesis: systemd-boot consoleMode freeze (original issue)
  evidence: Previously fixed with consoleMode = "0", but now different symptom (Home Manager failure)
  timestamp: 2026-02-01

## Evidence

- timestamp: 2026-02-01
  checked: Previous debug session - boot-menu-password-stuck.md
  found: Applied fix: boot.loader.systemd-boot.consoleMode = "0" in modules/system/boot.nix
  implication: The consoleMode fix was applied, but now Home Manager service fails

## Resolution

root_cause: home/modules/desktop/niri.nix has incorrect path - references ../../dotfiles/niri/config.kdl but file is at home/dotfiles/niri/config.kdl (one directory up, not two)
fix: Fixed path from ../../dotfiles/niri/config.kdl to ../dotfiles/niri/config.kdl in home/modules/desktop/niri.nix
verification: Pending - user needs to rebuild and test
files_changed:
  - home/modules/desktop/niri.nix: Fixed dotfiles path from ../../ to ../
  - home/milgraph.nix: Re-enabled niri.nix import (was temporarily commented)
