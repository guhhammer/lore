# pnmcut

This command has been superseded by `pamcut`.

## Usage
- `pamcut -cropl value -cropr value -cropt value -cropb value path/to/image.ppm >path/to/output.ppm` : discard the specified number of columns/rows on each side of the image
- `pamcut -l value -ri value path/to/image.ppm >path/to/output.ppm` : keep only the columns between the specified columns (inclusively)
- `pamcut -t value -b value -pad path/to/image.ppm >path/to/output.ppm` : fill missing areas with black pixels if the specified rectangle does not entirely lie within the input image

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmcut.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmcut
