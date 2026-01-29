{ config, pkgs, ... }:

{
  # Bluetooth support
  hardware.bluetooth = {
    enable = true;
    powerOnBoot = true;  # Power on Bluetooth adapter on boot
    settings = {
      General = {
        Enable = "Source,Sink,Media,Socket";
        Experimental = true;  # Enable experimental features
      };
    };
  };

  # Bluetooth manager service
  services.blueman.enable = true;

  # Printing support (CUPS)
  services.printing = {
    enable = true;
    drivers = with pkgs; [
      gutenprint
      hplip  # HP printer support
    ];
  };

  # Scanner support
  hardware.sane = {
    enable = true;
    extraBackends = [ pkgs.hplipWithPlugin ];
  };

  # OpenGL/graphics support
  hardware.graphics = {
    enable = true;
    enable32Bit = true;  # 32-bit app support
  };
}
