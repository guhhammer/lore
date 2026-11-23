# pnmhisteq

Histogram-equalize a PNM image.

## Usage
- `pnmhisteq path/to/input.pnm >path/to/output.pnm` : increase the contrast of a PNM image using histogram equalization
- `pnmhisteq -g path/to/input.pnm >path/to/output.pnm` : only modify grey pixels
- `pnmhisteq -noblack|white path/to/input.pnm >path/to/output.pnm` : do not include black or white pixels in the histogram equalization

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmhisteq.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmhisteq
