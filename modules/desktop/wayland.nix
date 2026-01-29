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
    libinput
    libdrm
    libxkbcommon
    pixman

    # Build tools (may be needed for some packages)
    meson
    ninja

    # Wayland-specific libraries
    libdisplay-info
    libliftoff
    hwdata
    seatd
    pcre2
    glibc
  ];
}
