# pnmnlfilt

Apply a non-linear filter onto a PNM image.

## Usage
- `pnmnlfilt 0.0..0.5 radius path/to/image.pnm >path/to/output.pnm` : apply the "alpha trimmed mean" filter with the specified alpha and radius values onto the PNM image
- `pnmnlfilt 1.0..2.0 radius path/to/image.pnm >path/to/output.pnm` : apply the "optimal estimation smoothing" filter with the specified noise threshold and radius onto the PNM image
- `pnmnlfilt -0.9..(-0.1) radius path/to/image.pnm >path/to/output.pnm` : apply the "edge enhancement" filter with the specified alpha and radius onto the PNM image

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmnlfilt.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmnlfilt
