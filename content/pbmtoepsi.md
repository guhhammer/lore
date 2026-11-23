# pbmtoepsi

Convert a PBM image to an encapsulated PostScript style preview bitmap.

## Usage
- `pbmtoepsi path/to/image.pbm >path/to/output.bmp` : convert a PBM image to an encapsulated PostScript style preview bitmap
- `pbmtoepsi -d 144 path/to/image.pbm >path/to/output.bmp` : produce a quadratic output image with the specified resolution
- `pbmtoepsi -d 72x144 path/to/image.pbm >path/to/output.bmp` : produce an output image with the specified horizontal and vertical resolution
- `pbmtoepsi -b path/to/image.pbm >path/to/output.bmp` : only create a boundary box

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtoepsi.html
- tldr page: https://tldr.inbrowser.app/pages/common/pbmtoepsi
