{ config, pkgs, ... }:

{
  # Enable Niri compositor
  programs.niri.enable = true;

  # Niri package and related tools
  environment.systemPackages = with pkgs; [
    niri
    xwayland-satellite
  ];
}
