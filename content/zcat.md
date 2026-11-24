# zcat

This command is an alias of `gzip --stdout --decompress`.

## Usage
- `gzip path/to/file` : compress a file, replacing it with a `gzip` archive
- `gzip -d path/to/file.gz` : decompress a file, replacing it with the original uncompressed version
- `gzip -v path/to/file` : display the name and reduction percentage for each file compressed
- `gzip -k path/to/file` : compress a file, keeping the original file
- `gzip -c path/to/file >path/to/compressed_file.gz` : compress a file, specifying the output filename
- `gzip -cd path/to/file.gz >path/to/uncompressed_file` : decompress a `gzip` archive specifying the output filename
- `gzip -1..9 -c path/to/file >path/to/compressed_file.gz` : specify the compression level. 1 is the fastest (low compression), 9 is the slowest (high compression), 6 is the default
- `gzip -l path/to/file.txt.gz` : list the contents of a compressed file

## Categories
files, archives, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/zcat
- Wikipedia: https://en.wikipedia.org/wiki/Gzip
