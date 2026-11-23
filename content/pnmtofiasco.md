# pnmtofiasco

Convert a PNM image to a compressed FIASCO file.

## Usage
- `pnmtofiasco path/to/file.pnm >path/to/file.fiasco` : convert a PNM image to a compressed FIASCO file
- `pnmtofiasco -i "img[01-09+1].pnm" >path/to/file.fiasco` : specify the input files through a pattern
- `pnmtofiasco -q quality_level path/to/file.pnm >path/to/file.fiasco` : specify the compression quality
- `pnmtofiasco -f path/to/fiascorc path/to/file.pnm >path/to/file.fiasco` : load the options to be used from the specified configuration file

## Categories
files, containers, media

## Links
- More information: https://netpbm.sourceforge.net/doc/pnmtofiasco.html
- tldr page: https://tldr.inbrowser.app/pages/common/pnmtofiasco
