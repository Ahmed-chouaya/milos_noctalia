{ config, pkgs, ... }:

{
  # Enable X server (for Xwayland support)
  services.xserver.enable = true;

  # Enable libinput for input device management
  services.libinput.enable = true;

  # Wayland-related packages
  environment.systemPackages = with pkgs; [
    # Wayland protocols and libraries
    kdePackages.wayland-protocols
    libxkbcommon

    # Wayland-specific libraries
    libdisplay-info
    libliftoff
    hwdata
  ];
}
