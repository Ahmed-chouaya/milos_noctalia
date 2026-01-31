{ inputs, config, lib, ... }:
{
  users.users.{{ username }} = {
    isNormalUser = true;
    description = "{{ full_name }}";
    extraGroups = [
      "wheel"
      "sudo"
      "audio"
      "video"
      "users"
    ];
    home = "/home/{{ username }}";
    createHome = true;
  };

  users.groups.{{ username }} = {};

  # Git configuration link
  environment.etc."gitconfig".text = ''
    [user]
      email = {{ git_email }}
      name = {{ git_username }}
  '';
}
