{ config, pkgs, ... }:

{
  # Hostname (can be overridden per host)
  networking.hostName = "nixos";

  # Enable NetworkManager for network management
  networking.networkmanager.enable = true;

  # Firewall configuration
  # networking.firewall.allowedTCPPorts = [ ];
  # networking.firewall.allowedUDPPorts = [ ];
  # Or disable the firewall altogether:
  # networking.firewall.enable = false;
}
