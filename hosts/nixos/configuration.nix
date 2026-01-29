{ config, pkgs, ... }:

{
  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;

  # Enable experimental features for flakes
  nix.settings.experimental-features = [ "nix-command" "flakes" ];

  # System state version
  # IMPORTANT: Don't change this after installation
  system.stateVersion = "25.11";
}
