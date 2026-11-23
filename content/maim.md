# maim

Screenshot utility.

## Usage
- `maim path/to/screenshot.png` : capture a screenshot and save it to the given path
- `maim -s path/to/screenshot.png` : capture a screenshot of the selected region
- `maim -s | xclip -se c -t image/png` : capture a screenshot of the selected region and save it in the clipboard (requires `xclip`)
- `maim -i $(xdotool getactivewindow) path/to/screenshot.png` : capture a screenshot of the current active window (requires `xdotool`)

## Categories
media, files

## Links
- More information: https://manned.org/maim
- tldr page: https://tldr.inbrowser.app/pages/linux/maim
