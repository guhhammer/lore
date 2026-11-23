# hyprpaper

Wallpaper utility for Hyprland with the ability to dynamically change wallpapers. Controlled by the config file `~/.config/hypr/hyprpaper.conf`.

## Usage
- `hyprpaper` : start the `hyprpaper` daemon
- `hyprctl hyprpaper wallpaper "monitor,path/to/image.png"` : change the wallpaper for a specific monitor
- `hyprctl hyprpaper wallpaper ",path/to/image.png,contain|cover|tile|fill"` : change the default wallpaper for all unspecified monitors and set its fit mode

## Categories
files

## Links
- More information: https://wiki.hypr.land/Hypr-Ecosystem/hyprpaper/
- tldr page: https://tldr.inbrowser.app/pages/linux/hyprpaper
