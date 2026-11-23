# pnmpad

Add borders to a PNM image.

## Usage
- `pnmpad -l 100 -ri 150 -t 123 -bo 456 path/to/image.pnm >path/to/output.pnm` : add borders of the specified sizes to the image
- `pnmpad -wi 1000 -he 500 path/to/image.pnm >path/to/output.pnm` : pad the image to the specified size
- `pnmpad -wi 1000 -ha 0.7 path/to/image.pnm >path/to/output.pnm` : pad the width of the image to the specified size, controlling the ratio between right and left padding
- `pnmpad -wi 1000 -c red path/to/image.pnm >path/to/output.pnm` : pad the width of the image using the specified color

## See also
- pnmmargin
- pamcut
- pamcomp

## Categories
media, containers, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmpad.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmpad
