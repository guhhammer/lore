# pamundice

Combine a grid of Netpbm images into one.

## Usage
- `pamundice filename_%1d_%1a.ppm -a grid_width -d grid_height >path/to/output.ppm` : combine the images whose names match the `printf`-style filename expression. Assume a grid with a specific size
- `pamundice filename_%1d_%1a.ppm -a x_value -d y_value -ho value -vo value >path/to/output.ppm` : assume that the tiles overlap horizontally and vertically by the specified amount
- `pamundice -l path/to/file.txt -a x_value -d y_value >path/to/output.ppm` : specify the images to be combined through a text file containing one filename per line

## See also
- pamdice

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pamundice.html
- tldr page: https://tldr.inbrowser.app/pages/common/pamundice
