# brightnessctl

Utility for reading and controlling device brightness for Linux operating systems.

## Usage
- `brightnessctl -l` : list devices with changeable brightness
- `brightnessctl g` : print the current brightness of the default device
- `brightnessctl g -d 'device_name'` : print the current brightness of a specific device (can be a wildcard)
- `brightnessctl s 50%` : set the brightness to a specified percentage
- `brightnessctl s +10%` : increase brightness by a specified percentage
- `brightnessctl s 10%-` : decrease brightness by a specified percentage

## Categories
hardware

## Links
- More information: https://github.com/Hummer12007/brightnessctl#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/brightnessctl
