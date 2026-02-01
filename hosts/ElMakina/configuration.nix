{ config, pkgs, ... }:

{
  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;

  # Nix configuration
  nix.settings = {
    # Enable experimental features for flakes
    experimental-features = [ "nix-command" "flakes" ];

    # Optimize store automatically
    auto-optimise-store = true;

    # Trusted users (can use nix without sudo)
    trusted-users = [ "root" "ahmed" ];
  };

  # Automatic garbage collection
  nix.gc = {
    automatic = true;
    dates = "weekly";
    options = "--delete-older-than 30d";
  };

  # Wayland environment variables
  environment.sessionVariables = {
    # Enable Wayland for Electron apps (VSCode, Discord, etc.)
    NIXOS_OZONE_WL = "1";

    # Enable Wayland for Firefox
    MOZ_ENABLE_WAYLAND = "1";

    # XDG compliance
    XDG_CURRENT_DESKTOP = "niri";
    XDG_SESSION_TYPE = "wayland";
  };

  # System state version
  # IMPORTANT: Don't change this after installation
  system.stateVersion = "25.11";
}
