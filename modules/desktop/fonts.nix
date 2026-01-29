{ config, pkgs, ... }:

{
  # Font packages
  fonts.packages = with pkgs; [
    nerd-fonts.jetbrains-mono
  ];
}
