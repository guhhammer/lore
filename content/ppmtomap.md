# ppmtomap

This command has been superseded by `pnmcolormap`.

## Usage
- `pnmcolormap n_colors path/to/input.pnm >path/to/output.ppm` : generate an image using only `n_colors` or less colors as close as possible to the input image
- `pnmcolormap -splits n_colors path/to/input.pnm >path/to/output.ppm` : use the splitspread strategy for determining the output colors, possibly producing a better result for images with small details
- `pnmcolormap -so path/to/input.pnm >path/to/output.ppm` : sort the resulting colormap, which is useful for comparing colormaps

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtomap.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmtomap
