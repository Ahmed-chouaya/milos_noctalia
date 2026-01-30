{ config, pkgs, ... }:

{
  # Enable X server (for Xwayland support)
  services.xserver.enable = true;

  # Enable libinput for input device management
  services.libinput.enable = true;

  # XDG Desktop Portal for Wayland
  # Enables screen sharing, file pickers, and other desktop integrations
  xdg.portal = {
    enable = true;
    wlr.enable = true;  # wlroots-based compositors (like niri)
    extraPortals = with pkgs; [
      xdg-desktop-portal-gtk  # GTK file picker
    ];
    config = {
      common = {
        default = [ "gtk" ];
      };
      niri = {
        default = pkgs.lib.mkForce [ "wlr" "gtk" ];
        "org.freedesktop.impl.portal.ScreenCast" = [ "wlr" ];
        "org.freedesktop.impl.portal.Screenshot" = [ "wlr" ];
      };
    };
  };

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
