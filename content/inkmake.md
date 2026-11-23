# inkmake

GNU Makefile-style SVG exporting using Inkscape's backend.

## Usage
- `inkmake path/to/Inkfile` : export an SVG file executing the specified Inkfile
- `inkmake -v path/to/Inkfile` : execute an Inkfile and show detailed information
- `inkmake -s path/to/file.svg -o path/to/output_image path/to/Inkfile` : execute an Inkfile, specifying SVG input file(s) and an output file
- `inkmake -i /Applications/Inkscape.app/Contents/Resources/bin/inkscape path/to/Inkfile` : use a custom Inkscape binary as the backend
- `inkmake -h` : display help

## Categories
files

## Links
- More information: https://github.com/wader/inkmake#usage
- tldr page: https://tldr.inbrowser.app/pages/common/inkmake
