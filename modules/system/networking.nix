{ config, pkgs, ... }:

{
  # Hostname (can be overridden per host)
  networking.hostName = "nixos";

  # Enable NetworkManager for network management
  networking.networkmanager.enable = true;

  # Enable firewall
  networking.firewall = {
    enable = true;

    # Allow SSH access
    allowedTCPPorts = [ 22 ];

    # Allow common development server ports
    allowedTCPPortRanges = [
      { from = 3000; to = 3999; }  # Node.js, React, Vite, etc.
      { from = 8000; to = 8999; }  # Python, Django, HTTP test servers, etc.
    ];

    # For Docker exposed ports, add them here as needed
    # Example: allowedTCPPorts = [ 22 8080 9000 ];

    # Uncomment if you need UDP ports (gaming, VoIP, etc.)
    # allowedUDPPorts = [ ];
  };
}
