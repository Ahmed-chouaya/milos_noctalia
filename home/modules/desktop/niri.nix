{ config, pkgs, inputs, ... }:

{
  imports = [
    inputs.niri.homeModules.config
  ];

  programs.niri = {
    settings = {
      # TODO: Extract and convert from ~/.config/niri/config.kdl
      # This requires manual conversion from KDL to Nix attributes
      # Reference: https://github.com/sodiboo/niri-flake
    };
  };
}
