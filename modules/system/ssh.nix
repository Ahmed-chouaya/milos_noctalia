{ config, pkgs, ... }:

{
  # Enable OpenSSH daemon
  services.openssh = {
    enable = true;

    settings = {
      # TEMPORARY: Password authentication enabled for initial setup
      # TODO: After setting up SSH keys, change this to false
      # To set up keys: ssh-copy-id milgraph@nixos
      PasswordAuthentication = true;
      PermitRootLogin = "no";

      # Enable X11 forwarding if needed
      X11Forwarding = false;
    };
  };

  # To disable password auth after setting up keys:
  # 1. Copy your public key: ssh-copy-id milgraph@nixos
  # 2. Set PasswordAuthentication = false above
  # 3. Rebuild: sudo nixos-rebuild switch --flake .#nixos

  # To change SSH port (useful for security through obscurity):
  # services.openssh.ports = [ 2222 ];
  # Don't forget to update the firewall in networking.nix if you change the port
}
