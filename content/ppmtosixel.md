# ppmtosixel

Convert a PPM image to DEC sixel format.

## Usage
- `ppmtosixel path/to/file.ppm >path/to/file.sixel` : convert a PPM image to DEC sixel format
- `ppmtosixel -r path/to/file.ppm >path/to/file.sixel` : produce an uncompressed SIXEL file that is much slower to print
- `ppmtosixel -m path/to/file.ppm >path/to/file.sixel` : add a left margin of 1.5 inches
- `ppmtosixel -7bit path/to/file.ppm >path/to/file.sixel` : encode control codes in a more portable (although less space-efficient) way

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtosixel.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmtosixel
