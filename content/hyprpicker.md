# hyprpicker

Minimal Wayland color picker for wlroots compositors (e.g., Hyprland). Requires a Wayland session. For clipboard autocopy, `wl-copy` must be installed.

## Usage
- `hyprpicker` : pick a color in the default (hex) format
- `hyprpicker -f hex|rgb|hsl|hsv|cmyk` : pick a color and output in a specific format
- `hyprpicker -a` : copy the picked color to the clipboard
- `hyprpicker -n` : disable colored output (print plain text only)
- `color=$(hyprpicker -f hex)` : store the picked color in a shell variable
- `hyprpicker -h` : display help

## Categories
desktop, media, files

## Links
- More information: https://wiki.hypr.land/Hypr-Ecosystem/hyprpicker/
- tldr page: https://tldr.inbrowser.app/pages/linux/hyprpicker
