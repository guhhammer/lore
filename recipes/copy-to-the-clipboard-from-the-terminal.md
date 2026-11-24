# copy to the clipboard from the terminal

Send a command's output or a file to the clipboard, and paste from it.

## Steps
- `command | xclip -selection clipboard` : copy, on X11 (xclip)
- `xclip -selection clipboard -o` : paste, on X11
- `command | wl-copy` : copy, on Wayland (wl-clipboard)
- `wl-paste` : paste, on Wayland
- `command | pbcopy` : copy, on macOS
- `pbpaste` : paste, on macOS

## Notes
- Over ssh, your terminal can take copies itself: lore's c key does it that way.

## See also
- xclip
- wl-copy
- pbcopy

## Categories
shell
