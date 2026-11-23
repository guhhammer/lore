# pnmscalefixed

Scale a PNM file quickly with possibly reduced quality.

## Usage
- `pnmscalefixed -w width -h height path/to/input.pnm >path/to/output.pnm` : scale an image such that the result has the specified dimensions
- `pnmscalefixed -w width path/to/input.pnm >path/to/output.pnm` : scale an image such that the result has the specified width, keeping the aspect ratio
- `pnmscalefixed -xsc x_factor -ysc y_factor path/to/input.pnm >path/to/output.pnm` : scale an image such that its width and height is changed by the specified factors

## See also
- pamscale

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmscalefixed.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmscalefixed
