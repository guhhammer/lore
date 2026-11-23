# pbzip2

A parallel implementation of the `bzip2` file compressor.

## Usage
- `pbzip2 path/to/file` : compress a file
- `pbzip2 -p4 path/to/file` : compress a file using the specified number of processors
- `tar -cf path/to/compressed_file.tar.bz2 -I "pbzip2 -option1 -option2 ..." path/to/file` : compress in combination with `tar` (options can be passed to `pbzip2`)
- `pbzip2 -d path/to/compressed_file.bz2` : decompress a file
- `pbzip2 -h` : display help

## See also
- bzip2
- tar

## Categories
files, archives

## Links
- More information: https://manned.org/pbzip2
- tldr page: https://tldr.inbrowser.app/pages/common/pbzip2
