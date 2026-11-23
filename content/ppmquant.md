# ppmquant

This command has been replaced by `pnmquant` and `pnmremap`.

## Usage
- `pnmquant n_colors path/to/input.pnm >path/to/output.pnm` : generate an image using only `n_colors` or less colors as close as possible to the input image
- `pnmremap -ma path/to/palette_file.ppm path/to/input.pnm >path/to/output.pnm` : replace the colors in an image with those in the specified color palette
- `pnmremap -ma path/to/palette_file.ppm -fs path/to/input.pnm >path/to/output.pnm` : use Floyd-Steinberg dithering for representing colors missing in the color palette

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmquant.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmquant
