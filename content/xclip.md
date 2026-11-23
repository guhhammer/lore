# xclip

X11 clipboard manipulation tool, similar to `xsel`. Handles the X primary and secondary selections, plus the system clipboard (`<Ctrl c>`/`<Ctrl v>`).

## Usage
- `echo 123 | xclip` : copy the output from a command to the X11 primary selection area (clipboard)
- `echo 123 | xclip -se primary|secondary|clipboard` : copy the output from a command to a given X11 selection area
- `echo 123 | xclip -se c` : copy the output from a command to the system clipboard, using short notation
- `xclip -se c path/to/input_file.txt` : copy the contents of a file into the system clipboard
- `xclip -se c -t image/png path/to/input_file.png` : copy the contents of a PNG into the system clipboard (can be pasted in other programs correctly)
- `xclip -i` : copy the user input in the console into the system clipboard
- `xclip -o` : paste the contents of the X11 primary selection area to the console
- `xclip -o -se c` : paste the contents of the system clipboard to the console

## See also
- wl-copy

## Categories
desktop

## Links
- More information: https://manned.org/xclip
- tldr page: https://tldr.inbrowser.app/pages/linux/xclip
