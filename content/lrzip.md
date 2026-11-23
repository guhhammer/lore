# lrzip

A large file compression program.

## Usage
- `lrzip path/to/file` : compress a file with LZMA - slow compression, fast decompression
- `lrzip -b path/to/file` : compress a file with BZIP2 - good middle ground for compression/speed
- `lrzip -z path/to/file` : compress with ZPAQ - extreme compression, but very slow
- `lrzip -l path/to/file` : compress with LZO - light compression, extremely fast decompression
- `lrzip -e path/to/file` : compress a file and password protect/encrypt it
- `lrzip -p 8 path/to/file` : override the number of processor threads to use

## See also
- lrunzip
- lrztar
- lrzuntar

## Categories
files, archives

## Links
- More information: https://manned.org/lrzip
- tldr page: https://tldr.inbrowser.app/pages/linux/lrzip
