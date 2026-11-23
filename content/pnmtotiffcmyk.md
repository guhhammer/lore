# pnmtotiffcmyk

Convert a PNM image to a CMYK encoded TIFF.

## Usage
- `pnmtotiffcmyk path/to/input_file.pnm >path/to/output_file.tiff` : convert a PNM image to a CMYK encoded TIFF
- `pnmtotiffcmyk -none|packbits|lzw path/to/input_file.pnm >path/to/output_file.tiff` : specify the TIFF compression method
- `pnmtotiffcmyk -msb2lsb|lsb2msb path/to/input_file.pnm >path/to/output_file.tiff` : control the fill order

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmtotiffcmyk.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmtotiffcmyk
