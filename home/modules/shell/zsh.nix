{ config, pkgs, ... }:

{
  programs.zsh = {
    enable = true;

    # Enable completion
    enableCompletion = true;

    # History configuration
    history = {
      size = 10000;
      path = "${config.home.homeDirectory}/.zsh_history";
    };

    # Shell aliases - add your custom aliases here
    shellAliases = {
      ll = "ls -alh";
      la = "ls -A";
      l = "ls -CF";
      ".." = "cd ..";
    };

    # Additional init commands
    initExtra = ''
      # Add custom zsh configuration here
    '';
  };
}
