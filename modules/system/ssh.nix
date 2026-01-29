{ config, pkgs, ... }:

{
  # Enable OpenSSH daemon
  services.openssh = {
    enable = true;

    settings = {
      # Disable password authentication for better security
      # Only allow key-based authentication
      PasswordAuthentication = false;
      PermitRootLogin = "no";

      # Enable X11 forwarding if needed
      X11Forwarding = false;
    };
  };

  # Uncomment these if you want password authentication (less secure)
  # services.openssh.settings.PasswordAuthentication = true;

  # Uncomment to change SSH port (useful for security through obscurity)
  # services.openssh.ports = [ 2222 ];
  # Don't forget to update the firewall in networking.nix if you change the port
}
