# fiascotopnm

Convert a compressed FIASCO file to a PNM image.

## Usage
- `fiascotopnm path/to/file.fiasco -o output_file_basename` : convert a compressed FIASCO file to a PNM file or in the case of video streams multiple PNM files
- `fiascotopnm -z path/to/file.fiasco -o output_file_basename` : use fast decompression, resulting in a slightly decreased quality of the output file(s)
- `fiascotopnm -f path/to/fiascorc path/to/file.fiasco -o output_file_basename` : load the options to be used from the specified configuration file
- `fiascotopnm -m n path/to/file.fiasco -o output_file_basename` : magnify the decompressed image(s) by a factor of 2^n
- `fiascotopnm -s n path/to/file.fiasco -o output_file_basename` : smooth the decompressed image by the specified amount

## Categories
files, media, containers

## Links
- More information: https://netpbm.sourceforge.net/doc/fiascotopnm.html
- tldr page: https://tldr.inbrowser.app/pages/common/fiascotopnm
