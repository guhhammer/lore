# pamditherbw

Apply dithering to a greyscale image, i.e. turn it into a pattern of black and white pixels that look the same as the original greyscale.

## Usage
- `pamditherbw path/to/image.pgm >path/to/file.pgm` : read a PGM image, apply dithering, and save it to a file
- `pamditherbw -floyd|fs|atkinson|threshold|hilbert|... path/to/image.pgm >path/to/file.pgm` : use the specified quantization method
- `pamditherbw -a -r 1337 path/to/image.pgm >path/to/file.pgm` : use the atkinson quantization method and the specified seed for a pseudo-random number generator
- `pamditherbw -fs|atkinson|thresholding -va 0.3 path/to/image.pgm >path/to/file.pgm` : specify the thresholding value for quantization methods that perform some sort of thresholding

## See also
- pbmreduce

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pamditherbw.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamditherbw
