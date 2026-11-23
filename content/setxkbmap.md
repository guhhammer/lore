# setxkbmap

Set the keyboard using the X Keyboard Extension.

## Usage
- `setxkbmap fr` : set the keyboard in French AZERTY
- `setxkbmap -layout us,de -variant ,qwerty -option 'grp:alt_caps_toggle'` : set multiple keyboard layouts, their variants, and switching option
- `localectl list-x11-keymap-layouts` : list all layouts
- `localectl list-x11-keymap-variants de` : list variants for the layout
- `localectl list-x11-keymap-options | grep grp:` : list available switching options
- `setxkbmap -help` : display help

## Categories
hardware, desktop

## Links
- More information: https://manned.org/setxkbmap
- tldr page: https://tldr.inbrowser.app/pages/linux/setxkbmap
