# pstopnm

Convert a PostScript file to a PNM image.

## Usage
- `pstopnm path/to/file.ps` : convert a PS file to PNM images, storing page N of the input to `path/to/fileN.ppm`
- `pstopnm -pbm|pgm|ppm path/to/file.ps` : explicitly specify the output format
- `pstopnm -dpi n path/to/file.ps` : specify the resolution of the output in dots per inch

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pstopnm.html
- tldr page: https://tldr.inbrowser.app/pages/common/pstopnm
