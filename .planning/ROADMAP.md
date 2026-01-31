# Roadmap: milos_niri - NixOS TUI Installer

**Created:** 2026-01-31
**Depth:** Comprehensive (derived from requirements)

## Overview

A 4-phase roadmap to build an interactive TUI installer that guides users through configuring and applying a Niri + Noctalia desktop environment on any NixOS machine. Each phase delivers a coherent, verifiable capability with observable success criteria.

---

## Phase Structure

| Phase | Goal | Requirements | Success Criteria |
|-------|------|--------------|------------------|
| 1 - TUI Foundation | Working installer shell with navigation | 8 (TUI-01 to TUI-08) | 5 criteria |
| 2 - Input Collection | User data gathering wizard | 10 (INP-01 to INP-10) | 6 criteria |
| 3 - Config Generation | Template substitution engine | 9 (CFG-01 to CFG-09) | 4 criteria |
| 4 - Execution | System reconfiguration runner | 5 (EXEC-01 to EXEC-05) | 5 criteria |

---

## Phase Details

### Phase 1: TUI Foundation

**Goal:** Working installer shell with navigation and error handling

**Dependencies:**
- None (foundation phase)

**Requirements:** TUI-01, TUI-02, TUI-03, TUI-04, TUI-05, TUI-06, TUI-07, TUI-08

**Plans:**
- [x] 01-01-PLAN.md — Project initialization (TUI-01)
- [x] 01-02-PLAN.md — Core framework: event loop + state (TUI-02, TUI-03)
- [x] 01-03-PLAN.md — Navigation & wizard flow (TUI-04)
- [x] 01-04-PLAN.md — Logo display & animation (TUI-06, TUI-07, TUI-08)
- [x] 01-05-PLAN.md — Error handling (TUI-05)

**Success Criteria:**

1. **Installer launches with pixel art logo** - User runs `milos-niri` and sees the MILOS logo rendered in the terminal with Noctalia color scheme

2. **Keyboard navigation works** - User can navigate between wizard steps using arrow keys, Tab, Enter, and Escape with responsive input handling

3. **State persists across navigation** - User can move backward and forward through wizard steps without losing entered data

4. **Errors display beautifully** - When errors occur, user sees color-eyre formatted backtraces with helpful context, not raw Rust panics

5. **Logo polish complete** - MILOS logo animates on startup (optional enhancement TUI-08) with smooth visual effect

---

### Phase 2: Input Collection

**Goal:** Complete user data gathering wizard

**Dependencies:**
- Phase 1 complete (needs TUI framework and state management)

**Requirements:** INP-01, INP-02, INP-03, INP-04, INP-05, INP-06, INP-07, INP-08, INP-09, INP-10

**Plans:**
- [x] 02-01-PLAN.md — Account step: hostname, username, full name, git credentials with validation ✓
- [x] 02-02-PLAN.md — Timezone & keyboard selection with type-to-filter ✓
- [x] 02-03-PLAN.md — Path configuration: wallpaper, avatar, screenshot paths ✓
- [x] 02-04-PLAN.md — Polish: sidebar, validation summary, Summary step ✓

**Success Criteria:**

1. **Hostname input with validation** - User can type hostname, installer rejects names with spaces or special characters, shows helpful error

2. **Username input with validation** - User creates account with lowercase alphanumeric username, installer validates format

3. **Personal info collected** - User enters full name and Git credentials (username + email) for commit authoring

4. **Timezone selection works** - User sees region/city list, can search or scroll to select their timezone

5. **Keyboard layout selection works** - User can select from common layouts (us, fr, de, etc.) with preview

6. **Path inputs complete** - User configures wallpaper directory, optional avatar path, and screenshot path with defaults pre-filled

---

### Phase 3: Configuration Generation

**Goal:** Template substitution engine for flake and module files

**Dependencies:**
- Phase 2 complete (needs collected user data to substitute)

