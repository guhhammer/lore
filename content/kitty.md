# kitty

A fast, feature-rich, GPU based terminal emulator.

## Usage
- `kitty` : open a new terminal
- `kitty command` : run a command in a new terminal window
- `kitty -T "title"` : open a terminal with a fixed operating system window title
- `kitty -c NONE` : open a terminal using the built-in defaults
- `kitty +kitten themes` : choose and apply a color theme interactively (updates `kitty.conf`)
- `kitty +kitten icat "path/to/image"` : display an image in a terminal that supports the kitty graphics protocol
- `printf '%s' "text" | kitty +kitten clipboard` : copy text to the clipboard (requires a terminal that allows OSC 52 writes)

## Categories
shell, desktop, hardware

## Links
- More information: https://sw.kovidgoyal.net/kitty/
- tldr page: https://tldr.inbrowser.app/pages/common/kitty
