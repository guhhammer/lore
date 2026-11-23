# wireplumber

A modular session/policy manager for PipeWire and a GObject-based high-level library that wraps PipeWire's API.

## Usage
- `systemctl enable wireplumber --user --now` : make WirePlumber start with the user session immediately (for systemd systems)
- `wireplumber` : run WirePlumber, after `pipewire` is started (for non-systemd systems)
- `wireplumber -c path/to/file` : specify a different context configuration file
- `wireplumber -h` : display help
- `wireplumber -v` : display version

## See also
- wpctl
- pipewire

## Categories
development, files

## Links
- More information: https://pipewire.pages.freedesktop.org/wireplumber/
- tldr page: https://tldr.inbrowser.app/pages/linux/wireplumber
