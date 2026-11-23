# xsetwacom

Change settings for Wacom pen tablets at runtime.

## Usage
- `xsetwacom list` : list all the available Wacom devices. The device name is in the first column
- `xsetwacom set "device_name" MapToOutput screen` : set Wacom area to specific screen. Get name of the screen with `xrandr`
- `xsetwacom set "device_name" Mode "Relative|Absolute"` : set mode to relative (like a mouse) or absolute (like a pen) mode
- `xsetwacom set "device_name" Rotate none|half|cw|ccw` : rotate the input (useful for tablet-PC when rotating screen) by 0|90|180|270 degrees from "natural" rotation
- `xsetwacom set "device_name" TabletPCButton "on"` : set button to only work when the tip of the pen is touching the tablet

## Categories
desktop, hardware

## Links
- More information: https://manned.org/xsetwacom
- tldr page: https://tldr.inbrowser.app/pages/linux/xsetwacom
