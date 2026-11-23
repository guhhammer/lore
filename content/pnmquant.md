# pnmquant

Quantize the colors in a PNM image into a smaller set. This command is a combination of `pnmcolormap` and `pnmremap` and accepts the union of their options, except `-mapfile`.

## Usage
- `pnmquant n_colors path/to/input.pnm >path/to/output.pnm` : generate an image using only `n_colors` or less colors as close as possible to the input image

## See also
- pnmquantall

## Categories
containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmquant.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmquant
