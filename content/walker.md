# walker

A Wayland-native application launcher and runner with pluggable providers.

## Usage
- `walker` : open the launcher
- `walker -m desktopapplications|calc|clipboard|runner|...` : restrict the launcher to a single provider
- `printf "Choice1\nChoice2\nChoice3" | walker -d` : read items from `stdin` and print the selected item to `stdout` (dmenu mode)
- `walker -t theme_name` : open the launcher with a specific theme

## See also
- fuzzel
- wofi
- rofi

## Categories
desktop

## Links
- More information: https://github.com/abenz1267/walker
- tldr page: https://tldr.inbrowser.app/pages/linux/walker
