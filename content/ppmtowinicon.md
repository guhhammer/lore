# ppmtowinicon

This command has been superseded by `pamtowinicon`.

## Usage
- `pamtowinicon path/to/input_file.pam >path/to/output.ico` : convert a PAM image file to an ICO file
- `pamtowinicon -pn t path/to/input_file.pam >path/to/output.ico` : encode images with resolutions smaller than `t` in the BMP format and all other images in the PNG format
- `pamtowinicon -t path/to/input_file.pam >path/to/output.ico` : make all pixels outside the non-opaque area black

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtowinicon.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmtowinicon
