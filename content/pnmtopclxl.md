# pnmtopclxl

Convert a PNM file to an HP LaserJet PCL XL printer stream.

## Usage
- `pnmtopclxl path/to/input1.pnm path/to/input2.pnm ... >path/to/output.pclxl` : convert PNM files to an HP LaserJet PCL XL printer stream
- `pnmtopclxl -dpi resolution -x x_offset -y y_offset path/to/input1.pnm path/to/input2.pnm ... >path/to/output.pclxl` : specify the resolution of the image as well as the location of the page from the upper left corner of each image
- `pnmtopclxl -du vertical|horizontal -fo letter|legal|a3|a4|a5|... path/to/input1.pnm path/to/input2.pnm ... >path/to/output.pclxl` : generate a duplex printer stream for the specified paper format

## Categories
files, media, hardware

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmtopclxl.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmtopclxl
