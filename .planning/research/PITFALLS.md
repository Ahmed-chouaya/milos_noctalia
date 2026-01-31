# Pitfalls Research

**Domain:** NixOS TUI Installer
**Researched:** January 31, 2026
**Confidence:** MEDIUM-HIGH

## Critical Pitfalls

### Pitfall 1: SSH Connection Loss After Kexec/Reboot

**What goes wrong:**
After the installer transfers control via `exec` to the new kernel (kexec), the SSH connection drops permanently. The target machine becomes unreachable, and the installation stalls with repeated "Connection timed out" or "No route to host" errors. In severe cases, the target machine freezes entirely, requiring a hard reboot.

**Why it happens:**
The kexec process replaces the running kernel without a full reboot, but SSH connections don't survive this transition. The new system starts SSH on a different port configuration, or the network stack gets reinitialized differently. Additionally, the installer may not properly handle the timing between kexec initiating and SSH becoming available on the new system.

**Consequences:**
- Complete installation failure
- Target machine may be left in an unbootable or partially configured state
- User has no way to recover remotely
- Potential data loss if machine requires hard reset during partition operations

**How to avoid:**
- Implement SSH connection retry logic with exponential backoff (minimum 30 seconds, up to 5 minutes)
- Use `--ssh-timeout` flags appropriately with aggressive retry intervals
- Verify network connectivity post-kexec before declaring success
- Consider implementing a "heartbeat" mechanism that checks SSH availability after kexec
- Add a pre-kexec checkpoint that saves state so installation can be resumed

**Warning signs:**
- SSH commands show "Connection timed out" after kexec
- Target machine becomes unresponsive during installation
- Multiple SSH connection attempts fail with "No route to host"
- Installation logs show kexec succeeded but no subsequent SSH activity

**Phase to address:** Phase 2 (Remote Execution Infrastructure)

---

### Pitfall 2: Device Naming Instability During Disk Operations

**What goes wrong:**
The installer references disks by volatile names like `/dev/nvme0n1` or `/dev/sda`, but these can change between boots or depending on boot order. This causes disko to partition the wrong disk, install to a non-boot disk, or fail entirely with "device not found" errors.

**Why it happens:**
Linux device names are assigned based on PCI enumeration order, which can vary. The NixOS installer environment may see devices in a different order than the final installed system. Disko defaults to using by-path or by-id identifiers but users often override with simpler names for "clarity."

**Consequences:**
- Wrong disk partitioned and formatted (potential data loss on other drives)
- System fails to boot because bootloader installed to wrong device
- Configuration works in installer but fails after reboot
- "No such device" errors during partition operations

**How to avoid:**
- **Always** use stable identifiers: `/dev/disk/by-id/*` for all disk references
- Validate that specified devices exist and are writable before operations
- Display the detected device tree to users for verification before proceeding
- Implement a disk selection UI that shows model/serial numbers, not just device paths
- Save the chosen disk identifier to configuration for reproducibility

**Warning signs:**
- User sees device paths like `/dev/nvme0n1` instead of by-id paths
- Multiple disks present in the system
- Configuration doesn't persist disk selection across runs
- Installation succeeds but system doesn't boot

**Phase to address:** Phase 3 (Disk Partitioning & Configuration)

---

### Pitfall 3: Network Connectivity Detection Failures

**What goes wrong:**
The installer fails to detect or maintain internet connectivity, causing downloads to fail, flake evaluations to timeout, or substitute lookups to error out. Users see cryptic errors like "cannot connect to socket" or "HTTP error 200" with timeout messages.

**Why it happens:**
NixOS installer images sometimes have incomplete network configuration, especially for wireless or corporate networks. The installer may not properly handle network bring-up, DNS resolution, or proxy configurations. SSH-based installers rely on the target machine having network access which may not be configured.

**Consequences:**
- Inability to fetch nixpkgs or flake inputs
- Installation halts during evaluation phase
- Caches unavailable, causing long builds
- Flake lock file cannot be updated

**How to avoid:**
- Implement robust network detection with multiple fallback mechanisms
- Provide manual network configuration options (static IP, WiFi, proxy)
- Cache nixpkgs and inputs locally during installation
- Implement connection health checks before critical operations
- Allow offline installation mode with pre-cached inputs

**Warning signs:**
- `curl` or `wget` tests fail
- DNS resolution returns errors
- SSH connects but `nix copy` operations timeout
- Warnings about substituters being unavailable

**Phase to address:** Phase 1 (Core Infrastructure)

---

### Pitfall 4: Secrets Exposure in Installer Environment

**What goes wrong:**
SSH keys, disk encryption passwords, git credentials, or other secrets get written to the installer image, persist in `/nix/store` (which is world-readable), or get logged in plain text. This creates security vulnerabilities where unprivileged users or processes can access sensitive credentials.

**Why it happens:**
Nix store is globally readable by design - everything in `/nix/store` can be read by any user on the system. When secrets are passed as files, environment variables, or configuration options, they often end up in the store. Additionally, installer logs may capture sensitive data.

**Consequences:**
- Credential compromise for subsequent systems
- Disk encryption keys exposed to local users
- Git credentials accessible to any process on the system
- Violation of security expectations for enterprise deployments

**How to avoid:**
- Use sops-nix or agenix for secrets management, never raw files
- Ensure secrets are placed in non-store locations (`/persist`, `/run`, `/tmp` with restricted permissions)
- Implement secret redaction in all logging
- Useage the `mutableUsers = false` pattern to prevent users from reading each other's data
- For disk encryption, use TPM2 or similar hardware-backed key storage
- Clean up secrets from `/tmp` after use

**Warning signs:**
- Secrets stored in files that will be imported to `/nix/store`
- No secret redaction in installer logs
- Permissions on secret files are too permissive (644 instead of 600)
- Configuration uses plain-text passwords in option values

**Phase to address:** Phase 4 (Secrets & Credentials)

---

### Pitfall 5: Module Evaluation Failures with Cryptic Errors

**What goes wrong:**
The NixOS module system produces opaque error messages during evaluation, such as "Module imports can't be nested lists" or "attribute 'lib' missing". Users cannot understand what went wrong or how to fix it, leading to installation abandonment.

**Why it happens:**
The Nix module system has complex import semantics. Nested import lists, missing arguments to modules, type mismatches, or referencing undefined options all produce errors from deep within the evaluation stack. Error messages often show internal implementation details rather than user-friendly explanations.

**Consequences:**
- Installation fails during configuration evaluation
- Users cannot diagnose or fix configuration errors
- Time wasted on debugging cryptic error messages
- Frustration leads to abandoning NixOS entirely

**How to avoid:**
- Implement validation phases that catch common configuration errors early
- Provide user-friendly error messages with suggested fixes
- Validate module imports and arguments before evaluation
- Use `--show-trace` only for debugging, never in user-facing output
- Implement configuration linting that checks for known problematic patterns
- Create starter templates that are known to work, then validate against them

**Warning signs:**
- Error messages contain `/nix/store/*/lib/modules.nix`
- "while calling the 'seq' builtin" or similar internal terms
- "attribute 'X' missing" errors
- Configuration uses experimental or unstable options

**Phase to address:** Phase 2 (Configuration System)

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hardcoding disk device paths | Simpler configuration, faster development | Breaks on different hardware, causes data loss | Never for production |
| Skipping hardware detection | Faster initial implementation | Fails on non-standard configurations | Only for known-target deployments |
| Storing secrets in flake | Easier testing, no setup required | Security vulnerability, breaks in production | Only during initial development with explicit warnings |
| Using unstable nixpkgs | Access to latest features | Breaking changes, unpredictable behavior | Only with explicit version pinning in flake.lock |
| Single SSH connection | Simpler implementation | No resilience to network issues | Only for fully reliable networks |
| Skipping error recovery | Less code to write | Complete failure on any error | Never for installer use cases |
| No configuration validation | Faster initial development | Late failures with cryptic errors | Only for personal, throwaway configs |

---

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| SSH | Assuming connection survives reboot/kexec | Implement reconnection with retries and verification |
| Disko | Using volatile device names | Always use `/dev/disk/by-id/*` identifiers |
| Flakes | Not updating flake.lock before deployment | Run `nix flake update` and commit lock file |
| Home Manager | Configuration conflicts with system modules | Use proper imports and avoid option conflicts |
| Secure Boot | Not enrolling keys before reboot | Enroll keys before final system activation |
| ZFS | Creating pools without ashift=12 | Explicitly set ashift for optimal performance |
| LUKS | Missing TPM2 enrollment for auto-unlock | Configure `crypttabExtraOpts` for TPM |
| Git | Using uncommitted flake directory | Ensure flake is committed before installation |

---

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Evaluation without cache | Each rebuild re-downloads everything | Use `--impure` sparingly, prefer cached substituters | On slow/unreliable networks |
| No substituters configured | All packages built from source | Configure cachix or other binary caches | When building complex packages |
| Large nixpkgs checkout | Disk space fills during installation | Use minimal flake inputs, prune unused modules | On systems with limited storage |
| Too many modules imported | Slow evaluation, high memory usage | Split configuration, use imports strategically | With complex multi-host configurations |

---

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Secrets in nix store | Global readable credentials | Use sops-nix/agenix, store outside /nix/store |
| Mutable users enabled | Users can modify their accounts | Set `users.mutableUsers = false` |
| No disk encryption | Data accessible on stolen hardware | Require LUKS for all installations |
| SSH with password auth | Brute force attacks | Require key-based authentication |
| No firewall configuration | Exposed services | Configure `networking.firewall` |
| Unencrypted Git credentials | Credential theft | Use SSH keys or credential helpers |
| World-readable tmp files | Information disclosure | Use restricted permissions (600) |

---

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Silent failures | User doesn't know installation failed | Provide progress indicators and clear success/failure states |
| No abort option | User trapped in long operation | Implement SIGINT handling and graceful shutdown |
| Unclear disk selection | User formats wrong disk | Show model/serial numbers, require confirmation |
| No progress feedback | Uncertain wait times | Show current step, estimated time, percentage complete |
| Cryptic error messages | User cannot diagnose issues | Translate Nix errors to user-friendly explanations |
| No recovery option | Failure requires complete restart | Implement checkpointing and resume capability |
| Terminal resize breaks UI | Garbled display | Handle resize events, use flexible layouts |
| No help or documentation | User cannot complete installation | Integrate contextual help, link to documentation |

---

## "Looks Done But Isn't" Checklist

- [ ] **SSH Connection:** Tested reconnection after simulated network interruption, not just initial connection
- [ ] **Disk Operations:** Verified device path stability across reboots
- [ ] **Secrets:** All sensitive values tested with `find /nix/store -perm /111` to ensure non-readable
- [ ] **Network:** Tested with actual disconnection during operation, not just initial connectivity check
- [ ] **Error Recovery:** Verified resume capability after each failure point
- [ ] **Module Evaluation:** Tested configuration with intentional errors, verified user-friendly messages
- [ ] **Progress Feedback:** Verified all long operations (5+ seconds) show progress indication
- [ ] **Secure Boot:** Tested key enrollment and TPM unlock workflow
- [ ] **User Data:** Verified user configuration persists after reboot with `mutableUsers = false`
- [ ] **Logging:** Verified logs contain no secrets (tested with grep for common patterns)

---

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| SSH lost after kexec | HIGH | Requires physical access or out-of-band management to reboot and reconnect |
| Wrong disk formatted | HIGH | Data recovery from affected disk, may require complete reinstall |
| Network timeout during flake eval | MEDIUM | Re-run with `--retry` flag, verify network, check cache availability |
| Secrets exposed | HIGH | Rotate all exposed credentials, re-encrypt configurations |
| Module evaluation failure | LOW | Review error message, fix configuration, re-run |
| User lockout after install | MEDIUM | Boot installer, fix configuration, re-run `nixos-install` |
| Broken bootloader | MEDIUM | Boot installer, chroot, reinstall bootloader manually |
| Partial partition write | HIGH | May require disk wipe and complete reinstall |

---

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| SSH Connection Loss After Kexec | Phase 2: Remote Execution | Test kexec with simulated network interruption |
| Device Naming Instability | Phase 3: Disk Configuration | Verify by-id paths work across reboots |
| Network Detection Failures | Phase 1: Core Infrastructure | Test with intentionally broken network |
| Secrets Exposure | Phase 4: Secrets Management | Audit logs and store permissions |
| Module Evaluation Failures | Phase 2: Configuration System | Test with intentional bad config |
| ZFS/Disko Configuration | Phase 3: Disk Configuration | Test with multiple disk configurations |
| User Lockout | Phase 4: User Configuration | Test SSH access after fresh install |
| Progress/Feedback UI | Phase 1: Core Infrastructure | User testing with various failure scenarios |

---

## Sources

- [nix-community/nixos-anywhere Issue #112: exec causes SSH connection loss](https://github.com/nix-community/nixos-anywhere/issues/112) - Documented persistent SSH issues after kexec
- [DeterminateSystems/nix-installer Issue #1498: Nix Store password prompt on macOS](https://github.com/DeterminateSystems/nix-installer/issues/1498) - Secret management and keychain issues
- [nix-community/disko Issue #551: Partition confusion with by-partlabel](https://github.com/nix-community/disko/issues/551) - Device naming issues
- [nix-community/disko Issue #743: Wrong boot partition](https://github.com/nix-community/disko/issues/743) - Disk selection issues
- [Ryan Seipp: Unattended NixOS Installs](https://ryanseipp.com/posts/nixos-automated-deployment/) - Comprehensive lessons learned from automation
- [NixOS Wiki: Installation Guide](https://wiki.nixos.org/wiki/NixOS_Installation_Guide) - Official installation documentation
- [NixOS Discourse: Module import errors](https://discourse.nixos.org/t/error-when-trying-to-modularize-nixos-config-module-imports-cant-be-nested-lists-perhaps-you-meant-to-remove-one-level-of-lists-definitions-showdefs-defs/60509) - Common module errors
- [Michael Stapelberg: Secret Management with sops-nix](https://michael.stapelberg.ch/posts/2025-08-24-secret-management-with-sops-nix/) - Proper secrets handling
- [0xdade: Framework and NixOS - Sops-nix Secrets Management](https://0xda.de/blog/2024/07/framework-and-nixos-sops-nix-secrets-management/) - Enterprise secrets patterns
- [Ratatui FAQ: Duplicate key events on Windows](https://ratatui.rs/faq/) - Cross-platform TUI considerations
- [NixOS/nixpkgs Issue #360593: Internet detection failures](https://github.com/NixOS/nixpkgs/issues/360593) - Network detection problems

---

*Research for: NixOS TUI Installer*
*Researched: January 31, 2026*
