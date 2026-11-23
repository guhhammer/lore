# pngtopnm

This command has been superseded by `pngtopam`.

## Usage
- `pngtopam path/to/image.png >path/to/output.pam` : convert the specified PNG image to a Netpbm image
- `pngtopam -alphapam path/to/image.png >path/to/output.pam` : create an output image that includes both the main image and transparency mask of the input image
- `pngtopam -m -ba color path/to/image.png >path/to/output.pam` : replace transparent pixels by the specified color
- `pngtopam -te path/to/file.txt path/to/image.png >path/to/output.pam` : write tEXt chunks found in the input image to the specified text file

## Categories
media, files, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/pngtopnm.html
- tldr page: https://tldr.inbrowser.app/pages/common/pngtopnm