**Requirements:** CFG-01, CFG-02, CFG-03, CFG-04, CFG-05, CFG-06, CFG-07, CFG-08, CFG-09

**Plans:**
- [ ] 03-01-PLAN.md — Generator infrastructure: dependencies, Generator trait, error types, UserConfig
- [ ] 03-02-PLAN.md — Templates: 7 Askama templates with context structs (flake, users, git, locale, noctalia, niri, nix.conf)
- [ ] 03-03-PLAN.md — Validation & atomic writing: validate templates, write files safely
- [ ] 03-04-PLAN.md — Orchestrator: generate_all() entry point for TUI integration
- [ ] 03-05-PLAN.md — TUI integration: Generate step in wizard flow
- [ ] 03-06-PLAN.md — Verification: Human checkpoint to verify generated configs

**Success Criteria:**

1. **All placeholders substituted** - Generated flake.nix contains user's hostname, users.nix contains username, git.nix contains credentials, locale.nix contains timezone/keyboard

2. **Path configurations set** - Noctalia.nix has wallpaper/avatar paths, niri config.kdl has screenshot path

3. **Nix settings updated** - trusted-users includes the new user in nix.conf

4. **Validation prevents bad output** - Installer validates all substitutions before writing, catches template errors early with user-friendly messages

---

### Phase 4: Execution

**Goal:** System reconfiguration with progress display and error recovery

**Dependencies:**
- Phase 3 complete (needs generated configs to apply)

**Requirements:** EXEC-01, EXEC-02, EXEC-03, EXEC-04, EXEC-05

**Success Criteria:**

1. **Git commit created** - Installer runs `git commit` on generated configs, user sees confirmation

2. **Rebuild executes** - Installer runs `nixos-rebuild switch --flake`, applies configuration changes

3. **Progress visible** - User sees real-time logs and progress indicators during rebuild, understands what's happening

4. **Errors handled gracefully** - On failure, user sees clear explanation of what went wrong, not cryptic Nix output

5. **Rollback offered** - User can choose to rollback if rebuild fails, restoring previous configuration

---

## Requirement Coverage

| Category | Requirements | Phase |
|----------|--------------|-------|
| TUI Framework | TUI-01, TUI-02, TUI-03, TUI-04, TUI-05, TUI-06, TUI-07, TUI-08 | Phase 1 |
| Input Collection | INP-01, INP-02, INP-03, INP-04, INP-05, INP-06, INP-07, INP-08, INP-09, INP-10 | Phase 2 |
| Configuration Generation | CFG-01, CFG-02, CFG-03, CFG-04, CFG-05, CFG-06, CFG-07, CFG-08, CFG-09 | Phase 3 (03-02, 03-03, 03-04) |
| Execution | EXEC-01, EXEC-02, EXEC-03, EXEC-04, EXEC-05 | Phase 4 |

**Coverage:** 32/32 v1 requirements mapped ✓

---

## Dependencies Between Phases

```
Phase 1 (TUI Foundation)
    │
    ▼
Phase 2 (Input Collection) ───► Requires Phase 1 state management
    │
    ▼
Phase 3 (Config Generation) ───► Requires Phase 2 collected data
    │
    ▼
Phase 4 (Execution) ───► Requires Phase 3 generated configs
```

---

## Research Flags (from SUMMARY.md)

Phases needing deeper research during planning:

- **Phase 3:** Module imports and flake output types—Nix module system has complex semantics
- **Phase 4:** Secrets management patterns if extending beyond basic Git credentials

Phases with standard patterns (can proceed):

- **Phase 1:** Ratatui and state machine patterns well-documented
- **Phase 2:** Form input patterns well-established in tui-textarea

---

## Execution Order

Start with: `/gsd/plan-phase 1`

Progress through phases sequentially. Each phase's success criteria inform what must be built and tested before moving to the next.

---

*Generated: 2026-01-31*
