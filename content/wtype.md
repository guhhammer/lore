# wtype

Simulate keyboard input on Wayland, similar to `xdotool type` for X11.

## Usage
- `wtype "Hello World"` : simulate typing text
- `wtype -k Left` : type a specific key
- `wtype -M shift|ctrl|...` : press a modifier
- `wtype -m ctrl` : release a modifier
- `wtype -d 500 -- "text"` : wait between keystrokes (in milliseconds)
- `echo "text" | wtype -` : read text from `stdin`

## See also
- ydotool

## Categories
desktop, text, hardware

## Links
- More information: https://manned.org/wtype
- tldr page: https://tldr.inbrowser.app/pages/linux/wtype
