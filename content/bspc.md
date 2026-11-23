# bspc

Configure and control `bspwm`, managing nodes, desktops, monitors, and more.

## Usage
- `bspc monitor -d desktop_name1 desktop_name2` : define two virtual desktops
- `bspc desktop -f number` : focus the given desktop
- `bspc node -c` : close the windows rooted at the selected node
- `bspc node -d number` : send the selected node to the given desktop
- `bspc node -t ~fullscreen` : toggle full screen mode for the selected node
- `bspc config setting_name value` : set the value of a specific setting

## See also
- bspwm

## Categories
desktop

## Links
- More information: https://github.com/baskerville/bspwm/blob/master/doc/bspwm.1.asciidoc
- tldr page: https://tldr.inbrowser.app/pages/linux/bspc
