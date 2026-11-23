# pnmquantall

Run `pnmquant` on multiple files at once such that they share a common colormap.

## Usage
- `pnmquantall n_colors path/to/input1.pnm path/to/input2.pnm ...` : run `pnmquant` on multiple files with the specified parameters, overwriting the original files
- `pnmquantall -e extension n_colors path/to/input1.pnm path/to/input2.pnm ...` : save the quantized images to files named the same as the input files, but with the specified extension appended

## See also
- pnmquant

## Categories
files

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmquantall.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmquantall
