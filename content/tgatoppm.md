# tgatoppm

Convert a TrueVision Targa file to a Netpbm image.

## Usage
- `tgatoppm path/to/file.tga >path/to/output.ppm` : convert a TrueVision Targa file to a PPM image
- `tgatoppm -h path/to/file.tga >path/to/output.ppm` : dump information from the TGA header to `stdout`
- `tgatoppm -a path/to/transparency_file.pgm path/to/file.tga >path/to/output.ppm` : write the transparency channel values of the input image to the specified file
- `tgatoppm -v` : display version

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/tgatoppm.html
- tldr page: https://tldr.inbrowser.app/pages/common/tgatoppm
