# tifftopnm

Convert a TIFF image to a PNM image.

## Usage
- `tifftopnm path/to/input_file.tiff >path/to/output_file.pnm` : convert a TIFF to a PNM file
- `tifftopnm -a path/to/alpha_file.pgm path/to/input_file.tiff >path/to/output_file.pnm` : create a PGM file containing the alpha channel of the input image
- `tifftopnm -r path/to/input_file.tiff >path/to/output_file.pnm` : respect the `fillorder` tag in the input TIFF image
- `tifftopnm -h path/to/input_file.tiff >path/to/output_file.pnm` : print TIFF header information to `stderr`

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/tifftopnm.html
- tldr page: https://tldr.inbrowser.app/pages/common/tifftopnm
