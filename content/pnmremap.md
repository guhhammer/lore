# pnmremap

Replace the colors in a PNM image.

## Usage
- `pnmremap -ma path/to/palette_file.ppm path/to/input.pnm >path/to/output.pnm` : replace the colors in an image with those in the specified color palette
- `pnmremap -ma path/to/palette_file.ppm -fs path/to/input.pnm >path/to/output.pnm` : use Floyd-Steinberg dithering for representing colors missing in the color palette
- `pnmremap -ma path/to/palette_file.ppm -fi path/to/input.pnm >path/to/output.pnm` : use the first color in the palette for representing colors missing in the color palette
- `pnmremap -ma path/to/palette_file.ppm -m color path/to/input.pnm >path/to/output.pnm` : use the specified color for representing colors missing in the color palette

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmremap.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmremap
