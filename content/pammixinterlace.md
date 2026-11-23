# pammixinterlace

Merge each row in an image with its two neighbors.

## Usage
- `pammixinterlace path/to/image.ppm >path/to/output.ppm` : merge each row in an image with its two neighbors
- `pammixinterlace -f linear|fir|ffmpeg path/to/image.ppm >path/to/output.ppm` : use the specified filtering mechanism
- `pammixinterlace -a path/to/image.ppm >path/to/output.ppm` : turn on adaptive filtering mode, i.e., only modify pixels that are obviously part of a comb pattern

## See also
- pamdeinterlace

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pammixinterlace.html
- tldr page: https://tldr.inbrowser.app/pages/common/pammixinterlace
