# grim

Grab images (screenshots) from a Wayland compositor. By default, files are saved to `$GRIM_DEFAULT_DIR` or `$XDG_PICTURES_DIR` if the first variable is unset. If none of the environment variables are set, files are saved to the current working directory.

## Usage
- `grim` : screenshot all outputs to the default path
- `grim -o DP-1 path/to/output.png` : screenshot a specific output to the specified file
- `grim -g "$(slurp)" path/to/output.png` : select a specific region using `slurp` and screenshot it
- `grim - | clipboard_manager` : screenshot and copy the image to the clipboard
- `grim -t png|ppm|jpeg path/to/output_file` : set the output file type (default: `png`)
- `grim -l 0..9 path/to/output.png` : set the PNG compression level (default: 6)
- `grim -t jpeg -q 0..100 path/to/output.jpg` : set the JPEG quality level (default: 80)
- `grim -c` : include cursors in the screenshot

## Categories
files, media, containers

## Links
- More information: https://manned.org/grim
- tldr page: https://tldr.inbrowser.app/pages/linux/grim
