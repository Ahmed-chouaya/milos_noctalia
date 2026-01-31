{ inputs, config, lib, ... }:
{
  # Git configuration for {{ full_name }} ({{ git_email }})

  programs.git = {
    enable = true;
    userName = "{{ git_username }}";
    userEmail = "{{ git_email }}";
    extraConfig = {
      init = {
        defaultBranch = "main";
      };
      pull = {
        rebase = true;
      };
    };
  };

  # Sign commits with SSH key if available
  programs.ssh.authorizedKeysFiles = [
    "/etc/ssh/authorized_keys.d/%u"
  ];
}
