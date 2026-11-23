# rawtopgm

Convert a raw greyscale image to a PGM image.

## Usage
- `rawtopgm width height path/to/image.raw >path/to/output.pgm` : convert a raw greyscale image to a PGM image
- `rawtopgm path/to/image.raw >path/to/output.pgm` : convert a raw greyscale image to a PGM image, assume the image to be a square
- `rawtopgm width height -bt path/to/image.raw >path/to/output.pgm` : convert a raw greyscale image in which the pixels come bottom-first instead of top-first to a PGM image
- `rawtopgm width height -h n path/to/image.raw >path/to/output.pgm` : ignore the first `n` bytes of the specified file
- `rawtopgm width height -r m path/to/image.raw >path/to/output.pgm` : ignore the last `m` bytes of each row in the specified file
- `rawtopgm width height -m n path/to/image.raw >path/to/output.pgm` : specify the maxval for the grey values in the input to be equal to `n`
- `rawtopgm width height -bpp 1|2 -l path/to/image.raw >path/to/output.pgm` : specify the number of bytes that represent each sample in the input and that the byte-sequence is to be interpreted as little-endian

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/rawtopgm.html
- tldr page: https://tldr.inbrowser.app/pages/common/rawtopgm
