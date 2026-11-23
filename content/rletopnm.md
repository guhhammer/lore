# rletopnm

Convert a Utah Raster Tools RLE image file to a PNM file.

## Usage
- `rletopnm path/to/input.rle >path/to/output.pnm` : convert an RLE image to a PNM file
- `rletopnm --a path/to/alpha_file.pgm path/to/input.rle >path/to/output.pnm` : create a PGM image containing the RLE file's alpha channel
- `rletopnm --verb path/to/input.rle >path/to/output.pnm` : operate in verbose mode and print the contents of the RLE header to `stdout`

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/rletopnm.html
- tldr page: https://tldr.inbrowser.app/pages/common/rletopnm
