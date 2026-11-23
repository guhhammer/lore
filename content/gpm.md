# gpm

Enable mouse support for the Linux virtual console.

## Usage
- `sudo gpm -m /dev/input/mice -t ps2` : start gpm with a PS/2 mouse of type ps2
- `sudo gpm -m /dev/ttyS0 -t ms` : start gpm with a Microsoft serial mouse of type ms
- `sudo gpm -m path/to/mouse_device -t mouse_type -D` : start gpm with a mouse and type in the foreground for Debugging
- `sudo gpm -k` : kill the running gpm
- `sudo gpm -m path/to/mouse_device -t mouse_type -R` : start gpm with a mouse and type in Repeater mode for X server compatibility
- `gpm -t help` : list the available mouse types

## Categories
hardware, users

## Links
- More information: https://manned.org/gpm
- tldr page: https://tldr.inbrowser.app/pages/linux/gpm
