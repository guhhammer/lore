# ppmtoxpm

Convert a PPM image to an X11 version 3 pixmap.

## Usage
- `ppmtoxpm path/to/input_file.ppm >path/to/output_file.xpm` : convert a PPM image to an XPM image
- `ppmtoxpm -n prefix_string path/to/input_file.ppm >path/to/output_file.xpm` : specify the prefix string in the output XPM image
- `ppmtoxpm -h path/to/input_file.ppm >path/to/output_file.xpm` : in the output XPM file, specify colors by their hexadecimal code instead of their name
- `ppmtoxpm -a path/to/alpha_file.pgm path/to/input_file.ppm >path/to/output_file.xpm` : use the specified PGM file as a transparency mask

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtoxpm.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmtoxpm
