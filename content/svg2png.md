# svg2png

Render an SVG image to a PNG image using cairo.

## Usage
- `svg2png path/to/file.svg path/to/output.png` : convert an SVG file to PNG
- `svg2png -w 800 path/to/file.svg path/to/output.png` : convert an SVG file to PNG with a specific width (preserving aspect ratio)
- `svg2png -h 600 path/to/file.svg path/to/output.png` : convert an SVG file to PNG with a specific height (preserving aspect ratio)
- `svg2png -w 800 -h 600 path/to/file.svg path/to/output.png` : convert an SVG file to PNG with both width and height (image centered in space)
- `svg2png -s 2.0 path/to/file.svg path/to/output.png` : convert an SVG file to PNG scaled by a factor
- `cat path/to/file.svg | svg2png - - >path/to/output.png` : convert an SVG from `stdin` to PNG on `stdout`
- `svg2png --flipx path/to/file.svg path/to/output.png` : flip the output image horizontally or vertically

## Categories
files, containers, media

## Links
- More information: https://cairographics.org/
- tldr page: https://tldr.inbrowser.app/pages/common/svg2png
