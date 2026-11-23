# pamtopng

Convert a PAM image to PNG.

## Usage
- `pamtopng path/to/image.pam >path/to/output.png` : convert the specified PAM image to PNG
- `pamtopng -t color path/to/image.pam >path/to/output.png` : mark the specified color as transparent in the output image
- `pamtopng -te path/to/file.txt path/to/image.pam >path/to/output.png` : include the text in the specified file as tEXt chunks in the output
- `pamtopng -in path/to/image.pam >path/to/output.png` : cause the output file to be interlaced in Adam7 format

## See also
- pnmtopng
- pngtopam

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/pamtopng.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamtopng
