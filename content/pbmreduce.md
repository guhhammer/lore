# pbmreduce

Proportionally reduce a PBM image.

## Usage
- `pbmreduce n path/to/image.pbm >path/to/output.pbm` : reduce the specified image by the specified factor
- `pbmreduce -t n path/to/image.pbm >path/to/output.pbm` : use simple thresholding when reducing
- `pbmreduce -va 0.6 n path/to/image.pbm >path/to/output.pbm` : use the specified threshold for all quantizations

## See also
- pamenlarge
- pamditherbw

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmreduce.html
- tldr page: https://tldr.inbrowser.app/pages/common/pbmreduce
