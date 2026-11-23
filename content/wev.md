# wev

Print contents of Wayland events.

## Usage
- `wev` : monitor all occurring Wayland events
- `wev -f wl_keyboard` : print all events received by a specific Wayland interface
- `wev -f wl_keyboard:key` : print only specific events received by a Wayland interface
- `wev -F wl_keyboard:key` : print everything but the specified wayland events
- `wev -M path/to/file` : write the `wl_keyboards`'s keymap to a file
- `wev -g` : print Wayland globals

## Categories
desktop, documents, network

## Links
- More information: https://manned.org/wev
- tldr page: https://tldr.inbrowser.app/pages/linux/wev
