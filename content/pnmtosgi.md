# pnmtosgi

Convert a PNM file to an SGI image file.

## Usage
- `pnmtosgi path/to/input.pnm >path/to/output.sgi` : convert a PNM image to an SGI image
- `pnmtosgi -verbatim|rle path/to/input.pnm >path/to/output.sgi` : disable or enable compression
- `pnmtosgi -i string path/to/input.pnm >path/to/output.sgi` : write the specified string into the SGI image header's `imagename` field

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmtosgi.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmtosgi
