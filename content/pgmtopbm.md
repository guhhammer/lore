# pgmtopbm

This command has been superseded by `pamditherbw`.

## Usage
- `pamditherbw path/to/image.pgm >path/to/file.pgm` : read a PGM image, apply dithering, and save it to a file
- `pamditherbw -floyd|fs|atkinson|threshold|hilbert|... path/to/image.pgm >path/to/file.pgm` : use the specified quantization method
- `pamditherbw -a -r 1337 path/to/image.pgm >path/to/file.pgm` : use the atkinson quantization method and the specified seed for a pseudo-random number generator
- `pamditherbw -fs|atkinson|thresholding -va 0.3 path/to/image.pgm >path/to/file.pgm` : specify the thresholding value for quantization methods that perform some sort of thresholding

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pgmtopbm.html
- tldr page: https://tldr.inbrowser.app/pages/common/pgmtopbm
