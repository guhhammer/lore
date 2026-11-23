# wpctl

Manage WirePlumber, a session and policy manager for PipeWire. Note: You can use the special name `@DEFAULT_SINK@` in place of `id` to operate on the default sink.

## Usage
- `wpctl status` : list all objects managed by WirePlumber
- `wpctl inspect object_id` : print all properties of an object
- `wpctl set-default object_id` : set an object to be the default in its group
- `wpctl get-volume sink_id` : get the volume of a sink
- `wpctl set-volume sink_id n%|n%+|n%-` : set the volume of a sink to `n` percent or increase or decrease it by `n`
- `wpctl list audio|video devices|sinks|sources` : filter objects by type
- `wpctl set-volume -l 1 sink_id n%-` : increase the volume of a sink by `n` percent but limit the volume below 100%
- `wpctl set-mute @DEFAULT_SINK|SOURCE@ 1|0|toggle` : mute/Unmute the default audio sink or source (1 is mute, 0 is unmute)

## See also
- pw-cli

## Categories
disks, media

## Links
- More information: https://pipewire.pages.freedesktop.org/wireplumber/
- tldr page: https://tldr.inbrowser.app/pages/linux/wpctl
