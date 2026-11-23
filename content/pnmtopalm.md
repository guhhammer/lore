# pnmtopalm

Convert a PNM image to a Palm bitmap.

## Usage
- `pnmtopalm path/to/file.pnm >path/to/file.palm` : convert a PNM image to a Palm bitmap
- `pnmtopalm -dep 1|2|4|8|16 path/to/file.pnm >path/to/file.palm` : specify the color depth of the resulting bitmap
- `pnmtopalm -scanline_compression|rle_compression|packbits_compression path/to/file.pnm >path/to/file.palm` : choose a compression method for the resulting bitmap
- `pnmtopalm -c path/to/file.pnm >path/to/file.palm` : build a custom colormap and include it in the resulting bitmap
- `pnmtopalm -den 72|108|144|216|288 path/to/file.pnm >path/to/file.palm` : specify the bitmap's density

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmtopalm.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmtopalm
