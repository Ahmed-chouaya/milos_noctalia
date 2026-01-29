{ config, pkgs, ... }:

{
  # Polkit for privilege escalation
  security.polkit.enable = true;

  # Seatd for seat management
  services.seatd.enable = true;

  # Power management
  services.power-profiles-daemon.enable = true;
  services.upower.enable = true;
}
