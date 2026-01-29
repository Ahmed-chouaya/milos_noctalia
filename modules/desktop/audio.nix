{ config, pkgs, ... }:

{
  # Pipewire audio server
  services.pipewire = {
    enable = true;
    pulse.enable = true;  # PulseAudio compatibility
    alsa.enable = true;   # ALSA support
  };
}
