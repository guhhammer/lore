# spottopgm

Convert a SPOT satellite image to PGM format.

## Usage
- `spottopgm path/to/file.spot >path/to/output.pgm` : convert the specified SPOT image to PGM format
- `spottopgm -1|2|3 path/to/file.spot >path/to/output.pgm` : extract the specified color channel
- `spottopgm first_col first_row last_col last_row path/to/file.spot >path/to/output.pgm` : extract the specified rectangle from the input image

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/spottopgm.html
- tldr page: https://tldr.inbrowser.app/pages/common/spottopgm
