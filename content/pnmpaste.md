# pnmpaste

Paste a PNM image into another PNM image.

## Usage
- `pnmpaste x y path/to/image1.pnm path/to/image2.pnm >path/to/output.pnm` : paste a PNM image into another PNM image at the specified coordinates
- `command | pnmpaste x y path/to/image.pnm >path/to/output.pnm` : paste the image read from `stdin` into the specified image
- `pnmpaste -and|nand|or|nor|xor|xnor x y path/to/image1.pnm path/to/image2.pnm >path/to/output.pnm` : combine the overlapping pixels by the specified boolean operation, where white pixels represent `true` while black pixels represent `false`

## Categories
containers, media, files

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmpaste.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmpaste
