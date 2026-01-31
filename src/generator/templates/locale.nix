{ inputs, config, lib, ... }:
{
  # Locale and timezone settings

  time.timeZone = "{{ timezone }}";

  i18n.defaultLocale = "{{ timezone }}";

  # Keyboard layout for console and X11
  console = {
    fonts = lib.mkDefault [
      "Libertinus Mono"
      "Unicode"
    ];
    keyMap = "{{ keyboard_layout }}";
  };

  services.xserver = {
    layout = "{{ keyboard_layout }}";
    xkbOptions = {
      "grp" = "alt_shift_toggle";
    };
  };
}
