# xrandr

Set the size, orientation and/or reflection of the outputs for a screen.

## Usage
- `xrandr -q` : display the current state of the system (known screens, resolutions, ...)
- `xrandr --auto` : disable disconnected outputs and enable connected ones with default settings
- `xrandr --output DP1 --mode 1920x1080 -r 60` : change the resolution and update frequency of DisplayPort 1 to 1920x1080, 60Hz
- `xrandr --output HDMI2 --mode 1280x1024 --right-of DP1` : set the resolution of HDMI2 to 1280x1024 and put it on the right of DP1
- `xrandr --output VGA1 --off` : disable the VGA1 output
- `xrandr --output LVDS1 --brightness 0.5` : set the brightness for LVDS1 to 50%
- `xrandr -d :0 -q` : display the current state of any X server

## Categories
desktop

## Links
- More information: https://www.x.org/releases/current/doc/man/man1/xrandr.1.xhtml
- tldr page: https://tldr.inbrowser.app/pages/linux/xrandr
