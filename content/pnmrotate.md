# pnmrotate

Rotate a PNM image.

## Usage
- `pnmrotate angle path/to/input.pnm >path/to/output.pnm` : rotate a PNM image by some angle (measured in degrees, counter-clockwise)
- `pnmrotate -b color angle path/to/input.pnm >path/to/output.pnm` : specify the background color exposed by rotating the input image
- `pnmrotate -n angle path/to/input.pnm >path/to/output.pnm` : disable anti-aliasing, improving performance but decreasing quality

## Categories
media, files, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmrotate.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmrotate
