# pamcut

Cut out a rectangular region from a Netpbm image.

## Usage
- `pamcut -cropl value -cropr value -cropt value -cropb value path/to/image.ppm >path/to/output.ppm` : discard the specified number of columns/rows on each side of the image
- `pamcut -l value -ri value path/to/image.ppm >path/to/output.ppm` : keep only the columns between the specified columns (inclusively)
- `pamcut -t value -b value -pad path/to/image.ppm >path/to/output.ppm` : fill missing areas with black pixels if the specified rectangle does not entirely lie within the input image

## See also
- pamdice
- pamcomp

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pamcut.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamcut
