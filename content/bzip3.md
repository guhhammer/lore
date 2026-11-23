# bzip3

An efficient statistical file compressor.

## Usage
- `bzip3 path/to/file_to_compress` : compress a file
- `bzip3 -d path/to/compressed_file.bz3` : decompress a file
- `bzip3 -dc path/to/compressed_file.bz3` : decompress a file to `stdout`
- `bzip3 -t path/to/compressed_file.bz3` : test the integrity of each file inside the archive file
- `bzip3 -v path/to/compressed_files.bz3` : show the compression ratio for each file processed with detailed information
- `bzip3 -d -f path/to/compressed_file.bz3` : decompress a file overwriting existing files
- `bzip3 -h` : display help

## Categories
files, archives

## Links
- More information: https://manned.org/bzip3
- tldr page: https://tldr.inbrowser.app/pages/common/bzip3
