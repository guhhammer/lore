# ppmtomitsu

Convert a PPM image to a Mitsubishi S340-10 file.

## Usage
- `ppmtomitsu path/to/file.ppm >path/to/file.mitsu` : convert a PPM image to a MITSU file
- `ppmtomitsu -e 1|2|3 -s 1|2|3|4 -c n path/to/file.ppm >path/to/file.mitsu` : enlarge the image by the specified factor, use the specified sharpness, and produce `n` copies
- `ppmtomitsu -m A|A4|AS|A4S path/to/file.ppm >path/to/file.mitsu` : use the given medium for the printing process

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtomitsu.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmtomitsu
