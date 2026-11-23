# pnmindex

Build a visual index of multiple PNM images.

## Usage
- `pnmindex path/to/input1.pnm path/to/input2.pnm ... >path/to/output.pnm` : produce an image containing thumbnails of the specified images in a grid
- `pnmindex -s 50 path/to/input1.pnm path/to/input2.pnm ... >path/to/output.pnm` : specify the size of the (quadratic) thumbnails
- `pnmindex -a 10 path/to/input1.pnm path/to/input2.pnm ... >path/to/output.pnm` : specify the number of thumbnails per row
- `pnmindex -c 512 path/to/input1.pnm path/to/input2.pnm ... >path/to/output.pnm` : specify the maximum number of colors in the output

## See also
- pamundice

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmindex.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmindex
