# pbmclean

Clean up a PBM image by erasing isolated black and white pixels.

## Usage
- `pbmclean path/to/image.pbm >path/to/output.pbm` : clean up a PBM image by erasing isolated black and white pixels
- `pbmclean -black|white path/to/image.pbm >path/to/output.pbm` : clean up only black/white pixels
- `pbmclean -m 3 path/to/image.pbm >path/to/output.pbm` : specify the minimum number of neighboring pixels of the same color in order for a pixel not to be considered isolated

## Categories
media, containers, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pbmclean.html
- tldr page: https://tldr.inbrowser.app/pages/common/pbmclean
