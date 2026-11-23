# pgmcrater

This command has been superseded by `pamcrater`, `pamshadedrelief`, and `pamtopnm`.

## Usage
- `pamcrater -h height -w width >path/to/output.pam` : create an image of cratered terrain with the specified dimensions
- `pamcrater -n n_craters >path/to/output.pam` : create an image containing the specified number of craters
- `pamshadedrelief <path/to/input.pam >path/to/output.pam` : generate a shaded relief image with the input image interpreted as an elevation map
- `pamshadedrelief <path/to/input.pam -g factor >path/to/output.pam` : gamma adjust the image by the specified factor
- `pamtopnm path/to/image.pam >path/to/output.pbm|pgm|ppm` : convert a PAM image to an equivalent PNM image, i.e. a PBM, PGM, or PPM image
- `pamtopnm -v` : display version

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pgmcrater.html
- tldr page: https://tldr.inbrowser.app/pages/common/pgmcrater
