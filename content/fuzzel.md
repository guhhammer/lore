# fuzzel

A Wayland-native application launcher and fuzzy finder, inspired by `rofi` and `dmenu`.

## Usage
- `fuzzel` : run applications
- `fuzzel -d` : run `fuzzel` in dmenu mode
- `ls | fuzzel -d` : display a menu of the output of the `ls` command
- `echo -e "red\ngreen\nblue" | fuzzel -d` : display a menu with custom items separated by a new line (`\n`)
- `echo -e "red\ngreen\nblue" | fuzzel -d >color.txt` : let the user choose between multiple items and save the selected one to a file
- `rm -v $HOME/.cache/fuzzel` : reset apps usage count (default cache directory: `$XDG_CACHE_HOME/fuzzel`)
- `fuzzel -o "DP-1"` : launch `fuzzel` on a specific monitor, see `wlr-randr` or `swaymsg --type get_outputs`
- `fuzzel -d -l 0 --placeholder "Type your search" | sed 's/^/\"/g;s/$/\"/g' | xargs firefox --search` : use `fuzzel` to do an online search

## Categories
desktop

## Links
- More information: https://codeberg.org/dnkl/fuzzel
- tldr page: https://tldr.inbrowser.app/pages/linux/fuzzel
