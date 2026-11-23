# pamtotiff

Convert a PAM image to a TIFF file.

## Usage
- `pamtotiff path/to/input_file.pam >path/to/output_file.tiff` : convert a PAM image to a TIFF image
- `pamtotiff -none|packbits|lzw|g3|g4|flate|adobeflate path/to/input_file.pam >path/to/output_file.tiff` : explicitly specify a compression method for the output file
- `pamtotiff -c path/to/input_file.pam >path/to/output_file.tiff` : always produce a color TIFF image, even if the input image is greyscale

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/pamtotiff.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamtotiff
