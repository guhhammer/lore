# swaylock

Screen locking utility for Wayland compositors.

## Usage
- `swaylock` : lock the screen using the config in `$HOME/.swaylock/config` or `$XDG_CONFIG_HOME/swaylock/config`
- `swaylock -c 0000ff` : lock the screen with a simple color background (`rrggbb` format)
- `swaylock -i path/to/image` : lock the screen with a background image
- `swaylock -u` : lock the screen and disable the unlock indicator (removes feedback on keypress)
- `swaylock -f` : detach from the controlling terminal after locking (like `i3lock`)
- `swaylock -i path/to/image -t` : lock the screen with a background image tiled over all monitors
- `swaylock -F` : lock the screen and show the number of failed login attempts
- `swaylock -C path/to/config` : load the configuration from a specific file

## Categories
desktop, media, containers

## Links
- More information: https://manned.org/swaylock
- tldr page: https://tldr.inbrowser.app/pages/common/swaylock
