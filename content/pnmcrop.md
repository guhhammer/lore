# pnmcrop

Crop PNM images.

## Usage
- `pnmcrop -w path/to/image.pnm >path/to/output.pnm` : remove white borders on a PNM image
- `pnmcrop -bg-color color -t -l path/to/image.pnm >path/to/output.pnm` : remove borders of the specified color that are on the top and left side of the image
- `pnmcrop -bg-corner topleft|topright|bottomleft|bottomright path/to/image.pnm >path/to/output.pnm` : determine the color of the borders to be removed by the color of the pixel in the specified corner
- `pnmcrop -m n -blan pass|minimize|maxcrop path/to/image.pnm >path/to/output.pnm` : leave a border with a width of `n` pixels. Additionally, specify the behavior if the image is entirely made out of background

## Categories
media, containers, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmcrop.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmcrop
