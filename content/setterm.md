# setterm

Set terminal attributes such as colors, cursor visibility, and screen blanking. Note: Some options only work in virtual consoles (TTY).

## Usage
- `setterm --clear` : clear the screen
- `setterm --cursor on|off` : make the cursor visible or invisible
- `setterm --blink on|off` : turn blinking text on or off
- `setterm --foreground default|black|blue|red|...` : set the text color
- `setterm --background default|black|blue|red|...` : set the background color
- `setterm --blank 0..60` : set the screen blanking timeout in minutes (`0` disables blanking)
- `setterm --reset` : reset the terminal to its default state

## Categories
shell, desktop

## Links
- More information: https://manned.org/setterm
- tldr page: https://tldr.inbrowser.app/pages/linux/setterm
