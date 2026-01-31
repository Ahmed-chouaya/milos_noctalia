# Codebase Concerns

**Analysis Date:** 2026-01-31

## Build Errors

### Helix Dotfiles Path Issue

**Issue:** Build fails when trying to include helix configuration
```
error: path '/nix/store/.../home/modules/dotfiles/helix' does not exist
```

**Files:** Potentially `home/modules/development/editors.nix` or similar

**Trigger:** Running `nixos-rebuild` with helix configuration referenced

**Root cause:** Missing or misconfigured helix dotfile directory

**Fix approach:**
- Verify `home/dotfiles/helix/config.toml` exists
- Check any XDG configFile references to helix
- Remove or correct the invalid path reference

**Status:** Build currently failing - needs immediate attention

### RPC Connection Timeout

**Issue:** Git RPC timeout during build
```
RPC failed; curl 56 Recv failure: Connection timed out
fatal: expected flush after ref listing
```

**Impact:** Build may fail on first attempt due to network

**Workaround:** Retry build - uses cached refs on subsequent attempts

**Recommendation:** Improve network robustness or add retries

## Deprecated Options

### Zsh initExtra Deprecation

**Issue:** `programs.zsh.initExtra` is deprecated
```
evaluation warning: `programs.zsh.initExtra` is deprecated, use `programs.zsh.initContent` instead.
```

**File:** `home/modules/shell/zsh.nix`

**Current:** Uses deprecated `initExtra` or `initContent` (check actual usage)

**Fix:** Change to `programs.zsh.initContent`

**Priority:** Low - still works but generates warnings

### Git Options Deprecation

**Issue:** Multiple deprecated Git options
```
warning: `programs.git.aliases' has been renamed to `programs.git.settings.alias'
warning: `programs.git.userEmail' has been renamed to `programs.git.settings.user.email'
warning: `programs.git.userName' has been renamed to `programs.git.settings.user.name'
warning: `programs.git.extraConfig' has been renamed to `programs.git.settings'
```

**File:** `home/modules/development/git.nix`

**Current structure:**
```nix
programs.git = {
  enable = true;
  settings = { ... };  # Current approach
};
```

**Fix approach:** Already using `settings` - verify no legacy options remain

**Priority:** Low - warnings only, functionality works

## Tech Debt

### Node Configuration TODO

**Issue:** TODO comment in node module
```nix
# TODO: Extract from ~/.npmrc if exists and contains no secrets
```

**File:** `home/modules/development/node.nix`

**Impact:** User npm configuration not automated

**Fix approach:** 
- Create npm config generator
- Or document manual npm setup
- Or use Home Manager's npm module if available

**Priority:** Low - npm works without it

### SSH Password Authentication

**Issue:** TODO comment about SSH password auth
```nix
# TODO: After setting up SSH keys, change this to false
services.openssh.settings.PasswordAuthentication = true;
```

**File:** `modules/system/ssh.nix`

**Impact:** SSH allows password auth (less secure)

**Fix approach:** 
- Set up SSH keys for the user
- Change to `PasswordAuthentication = false`

**Priority:** Medium - security concern

### State Version Documentation

**Issue:** `system.stateVersion = "25.11"` documented as "don't change after installation"

**Files:** 
- `hosts/nixos/configuration.nix`
- `home/milgraph.nix`

**Impact:** Changing this may cause issues with existing configurations

**Fix approach:** Document what changes when upgrading

**Priority:** Low - documentation only

## Security Considerations

### SSH Password Authentication Enabled

**Risk:** SSH allows password authentication

**Current mitigation:** Root login disabled, key-based auth available

**Files:** `modules/system/ssh.nix`

**Recommendations:**
1. Complete SSH key setup
2. Disable password authentication
3. Consider changing default port
4. Limit to specific IPs if possible

### Docker Group Membership

**Risk:** User in `docker` group has root-equivalent access

**Current mitigation:** None explicit

**Files:** `modules/system/users.nix`, `modules/development/docker.nix`

**Recommendations:**
1. Document the security implication
2. Consider if Docker is actually needed
3. Use rootless Docker as configured

## Documentation Issues

### Build Logs in Repository

**Files:** `build-errors.log`, `build-result.log`

**Issue:** Build artifacts committed to repo

**Impact:** Pollutes git history, may contain sensitive info

**Fix approach:** Add to `.gitignore` or remove from repo

**Priority:** Low - cosmetic/convenience

### Large Binary Files

**Directory:** `home/dotfiles/opencode/node_modules/`

**Issue:** npm packages committed to repo

**Impact:** Large binary files in git history

**Recommendation:** 
- Add to `.gitignore`
- Or use npm install on first run

**Priority:** Low - storage/convenience

## Missing Features

### Multi-Host Support Not Implemented

**Current state:** Single host ("nixos") hardcoded

**Issue:** Can't easily add new hosts

**Fix approach:**
- Rename `hosts/nixos/` to `hosts/default/` or similar
- Add host selection in flake.nix
- Document multi-host patterns

**Priority:** Low - works for single-host setup

### No Automated Testing

**Current state:** No test framework for configuration

**Issue:** Changes may break builds

**Fix approach:**
- Add `nix flake check`
- Use `nixos-test` for system tests
- Add CI pipeline

**Priority:** Medium - would improve reliability

### No Configuration Validation Pre-Build

**Current state:** Validation only during build

**Issue:** Errors only caught at build time

**Fix approach:**
- Add `nix eval` checks
- Validate Niri config separately: `niri validate`
- Add pre-commit hooks

**Priority:** Low - current workflow works

## Performance Concerns

### Garbage Collection Frequency

**Current:** Weekly garbage collection

**Impact:** May accumulate unused nix store entries

**Recommendation:** Verify GC is working, consider more frequent

### Store Optimization

**Current:** `auto-optimise-store = true`

**Impact:** Good - Nix automatically optimizes

## Dependency Risks

### Unstable Channel Usage

**Risk:** Using `nixos-unstable` may have breaking changes

**Current mitigation:** `flake.lock` locks versions

**Recommendation:** Consider `nixos-25.05` or similar stable channel

**Priority:** Medium - could break on updates

### External Flake Dependencies

**Flakes used:**
- `sodiboo/niri-flake`
- `noctalia-dev/noctalia-shell`

**Risk:** External repos may disappear or change

**Current mitigation:** `flake.lock` pins versions

**Recommendation:** 
- Periodically update and test
- Consider maintaining local copies if critical

## User Configuration Gaps

### Hardware-Specific Config

**Issue:** `hardware-configuration.nix` not in git

**Impact:** Can't share full configuration

**Workaround:** Document what's needed for new installs

### Personal Information Hardcoded

**Files:**
- `home/modules/development/git.nix` (name, email)
- `modules/system/users.nix` (username)

**Impact:** Configuration tied to specific user

**Recommendation:** Document customization needed for new users

---

*Concerns audit: 2026-01-31*
