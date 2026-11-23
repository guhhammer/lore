# swaybg

Wallpaper tool for Wayland compositors. Note: Sway starts `swaybg` automatically if a wallpaper is set in the config file.

## Usage
- `swaybg -i path/to/image` : set the wallpaper on all monitors to an image
- `swaybg -i path/to/image -m stretch|fit|fill|center|tile|solid_color` : set the image scaling mode (default: `stretch`)
- `swaybg -c rrggbb` : set the wallpaper to a static color
- `swaybg -o HDMI-1 -i path/to/image` : set the wallpaper on a specific monitor
- `swaybg -o HDMI-1 -i path/to/image -o DP-1 -i path/to/image` : set different wallpapers on multiple monitors

## Categories
files, media, containers

## Links
- More information: https://github.com/swaywm/swaybg/blob/master/swaybg.1.scd
- tldr page: https://tldr.inbrowser.app/pages/common/swaybg
