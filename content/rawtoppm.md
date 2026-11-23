# rawtoppm

Convert a raw RGB stream to a PPM image.

## Usage
- `rawtoppm width height path/to/image.raw >path/to/output.ppm` : convert a raw RGB stream to a PPM image
- `rawtoppm width height path/to/image.raw | pamflip -tb >path/to/output.ppm` : convert a raw RGB stream in which the pixels come bottom-first instead of top-first to a PPM image
- `rawtoppm width height -h n path/to/image.raw >path/to/output.ppm` : ignore the first n bytes of the specified file
- `rawtoppm width height -ro m path/to/image.raw >path/to/output.ppm` : ignore the last m bytes of each row in the specified file
- `rawtoppm width height -rgb|rbg|grb|gbr|brg|bgr path/to/image.raw >path/to/output.ppm` : specify the order of color components for each pixel

## Categories
media, files, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/rawtoppm.html
- tldr page: https://tldr.inbrowser.app/pages/common/rawtoppm
