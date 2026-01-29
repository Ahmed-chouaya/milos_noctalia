{ config, pkgs, ... }:

{
  programs.alacritty = {
    enable = true;

    # Basic configuration - no existing config found, using defaults
    # Customize as needed
    settings = {
      env.TERM = "xterm-256color";

      window = {
        padding = {
          x = 10;
          y = 10;
        };
        opacity = 0.95;
      };

      font = {
        size = 11.0;
      };

      cursor.style = {
        shape = "Block";
        blinking = "On";
      };

      scrolling.history = 10000;
    };
  };
}
