{ config, pkgs, ... }:

{
  # Hostname (can be overridden per host)
  networking.hostName = "nixos";

  # Enable NetworkManager for network management
  networking.networkmanager.enable = true;

  # Enable firewall
  networking.firewall.enable = true;
  # networking.firewall.allowedTCPPorts = [ ];
  # networking.firewall.allowedUDPPorts = [ ];
}
