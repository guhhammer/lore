# ppmtoilbm

Convert a PPM image to an ILBM file.

## Usage
- `ppmtoilbm path/to/file.ppm >path/to/file.ilbm` : convert a PPM image to an ILBM file
- `ppmtoilbm -mp n -hamif|24if|dcif path/to/file.ppm >path/to/file.ilbm` : write a maximum of `n` planes to the ILBM file and produce a HAM/24bit/direct color file if this number is exceeded
- `ppmtoilbm -fp n path/to/file.ppm >path/to/file.ilbm` : produce an ILBM file with exactly `n` planes
- `ppmtoilbm -compress|nocompress|savemem path/to/file.ppm >path/to/file.ilbm` : select the compression method to be used

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/ppmtoilbm.html
- tldr page: https://tldr.inbrowser.app/pages/common/ppmtoilbm
