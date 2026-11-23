# i3-scrot

Wrapper script around the screenshot utility `scrot` for the i3 window manager. The default save location is `~/Pictures` and can be changed in `~/.config/i3-scrot.conf`.

## Usage
- `i3-scrot` : capture a screenshot of the whole screen and save it to the default directory
- `i3-scrot --window` : capture a screenshot of the active window
- `i3-scrot --select` : capture a screenshot of a specific rectangular selection
- `i3-scrot --desk-to-clipboard` : capture a screenshot of the whole screen and copy it to the clipboard
- `i3-scrot --window-to-clipboard` : capture a screenshot of the active window and copy it to the clipboard
- `i3-scrot --select-to-clipboard` : capture a screenshot of a specific selection and copy it to the clipboard
- `i3-scrot --window 5` : capture a screenshot of the active window after a delay of 5 seconds

## Categories
desktop, media, files

## Links
- More information: https://gitlab.manjaro.org/packages/extra/i3-scrot
- tldr page: https://tldr.inbrowser.app/pages/linux/i3-scrot
