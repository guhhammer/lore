# palmtopnm

Convert a Palm bitmap file to a PNM image.

## Usage
- `palmtopnm path/to/file.palm >path/to/file.pnm` : convert a Palm bitmap to a PNM image
- `palmtopnm -verb path/to/file.palm >path/to/file.pnm` : display information about the input file
- `palmtopnm -r n path/to/file.palm >path/to/file.pnm` : convert the n'th rendition of the image contained in the input file
- `palmtopnm -s path/to/file.palm >path/to/file.pnm` : write a histogram of the colors in the input file to `stdout`
- `palmtopnm -t path/to/file.palm` : output the transparent color of the input image if set

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/palmtopnm.html
- tldr page: https://tldr.inbrowser.app/pages/common/palmtopnm
