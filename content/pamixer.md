# pamixer

A simple command-line mixer for PulseAudio.

## Usage
- `pamixer --list-sinks --list-sources` : list all sinks and sources with their corresponding IDs
- `pamixer --set-volume 75` : set the volume to 75% on the default sink
- `pamixer --toggle-mute --sink sink_id` : toggle mute on a sink other than the default
- `pamixer -i 5` : increase the volume on default sink by 5%
- `pamixer -d 5 --source source_id` : decrease the volume on a source by 5%
- `pamixer --set-volume 105 --allow-boost` : use the allow boost option to increase, decrease, or set the volume above 100%
- `pamixer -m` : mute the default sink (use `--unmute` instead to unmute)

## Categories
disks, text

## Links
- More information: https://github.com/cdemoulins/pamixer#installation
- tldr page: https://tldr.inbrowser.app/pages/common/pamixer
