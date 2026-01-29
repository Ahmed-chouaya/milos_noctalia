{ config, pkgs, ... }:

{
  # Browser packages
  environment.systemPackages = with pkgs; [
    brave
  ];
}
