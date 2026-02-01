{ config, pkgs, inputs, ... }:

{
  imports = [
    inputs.niri.homeModules.config
  ];

  # Copy niri configuration files from dotfiles
  home.file.".config/niri/config.kdl".source = ../../dotfiles/niri/config.kdl;
}
