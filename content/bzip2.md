# bzip2

A block-sorting file compressor.

## Usage
- `bzip2 path/to/file_to_compress` : compress a file
- `bzip2 -d path/to/compressed_file.bz2` : decompress a file
- `bzip2 -dc path/to/compressed_file.bz2` : decompress a file to `stdout`
- `bzip2 -t path/to/compressed_file.bz2` : test the integrity of each file inside the archive file
- `bzip2 -v path/to/compressed_files.bz2` : show the compression ratio for each file processed with detailed information
- `bzip2 -f path/to/compressed_file.bz2` : decompress a file overwriting existing files
- `bzip2 -h` : display help

## See also
- bzcat
- bunzip2
- bzip2recover

## Categories
files, archives

## Links
- More information: https://manned.org/bzip2
- tldr page: https://tldr.inbrowser.app/pages/common/bzip2
- Wikipedia: https://en.wikipedia.org/wiki/Bzip2
