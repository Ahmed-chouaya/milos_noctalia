{ config, pkgs, ... }:

{
  # Bootloader configuration
  boot.loader.systemd-boot.enable = true;
  boot.loader.efi.canTouchEfiVariables = true;

  # HP-specific kernel modules for input devices
  boot.kernelModules = [ "i8042" "hid_generic" "usbhid" ];

  # HP-specific kernel parameters for stability
  boot.kernelParams = [
    "i8042.reset"      # Reset keyboard controller
    "i8042.nomux"      # Disable i8042 multiplexing
    "i915.enable_psr=0" # Disable Intel PSR (panel self refresh)
  ];

  # Video drivers
  services.xserver.videoDrivers = [ "modesetting" ];
}
