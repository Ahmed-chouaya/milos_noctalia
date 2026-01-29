{ config, pkgs, ... }:

{
  # Terminal emulators
  environment.systemPackages = with pkgs; [
    alacritty
  ];
}
