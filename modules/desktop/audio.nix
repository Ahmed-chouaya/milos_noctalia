{ config, pkgs, ... }:

{
  # Enable rtkit for real-time audio scheduling
  security.rtkit.enable = true;

  # PipeWire audio server
  services.pipewire = {
    enable = true;

    # Enable audio server components
    audio.enable = true;

    # PulseAudio compatibility
    pulse.enable = true;

    # ALSA support
    alsa = {
      enable = true;
      support32Bit = true;  # 32-bit app support
    };

    # JACK support (for professional audio)
    jack.enable = true;

    # WirePlumber session manager
    wireplumber.enable = true;
  };
}
