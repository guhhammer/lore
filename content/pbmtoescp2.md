# pbmtoescp2

Convert a PBM image to an ESC/P2 printer file.

## Usage
- `pbmtoescp2 path/to/image.pbm >path/to/output.escp2` : convert a PBM image to an ESC/P2 printer file
- `pbmtoescp2 -c 0|1 path/to/image.pbm >path/to/output.escp2` : specify the compression of the output
- `pbmtoescp2 -re 180|360|720 path/to/image.pbm >path/to/output.escp2` : specify the horizontal and vertical resolution of the output in dots per inch
- `pbmtoescp2 -f path/to/image.pbm >path/to/output.escp2` : place a formfeed command at the end of the output

## See also
- pbmtoepson
- escp2topbm

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmtoescp2.html
- tldr page: https://tldr.inbrowser.app/pages/common/pbmtoescp2
