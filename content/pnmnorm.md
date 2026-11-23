# pnmnorm

Normalize the contrast in a PNM image.

## Usage
- `pnmnorm path/to/image.pnm >path/to/output.pnm` : force the brightest pixels to be white, the darkest pixels to be black, and spread out the ones in between linearly
- `pnmnorm -midv n path/to/image.pnm >path/to/output.pnm` : force the brightest pixels to be white, the darkest pixels to be black, and spread out the ones in between quadratically such that pixels with a brightness of `n` become 50 % bright
- `pnmnorm -k path/to/image.pnm >path/to/output.pnm` : keep the pixels' hue, only modify the brightness
- `pnmnorm -luminosity|colorvalue|saturation path/to/image.pnm >path/to/output.pnm` : specify a method to calculate a pixel's brightness

## See also
- pnmhisteq

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmnorm.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmnorm
