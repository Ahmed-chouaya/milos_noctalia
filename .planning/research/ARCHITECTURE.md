# Architecture Research: NixOS TUI Installer

**Domain:** NixOS TUI Installer
**Researched:** 2025-01-31
**Confidence:** MEDIUM

## Standard Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        TUI Presentation Layer                            │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │   Wizard    │  │  Form Step  │  │  Progress   │  │  Summary    │     │
│  │  Navigator  │  │  Renderer   │  │  Display    │  │  View       │     │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘     │
│         │                │                │                │              │
├─────────┴────────────────┴────────────────┴────────────────┴─────────────┤
│                        State Management Layer                            │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                    Installation State Store                        │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐          │  │
│  │  │ User     │  │ System   │  │ Disk     │  │ Flake    │          │  │
│  │  │ Config   │  │ Config   │  │ Config   │  │ Config   │          │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘          │  │
│  └───────────────────────────────────────────────────────────────────┘  │
├─────────────────────────────────────────────────────────────────────────┤
│                        Configuration Generation Layer                    │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │   Flake     │  │  NixOS      │  │   Disk      │  │   Host      │     │
│  │  Template   │  │  Module     │  │  Config     │  │  Config     │     │
│  │  Engine     │  │  Generator  │  │  Generator  │  │  Generator  │     │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘     │
│         │                │                │                │              │
├─────────┴────────────────┴────────────────┴────────────────┴─────────────┤
│                        Execution Layer                                   │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │
│  │   Disk      │  │   User      │  │  Bootloader │  │   System    │     │
│  │  Partitioner│  │  Creator    │  │  Installer  │  │  Switcher   │     │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘     │
└─────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| **Wizard Navigator** | Manages step progression, validation gates, navigation state | State machine pattern with step enumeration |
| **Form Step Renderer** | Renders individual configuration screens (hostname, user, locale, git) | Ratatui widgets with validation callbacks |
| **Progress Display** | Shows installation progress with real-time feedback | Async task tracking with cancellable operations |
| **Summary View** | Reviews all collected settings before execution | Read-only form view with confirmation |
| **Installation State Store** | Centralized state for all collected configuration data | Singleton or context pattern |
| **Flake Template Engine** | Generates flake.nix from collected settings | Template substitution with Nix expression generation |
| **NixOS Module Generator** | Creates configuration.nix with imports | Nix attribute set building |
| **Disk Config Generator** | Creates partition table and mount options | Disko format generation |
| **Disk Partitioner** | Executes partitioning operations | Parted/sfdisk commands wrapped in actions |
| **User Creator** | Creates users and groups during installation | useradd/groupadd with nixos-users module |
| **Bootloader Installer** | Configures systemd-boot or GRUB | nixos-generate-config + nixos-install |
| **System Switcher** | Activates new system configuration | nixos-switch or reboot orchestration |

## Recommended Project Structure

```
src/
├── tui/                      # Terminal UI layer
│   ├── app.rs               # Main TUI application and event loop
│   ├── wizard.rs            # Step navigation and state machine
│   ├── steps/               # Individual form steps
│   │   ├── mod.rs
│   │   ├── hostname.rs      # Hostname input with validation
│   │   ├── user.rs          # Username, password, sudo access
│   │   ├── git.rs           # Git credentials configuration
│   │   ├── locale.rs        # Locale, timezone, keyboard layout
│   │   ├── paths.rs         # Home directory, data paths
│   │   ├── disk.rs          # Disk selection and partitioning
│   │   └── summary.rs       # Final review before install
│   ├── widgets/             # Reusable TUI components
│   │   ├── input.rs         # Validated text input
│   │   ├── select.rs        # Option selection with fuzzy search
│   │   ├── confirm.rs       # Yes/no confirmation dialogs
│   │   └── progress.rs      # Progress bar with status updates
│   └── theme.rs             # Color scheme and styling

├── state/                    # State management
│   ├── mod.rs               # State store public API
│   ├── config.rs            # InstallationConfig struct
│   ├── validation.rs        # Field validators
│   └── persistence.rs       # Save/load configuration to file

├── generator/                # Configuration generation
│   ├── mod.rs               # Generator public API
│   ├── flake.rs             # Flake.nix template engine
│   ├── nixos.rs             # Configuration.nix generation
│   ├── disk.rs              # Disk configuration (disko format)
│   └── hardware.rs          # Hardware-specific configuration

├── executor/                 # Installation execution
│   ├── mod.rs               # Executor public API
│   ├── partition.rs         # Disk partitioning operations
│   ├── user.rs              # User/group creation
│   ├── bootloader.rs        # Bootloader configuration
│   ├── install.rs           # nixos-install orchestration
│   └── rollback.rs          # Cleanup on failure

└── main.rs                  # CLI entry point with argument parsing
```

### Structure Rationale

- **tui/**: Isolates presentation concerns from business logic; enables testing UI components independently
- **state/**: Centralized state management following the singleton pattern; single source of truth
- **generator/**: Separates template logic from execution; enables preview before apply
- **executor/**: Groups destructive operations; facilitates rollback and error handling

## Architectural Patterns

### Pattern 1: State Machine Wizard Flow

**What:** Multi-step form navigation with validation gates that prevent progression until requirements are met.

**When to use:** For any TUI installer with sequential configuration steps where later steps depend on earlier selections.

**Trade-offs:**
- Pros: Clear progression model, prevents invalid states, enables back navigation
- Cons: More complex state management, requires careful step dependencies

**Example:**
```rust
// State machine definition
enum WizardStep {
    Welcome,
    Hostname(ValidationState<String>),
    User(ValidationState<UserConfig>),
    Git(GitConfig),
    Locale(LocaleConfig),
    Paths(PathConfig),
    Disk(DiskConfig),
    Summary,
    Installing,
    Complete,
}

// Navigation with validation gate
fn next_step(current: WizardStep) -> Option<WizardStep> {
    match current {
        WizardStep::Hostname(ref state) if state.is_valid() => {
            Some(WizardStep::User(UserConfig::default()))
        }
        WizardStep::User(ref state) if state.is_valid() => {
            Some(WizardStep::Git(GitConfig::default()))
        }
        _ => None, // Block progression if invalid
    }
}
```

### Pattern 2: Template-Based Configuration Generation

**What:** Separate template files (Nix expressions) from runtime data, generating final configurations through substitution.

**When to use:** For generating complex configuration files with many optional parameters and proper Nix syntax.

**Trade-offs:**
- Pros: Separates concerns, enables template preview, reduces runtime errors
- Cons: Template complexity, escaping concerns

**Example:**
```rust
// Template in resources/templates/flake.nix
let
  system = "{{SYSTEM}}";
  hostname = "{{HOSTNAME}}";
in
{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-{{NIXOS_VERSION}}";
  
  outputs = { self, nixpkgs }: {
    nixosConfigurations.${hostname} = nixpkgs.lib.nixosSystem {
      inherit system;
      modules = [
        ./configuration.nix
      ];
    };
  };
}

// Runtime generation
fn generate_flake(config: &InstallationConfig) -> String {
    let template = read_template("flake.nix");
    template
        .replace("{{SYSTEM}}", &config.target_architecture)
        .replace("{{HOSTNAME}}", &config.hostname)
        .replace("{{NIXOS_VERSION}}", &config.nixos_channel)
}
```

### Pattern 3: Action-Based Execution with Rollback

**What:** Each installation step is an "action" that can execute and revert on failure, similar to the nix-installer crate pattern.

**When to use:** For disk operations where partial failures require cleanup to maintain system integrity.

**Trade-offs:**
- Pros: Automatic cleanup on failure, retry capability, testable steps
- Cons: More complex action definitions, rollback implementation overhead

**Example:**
```rust
trait InstallAction {
    fn execute(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn description(&self) -> &str;
}

struct PartitionAction {
    device: PathBuf,
    scheme: PartitionScheme,
}

impl InstallAction for PartitionAction {
    fn execute(&self) -> Result<()> {
        // Execute partitioning
        Command::new("parted")
            .args(&["mklabel", "gpt", &self.device.to_string_lossy()])
            .status()?;
        // Create partitions...
        Ok(())
    }
    
    fn rollback(&self) -> Result<()> {
        // Revert partitioning
        Command::new("wipefs")
            .args(&["--all", "-f", &self.device.to_string_lossy()])
            .status()?;
        Ok(())
    }
    
    fn description(&self) -> &str {
        "Creating partition table"
    }
}
```

### Pattern 4: Validator Chain for Input Validation

**What:** Sequential validation functions that can be composed for complex validation rules.

**When to use:** For user input that requires multiple validation passes (format, availability, constraints).

**Trade-offs:**
- Pros: Composable, testable, clear error messages
- Cons: Potential for conflicting validators

**Example:**
```rust
struct UsernameValidator;

impl Validator for UsernameValidator {
    type Err = ValidationError;
    
    fn validate(&self, input: &str) -> Result<(), Self::Err> {
        // Check format
        if !input.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(ValidationError::InvalidFormat(
                "Username must be alphanumeric with _ or -".into()
            ));
        }
        
        // Check length
        if input.len() < 2 || input.len() > 32 {
            return Err(ValidationError::OutOfRange(
                "Username must be 2-32 characters".into()
            ));
        }
        
        // Check availability (system users)
        if is_system_user(input) {
            return Err(ValidationError::AlreadyExists(
                "Username is reserved".into()
            ));
        }
        
        Ok(())
    }
}
```

## Data Flow

### Request Flow

```
[User Input in TUI]
         ↓
[Validator Chain]
         ↓
[State Store Update]
         ↓
[Template Engine Substitution]
         ↓
[Nix Configuration Files Generated]
         ↓
[Action Execution (Partition → User → Bootloader → Switch)]
         ↓
[Progress Feedback to TUI]
         ↓
[Completion Signal]
```

### State Management

```
┌─────────────────────────────────────────────────────────────┐
│                  Installation State Store                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  InstallationConfig                                  │   │
│  │  - hostname: String (validated)                      │   │
│  │  - user: UserConfig (username, password, groups)     │   │
│  │  - git: GitConfig (user.name, user.email, signingkey)│   │
│  │  - locale: LocaleConfig (locale, timezone, layout)   │   │
│  │  - paths: PathConfig (home, data, config dirs)       │   │
│  │  - disk: DiskConfig (device, scheme, partitions)     │   │
│  │  - flake: FlakeConfig (channel, modules)             │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
         ↓ (publish/subscribe)
┌─────────────────────────────────────────────────────────────┐
│                    TUI Components                            │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│  │  Wizard  │←→│  Steps   │←→│Progress  │←→│ Summary  │   │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Key Data Flows

1. **User Input Flow:**
   - User enters hostname in TUI input field
   - Validator chain checks format and uniqueness
   - Valid input stored in State Store
   - UI updates to show validation success

2. **Configuration Generation Flow:**
   - State Store provides all collected values
   - Template Engine renders flake.nix with substitutions
   - NixOS Module Generator creates configuration.nix
   - Disk Config Generator creates disko format config
   - Generated files written to temporary directory

3. **Installation Execution Flow:**
   - Executor pulls configuration from Generator layer
   - Actions execute sequentially with progress callbacks
   - Each action reports status back to TUI
   - Failure triggers rollback actions in reverse order

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|-------------------------|
| **Single machine** | Monolith is appropriate; single process with in-memory state |
| **Network installer** | Add SSH transport layer; state synchronization |
| **Batch deployment** | Add job queue; parallel installation coordinator |
| **Enterprise multi-node** | Add configuration management service; orchestration layer |

### Scaling Priorities

1. **First bottleneck:** State serialization for crash recovery
   - Fix: Add periodic state snapshots to persistent storage
   
2. **Second bottleneck:** Long-running operations blocking TUI
   - Fix: Move executor to async tasks with progress channels

## Anti-Patterns

### Anti-Pattern 1: Storing State in UI Components

**What people do:** Keeping configuration values in widget state or closure captures.

**Why it's wrong:** Makes validation difficult, prevents navigation, creates inconsistent state on back navigation.

**Do this instead:** Use a centralized state store that survives UI component lifecycle.

### Anti-Pattern 2: Generating Nix on-the-Fly During Input

**What people do:** Calling `nix-instantiate` on every keystroke to validate configuration.

**Why it's wrong:** Extremely slow, creates race conditions, provides poor UX.

**Do this instead:** Use lightweight Rust-based validation for syntax; only generate Nix on step completion or preview.

### Anti-Pattern 3: Blocking Execution in TUI Event Loop

**What people do:** Running `nixos-install` synchronously in the render loop.

**Why it's wrong:** Freezes the terminal, prevents cancellation, breaks progress reporting.

**Do this instead:** Use async/await with tokio to run installation in background tasks.

### Anti-Pattern 4: Hardcoding Configuration Paths

**What people do:** Using fixed paths like `/etc/nixos/configuration.nix`.

**Why it's wrong:** Prevents testing, doesn't work in installer environment, breaks on different distributions.

**Do this instead:** Use configurable paths with safe defaults for installer context.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| **NixOS installer environment** | Process execution via SSH or chroot | Requires running from installer ISO or nixos-anywhere |
| **Disko** | Subprocess with generated config | Generates partition schema in disko format |
| **nixos-generate-config** | Subprocess execution | Captures hardware-configuration.nix |
| **nixos-install** | Subprocess execution | Final system installation command |
| **Systemd-boot** | Configuration file generation + efibootmgr | Generates boot entries |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| TUI ↔ State | Direct method calls + observers | UI subscribes to state changes |
| State ↔ Generator | Request/response | State provides data, generator returns files |
| Generator ↔ Executor | File passing + configuration objects | Generator writes files, executor reads |
| Executor ↔ System | Subprocess execution + SSH | Requires careful error handling |

## Build Order Implications

Based on the architecture analysis, the recommended build order is:

### Phase 1: Foundation
1. **State Store** — Establish the data model first
2. **Basic CLI Arguments** — Simple argument parsing with clap
3. **Project Structure** — Set up the module boundaries

### Phase 2: TUI Core
4. **Wizard State Machine** — Navigation logic without UI
5. **TUI Framework Setup** — Ratatui initialization
6. **Form Steps** — Implement each step with validation

### Phase 3: Configuration Generation
7. **Template Engine** — Basic template substitution
8. **Flake Generator** — Generate flake.nix
9. **NixOS Module Generator** — Generate configuration.nix
10. **Disk Config Generator** — Generate disko format

### Phase 4: Execution
11. **Action Traits** — Define the execution interface
12. **Disk Operations** — Partitioning and formatting
13. **User/Group Creation** — Account setup
14. **Bootloader Configuration** — System boot setup
15. **System Installation** — nixos-install orchestration

### Phase 5: Integration
16. **Progress Reporting** — Connect executor to TUI
17. **Rollback Logic** — Error recovery
18. **Summary View** — Pre-installation review
19. **Testing** — Integration tests

### Dependencies Summary

```
State Store ← Wizard ← TUI ← Form Steps
                           ↓
                     Generators ← Templates
                           ↓
                     Executors ← Actions
                           ↓
                     System Commands
```

## Sources

- [Calamares Module Architecture](https://calamares.euroquis.nl/docs/develop-modules) — Modular installer framework inspiration
- [nix-installer Architecture](https://docs.rs/nix-installer/latest/nix_installer/) — Action/Planner pattern
- [Ratatui TUI Framework](https://ratatui.rs/) — Modern Rust TUI library
- [dialoguer Crate](https://docs.rs/dialoguer/latest/dialoguer/) — Input validation patterns
- [nixos-anywhere](https://github.com/nix-community/nixos-anywhere) — Network installation reference
- [Disko Disk Configuration](https://github.com/nix-community/disko) — Disk config format
- [NixOS Installation Guide 2025](https://michael.stapelberg.ch/posts/2025-06-01-nixos-installation-declarative/) — Modern installation patterns

---

*Architecture research for: NixOS TUI Installer*
*Researched: 2025-01-31*
