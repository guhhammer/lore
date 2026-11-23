# ppmfade

Generate a transition between two PPM images.

## Usage
- `ppmfade -f path/to/image1.ppm -l path/to/image2.ppm -mix|spread|shift|relief|oil|...` : generate a transition between two PPM images (first and last) using the specified effect
- `ppmfade -f path/to/image.ppm -mix|spread|shift|relief|oil|...` : generate a transition starting with the specified image and ending in a solid black image
- `ppmfade -l path/to/image.ppm -mix|spread|shift|relief|oil|...` : generate a transition starting with a solid black image and ending with the specified image
- `ppmfade -f path/to/image1.ppm -l path/to/image2.ppm -mix|spread|shift|relief|oil|... -base base` : store the resulting images in files named `base.NNNN.ppm` where `NNNN` is an increasing number

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmfade.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmfade
