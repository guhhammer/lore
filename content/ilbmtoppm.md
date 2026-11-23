# ilbmtoppm

Convert an ILBM file to a PPM image.

## Usage
- `ilbmtoppm path/to/file.ilbm >path/to/file.ppm` : convert an ILBM file to a PPM image
- `ilbmtoppm -t color path/to/file.ilbm >path/to/file.ppm` : use the specified color to "show through" where the image is transparent
- `ilbmtoppm -ig chunk_id path/to/file.ilbm >path/to/file.ppm` : ignore the chunk with the specified chunk ID
- `ilbmtoppm -m path/to/maskfile.pbm path/to/file.ilbm >path/to/file.ppm` : store the input's transparency information to the specified PBM file

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/ilbmtoppm.html
- tldr page: https://tldr.inbrowser.app/pages/common/ilbmtoppm
