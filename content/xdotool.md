# xdotool

Automate X11 actions.

## Usage
- `xdotool search --onlyvisible --name firefox` : retrieve the X-Windows window ID of the running Firefox window(s)
- `xdotool click 3` : perform a mouse `<RightClick>`
- `xdotool getactivewindow` : get the ID of the currently active window
- `xdotool windowfocus --sync 12345` : focus on the window with a specific ID
- `xdotool type --delay 500 "Hello world"` : type a message, with a 500ms delay for each letter
- `xdotool key KP_Enter` : press the `<Enter>` key

## Categories
desktop

## Links
- More information: https://manned.org/xdotool
- tldr page: https://tldr.inbrowser.app/pages/linux/xdotool
