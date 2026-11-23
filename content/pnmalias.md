# pnmalias

Apply antialiasing onto a PNM image.

## Usage
- `pnmalias path/to/input.pnm >path/to/output.ppm` : perform antialiasing on a PNM image, taking black pixels as background and white pixels as foreground
- `pnmalias -bcolor background_color -fcolor foreground_color path/to/input.pnm >path/to/output.ppm` : explicitly specify the background and foreground color
- `pnmalias -fo path/to/input.pnm >path/to/output.ppm` : apply altialiasing to foreground pixels only
- `pnmalias -ba path/to/input.pnm >path/to/output.ppm` : apply antialiasing to all surrounding pixels of background pixels

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmalias.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmalias
