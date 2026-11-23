# ppmdither

Reduce the number of colors in an image by applying dithering.

## Usage
- `ppmdither path/to/image.ppm >path/to/file.ppm` : read a PPM image, apply dithering, and save it to a file
- `ppmdither -r 2 -g 3 -b 2 path/to/image.ppm >path/to/file.ppm` : specify the desired number of shades for each primary color
- `ppmdither -d 2 path/to/image.ppm >path/to/file.ppm` : specify the dimensions of the dithering matrix

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmdither.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmdither
