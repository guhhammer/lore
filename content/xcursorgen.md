# xcursorgen

Create an X cursor file from a collection of PNGs. If `--prefix` is omitted, the image files must be located in the current working directory.

## Usage
- `xcursorgen path/to/config.cursor path/to/output_file` : create an X cursor file using a configuration file
- `xcursorgen --prefix path/to/image_directory/ path/to/config.cursor path/to/output_file` : create an X cursor file using a configuration file and specify the path to the image files
- `xcursorgen path/to/config.cursor` : create an X cursor file using a configuration file and write the output to `stdout`

## Categories
files, containers, media

## Links
- More information: https://manned.org/xcursorgen
- tldr page: https://tldr.inbrowser.app/pages/linux/xcursorgen
