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

    # Shell aliases (to be populated in Task 31)
    shellAliases = {
      # TODO: Extract from ~/.zshrc
    };

    # Additional init commands (to be populated in Task 31)
    initExtra = ''
      # TODO: Extract custom zsh configuration from ~/.zshrc
    '';
  };
}
