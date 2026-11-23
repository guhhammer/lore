# pamtowinicon

Convert a PAM image to a Windows ICO file.

## Usage
- `pamtowinicon path/to/input_file.pam >path/to/output.ico` : convert a PAM image file to an ICO file
- `pamtowinicon -pn t path/to/input_file.pam >path/to/output.ico` : encode images with resolutions smaller than `t` in the BMP format and all other images in the PNG format
- `pamtowinicon -t path/to/input_file.pam >path/to/output.ico` : make all pixels outside the non-opaque area black

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pamtowinicon.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamtowinicon
