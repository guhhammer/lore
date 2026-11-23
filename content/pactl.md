# pactl

Control a running PulseAudio sound server.

## Usage
- `pactl info` : show information about the sound server
- `pactl list sinks short` : list all sinks (or other types - sinks are outputs and sink-inputs are active audio streams)
- `pactl set-default-sink 1` : change the default sink (output) to 1 (the number can be retrieved via the `list` subcommand)
- `pactl move-sink-input 627 1` : move sink-input 627 to sink 1
- `pactl set-sink-volume 1 0.75` : set the volume of sink 1 to 75%
- `pactl set-sink-mute @DEFAULT_SINK@ toggle` : toggle mute on the default sink (using the special name `@DEFAULT_SINK@`)
- `pactl set-card-profile alsa_card.pci-0000_0c_00.1 output:hdmi-stereo-extra1` : specify which HDMI output is being used

## Categories
media

## Links
- More information: https://manned.org/pactl
- tldr page: https://tldr.inbrowser.app/pages/common/pactl
