# i3-nagbar

Display an error/warning in a bar on top of a screen.

## Usage
- `i3-nagbar -m "error_message"` : display an error
- `i3-nagbar -t warning -m "warning_message"` : display a warning
- `i3-nagbar -f "pango:monospace bold 9" -m "error_message"` : use the specified font
- `i3-nagbar -b "button_text" command -m "error_message"` : create a button and run a command in a terminal on click
- `i3-nagbar -B "button_text" command -m "error_message"` : create a button and run a command on click
- `i3-nagbar -pm "error_message"` : always open `i3-nagbar` on the primary monitor (default: focused monitor)

## Categories
desktop

## Links
- More information: https://manned.org/i3-nagbar
- tldr page: https://tldr.inbrowser.app/pages/common/i3-nagbar
