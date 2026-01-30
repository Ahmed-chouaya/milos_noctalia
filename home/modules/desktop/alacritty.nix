{ config, pkgs, ... }:

{
  programs.alacritty = {
    enable = true;

    # Force overwrite existing config file to prevent conflicts
    # This handles cases where alacritty.toml was created outside Home Manager
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
