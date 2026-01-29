{ config, pkgs, ... }:

{
  # NPM configuration (to be populated in Task 29)
  # TODO: Extract from ~/.npmrc if exists and contains no secrets
  # home.file.".npmrc".text = ''
  #   # Extract non-sensitive settings from ~/.npmrc
  # '';

  # If .npmrc contains auth tokens, do NOT commit them
  # Use environment variables or secrets management instead
}
