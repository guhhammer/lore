# pnmtorle

Convert a PNM file to an Utah Raster Tools RLE image file.

## Usage
- `pnmtorle path/to/input.pnm >path/to/output.rle` : convert a PNM image to an RLE image
- `pnmtorle -verb path/to/input.pnm >path/to/output.rle` : print PNM header information to `stdout`
- `pnmtorle -a path/to/input.pnm >path/to/output.rle` : include a transparency channel in the output image in which every black pixel is set to fully transparent and every other pixel is set to fully opaque

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmtorle.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmtorle
